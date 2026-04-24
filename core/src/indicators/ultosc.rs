pub fn ultosc(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    period_short: usize,
    period_medium: usize,
    period_long: usize,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut ultosc = vec![None; len];

    if len != closes.len() || len != lows.len() || len < period_long + 1 {
        return ultosc;
    }

    let mut bp_values = vec![0.0; len];
    let mut tr_values = vec![0.0; len];
    let mut bp_valid = vec![false; len];
    let mut tr_valid = vec![false; len];

    let mut short_bp_sum = 0.0;
    let mut medium_bp_sum = 0.0;
    let mut long_bp_sum = 0.0;
    let mut short_tr_sum = 0.0;
    let mut medium_tr_sum = 0.0;
    let mut long_tr_sum = 0.0;
    let mut short_bp_valid = 0usize;
    let mut medium_bp_valid = 0usize;
    let mut long_bp_valid = 0usize;
    let mut short_tr_valid = 0usize;
    let mut medium_tr_valid = 0usize;
    let mut long_tr_valid = 0usize;

    for i in 1..len {
        if let (Some(close), Some(low), Some(prev_close)) = (closes[i], lows[i], closes[i - 1]) {
            let bp = close - low.min(prev_close);
            bp_values[i] = bp;
            bp_valid[i] = true;
            short_bp_sum += bp;
            medium_bp_sum += bp;
            long_bp_sum += bp;
            short_bp_valid += 1;
            medium_bp_valid += 1;
            long_bp_valid += 1;
        }

        if let (Some(prev_close), Some(high), Some(low)) = (closes[i - 1], highs[i], lows[i]) {
            let tr = high.max(prev_close) - low.min(prev_close);
            tr_values[i] = tr;
            tr_valid[i] = true;
            short_tr_sum += tr;
            medium_tr_sum += tr;
            long_tr_sum += tr;
            short_tr_valid += 1;
            medium_tr_valid += 1;
            long_tr_valid += 1;
        }

        if i >= period_short {
            short_bp_sum -= bp_values[i - period_short];
            short_tr_sum -= tr_values[i - period_short];
            short_bp_valid -= bp_valid[i - period_short] as usize;
            short_tr_valid -= tr_valid[i - period_short] as usize;
        }
        if i >= period_medium {
            medium_bp_sum -= bp_values[i - period_medium];
            medium_tr_sum -= tr_values[i - period_medium];
            medium_bp_valid -= bp_valid[i - period_medium] as usize;
            medium_tr_valid -= tr_valid[i - period_medium] as usize;
        }
        if i >= period_long {
            long_bp_sum -= bp_values[i - period_long];
            long_tr_sum -= tr_values[i - period_long];
            long_bp_valid -= bp_valid[i - period_long] as usize;
            long_tr_valid -= tr_valid[i - period_long] as usize;
        }

        if short_bp_valid == period_short
            && medium_bp_valid == period_medium
            && long_bp_valid == period_long
            && short_tr_valid == period_short
            && medium_tr_valid == period_medium
            && long_tr_valid == period_long
        {
            if short_tr_sum == 0.0 || medium_tr_sum == 0.0 || long_tr_sum == 0.0 {
                continue;
            }

            let uo_point = ((long_bp_sum / long_tr_sum
                + 2.0 * (medium_bp_sum / medium_tr_sum)
                + 4.0 * (short_bp_sum / short_tr_sum))
                * 100.0)
                / 7.0;

            if uo_point.is_finite() {
                ultosc[i] = Some(uo_point);
            }
        }
    }

    ultosc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_ultosc() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let result = ultosc(&high, &low, &close, 7, 14, 28);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ultosc_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "ULTOSC test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_ultosc_gap_invalidates_all_windows_until_full_valid_recovery() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            Some(13.0),
            None,
            Some(15.0),
            Some(16.0),
            Some(17.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(10.0),
            Some(11.0),
            None,
            Some(13.0),
            Some(14.0),
            Some(15.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            Some(12.0),
            None,
            Some(14.0),
            Some(15.0),
            Some(16.0),
        ];

        let result = ultosc(&highs, &lows, &closes, 2, 3, 4);

        assert_eq!(result, vec![None, None, None, None, None, None, None]);
    }

    #[test]
    fn test_ultosc_flat_windows_fail_closed_instead_of_emitting_nan() {
        let highs = vec![Some(10.0); 6];
        let lows = vec![Some(10.0); 6];
        let closes = vec![Some(10.0); 6];

        let result = ultosc(&highs, &lows, &closes, 2, 3, 4);

        assert_eq!(result, vec![None, None, None, None, None, None]);
    }
}
