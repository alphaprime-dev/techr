use crate::utils::{find_max, find_min};

pub fn pchan(
    highs: &[f64],
    lows: &[f64],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = highs.len();
    let mut upper = vec![None; len];
    let mut lower = vec![None; len];
    let mut middle = vec![None; len];

    if len < period {
        return (upper, middle, lower);
    }

    for i in period..len {
        let max_high = find_max(&highs[i - period..i]);
        let min_low = find_min(&lows[i - period..i]);

        upper[i] = Some(max_high);
        lower[i] = Some(min_low);
        middle[i] = Some((max_high + min_low) / 2.0);
    }

    (upper, middle, lower)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_pchan() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let (upper, middle, lower) = pchan(&highs, &lows, 20);

            let expected_upper = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pchan_upper_{}.json",
                symbol
            ));
            let expected_middle = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pchan_middle_{}.json",
                symbol
            ));
            let expected_lower = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/pchan_lower_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(upper, 8),
                round_vec(expected_upper, 8),
                "PCHAN upper test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(middle, 8),
                round_vec(expected_middle, 8),
                "PCHAN middle test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(lower, 8),
                round_vec(expected_lower, 8),
                "PCHAN lower test failed for symbol {}.",
                symbol
            );
        }
    }
}
