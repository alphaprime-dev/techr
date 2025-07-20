use crate::utils::{find_max, find_min};

pub fn willr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len < period {
        return result;
    }

    for i in period - 1..len {
        let max_high = find_max(&highs[i + 1 - period..=i]);
        let min_low = find_min(&lows[i + 1 - period..=i]);

        let cc = closes[i];
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
