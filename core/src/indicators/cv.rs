use crate::indicators::ema::ema;

pub fn cv(highs: &[f64], lows: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut cv = vec![None; highs.len()];
    let len = highs.len();

    if len != lows.len() || len < period * 2 || period <= 1 {
        return cv;
    }

    let high_low_diffs: Vec<f64> = highs.iter().zip(lows.iter()).map(|(h, l)| h - l).collect();
    let ema_high_low_diffs = ema(&high_low_diffs, period);

    for i in period * 2 - 1..len {
        if let (Some(current_ema), Some(previous_ema)) =
            (ema_high_low_diffs[i], ema_high_low_diffs[i - period])
        {
            let cv_point = ((current_ema - previous_ema) / previous_ema) * 100.0;
            cv[i] = Some(cv_point);
        }
    }

    cv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_cv() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let result = cv(&high, &low, 10);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/cv_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "CV test failed for symbol {}.",
                symbol
            );
        }
    }
}
