use crate::indicators::ema::ema;
use crate::utils::rolling_sum_strict;

pub fn massi(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period_ema: usize,
    period_sum: usize,
    period_signal: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let mass = massi_line(highs, lows, period_ema, period_sum);
    let signal = ema(&mass, period_signal);

    (mass, signal)
}

pub fn massi_signal(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period_ema: usize,
    period_sum: usize,
    period_signal: usize,
) -> Vec<Option<f64>> {
    let mass = massi_line(highs, lows, period_ema, period_sum);
    ema(&mass, period_signal)
}

pub fn massi_line(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period_ema: usize,
    period_sum: usize,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut mass = vec![None; len];

    if len != lows.len()
        || period_ema == 0
        || period_sum == 0
        || len < 2 * (period_ema - 1) + (period_sum - 1) + 1
    {
        return mass;
    }

    let high_low_diffs = highs
        .iter()
        .zip(lows.iter())
        .map(|(high, low)| match (high, low) {
            (Some(high), Some(low)) => Some(high - low),
            _ => None,
        })
        .collect::<Vec<_>>();
    let s_ema = ema(&high_low_diffs, period_ema);
    let d_ema = ema(&s_ema, period_ema);
    let ema_ratio = s_ema
        .iter()
        .zip(d_ema.iter())
        .map(|(&single, &double)| match (single, double) {
            (Some(single), Some(double)) if double != 0.0 => Some(single / double),
            _ => None,
        })
        .collect::<Vec<_>>();

    mass = rolling_sum_strict(&ema_ratio, period_sum);
    mass
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard MASSI outputs against fixture data.
    #[test]
    fn test_massi() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let highs = highs.into_iter().map(Some).collect::<Vec<_>>();
            let lows = lows.into_iter().map(Some).collect::<Vec<_>>();

            let (mass, signal) = massi(&highs, &lows, 9, 25, 9);

            let expected_mass = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/massi_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/massi_signal_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(mass, 8),
                round_vec(expected_mass, 8),
                "MASSI mass test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal, 8),
                round_vec(expected_signal, 8),
                "MASSI signal test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_massi_with_gap_invalidates_ratio_window() {
        let highs = vec![
            Some(5.0),
            Some(6.0),
            Some(7.0),
            None,
            Some(8.0),
            Some(9.0),
            Some(10.0),
        ];
        let lows = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            None,
            Some(4.0),
            Some(5.0),
            Some(6.0),
        ];

        let line = massi_line(&highs, &lows, 2, 2);

        assert_eq!(
            line,
            vec![None, None, None, None, None, Some(2.0), Some(2.0)]
        );
    }

    #[test]
    fn test_massi_signal_follows_base_ema_contract_across_gaps() {
        let highs = vec![
            Some(5.0),
            Some(6.0),
            Some(7.0),
            Some(8.0),
            Some(9.0),
            None,
            Some(10.0),
            Some(11.0),
            Some(12.0),
            Some(13.0),
        ];
        let lows = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            Some(5.0),
            None,
            Some(6.0),
            Some(7.0),
            Some(8.0),
            Some(9.0),
        ];

        let (line, signal) = massi(&highs, &lows, 2, 2, 2);

        assert_eq!(
            line,
            vec![
                None,
                None,
                None,
                Some(2.0),
                Some(2.0),
                None,
                None,
                Some(2.0),
                Some(2.0),
                Some(2.0),
            ]
        );
        assert_eq!(
            signal,
            vec![
                None,
                None,
                None,
                None,
                Some(2.0),
                None,
                None,
                Some(2.0),
                Some(2.0),
                Some(2.0),
            ]
        );
        assert_eq!(signal, ema(&line, 2));
    }
}
