use crate::utils::rolling_max_min;

pub fn pchan(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = highs.len();
    let mut upper = vec![None; len];
    let mut lower = vec![None; len];
    let mut middle = vec![None; len];

    if len != lows.len() || period == 0 || len <= period {
        return (upper, middle, lower);
    }

    let (rolling_highs, rolling_lows) =
        rolling_max_min(&highs[..len - 1], &lows[..len - 1], period);

    for i in period..len {
        let (Some(max_high), Some(min_low)) = (rolling_highs[i - 1], rolling_lows[i - 1]) else {
            continue;
        };

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
            let highs = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "l");
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

    #[test]
    fn test_pchan_gap_invalidates_prior_window_until_recovery() {
        let highs = vec![Some(3.0), Some(5.0), None, Some(9.0), Some(11.0)];
        let lows = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(6.0)];

        let (upper, middle, lower) = pchan(&highs, &lows, 2);

        assert_eq!(upper, vec![None, None, Some(5.0), None, None]);
        assert_eq!(middle, vec![None, None, Some(3.0), None, None]);
        assert_eq!(lower, vec![None, None, Some(1.0), None, None]);
    }
}
