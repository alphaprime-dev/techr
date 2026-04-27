use crate::indicators::ema::ema_aligned;

pub fn obv(
    data: &[Option<f64>],
    volumes: &[Option<f64>],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let obv_line = obv_line(data, volumes);
    let obv_signal = ema_aligned(&obv_line, signal_period);

    (obv_line, obv_signal)
}

pub fn obv_signal(
    data: &[Option<f64>],
    volumes: &[Option<f64>],
    signal_period: usize,
) -> Vec<Option<f64>> {
    let obv_line = obv_line(data, volumes);
    ema_aligned(&obv_line, signal_period)
}

pub fn obv_line(data: &[Option<f64>], volumes: &[Option<f64>]) -> Vec<Option<f64>> {
    let mut obv = vec![None; data.len()];

    let len = data.len();

    if len == 0 || len != volumes.len() {
        return obv;
    }

    let mut obv_point = None;

    if let (Some(_), Some(volume)) = (data[0], volumes[0]) {
        obv_point = Some(volume);
        obv[0] = Some(volume);
    }

    for i in 1..len {
        let (Some(current_close), Some(prev_close), Some(volume), Some(current_obv)) =
            (data[i], data[i - 1], volumes[i], obv_point)
        else {
            continue;
        };

        let increment = if prev_close < current_close {
            volume
        } else if prev_close > current_close {
            -volume
        } else {
            0.0
        };

        let next_obv = current_obv + increment;
        obv_point = Some(next_obv);
        obv[i] = Some(next_obv);
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
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

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

    #[test]
    fn test_obv_with_gap_preserves_state_until_valid_predecessor_returns() {
        let closes = vec![Some(10.0), Some(11.0), None, Some(13.0), Some(12.0)];
        let volumes = vec![Some(100.0), Some(50.0), Some(40.0), Some(30.0), Some(20.0)];

        let result = obv_line(&closes, &volumes);

        assert_eq!(
            result,
            vec![Some(100.0), Some(150.0), None, None, Some(130.0)]
        );
    }

    #[test]
    fn test_obv_signal_follows_base_ema_contract_across_gaps() {
        let closes = vec![
            Some(10.0),
            Some(11.0),
            None,
            Some(13.0),
            Some(12.0),
            Some(14.0),
        ];
        let volumes = vec![
            Some(100.0),
            Some(50.0),
            Some(40.0),
            Some(30.0),
            Some(20.0),
            Some(10.0),
        ];

        let (line, signal) = obv(&closes, &volumes, 2);

        assert_eq!(
            line,
            vec![
                Some(100.0),
                Some(150.0),
                None,
                None,
                Some(130.0),
                Some(140.0)
            ]
        );
        assert_eq!(signal, ema_aligned(&line, 2));
        assert_eq!(
            round_vec(signal, 8),
            round_vec(
                vec![
                    None,
                    Some(125.0),
                    None,
                    None,
                    Some(128.33333333333334),
                    Some(136.11111111111111),
                ],
                8,
            )
        );
    }
}
