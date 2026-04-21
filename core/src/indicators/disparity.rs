use crate::indicators::sma::sma;

pub fn disparity(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = data.len();
    let mut result = vec![None; len];

    if len < period || period == 0 {
        return result;
    }

    let sma = sma(data, period);

    for i in period - 1..len {
        let Some(value) = data[i] else {
            continue;
        };

        if let Some(sma_value) = sma[i] {
            if sma_value != 0.0 {
                result[i] = Some((value / sma_value) * 100.0);
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
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
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

    #[test]
    fn test_disparity_with_interior_gap_invalidates_window() {
        let input = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];
        let expected = vec![
            None,
            Some(133.33333333333331),
            None,
            None,
            Some(111.11111111111111),
        ];

        let result = disparity(&input, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_disparity_full_window_invalidation() {
        let input = vec![None, Some(2.0), None, Some(4.0)];

        let result = disparity(&input, 2);

        assert_eq!(result, vec![None, None, None, None]);
    }
}
