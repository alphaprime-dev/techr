pub fn atr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut atr = vec![None; highs.len()];
    let len = highs.len();

    if len != lows.len() || len != closes.len() || len < period || period <= 1 {
        return atr;
    }

    let mut tr_sum = 0.0;

    for i in 0..len {
        if i == 0 {
            continue;
        }

        let prev_close = closes[i - 1];
        if i < period {
            let tr = calc_tr(highs[i], lows[i], prev_close);
            tr_sum += tr;
            if i == period - 1 {
                let atr_point = tr_sum / i as f64;
                atr[i] = Some(atr_point);
            }
        } else {
            let prev_atr = atr[i - 1].unwrap_or(0.0);
            let tr = calc_tr(highs[i], lows[i], prev_close);
            let atr_point = (prev_atr * (period - 1) as f64 + tr) / period as f64;
            atr[i] = Some(atr_point);
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
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
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
}
