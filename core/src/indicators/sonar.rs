use crate::indicators::ema::ema;

pub fn sonar(
    data: &[f64],
    period: usize,
    step: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let mut sonar_line = vec![None; data.len()];
    let mut signal_line = vec![None; data.len()];

    if data.len() < period + step {
        return (sonar_line, signal_line);
    }

    let ema_values = ema(data, period);

    for i in (period + step - 1)..data.len() {
        if let (Some(current_ema), Some(previous_ema)) = (ema_values[i], ema_values[i - step]) {
            sonar_line[i] = Some(current_ema - previous_ema);
        }
    }

    let sonar_values: Vec<f64> = sonar_line.iter().filter_map(|&x| x).collect();
    let signal_ema = ema(&sonar_values, signal_period);
    let offset = data.len() - sonar_values.len();

    for (i, &value) in signal_ema.iter().enumerate() {
        signal_line[i + offset] = value;
    }

    (sonar_line, signal_line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_sonar() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let (sonar_line, signal_line) = sonar(&input, 9, 6, 5);

            let expected_sonar = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sonar_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sonar_signal_{}.json",
                symbol
            ));

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
}
