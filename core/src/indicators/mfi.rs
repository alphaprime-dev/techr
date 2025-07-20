pub fn mfi(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    volumes: &[f64],
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

    let typical_prices: Vec<f64> = highs
        .iter()
        .zip(lows.iter())
        .zip(closes.iter())
        .map(|((h, l), c)| (h + l + c) / 3.0)
        .collect();

    let mut positive_money_flow = vec![0.0; highs.len()];
    let mut negative_money_flow = vec![0.0; highs.len()];

    for i in 1..highs.len() {
        let prev_tp = typical_prices[i - 1];
        let curr_tp = typical_prices[i];
        let raw_money_flow = curr_tp * volumes[i];

        if curr_tp >= prev_tp {
            positive_money_flow[i] = raw_money_flow;
            negative_money_flow[i] = 0.0;
        } else {
            positive_money_flow[i] = 0.0;
            negative_money_flow[i] = raw_money_flow;
        }

        if i >= period {
            let positive_sum = positive_money_flow[i - period + 1..=i].iter().sum::<f64>();
            let negative_sum = negative_money_flow[i - period + 1..=i].iter().sum::<f64>();

            let mfi_point = if negative_sum == 0.0 {
                100.0
            } else {
                let money_flow_ratio = positive_sum / negative_sum;
                100.0 - (100.0 / (1.0 + money_flow_ratio))
            };

            mfi[i] = Some(mfi_point);
        }
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
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v");
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
}
