use crate::indicators::ema::ema;

pub fn erbear(lows: &[f64], closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut erbear = vec![None; lows.len()];

    if lows.len() < period {
        return erbear;
    }

    let ema_values = ema(closes, period);

    for i in (period - 1)..lows.len() {
        if let Some(ema_value) = ema_values[i] {
            let bear_power = lows[i] - ema_value;
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
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");
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
}
