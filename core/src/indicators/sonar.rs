use crate::indicators::ema::{ema_aligned, ema_dense};

pub fn sonar(
    data: &[f64],
    period: usize,
    step: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let sonar_line = sonar_line(data, period, step);
    let signal_line = ema_aligned(&sonar_line, signal_period);

    (sonar_line, signal_line)
}

pub fn sonar_signal(
    data: &[f64],
    period: usize,
    step: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let sonar_line = sonar_line(data, period, step);
    ema_aligned(&sonar_line, signal_period)
}

pub fn sonar_line(data: &[f64], period: usize, step: usize) -> Vec<Option<f64>> {
    let mut sonar_line = vec![None; data.len()];

    if data.len() < period + step {
        return sonar_line;
    }

    let ema_values = ema_dense(data, period);

    for i in (period + step - 1)..data.len() {
        if let (Some(current_ema), Some(previous_ema)) = (ema_values[i], ema_values[i - step]) {
            sonar_line[i] = Some(current_ema - previous_ema);
        }
    }

    sonar_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard SONAR outputs against fixture data.
    #[test]
    fn test_sonar() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let (sonar_line, signal_line) = sonar(&input, 9, 6, 5);

            let expected_sonar = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sonar_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sonar_signal_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(sonar_line, 8),
                round_vec(expected_sonar, 8),
                "SONAR line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal_line, 8),
                round_vec(expected_signal, 8),
                "SONAR signal test failed for symbol {}.",
                symbol
            );
        }
    }
}
