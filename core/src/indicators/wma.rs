use crate::utils::rolling_weighted_mean_strict;

pub fn wma(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    rolling_weighted_mean_strict(data, period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_wma() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let result = wma(&input, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/wma_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                expected,
                "WMA test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_wma_with_prefix_gap() {
        let aligned = vec![None, Some(1.0), Some(2.0), Some(3.0)];
        let expected = vec![None, None, Some(5.0 / 3.0), Some(8.0 / 3.0)];

        let result = wma(&aligned, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_wma_with_interior_gap_invalidates_window() {
        let aligned = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];
        let expected = vec![None, Some(5.0 / 3.0), None, None, Some(14.0 / 3.0)];

        let result = wma(&aligned, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_wma_full_window_invalidation() {
        let aligned = vec![Some(1.0), None, None, Some(4.0)];
        let expected = vec![None, None, None, None];

        let result = wma(&aligned, 2);

        assert_eq!(result, expected);
    }
}
