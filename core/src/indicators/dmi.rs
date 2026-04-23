use crate::utils::{calc_true_ranges_aligned, wilders_smoothing_aligned};

pub fn dmi(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = highs.len();
    let mut plus_di = vec![None; len];
    let mut minus_di = vec![None; len];

    if len == 0 || len != lows.len() || len != closes.len() || period == 0 {
        return (plus_di, minus_di);
    }

    let trs = calc_true_ranges_aligned(highs, lows, closes);
    let mut plus_dm = vec![None; len];
    let mut minus_dm = vec![None; len];

    for i in 1..len {
        let (Some(prev_high), Some(high), Some(prev_low), Some(low), Some(_)) =
            (highs[i - 1], highs[i], lows[i - 1], lows[i], trs[i])
        else {
            continue;
        };

        let delta_high = (high - prev_high).max(0.0);
        let delta_low = (prev_low - low).max(0.0);

        plus_dm[i] = Some(if delta_high > delta_low && delta_high > 0.0 {
            delta_high
        } else {
            0.0
        });
        minus_dm[i] = Some(if delta_low > delta_high && delta_low > 0.0 {
            delta_low
        } else {
            0.0
        });
    }

    let plus_dm_sum = wilders_smoothing_aligned(&plus_dm, period);
    let minus_dm_sum = wilders_smoothing_aligned(&minus_dm, period);
    let tr_sum = wilders_smoothing_aligned(&trs, period);

    for i in 0..len {
        if let (Some(plus_sum), Some(minus_sum), Some(tr_total)) =
            (plus_dm_sum[i], minus_dm_sum[i], tr_sum[i])
        {
            if tr_total == 0.0 {
                plus_di[i] = Some(0.0);
                minus_di[i] = Some(0.0);
            } else {
                plus_di[i] = Some((plus_sum / tr_total) * 100.0);
                minus_di[i] = Some((minus_sum / tr_total) * 100.0);
            }
        }
    }

    (plus_di, minus_di)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_dmi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let (plus_di, minus_di) = dmi(&highs, &lows, &closes, 14);

            let expected_plus_di = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/dmi_plus_{}.json",
                symbol
            ));
            let expected_minus_di = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/dmi_minus_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(plus_di, 8),
                round_vec(expected_plus_di, 8),
                "DMI +DI test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(minus_di, 8),
                round_vec(expected_minus_di, 8),
                "DMI -DI test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_dmi_with_gap_requires_valid_predecessor_and_resumes() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            Some(14.0),
            None,
            Some(15.0),
            Some(16.0),
            Some(18.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(9.0),
            Some(11.0),
            None,
            Some(13.0),
            Some(14.0),
            Some(15.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            Some(13.0),
            None,
            Some(14.0),
            Some(15.0),
            Some(17.0),
        ];

        let (plus_di, minus_di) = dmi(&highs, &lows, &closes, 2);

        assert_eq!(
            plus_di,
            vec![
                None,
                None,
                Some(66.66666666666666),
                None,
                None,
                Some(58.82352941176471),
                Some(63.41463414634146),
            ]
        );
        assert_eq!(
            minus_di,
            vec![None, None, Some(0.0), None, None, Some(0.0), Some(0.0)]
        );
    }

    #[test]
    fn test_dmi_requires_contiguous_seed_window() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            None,
            Some(13.0),
            Some(14.0),
            Some(15.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(9.0),
            None,
            Some(11.0),
            Some(12.0),
            Some(13.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            None,
            Some(12.0),
            Some(13.0),
            Some(14.0),
        ];

        let (plus_di, minus_di) = dmi(&highs, &lows, &closes, 2);

        assert_eq!(plus_di, vec![None, None, None, None, None, Some(50.0)]);
        assert_eq!(minus_di, vec![None, None, None, None, None, Some(0.0)]);
    }

    #[test]
    fn test_dmi_non_synchronous_predecessor_gap_does_not_advance_tr_state() {
        let highs = vec![
            Some(10.0),
            None,
            Some(13.0),
            Some(14.0),
            Some(15.0),
            Some(16.0),
        ];
        let lows = vec![
            Some(8.0),
            None,
            Some(11.0),
            Some(12.0),
            Some(13.0),
            Some(14.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            Some(12.0),
            Some(13.0),
            Some(14.0),
            Some(15.0),
        ];

        let (plus_di, minus_di) = dmi(&highs, &lows, &closes, 2);

        assert_eq!(
            plus_di,
            vec![None, None, None, None, Some(50.0), Some(50.0),]
        );
        assert_eq!(minus_di, vec![None, None, None, None, Some(0.0), Some(0.0)]);
    }

    #[test]
    fn test_dmi_length_mismatch_fails_closed() {
        let highs = vec![Some(10.0), Some(12.0), Some(14.0)];
        let lows = vec![Some(8.0), Some(9.0)];
        let closes = vec![Some(9.0), Some(11.0), Some(13.0)];

        let (plus_di, minus_di) = dmi(&highs, &lows, &closes, 2);

        assert_eq!(plus_di, vec![None, None, None]);
        assert_eq!(minus_di, vec![None, None, None]);
    }
}
