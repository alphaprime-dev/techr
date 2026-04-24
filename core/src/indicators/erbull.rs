use crate::indicators::ema::ema;

pub fn erbull(highs: &[Option<f64>], closes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut erbull = vec![None; highs.len()];

    if highs.len() != closes.len() || highs.len() < period || period == 0 {
        return erbull;
    }

    let ema_values = ema(closes, period);

    for i in (period - 1)..highs.len() {
        if let (Some(high), Some(ema_value)) = (highs[i], ema_values[i]) {
            let bull_power = high - ema_value;
            erbull[i] = Some(bull_power);
        }
    }

    erbull
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_erbull() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "h");
            let closes = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let result = erbull(&highs, &closes, 13);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/erbull_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "ERBULL test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_erbull_gap_propagates_high_nulls_and_resumes_ema_state() {
        let highs = vec![Some(3.0), Some(4.0), None, Some(6.0), Some(7.0)];
        let closes = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];

        let result = round_vec(erbull(&highs, &closes, 2), 8);

        assert_eq!(
            result,
            round_vec(
                vec![
                    None,
                    Some(2.5),
                    None,
                    Some(2.8333333333333335),
                    Some(2.6111111111111116),
                ],
                8
            )
        );
    }
}
