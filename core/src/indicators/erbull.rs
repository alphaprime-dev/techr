use crate::indicators::ema::ema;

pub fn erbull(highs: &[f64], closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut erbull = vec![None; highs.len()];

    if highs.len() < period {
        return erbull;
    }

    let ema_values = ema(closes, period);

    for i in (period - 1)..highs.len() {
        if let Some(ema_value) = ema_values[i] {
            let bull_power = highs[i] - ema_value;
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
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");
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
}
