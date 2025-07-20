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
    let mut signal_line: Vec<Option<f64>> = vec![None; macd_line.len()];
    let null_count = macd_line.iter().take_while(|&&x| x.is_none()).count();
    let macd_values: Vec<f64> = macd_line
        .iter()
        .skip(null_count)
        .filter_map(|&x| x)
        .collect();
    let ema_values = ema(&macd_values, signal_period);

    for i in 0..ema_values.len() {
        if let Some(ema_value) = ema_values[i] {
            signal_line[i + null_count] = Some(ema_value);
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
