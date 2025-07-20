pub fn roc(closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len < period + 1 {
        return result;
    }

    for i in period..len {
        let curr_close = closes[i];
        let prev_close = closes[i - period];
        result[i] = Some(((curr_close - prev_close) / prev_close) * 100.0);
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
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
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
}
