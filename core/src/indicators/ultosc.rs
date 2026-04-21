use crate::utils::rolling_sum_strict;

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

    let buying_pressures = calc_buying_pressures(closes, lows);
    let true_ranges = calc_true_ranges(closes, highs, lows);
    let short_bp = rolling_sum_strict(&buying_pressures, period_short);
    let medium_bp = rolling_sum_strict(&buying_pressures, period_medium);
    let long_bp = rolling_sum_strict(&buying_pressures, period_long);
    let short_tr = rolling_sum_strict(&true_ranges, period_short);
    let medium_tr = rolling_sum_strict(&true_ranges, period_medium);
    let long_tr = rolling_sum_strict(&true_ranges, period_long);

    for i in 0..len {
        if let (
            Some(short_bp),
            Some(medium_bp),
            Some(long_bp),
            Some(short_tr),
            Some(medium_tr),
            Some(long_tr),
        ) = (
            short_bp[i],
            medium_bp[i],
            long_bp[i],
            short_tr[i],
            medium_tr[i],
            long_tr[i],
        ) {
            let uo_point =
                ((long_bp / long_tr + 2.0 * (medium_bp / medium_tr) + 4.0 * (short_bp / short_tr))
                    * 100.0)
                    / 7.0;
            ultosc[i] = Some(uo_point);
        }
    }

    ultosc
}

fn calc_buying_pressures(closes: &[Option<f64>], lows: &[Option<f64>]) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut buying_pressures = vec![None; len];

    for i in 1..len {
        let (Some(close), Some(low), Some(prev_close)) = (closes[i], lows[i], closes[i - 1]) else {
            continue;
        };

        buying_pressures[i] = Some(close - low.min(prev_close));
    }

    buying_pressures
}

fn calc_true_ranges(
    closes: &[Option<f64>],
    highs: &[Option<f64>],
    lows: &[Option<f64>],
) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut true_ranges = vec![None; len];

    for i in 1..len {
        let (Some(prev_close), Some(high), Some(low)) = (closes[i - 1], highs[i], lows[i]) else {
            continue;
        };

        true_ranges[i] = Some(high.max(prev_close) - low.min(prev_close));
    }

    true_ranges
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
}
