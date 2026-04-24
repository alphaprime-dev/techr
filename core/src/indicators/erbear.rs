use crate::indicators::ema::ema;

pub fn erbear(lows: &[Option<f64>], closes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut erbear = vec![None; lows.len()];

    if lows.len() != closes.len() || lows.len() < period || period == 0 {
        return erbear;
    }

    let ema_values = ema(closes, period);

    for i in (period - 1)..lows.len() {
        if let (Some(low), Some(ema_value)) = (lows[i], ema_values[i]) {
            let bear_power = low - ema_value;
            erbear[i] = Some(bear_power);
        }
    }

    erbear
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_erbear() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let lows = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "l");
            let closes = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let result = erbear(&lows, &closes, 13);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/erbear_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "ERBEAR test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_erbear_gap_propagates_low_nulls_and_resumes_ema_state() {
        let lows = vec![Some(0.0), Some(1.0), None, Some(3.0), Some(4.0)];
        let closes = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];

        let result = round_vec(erbear(&lows, &closes, 2), 8);

        assert_eq!(
            result,
            round_vec(
                vec![
                    None,
                    Some(-0.5),
                    None,
                    Some(-0.16666666666666652),
                    Some(-0.3888888888888884),
                ],
                8
            )
        );
    }
}
