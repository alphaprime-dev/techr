use crate::indicators::sma::{sma_aligned, sma_dense};

pub fn eom(
    highs: &[f64],
    lows: &[f64],
    volumes: &[f64],
    period: usize,
    signal_period: usize,
    scale: f64,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let eom_line = eom_line(highs, lows, volumes, period, scale);
    let signal = sma_aligned(&eom_line, signal_period);

    (eom_line, signal)
}

pub fn eom_signal(
    highs: &[f64],
    lows: &[f64],
    volumes: &[f64],
    period: usize,
    signal_period: usize,
    scale: f64,
) -> Vec<Option<f64>> {
    let eom_line = eom_line(highs, lows, volumes, period, scale);
    sma_aligned(&eom_line, signal_period)
}

pub fn eom_line(
    highs: &[f64],
    lows: &[f64],
    volumes: &[f64],
    period: usize,
    scale: f64,
) -> Vec<Option<f64>> {
    let len = highs.len();
    if len < 2 || len != lows.len() || len != volumes.len() {
        return vec![None; len];
    }

    let mut eom_values = Vec::with_capacity(len - 1);
    for i in 1..len {
        let high_low_avg = (highs[i] + lows[i]) / 2.0;
        let prev_high_low_avg = (highs[i - 1] + lows[i - 1]) / 2.0;
        let distance_moved = high_low_avg - prev_high_low_avg;

        let high_low_diff = highs[i] - lows[i];
        let box_ratio = if high_low_diff != 0.0 && volumes[i] != 0.0 {
            (volumes[i] / scale) / high_low_diff
        } else {
            0.0
        };

        eom_values.push(distance_moved / box_ratio);
    }

    let mut eom_line = vec![None; len];
    let eom_sma = sma_dense(&eom_values, period);
    for (i, &value) in eom_sma.iter().enumerate() {
        eom_line[i + 1] = value;
    }

    eom_line
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
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v");

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
}
