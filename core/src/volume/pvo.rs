use crate::overlap::ema;

pub fn pvo(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let pvo_line = calc_pvo_line(data, fast_period, slow_period);
    let signal_line = calc_pvo_signal(&pvo_line, signal_period);

    // Calculate the histogram
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

fn calc_pvo_line(data: &[f64], fast_period: usize, slow_period: usize) -> Vec<Option<f64>> {
    let mut pvo_line = vec![None; data.len()];

    if data.len() < slow_period || fast_period >= slow_period {
        return pvo_line;
    }

    let fast_ema = ema(data, fast_period);
    let slow_ema = ema(data, slow_period);

    for i in (slow_period - 1)..data.len() {
        if let (Some(fast), Some(slow)) = (fast_ema[i], slow_ema[i]) {
            if slow != 0.0 {
                pvo_line[i] = Some((fast - slow) * 100.0 / slow);
            }
        }
    }

    pvo_line
}

fn calc_pvo_signal(pvo_line: &[Option<f64>], signal_period: usize) -> Vec<Option<f64>> {
    let mut signal_line: Vec<Option<f64>> = vec![None; pvo_line.len()];
    let pvo_values: Vec<f64> = pvo_line.iter().filter_map(|&x| x).collect();
    let offset = pvo_line.len() - pvo_values.len();

    let ema_values = ema(&pvo_values, signal_period);

    for i in 0..ema_values.len() {
        if let Some(ema_value) = ema_values[i] {
            signal_line[i + offset] = Some(ema_value);
        }
    }

    signal_line
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_pvo() {
        let test_cases = vec!["005930", "TSLA"];
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
