use crate::indicators::ema::ema_aligned;

pub fn sonar(
    data: &[Option<f64>],
    period: usize,
    step: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let sonar_line = sonar_line(data, period, step);
    let signal_line = ema_aligned(&sonar_line, signal_period);

    (sonar_line, signal_line)
}

pub fn sonar_signal(
    data: &[Option<f64>],
    period: usize,
    step: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let sonar_line = sonar_line(data, period, step);
    ema_aligned(&sonar_line, signal_period)
}

pub fn sonar_line(data: &[Option<f64>], period: usize, step: usize) -> Vec<Option<f64>> {
    let mut sonar_line = vec![None; data.len()];

    if data.len() < period + step {
        return sonar_line;
    }

    let ema_values = ema_aligned(data, period);

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
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
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

    #[test]
    fn test_sonar_with_gap_requires_valid_current_and_stepped_prior_state() {
        let input = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            None,
            Some(4.0),
            Some(5.0),
            Some(6.0),
        ];

        let line = sonar_line(&input, 2, 2);

        assert_eq!(
            line,
            vec![None, None, None, None, Some(1.0), None, Some(2.0)]
        );
    }

    #[test]
    fn test_sonar_signal_follows_base_ema_contract_across_gaps() {
        let input = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            None,
            Some(5.0),
            Some(6.0),
            Some(7.0),
        ];

        let (line, signal) = sonar(&input, 2, 1, 2);

        assert_eq!(
            line,
            vec![
                None,
                None,
                Some(1.0),
                Some(1.0),
                None,
                None,
                Some(1.0),
                Some(1.0),
            ]
        );
        assert_eq!(
            signal,
            vec![
                None,
                None,
                None,
                Some(1.0),
                None,
                None,
                Some(1.0),
                Some(1.0),
            ]
        );
        assert_eq!(signal, ema_aligned(&line, 2));
    }
}
