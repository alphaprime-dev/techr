use crate::indicators::ema::ema_aligned;

pub fn pvi(
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let pvi_line = pvi_line(closes, volumes);
    let signal = ema_aligned(&pvi_line, signal_period);

    (pvi_line, signal)
}

pub fn pvi_signal(
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
    signal_period: usize,
) -> Vec<Option<f64>> {
    let pvi_line = pvi_line(closes, volumes);
    ema_aligned(&pvi_line, signal_period)
}

pub fn pvi_line(closes: &[Option<f64>], volumes: &[Option<f64>]) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut pvi_line = vec![None; len];

    if len < 2 || len != volumes.len() {
        return pvi_line;
    }

    let mut pvi_point = None;
    if let (Some(_), Some(_)) = (closes[0], volumes[0]) {
        pvi_point = Some(1000.0);
        pvi_line[0] = Some(1000.0);
    }

    for i in 1..len {
        let (Some(close), Some(prev_close), Some(volume), Some(prev_volume), Some(current_pvi)) = (
            closes[i],
            closes[i - 1],
            volumes[i],
            volumes[i - 1],
            pvi_point,
        ) else {
            continue;
        };

        let mut next_pvi = current_pvi;
        if volume > prev_volume {
            next_pvi += (close - prev_close) * 100.0 / prev_close;
        }
        pvi_point = Some(next_pvi);
        pvi_line[i] = Some(next_pvi);
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
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

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

    #[test]
    fn test_pvi_with_gap_requires_valid_predecessor_to_resume() {
        let closes = vec![Some(10.0), Some(12.0), None, Some(15.0), Some(18.0)];
        let volumes = vec![
            Some(100.0),
            Some(120.0),
            Some(90.0),
            Some(150.0),
            Some(180.0),
        ];

        let result = pvi_line(&closes, &volumes);

        assert_eq!(
            result,
            vec![Some(1000.0), Some(1020.0), None, None, Some(1040.0)]
        );
    }

    #[test]
    fn test_pvi_signal_follows_base_ema_contract_across_gaps() {
        let closes = vec![
            Some(10.0),
            Some(12.0),
            None,
            Some(15.0),
            Some(18.0),
            Some(27.0),
        ];
        let volumes = vec![
            Some(100.0),
            Some(120.0),
            Some(90.0),
            Some(150.0),
            Some(180.0),
            Some(200.0),
        ];

        let (line, signal) = pvi(&closes, &volumes, 2);

        assert_eq!(
            line,
            vec![
                Some(1000.0),
                Some(1020.0),
                None,
                None,
                Some(1040.0),
                Some(1090.0),
            ]
        );
        assert_eq!(signal, ema_aligned(&line, 2));
        assert_eq!(
            round_vec(signal, 8),
            round_vec(
                vec![None, Some(1010.0), None, None, Some(1030.0), Some(1070.0),],
                8,
            )
        );
    }
}
