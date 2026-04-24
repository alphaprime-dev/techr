use crate::utils::rolling_sum_strict;

pub fn mfi(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
    period: usize,
) -> Vec<Option<f64>> {
    let mut mfi = vec![None; highs.len()];
    let len = highs.len();

    if len != lows.len()
        || len != closes.len()
        || len != volumes.len()
        || len < period
        || period <= 1
    {
        return mfi;
    }

    let typical_prices: Vec<Option<f64>> = highs
        .iter()
        .zip(lows.iter())
        .zip(closes.iter())
        .map(|((&high, &low), &close)| match (high, low, close) {
            (Some(high), Some(low), Some(close)) => Some((high + low + close) / 3.0),
            _ => None,
        })
        .collect();

    let mut positive_money_flow = vec![None; highs.len()];
    let mut negative_money_flow = vec![None; highs.len()];

    for i in 1..highs.len() {
        let (Some(prev_tp), Some(curr_tp), Some(volume)) =
            (typical_prices[i - 1], typical_prices[i], volumes[i])
        else {
            continue;
        };
        let raw_money_flow = curr_tp * volume;

        if curr_tp >= prev_tp {
            positive_money_flow[i] = Some(raw_money_flow);
            negative_money_flow[i] = Some(0.0);
        } else {
            positive_money_flow[i] = Some(0.0);
            negative_money_flow[i] = Some(raw_money_flow);
        }
    }

    let positive_sums = rolling_sum_strict(&positive_money_flow, period);
    let negative_sums = rolling_sum_strict(&negative_money_flow, period);

    for i in 0..highs.len() {
        let (Some(positive_sum), Some(negative_sum)) = (positive_sums[i], negative_sums[i]) else {
            continue;
        };

        mfi[i] = Some(if negative_sum == 0.0 {
            100.0
        } else {
            let money_flow_ratio = positive_sum / negative_sum;
            100.0 - (100.0 / (1.0 + money_flow_ratio))
        });
    }

    mfi
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_mfi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "v");
            let result = mfi(&high, &low, &close, &volume, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/mfi_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "MFI test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_mfi_gap_invalidates_full_money_flow_window() {
        let highs = vec![Some(4.0), Some(5.0), None, Some(8.0), Some(9.0)];
        let lows = vec![Some(2.0), Some(3.0), None, Some(6.0), Some(7.0)];
        let closes = vec![Some(3.0), Some(4.0), None, Some(7.0), Some(8.0)];
        let volumes = vec![Some(10.0), Some(10.0), None, Some(10.0), Some(10.0)];

        let result = mfi(&highs, &lows, &closes, &volumes, 2);

        assert_eq!(result, vec![None, None, None, None, None]);
    }
}
