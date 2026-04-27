use crate::utils::calc_clv;

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

    let mut money_flow_values = vec![0.0; len];
    let mut volume_values = vec![0.0; len];
    let mut valid = vec![false; len];
    let mut sum_money_flow = 0.0;
    let mut sum_volume = 0.0;
    let mut valid_count = 0usize;

    for i in 0..len {
        if let (Some(high), Some(low), Some(close), Some(volume)) =
            (highs[i], lows[i], closes[i], volumes[i])
        {
            let money_flow = calc_clv(high, low, close) * volume;
            money_flow_values[i] = money_flow;
            volume_values[i] = volume;
            valid[i] = true;
            sum_money_flow += money_flow;
            sum_volume += volume;
            valid_count += 1;
        }

        if i >= period {
            sum_money_flow -= money_flow_values[i - period];
            sum_volume -= volume_values[i - period];
            valid_count -= valid[i - period] as usize;
        }

        if valid_count == period {
            cmf[i] = if sum_volume == 0.0 {
                if i == period - 1 {
                    None
                } else {
                    Some(0.0)
                }
            } else {
                Some(sum_money_flow / sum_volume)
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
