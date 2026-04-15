use crate::indicators::ema::ema;

pub fn macd(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let macd_line = calc_macd_line(data, fast_period, slow_period);
    let signal_line = calc_macd_signal(&macd_line, signal_period);

    // Calculate the histogram
    let histogram = macd_line
        .iter()
        .zip(signal_line.iter())
        .map(|(&macd, &signal)| match (macd, signal) {
            (Some(m), Some(s)) => Some(m - s),
            _ => None,
        })
        .collect();

    (macd_line, signal_line, histogram)
}

pub fn macd_line(data: &[f64], fast_period: usize, slow_period: usize) -> Vec<Option<f64>> {
    calc_macd_line(data, fast_period, slow_period)
}

pub fn macd_signal(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let macd_line = calc_macd_line(data, fast_period, slow_period);
    calc_macd_signal(&macd_line, signal_period)
}

pub fn macd_histogram(
    data: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Vec<Option<f64>> {
    let macd_line = calc_macd_line(data, fast_period, slow_period);
    let signal_line = calc_macd_signal(&macd_line, signal_period);

    macd_line
        .iter()
        .zip(signal_line.iter())
        .map(|(&macd, &signal)| match (macd, signal) {
            (Some(m), Some(s)) => Some(m - s),
            _ => None,
        })
        .collect()
}

fn calc_macd_line(data: &[f64], fast_period: usize, slow_period: usize) -> Vec<Option<f64>> {
    let mut macd_line = vec![None; data.len()];

    if data.len() < slow_period || fast_period >= slow_period {
        return macd_line;
    }

    let fast_ema = ema(data, fast_period);
    let slow_ema = ema(data, slow_period);

    for i in (slow_period - 1)..data.len() {
        if let (Some(fast), Some(slow)) = (fast_ema[i], slow_ema[i]) {
            macd_line[i] = Some(fast - slow);
        }
    }

    macd_line
}

fn calc_macd_signal(macd_line: &[Option<f64>], signal_period: usize) -> Vec<Option<f64>> {
    let mut signal_line = vec![None; macd_line.len()];
    let Some(first_valid_idx) = macd_line.iter().position(|value| value.is_some()) else {
        return signal_line;
    };

    let valid_len = macd_line.len() - first_valid_idx;
    if signal_period == 0 || valid_len < signal_period {
        return signal_line;
    }

    let first_signal_idx = first_valid_idx + signal_period - 1;
    let mut signal = mean_macd_window(macd_line, first_valid_idx, signal_period);
    let alpha = 2.0 / (signal_period as f64 + 1.0);

    signal_line[first_signal_idx] = Some(signal);

    for (idx, value) in macd_line.iter().enumerate().skip(first_signal_idx + 1) {
        let macd = value.expect("macd_line becomes contiguous after the first valid value");
        signal = alpha * macd + (1.0 - alpha) * signal;
        signal_line[idx] = Some(signal);
    }

    signal_line
}

fn mean_macd_window(macd_line: &[Option<f64>], start_idx: usize, period: usize) -> f64 {
    macd_line[start_idx..start_idx + period]
        .iter()
        .map(|value| value.expect("initial signal window must be fully populated"))
        .sum::<f64>()
        / period as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_macd() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let (macd_line, signal_line, histogram) = macd(&input, 12, 26, 9);

            let expected_macd = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/macd_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/macd_signal_{}.json",
                symbol
            ));
            let expected_histogram = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/macd_histogram_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(macd_line, 8),
                round_vec(expected_macd, 8),
                "MACD line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal_line, 8),
                round_vec(expected_signal, 8),
                "MACD signal test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(histogram, 8),
                round_vec(expected_histogram, 8),
                "MACD histogram test failed for symbol {}.",
                symbol
            );
        }
    }
}
