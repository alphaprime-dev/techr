use crate::indicators::ema::ema_aligned;

pub fn nvi(
    closes: &[f64],
    volumes: &[f64],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let nvi_line = nvi_line(closes, volumes);
    let signal = ema_aligned(&nvi_line, signal_period);

    (nvi_line, signal)
}

pub fn nvi_signal(closes: &[f64], volumes: &[f64], signal_period: usize) -> Vec<Option<f64>> {
    let nvi_line = nvi_line(closes, volumes);
    ema_aligned(&nvi_line, signal_period)
}

pub fn nvi_line(closes: &[f64], volumes: &[f64]) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut nvi_line = vec![None; len];

    if len < 2 || len != volumes.len() {
        return nvi_line;
    }

    let mut nvi_point = 1000.0;
    nvi_line[0] = Some(nvi_point);

    for i in 1..len {
        if volumes[i] < volumes[i - 1] {
            nvi_point += (closes[i] - closes[i - 1]) * 100.0 / closes[i - 1];
        }
        nvi_line[i] = Some(nvi_point);
    }

    nvi_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard NVI outputs against fixture data.
    #[test]
    fn test_nvi() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
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

            // Then
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
