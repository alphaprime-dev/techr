use crate::utils::rolling_max_min;

pub fn willr(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    period: usize,
) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len != highs.len() || len != lows.len() || len < period || period == 0 {
        return result;
    }

    let (rolling_highs, rolling_lows) = rolling_max_min(highs, lows, period);

    for i in period - 1..len {
        let (Some(max_high), Some(min_low)) = (rolling_highs[i], rolling_lows[i]) else {
            continue;
        };

        let Some(cc) = closes[i] else {
            continue;
        };
        if max_high == min_low {
            result[i] = None;
        } else {
            result[i] = Some(((max_high - cc) / (max_high - min_low)) * -100.0);
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
    fn test_willr() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let high = high.into_iter().map(Some).collect::<Vec<_>>();
            let low = low.into_iter().map(Some).collect::<Vec<_>>();
            let close = close.into_iter().map(Some).collect::<Vec<_>>();
            let result = willr(&high, &low, &close, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/willr_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "WILLR test failed for symbol {}.",
                symbol
            );
        }
    }
}
