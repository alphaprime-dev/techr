use crate::indicators::ema::ema;

pub fn cv(highs: &[Option<f64>], lows: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut cv = vec![None; highs.len()];
    let len = highs.len();

    if len != lows.len() || len < period * 2 || period <= 1 {
        return cv;
    }

    let high_low_diffs = highs
        .iter()
        .zip(lows.iter())
        .map(|(&high, &low)| match (high, low) {
            (Some(high), Some(low)) => Some(high - low),
            _ => None,
        })
        .collect::<Vec<_>>();
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
            let high = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "l");
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

    #[test]
    fn test_cv_gap_invalidates_until_shifted_ema_is_available() {
        let highs = vec![Some(3.0), Some(5.0), None, Some(9.0), Some(11.0)];
        let lows = vec![Some(1.0), Some(1.0), None, Some(3.0), Some(5.0)];

        let result = round_vec(cv(&highs, &lows, 2), 8);

        assert_eq!(
            result,
            round_vec(vec![None, None, None, Some(66.66666666666667), None], 8)
        );
    }
}
