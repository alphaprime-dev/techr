pub fn ema(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];

    if data.len() < period {
        return result;
    }

    let alpha = 2.0 / (period as f64 + 1.0);
    let mut ema = data[..period].iter().sum::<f64>() / period as f64;

    result[period - 1] = Some(ema);

    for i in period..data.len() {
        ema = alpha * data[i] + (1.0 - alpha) * ema;
        result[i] = Some(ema);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_ema() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = ema(&input, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ema_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "EMA test failed for symbol {}.",
                symbol
            );
        }
    }
}
