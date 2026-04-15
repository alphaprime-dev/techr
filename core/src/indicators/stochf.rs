use crate::utils::{rolling_max_min, rolling_mean_strict};

pub fn stochf_percent_k(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut percent_k = vec![None; len];

    if len < fastk_period {
        return percent_k;
    }

    let (rolling_highs, rolling_lows) = rolling_max_min(highs, lows, fastk_period);

    for i in (fastk_period - 1)..len {
        let (Some(max_high), Some(min_low)) = (rolling_highs[i], rolling_lows[i]) else {
            continue;
        };

        percent_k[i] = if max_high == min_low {
            None
        } else {
            Some(((closes[i] - min_low) / (max_high - min_low)) * 100.0)
        };
    }

    percent_k
}

pub fn stochf_percent_d(
    percent_k: &[Option<f64>],
    fastk_period: usize,
    fastd_period: usize,
) -> Vec<Option<f64>> {
    let len = percent_k.len();

    if len < fastk_period {
        return vec![None; len];
    }

    if fastd_period == 1 {
        percent_k.to_vec()
    } else {
        rolling_mean_strict(percent_k, fastd_period)
    }
}

pub fn stochf(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
    fastd_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let percent_k = stochf_percent_k(highs, lows, closes, fastk_period);
    let percent_d = stochf_percent_d(&percent_k, fastk_period, fastd_period);
    (percent_k, percent_d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_stochf() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");

            let (percent_k, percent_d) = stochf(&highs, &lows, &closes, 14, 3);

            let expected_k = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochf_K_{}.json",
                symbol
            ));
            let expected_d = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochf_D_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(percent_k, 8),
                round_vec(expected_k, 8),
                "STOCHF %K test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(percent_d, 8),
                round_vec(expected_d, 8),
                "STOCHF %D test failed for symbol {}.",
                symbol
            );
        }
    }
}
