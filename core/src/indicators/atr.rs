pub fn atr(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    period: usize,
) -> Vec<Option<f64>> {
    let mut atr = vec![None; highs.len()];
    let len = highs.len();

    if len != lows.len() || len != closes.len() || len < period || period <= 1 {
        return atr;
    }

    let mut tr_sum = 0.0;
    let mut seeded_ranges = 0usize;
    let mut prev_atr = None;

    for i in 1..len {
        let (Some(high), Some(low), Some(prev_close)) = (highs[i], lows[i], closes[i - 1]) else {
            if prev_atr.is_none() {
                tr_sum = 0.0;
                seeded_ranges = 0;
            }
            continue;
        };

        let tr = calc_tr(high, low, prev_close);
        if let Some(current_atr) = prev_atr {
            let next_atr = (current_atr * (period - 1) as f64 + tr) / period as f64;
            prev_atr = Some(next_atr);
            atr[i] = Some(next_atr);
        } else {
            tr_sum += tr;
            seeded_ranges += 1;
            if seeded_ranges == period - 1 {
                let atr_point = tr_sum / seeded_ranges as f64;
                prev_atr = Some(atr_point);
                atr[i] = Some(atr_point);
            }
        }
    }

    atr
}

#[inline]
fn calc_tr(high: f64, low: f64, prev_close: f64) -> f64 {
    let th = high.max(prev_close);
    let tl = low.min(prev_close);
    th - tl
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_atr() {
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
            let result = atr(&high, &low, &close, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/atr_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 6),
                round_vec(expected, 6),
                "ATR test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_atr_with_gap_resumes_from_prior_state() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            Some(13.0),
            None,
            Some(15.0),
            Some(16.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(10.0),
            Some(11.0),
            None,
            Some(13.0),
            Some(14.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            Some(12.0),
            None,
            Some(14.0),
            Some(15.0),
        ];

        let result = atr(&highs, &lows, &closes, 3);

        assert_eq!(
            result,
            vec![None, None, Some(2.5), None, None, Some(2.3333333333333335)]
        );
    }

    #[test]
    fn test_atr_requires_contiguous_seed_window() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            None,
            Some(14.0),
            Some(15.0),
            Some(16.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(10.0),
            None,
            Some(12.0),
            Some(13.0),
            Some(14.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            None,
            Some(13.0),
            Some(14.0),
            Some(15.0),
        ];

        let result = atr(&highs, &lows, &closes, 3);

        assert_eq!(result, vec![None, None, None, None, None, Some(2.0)]);
    }
}
