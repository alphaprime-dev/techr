use crate::indicators::sma::sma;

pub fn eom(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    volumes: &[Option<f64>],
    period: usize,
    signal_period: usize,
    scale: f64,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let eom_line = eom_line(highs, lows, volumes, period, scale);
    let signal = sma(&eom_line, signal_period);

    (eom_line, signal)
}

pub fn eom_signal(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    volumes: &[Option<f64>],
    period: usize,
    signal_period: usize,
    scale: f64,
) -> Vec<Option<f64>> {
    let eom_line = eom_line(highs, lows, volumes, period, scale);
    sma(&eom_line, signal_period)
}

pub fn eom_line(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    volumes: &[Option<f64>],
    period: usize,
    scale: f64,
) -> Vec<Option<f64>> {
    let len = highs.len();
    if len < 2 || len != lows.len() || len != volumes.len() {
        return vec![None; len];
    }

    let mut eom_values = vec![None; len];
    for i in 1..len {
        let (Some(high), Some(low), Some(prev_high), Some(prev_low), Some(volume)) =
            (highs[i], lows[i], highs[i - 1], lows[i - 1], volumes[i])
        else {
            continue;
        };

        let high_low_avg = (high + low) / 2.0;
        let prev_high_low_avg = (prev_high + prev_low) / 2.0;
        let distance_moved = high_low_avg - prev_high_low_avg;

        let high_low_diff = high - low;
        if high_low_diff == 0.0 || volume == 0.0 {
            continue;
        }

        let box_ratio = (volume / scale) / high_low_diff;
        let eom_point = distance_moved / box_ratio;

        if eom_point.is_finite() {
            eom_values[i] = Some(eom_point);
        }
    }

    sma(&eom_values, period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard EOM outputs against fixture data.
    #[test]
    fn test_eom() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

            let (eom, signal) = eom(&highs, &lows, &volumes, 14, 3, 10000.0);

            let expected_eom = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/eom_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/eom_signal_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(eom, 8),
                round_vec(expected_eom, 8),
                "EOM line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal, 8),
                round_vec(expected_signal, 8),
                "EOM signal test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_eom_gap_invalidates_sma_window_until_full_valid_window_returns() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            Some(14.0),
            None,
            Some(16.0),
            Some(18.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(10.0),
            Some(12.0),
            None,
            Some(14.0),
            Some(16.0),
        ];
        let volumes = vec![
            Some(100.0),
            Some(100.0),
            Some(100.0),
            None,
            Some(100.0),
            Some(100.0),
        ];

        let result = eom_line(&highs, &lows, &volumes, 2, 100.0);

        assert_eq!(result, vec![None, None, Some(4.0), None, None, None]);
    }

    #[test]
    fn test_eom_flat_candles_fail_closed_instead_of_emitting_nan() {
        let highs = vec![Some(10.0); 4];
        let lows = vec![Some(10.0); 4];
        let volumes = vec![Some(1000.0); 4];

        let result = eom_line(&highs, &lows, &volumes, 2, 100.0);

        assert_eq!(result, vec![None, None, None, None]);
    }

    #[test]
    fn test_eom_zero_volume_fails_closed_instead_of_emitting_inf() {
        let highs = vec![Some(10.0), Some(11.0), Some(12.0), Some(13.0)];
        let lows = vec![Some(9.0), Some(10.0), Some(11.0), Some(12.0)];
        let volumes = vec![Some(1000.0), Some(0.0), Some(1000.0), Some(1000.0)];

        let result = eom_line(&highs, &lows, &volumes, 2, 100.0);

        assert_eq!(result, vec![None, None, None, Some(0.1)]);
    }
}
