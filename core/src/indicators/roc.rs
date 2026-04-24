pub fn roc(closes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if period == 0 || len < period + 1 {
        return result;
    }

    for i in period..len {
        if let (Some(current), Some(previous)) = (closes[i], closes[i - period]) {
            result[i] = Some(((current - previous) / previous) * 100.0);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_roc() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let result = roc(&close, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/roc_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "ROC test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_roc_gap_invalidates_until_lagged_value_returns() {
        let closes = vec![Some(2.0), Some(4.0), None, Some(10.0), Some(15.0)];

        let result = round_vec(roc(&closes, 2), 8);

        assert_eq!(
            result,
            round_vec(vec![None, None, None, Some(150.0), None], 8)
        );
    }
}
