pub fn cci(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut result = vec![None; len];

    if len != lows.len() || len != closes.len() || len < period || period <= 1 {
        return result;
    }

    let typical_prices: Vec<f64> = highs
        .iter()
        .zip(lows.iter())
        .zip(closes.iter())
        .map(|((h, l), c)| (h + l + c) / 3.0)
        .collect();

    for i in period - 1..len {
        let slice = &typical_prices[i + 1 - period..=i];
        let sma_tp: f64 = slice.iter().sum::<f64>() / period as f64;
        let mean_deviation = slice.iter().map(|&x| (x - sma_tp).abs()).sum::<f64>() / period as f64;

        result[i] = if mean_deviation == 0.0 {
            None
        } else {
            Some((typical_prices[i] - sma_tp) / (0.015 * mean_deviation))
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_cci() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = cci(&high, &low, &close, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/cci_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "CCI test failed for symbol {}.",
                symbol
            );
        }
    }
}
