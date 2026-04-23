pub fn mom(closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len < period + 1 {
        return result;
    }

    for i in period..len {
        result[i] = Some(closes[i] - closes[i - period]);
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
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
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
}
