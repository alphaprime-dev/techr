use crate::indicators::ema::{ema_aligned, ema_dense};

pub fn ppo(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let ppo_line = ppo_line(data, fast_period, slow_period);
    let signal_line = ema_aligned(&ppo_line, signal_period);
    let histogram = ppo_line
        .iter()
        .zip(signal_line.iter())
        .map(|(&ppo, &signal)| match (ppo, signal) {
            (Some(p), Some(s)) => Some(p - s),
            _ => None,
        })
        .collect();

    (ppo_line, signal_line, histogram)
}

pub fn ppo_histogram(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let ppo_line = ppo_line(data, fast_period, slow_period);
    let signal_line = ema_aligned(&ppo_line, signal_period);
    ppo_line
        .iter()
        .zip(signal_line.iter())
        .map(|(&ppo, &signal)| match (ppo, signal) {
            (Some(p), Some(s)) => Some(p - s),
            _ => None,
        })
        .collect()
}

pub fn ppo_signal(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let ppo_line = ppo_line(data, fast_period, slow_period);
    ema_aligned(&ppo_line, signal_period)
}

pub fn ppo_line(data: &[f64], fast_period: usize, slow_period: usize) -> Vec<Option<f64>> {
    let mut ppo_line = vec![None; data.len()];

    if data.len() < slow_period || fast_period >= slow_period {
        return ppo_line;
    }

    let fast_ema = ema_dense(data, fast_period);
    let slow_ema = ema_dense(data, slow_period);

    for i in (slow_period - 1)..data.len() {
        if let (Some(fast), Some(slow)) = (fast_ema[i], slow_ema[i]) {
            if slow != 0.0 {
                ppo_line[i] = Some((fast - slow) * 100.0 / slow);
            }
        }
    }

    ppo_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard PPO output against fixture data.
    #[test]
    fn test_ppo() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let (ppo_line, signal_line, histogram) = ppo(&input, 12, 26, 9);

            let expected_ppo = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ppo_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ppo_signal_{}.json",
                symbol
            ));
            let expected_histogram = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ppo_histogram_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(ppo_line, 8),
                round_vec(expected_ppo, 8),
                "PPO line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal_line, 8),
                round_vec(expected_signal, 8),
                "PPO signal test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(histogram, 8),
                round_vec(expected_histogram, 8),
                "PPO histogram test failed for symbol {}.",
                symbol
            );
        }
    }
}
