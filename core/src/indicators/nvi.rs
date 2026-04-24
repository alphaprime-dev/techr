use crate::indicators::ema::ema;

pub fn nvi(
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let nvi_line = nvi_line(closes, volumes);
    let signal = ema(&nvi_line, signal_period);

    (nvi_line, signal)
}

pub fn nvi_signal(
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
    signal_period: usize,
) -> Vec<Option<f64>> {
    let nvi_line = nvi_line(closes, volumes);
    ema(&nvi_line, signal_period)
}

pub fn nvi_line(closes: &[Option<f64>], volumes: &[Option<f64>]) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut nvi_line = vec![None; len];

    if len < 2 || len != volumes.len() {
        return nvi_line;
    }

    let mut nvi_point = None;
    if let (Some(_), Some(_)) = (closes[0], volumes[0]) {
        nvi_point = Some(1000.0);
        nvi_line[0] = Some(1000.0);
    }

    for i in 1..len {
        let (Some(close), Some(prev_close), Some(volume), Some(prev_volume), Some(current_nvi)) = (
            closes[i],
            closes[i - 1],
            volumes[i],
            volumes[i - 1],
            nvi_point,
        ) else {
            continue;
        };

        let mut next_nvi = current_nvi;
        if volume < prev_volume {
            next_nvi += (close - prev_close) * 100.0 / prev_close;
        }
        nvi_point = Some(next_nvi);
        nvi_line[i] = Some(next_nvi);
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
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

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

    #[test]
    fn test_nvi_with_gap_requires_valid_predecessor_to_resume() {
        let closes = vec![Some(10.0), Some(12.0), None, Some(15.0), Some(18.0)];
        let volumes = vec![Some(100.0), Some(80.0), Some(90.0), Some(70.0), Some(60.0)];

        let result = nvi_line(&closes, &volumes);

        assert_eq!(
            result,
            vec![Some(1000.0), Some(1020.0), None, None, Some(1040.0)]
        );
    }

    #[test]
    fn test_nvi_signal_follows_base_ema_contract_across_gaps() {
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
            Some(80.0),
            Some(90.0),
            Some(70.0),
            Some(60.0),
            Some(50.0),
        ];

        let (line, signal) = nvi(&closes, &volumes, 2);

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
        assert_eq!(signal, ema(&line, 2));
        assert_eq!(
            round_vec(signal, 8),
            round_vec(
                vec![None, Some(1010.0), None, None, Some(1030.0), Some(1070.0),],
                8,
            )
        );
    }
}
