use crate::indicators::ema::ema_aligned;

pub fn obv(
    data: &[f64],
    volumes: &[f64],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let obv_line = obv_line(data, volumes);
    let obv_signal = ema_aligned(&obv_line, signal_period);

    (obv_line, obv_signal)
}

pub fn obv_signal(data: &[f64], volumes: &[f64], signal_period: usize) -> Vec<Option<f64>> {
    let obv_line = obv_line(data, volumes);
    ema_aligned(&obv_line, signal_period)
}

pub fn obv_line(data: &[f64], volumes: &[f64]) -> Vec<Option<f64>> {
    let mut obv = vec![None; data.len()];

    let len = data.len();

    if len == 0 || len != volumes.len() {
        return obv;
    }

    let mut obv_point = volumes[0];
    obv[0] = Some(obv_point);

    for i in 1..len {
        let current_close = data[i];
        let prev_close = data[i - 1];
        let volume = volumes[i];

        let increment = if prev_close < current_close {
            volume
        } else if prev_close > current_close {
            -volume
        } else {
            0.0
        };

        obv_point += increment;
        obv[i] = Some(obv_point);
    }

    obv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    /// Verifies the standard OBV outputs against fixture data.
    #[test]
    fn test_obv() {
        // Given
        let test_cases = vec!["005930", "TSLA"];

        // When
        for symbol in test_cases {
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v");

            let (line, signal) = obv(&close, &volume, 9);
            let expected_line = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/obv_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/obv_signal_{}.json",
                symbol
            ));

            // Then
            assert_eq!(
                round_vec(line, 4),
                round_vec(expected_line, 4),
                "OBV test failed for symbol {}.",
                symbol
            );

            assert_eq!(
                round_vec(signal, 4),
                round_vec(expected_signal, 4),
                "OBV test failed for symbol {}.",
                symbol
            );
        }
    }
}
