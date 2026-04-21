use crate::utils::{calc_clv, rolling_sum_strict};

pub fn cmf(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
    period: usize,
) -> Vec<Option<f64>> {
    let mut cmf = vec![None; highs.len()];
    let len = highs.len();

    if len != lows.len()
        || len != closes.len()
        || len != volumes.len()
        || len < period
        || period <= 1
    {
        return cmf;
    }

    let money_flow_volume = highs
        .iter()
        .zip(lows.iter())
        .zip(closes.iter())
        .zip(volumes.iter())
        .map(
            |(((high, low), close), volume)| match (high, low, close, volume) {
                (Some(high), Some(low), Some(close), Some(volume)) => {
                    Some(calc_clv(*high, *low, *close) * *volume)
                }
                _ => None,
            },
        )
        .collect::<Vec<_>>();
    let volume_sums = rolling_sum_strict(volumes, period);
    let money_flow_sums = rolling_sum_strict(&money_flow_volume, period);

    for i in 0..len {
        if let (Some(sum_money_flow_volume), Some(sum_volume)) =
            (money_flow_sums[i], volume_sums[i])
        {
            cmf[i] = if sum_volume == 0.0 {
                if i == period - 1 {
                    None
                } else {
                    Some(0.0)
                }
            } else {
                Some(sum_money_flow_volume / sum_volume)
            };
        }
    }

    cmf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_cmf() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let result = cmf(&high, &low, &close, &volume, 21);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/cmf_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "CMF test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_cmf_gap_invalidates_window_until_full_window_recovers() {
        let highs = vec![Some(10.0), Some(12.0), None, Some(14.0), Some(16.0)];
        let lows = vec![Some(8.0), Some(10.0), None, Some(12.0), Some(14.0)];
        let closes = vec![Some(9.0), Some(11.0), None, Some(13.0), Some(15.0)];
        let volumes = vec![Some(100.0), Some(100.0), None, Some(100.0), Some(100.0)];

        let result = cmf(&highs, &lows, &closes, &volumes, 2);

        assert_eq!(result, vec![None, Some(0.0), None, None, Some(0.0)]);
    }
}
