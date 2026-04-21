use crate::utils::rolling_mean_strict;

fn sma_impl(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    rolling_mean_strict(data, period)
}

pub(crate) fn sma_dense(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let nullable = data.iter().copied().map(Some).collect::<Vec<_>>();
    sma_impl(&nullable, period)
}

/// Computes a simple moving average over an aligned nullable series.
///
/// The returned vector keeps the same length as the input and emits `None`
/// until the first full `period` window has been observed.
pub fn sma(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    sma_impl(data, period)
}

pub(crate) fn sma_aligned(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    sma_impl(data, period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard SMA output against fixture data.
    #[test]
    fn test_sma() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let result = sma(&input, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sma_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(result, 8),
                expected,
                "SMA test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_sma_with_prefix_gap() {
        let aligned = vec![None, None, Some(1.0), Some(2.0), Some(3.0), Some(4.0)];
        let expected = vec![None, None, None, Some(1.5), Some(2.5), Some(3.5)];

        let result = sma(&aligned, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_sma_with_interior_gap_invalidates_window() {
        let aligned = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];
        let expected = vec![None, Some(1.5), None, None, Some(4.5)];

        let result = sma(&aligned, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_sma_full_window_invalidation() {
        let aligned = vec![Some(1.0), None, None, Some(4.0)];
        let expected = vec![None, None, None, None];

        let result = sma(&aligned, 2);

        assert_eq!(result, expected);
    }
}
