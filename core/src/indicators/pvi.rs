use crate::indicators::ema::ema_aligned;

pub fn pvi(
    closes: &[f64],
    volumes: &[f64],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let pvi_line = pvi_line(closes, volumes);
    let signal = ema_aligned(&pvi_line, signal_period);

    (pvi_line, signal)
}

pub fn pvi_signal(closes: &[f64], volumes: &[f64], signal_period: usize) -> Vec<Option<f64>> {
    let pvi_line = pvi_line(closes, volumes);
    ema_aligned(&pvi_line, signal_period)
}

pub fn pvi_line(closes: &[f64], volumes: &[f64]) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut pvi_line = vec![None; len];

    if len < 2 || len != volumes.len() {
        return pvi_line;
    }

    let mut pvi_point = 1000.0;
    pvi_line[0] = Some(pvi_point);

    for i in 1..len {
        if volumes[i] > volumes[i - 1] {
            pvi_point += (closes[i] - closes[i - 1]) * 100.0 / closes[i - 1];
        }
        pvi_line[i] = Some(pvi_point);
    }

    pvi_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard PVI outputs against fixture data.
    #[test]
    fn test_pvi() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
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

            // Then
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
