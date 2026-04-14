use crate::utils::{rolling_max_min, rolling_mean_strict};

pub fn stochs(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowd_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = closes.len();
    let empty = vec![None; len];

    if len < fastk_period {
        return (empty.clone(), empty);
    }

    let (rolling_highs, rolling_lows) = rolling_max_min(highs, lows, fastk_period);

    let mut raw_k = vec![None; len];
    for i in (fastk_period - 1)..len {
        let (Some(max_high), Some(min_low)) = (rolling_highs[i], rolling_lows[i]) else {
            continue;
        };

        raw_k[i] = if max_high == min_low {
            None
        } else {
            Some(((closes[i] - min_low) / (max_high - min_low)) * 100.0)
        };
    }

    let percent_k = rolling_mean_strict(&raw_k, slowk_period);
    let percent_d = if slowd_period == 1 {
        percent_k.clone()
    } else {
        rolling_mean_strict(&percent_k, slowd_period)
    };

    (percent_k, percent_d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_stochs() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");

            let (percent_k, percent_d) = stochs(&high, &low, &close, 14, 3, 3);

            let expected_k = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochs_K_{}.json",
                symbol
            ));
            let expected_d = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochs_D_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(percent_k, 8),
                round_vec(expected_k, 8),
                "STOCHS %K test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(percent_d, 8),
                round_vec(expected_d, 8),
                "STOCHS %D test failed for symbol {}.",
                symbol
            );
        }
    }
}
