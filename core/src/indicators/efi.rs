use crate::indicators::ema::ema;

pub fn efi(closes: &[Option<f64>], volumes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut efi = vec![None; len];

    if len != volumes.len() || len < 2 {
        return efi;
    }

    let mut force = vec![None; len];
    for i in 1..len {
        let (Some(close), Some(prev_close), Some(volume)) = (closes[i], closes[i - 1], volumes[i])
        else {
            continue;
        };
        force[i] = Some((close - prev_close) * volume);
    }

    if period == 1 {
        efi = force;
    } else {
        efi = ema(&force, period);
    }

    efi
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_efi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let result = efi(&close, &volume, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/efi_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 4),
                round_vec(expected, 4),
                "EFI test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_efi_gap_skips_missing_pair_and_resumes_ema_state() {
        let closes = vec![
            Some(10.0),
            Some(11.0),
            Some(13.0),
            None,
            Some(16.0),
            Some(17.0),
        ];
        let volumes = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            Some(5.0),
            Some(6.0),
        ];

        let result = efi(&closes, &volumes, 2);

        assert_eq!(
            round_vec(result, 8),
            round_vec(
                vec![None, None, Some(4.0), None, None, Some(5.333333333333333)],
                8
            )
        );
    }
}
