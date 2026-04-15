use crate::utils::{rolling_max_min, rolling_mean_strict};

fn stochs_raw_k(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut raw_k = vec![None; len];

    if len < fastk_period {
        return raw_k;
    }

    let (rolling_highs, rolling_lows) = rolling_max_min(highs, lows, fastk_period);

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

    raw_k
}

pub fn stoch_percent_k(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
    slowk_period: usize,
) -> Vec<Option<f64>> {
    let len = closes.len();

    if len < fastk_period {
        return vec![None; len];
    }

    let raw_k = stochs_raw_k(highs, lows, closes, fastk_period);
    rolling_mean_strict(&raw_k, slowk_period)
}

pub fn stoch_percent_d(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowd_period: usize,
) -> Vec<Option<f64>> {
    let percent_k = stoch_percent_k(highs, lows, closes, fastk_period, slowk_period);
    stoch_percent_d_from_k(&percent_k, fastk_period, slowk_period, slowd_period)
}

fn stoch_percent_d_from_k(
    percent_k: &[Option<f64>],
    fastk_period: usize,
    _slowk_period: usize,
    slowd_period: usize,
) -> Vec<Option<f64>> {
    let len = percent_k.len();

    if len < fastk_period {
        return vec![None; len];
    }

    if slowd_period == 1 {
        percent_k.to_vec()
    } else {
        rolling_mean_strict(percent_k, slowd_period)
    }
}

pub fn stochs(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowd_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let percent_k = stoch_percent_k(highs, lows, closes, fastk_period, slowk_period);
    let percent_d = stoch_percent_d_from_k(&percent_k, fastk_period, slowk_period, slowd_period);
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
