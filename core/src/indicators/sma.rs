/// Computes a simple moving average over a dense `f64` series.
///
/// The returned vector keeps the same length as the input and emits `None`
/// until the first full `period` window has been observed.
pub fn sma(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut sma = vec![None; data.len()];
    let mut sum = 0.0;

    if data.len() < period {
        return sma;
    }

    for i in 0..data.len() {
        sum += data[i];
        if i >= period {
            sum -= data[i - period];
        }
        if i >= period - 1 {
            sma[i] = Some(sum / period as f64);
        }
    }

    sma
}

/// Computes an SMA over an aligned optional series.
///
/// The input is expected to contain an optional prefix of `None` values followed
/// by a contiguous run of `Some(f64)` values. The returned vector preserves the
/// original alignment while avoiding the extra compaction/remapping pass that
/// would otherwise be needed before applying `sma`.
pub(crate) fn sma_aligned(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];
    let Some(first_valid_idx) = data.iter().position(|value| value.is_some()) else {
        return result;
    };

    let valid_len = data.len() - first_valid_idx;
    if period == 0 || valid_len < period {
        return result;
    }

    let mut sum = 0.0;
    for value in data.iter().skip(first_valid_idx).take(period) {
        sum += value.expect("initial SMA window must be fully populated");
    }

    let first_signal_idx = first_valid_idx + period - 1;
    result[first_signal_idx] = Some(sum / period as f64);

    for idx in (first_signal_idx + 1)..data.len() {
        let entering =
            data[idx].expect("aligned SMA input must be contiguous after the first value");
        let leaving =
            data[idx - period].expect("aligned SMA input must be contiguous after the first value");
        sum += entering - leaving;
        result[idx] = Some(sum / period as f64);
    }

    result
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
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
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

    /// Verifies that aligned SMA preserves offsets while matching dense SMA values.
    #[test]
    fn test_sma_aligned() {
        // Given
        let aligned = vec![None, None, Some(1.0), Some(2.0), Some(3.0), Some(4.0)];
        let expected = vec![None, None, None, Some(1.5), Some(2.5), Some(3.5)];

        // When
        let result = sma_aligned(&aligned, 2);

        // Then
        assert_eq!(result, expected);
    }
}
