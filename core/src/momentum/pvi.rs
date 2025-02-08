use crate::overlap::ema;

pub fn pvi(
    closes: &[f64],
    volumes: &[f64],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = closes.len();
    let mut pvi_line = vec![None; len];
    let mut signal = vec![None; len];

    if len < 2 {
        return (pvi_line, signal);
    }

    let mut pvi_point = 1000.0;
    pvi_line[0] = Some(pvi_point);

    for i in 1..len {
        if volumes[i] > volumes[i - 1] {
            pvi_point += (closes[i] - closes[i - 1]) * 100.0 / closes[i - 1];
        }
        pvi_line[i] = Some(pvi_point);
    }

    let pvi_values: Vec<f64> = pvi_line.iter().filter_map(|&x| x).collect();
    let signal_ema = ema(&pvi_values, signal_period);
    let signal_offset = len - signal_ema.len();
    for (i, &s) in signal_ema.iter().enumerate() {
        signal[i + signal_offset] = s;
    }

    (pvi_line, signal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_pvi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v");

            let (pvi, signal) = pvi(&closes, &volumes, 255);

            let expected_pvi = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pvi_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pvi_signal_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(pvi, 8),
                round_vec(expected_pvi, 8),
                "PVI line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal, 8),
                round_vec(expected_signal, 8),
                "PVI signal test failed for symbol {}.",
                symbol
            );
        }
    }
}
