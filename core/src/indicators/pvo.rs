use crate::indicators::ema::{ema_aligned, ema_dense};

pub fn pvo(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let pvo_line = pvo_line(data, fast_period, slow_period);
    let signal_line = ema_aligned(&pvo_line, signal_period);
    let histogram = pvo_line
        .iter()
        .zip(signal_line.iter())
        .map(|(&pvo, &signal)| match (pvo, signal) {
            (Some(p), Some(s)) => Some(p - s),
            _ => None,
        })
        .collect();

    (pvo_line, signal_line, histogram)
}

pub fn pvo_histogram(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let pvo_line = pvo_line(data, fast_period, slow_period);
    let signal_line = ema_aligned(&pvo_line, signal_period);
    pvo_line
        .iter()
        .zip(signal_line.iter())
        .map(|(&pvo, &signal)| match (pvo, signal) {
            (Some(p), Some(s)) => Some(p - s),
            _ => None,
        })
        .collect()
}

pub fn pvo_signal(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let pvo_line = pvo_line(data, fast_period, slow_period);
    ema_aligned(&pvo_line, signal_period)
}

pub fn pvo_line(data: &[f64], fast_period: usize, slow_period: usize) -> Vec<Option<f64>> {
    let mut pvo_line = vec![None; data.len()];

    if data.len() < slow_period || fast_period >= slow_period {
        return pvo_line;
    }

    let fast_ema = ema_dense(data, fast_period);
    let slow_ema = ema_dense(data, slow_period);

    for i in (slow_period - 1)..data.len() {
        if let (Some(fast), Some(slow)) = (fast_ema[i], slow_ema[i]) {
            if slow != 0.0 {
                pvo_line[i] = Some((fast - slow) * 100.0 / slow);
            }
        }
    }

    pvo_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard PVO output against fixture data.
    #[test]
    fn test_pvo() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "v");
            let (pvo_line, signal_line, histogram) = pvo(&input, 12, 26, 9);

            let expected_pvo = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pvo_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pvo_signal_{}.json",
                symbol
            ));
            let expected_histogram = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pvo_histogram_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(pvo_line, 8),
                round_vec(expected_pvo, 8),
                "PVO line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal_line, 8),
                round_vec(expected_signal, 8),
                "PVO signal test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(histogram, 8),
                round_vec(expected_histogram, 8),
                "PVO histogram test failed for symbol {}.",
                symbol
            );
        }
    }
}
