use crate::utils::rolling_mean_strict;

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

    let typical_prices = highs
        .iter()
        .zip(lows.iter())
        .zip(closes.iter())
        .map(|((high, low), close)| match (high, low, close) {
            (Some(high), Some(low), Some(close)) => Some((high + low + close) / 3.0),
            _ => None,
        })
        .collect::<Vec<_>>();
    let sma_tp = rolling_mean_strict(&typical_prices, period);

    for i in period - 1..len {
        let Some(sma_tp) = sma_tp[i] else {
            continue;
        };
        let mut mean_deviation = 0.0;
        let mut valid = true;
        for typical_price in &typical_prices[i + 1 - period..=i] {
            let Some(typical_price) = *typical_price else {
                valid = false;
                break;
            };
            mean_deviation += (typical_price - sma_tp).abs();
        }
        if !valid {
            continue;
        }
        mean_deviation /= period as f64;

        result[i] = if mean_deviation == 0.0 {
            None
        } else {
            Some(
                (typical_prices[i].expect("sma implies current value exists") - sma_tp)
                    / (0.015 * mean_deviation),
            )
        };
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
    fn test_cci_with_gap_invalidates_typical_price_window() {
        let high = vec![Some(3.0), Some(4.0), None, Some(6.0), Some(7.0)];
        let low = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];
        let close = vec![Some(2.0), Some(3.0), None, Some(5.0), Some(6.0)];

        let result = cci(&high, &low, &close, 3);

        assert_eq!(result, vec![None, None, None, None, None]);
    }
}
