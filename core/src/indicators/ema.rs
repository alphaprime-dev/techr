pub(crate) fn ema_dense(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let nullable = data.iter().copied().map(Some).collect::<Vec<_>>();
    ema(&nullable, period)
}

/// Computes an exponential moving average over an aligned nullable series.
///
/// The returned vector keeps the same length as the input and emits `None`
/// until the first contiguous run of `period` valid observations has been
/// observed. Once seeded, gaps emit `None` without resetting the EMA state.
pub fn ema(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];

    if data.len() < period || period == 0 {
        return result;
    }

    let alpha = 2.0 / (period as f64 + 1.0);
    let mut seeded_count = 0usize;
    let mut seed_sum = 0.0;
    let mut ema = None;

    for (idx, item) in data.iter().enumerate() {
        let Some(value) = *item else {
            if ema.is_none() {
                seeded_count = 0;
                seed_sum = 0.0;
            }
            continue;
        };

        if let Some(current_ema) = ema {
            let next_ema = alpha * value + (1.0 - alpha) * current_ema;
            ema = Some(next_ema);
            result[idx] = Some(next_ema);
            continue;
        }

        seed_sum += value;
        seeded_count += 1;
        if seeded_count == period {
            let initial_ema = seed_sum / period as f64;
            ema = Some(initial_ema);
            result[idx] = Some(initial_ema);
        }
    }

    result
}

pub(crate) use ema as ema_aligned;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard EMA output against fixture data.
    #[test]
    fn test_ema() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let input = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let result = ema(&input, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ema_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "EMA test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_ema_with_prefix_gap() {
        let aligned = vec![None, None, Some(1.0), Some(2.0), Some(3.0), Some(4.0)];
        let expected = vec![None, None, None, Some(1.5), Some(2.5), Some(3.5)];

        let result = ema(&aligned, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_with_interior_gaps_resumes_from_prior_state() {
        let aligned = vec![None, None, Some(1.0), Some(2.0), None, Some(3.0), Some(4.0)];
        let expected = vec![None, None, None, Some(1.5), None, Some(2.5), Some(3.5)];

        let result = ema(&aligned, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_requires_contiguous_values_before_seed() {
        let aligned = vec![
            Some(1.0),
            Some(2.0),
            None,
            Some(3.0),
            Some(4.0),
            Some(5.0),
            Some(6.0),
        ];
        let expected = vec![None, None, None, None, None, Some(4.0), Some(5.0)];

        let result = ema(&aligned, 3);

        assert_eq!(result, expected);
    }

}
