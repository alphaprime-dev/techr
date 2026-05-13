use crate::indicators::ema::ema;

pub fn trix(
    data: &[Option<f64>],
    period: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let line = trix_line(data, period);
    let signal = ema(&line, signal_period);

    (line, signal)
}

pub fn trix_line(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let ema_values = ema(data, period);
    let double_ema = ema(&ema_values, period);
    let triple_ema = ema(&double_ema, period);

    triple_ema
        .iter()
        .enumerate()
        .map(|(idx, &current)| {
            if idx == 0 {
                return None;
            }

            match (current, triple_ema[idx - 1]) {
                (Some(current), Some(previous)) if previous != 0.0 => {
                    Some((current - previous) * 100.0 / previous)
                }
                _ => None,
            }
        })
        .collect()
}

pub fn trix_signal(data: &[Option<f64>], period: usize, signal_period: usize) -> Vec<Option<f64>> {
    let line = trix_line(data, period);
    ema(&line, signal_period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::ema::ema;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_trix() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let (line, signal) = trix(&input, 12, 9);

            let expected_line = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/trix_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/trix_signal_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(line, 8),
                round_vec(expected_line, 8),
                "TRIX line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal, 8),
                round_vec(expected_signal, 8),
                "TRIX signal test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_trix_matches_composed_ema_across_gaps() {
        // Given
        let input = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            None,
            Some(5.0),
            Some(6.0),
            Some(7.0),
            Some(8.0),
            Some(9.0),
        ];

        // When
        let line = trix_line(&input, 2);
        let single = ema(&input, 2);
        let double = ema(&single, 2);
        let triple = ema(&double, 2);
        let expected = triple
            .iter()
            .enumerate()
            .map(|(idx, &current)| {
                if idx == 0 {
                    return None;
                }

                match (current, triple[idx - 1]) {
                    (Some(current), Some(previous)) if previous != 0.0 => {
                        Some((current - previous) * 100.0 / previous)
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        // Then
        assert_eq!(round_vec(line, 8), round_vec(expected, 8));
    }

    #[test]
    fn test_trix_signal_follows_base_ema_contract_across_gaps() {
        // Given
        let input = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            None,
            Some(5.0),
            Some(6.0),
            Some(7.0),
            Some(8.0),
            Some(9.0),
        ];

        // When
        let (line, signal) = trix(&input, 2, 2);

        // Then
        assert_eq!(signal, ema(&line, 2));
        assert_eq!(signal, trix_signal(&input, 2, 2));
    }

    #[test]
    fn test_trix_returns_none_when_previous_triple_ema_is_zero() {
        // Given
        let input = vec![Some(0.0), Some(0.0), Some(1.0), Some(2.0)];

        // When
        let line = trix_line(&input, 1);

        // Then
        assert_eq!(line, vec![None, None, None, Some(100.0)]);
    }
}
