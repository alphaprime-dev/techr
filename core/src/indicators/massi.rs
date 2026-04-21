use crate::indicators::ema::{ema_aligned, ema_dense};

pub fn massi(
    highs: &[f64],
    lows: &[f64],
    period_ema: usize,
    period_sum: usize,
    period_signal: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let mass = massi_line(highs, lows, period_ema, period_sum);
    let signal = ema_aligned(&mass, period_signal);

    (mass, signal)
}

pub fn massi_signal(
    highs: &[f64],
    lows: &[f64],
    period_ema: usize,
    period_sum: usize,
    period_signal: usize,
) -> Vec<Option<f64>> {
    let mass = massi_line(highs, lows, period_ema, period_sum);
    ema_aligned(&mass, period_signal)
}

pub fn massi_line(
    highs: &[f64],
    lows: &[f64],
    period_ema: usize,
    period_sum: usize,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut mass = vec![None; len];

    if len != lows.len() || len < 2 * (period_ema - 1) + (period_sum - 1) + 1 {
        return mass;
    }

    let high_low_diffs: Vec<f64> = highs.iter().zip(lows.iter()).map(|(h, l)| h - l).collect();
    let s_ema = ema_dense(&high_low_diffs, period_ema);
    let offset: usize = period_ema - 1;
    let d_ema = ema_aligned(&s_ema, period_ema);

    let mut ema_ratio = Vec::with_capacity(len.saturating_sub(2 * offset));
    for i in 0..len {
        if let (Some(s), Some(d)) = (s_ema[i], d_ema[i]) {
            ema_ratio.push(s / d);
        }
    }

    let mut ratio_sum = 0.0;
    for i in 0..ema_ratio.len() {
        ratio_sum += ema_ratio[i];
        if i >= period_sum - 1 {
            mass[i + 2 * offset] = Some(ratio_sum);
            ratio_sum -= ema_ratio[i - (period_sum - 1)];
        }
    }

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
}
