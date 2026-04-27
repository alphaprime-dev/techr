pub fn mom(closes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if period == 0 || len < period + 1 {
        return result;
    }

    for i in period..len {
        if let (Some(current), Some(previous)) = (closes[i], closes[i - period]) {
            result[i] = Some(current - previous);
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
    fn test_mom() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let result = mom(&close, 10);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/mom_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "MOM test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_mom_gap_invalidates_until_lagged_value_returns() {
        let closes = vec![Some(1.0), Some(3.0), None, Some(8.0), Some(13.0)];

        let result = mom(&closes, 2);

        assert_eq!(result, vec![None, None, None, Some(5.0), None]);
    }
}
