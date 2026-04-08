use crate::indicators::sma::sma;

pub fn disparity(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = data.len();
    let mut result = vec![None; len];

    if len < period {
        return result;
    }

    let sma = sma(data, period);

    for i in period - 1..len {
        if let Some(sma_value) = sma[i] {
            if sma_value != 0.0 {
                result[i] = Some((data[i] / sma_value) * 100.0);
            }
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
    fn test_disparity() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = disparity(&close, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/disparity_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "Disparity test failed for symbol {}.",
                symbol
            );
        }
    }
}
