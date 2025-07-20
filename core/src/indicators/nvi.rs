use crate::indicators::ema::ema;

pub fn nvi(
    closes: &[f64],
    volumes: &[f64],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = closes.len();
    let mut nvi_line = vec![None; len];
    let mut signal = vec![None; len];

    if len < 2 {
        return (nvi_line, signal);
    }

    let mut nvi_point = 1000.0;
    nvi_line[0] = Some(nvi_point);

    for i in 1..len {
        if volumes[i] < volumes[i - 1] {
            nvi_point += (closes[i] - closes[i - 1]) * 100.0 / closes[i - 1];
        }
        nvi_line[i] = Some(nvi_point);
    }

    let nvi_values: Vec<f64> = nvi_line.iter().filter_map(|&x| x).collect();
    let signal_ema = ema(&nvi_values, signal_period);
    let signal_offset = len - signal_ema.len();
    for (i, &s) in signal_ema.iter().enumerate() {
        signal[i + signal_offset] = s;
    }

    (nvi_line, signal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_nvi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v");

            let (nvi, signal) = nvi(&closes, &volumes, 255);

            let expected_nvi = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/nvi_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/nvi_signal_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(nvi, 8),
                round_vec(expected_nvi, 8),
                "NVI line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal, 8),
                round_vec(expected_signal, 8),
                "NVI signal test failed for symbol {}.",
                symbol
            );
        }
    }
}
