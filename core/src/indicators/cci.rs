pub fn cci(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    period: usize,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut result = vec![None; len];

    if len != lows.len() || len != closes.len() || len < period || period <= 1 {
        return result;
    }

    let typical_prices: Vec<Option<f64>> = highs
        .iter()
        .zip(lows.iter())
        .zip(closes.iter())
        .map(|((&high, &low), &close)| match (high, low, close) {
            (Some(high), Some(low), Some(close)) => Some((high + low + close) / 3.0),
            _ => None,
        })
        .collect();

    for i in period - 1..len {
        let mut window = Vec::with_capacity(period);
        let mut valid = true;
        for value in &typical_prices[i + 1 - period..=i] {
            if let Some(value) = value {
                window.push(*value);
            } else {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        let sma_tp = window.iter().sum::<f64>() / period as f64;
        let mean_deviation = window
            .iter()
            .map(|&value| (value - sma_tp).abs())
            .sum::<f64>()
            / period as f64;

        if mean_deviation != 0.0 {
            let current_tp = *window
                .last()
                .expect("validated CCI window must include the current typical price");
            result[i] = Some((current_tp - sma_tp) / (0.015 * mean_deviation));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_cci() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
            let result = cci(&high, &low, &close, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/cci_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "CCI test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_cci_gap_invalidates_full_typical_price_window() {
        let highs = vec![Some(4.0), Some(6.0), None, Some(10.0), Some(12.0)];
        let lows = vec![Some(2.0), Some(2.0), None, Some(6.0), Some(8.0)];
        let closes = vec![Some(3.0), Some(4.0), None, Some(9.0), Some(11.0)];

        let result = round_vec(cci(&highs, &lows, &closes, 2), 8);

        assert_eq!(
            result,
            round_vec(
                vec![
                    None,
                    Some(66.66666666666667),
                    None,
                    None,
                    Some(66.66666666666667)
                ],
                8
            )
        );
    }
}
