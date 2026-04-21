use crate::utils::{rolling_max_min, rolling_mean_strict};

fn stochs_raw_k(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    fastk_period: usize,
) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut raw_k = vec![None; len];

    if len != highs.len() || len != lows.len() || len < fastk_period || fastk_period == 0 {
        return raw_k;
    }

    let (rolling_highs, rolling_lows) = rolling_max_min(highs, lows, fastk_period);

    for i in (fastk_period - 1)..len {
        let (Some(max_high), Some(min_low)) = (rolling_highs[i], rolling_lows[i]) else {
            continue;
        };

        let Some(close) = closes[i] else {
            continue;
        };

        raw_k[i] = if max_high == min_low {
            None
        } else {
            Some(((close - min_low) / (max_high - min_low)) * 100.0)
        };
    }

    raw_k
}

pub fn stoch_percent_k(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
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
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
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
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
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

    #[test]
    fn test_stochs_with_gap_invalidates_slow_windows() {
        let high = vec![Some(3.0), Some(4.0), Some(5.0), None, Some(7.0), Some(8.0)];
        let low = vec![Some(1.0), Some(2.0), Some(3.0), None, Some(5.0), Some(6.0)];
        let close = vec![Some(2.0), Some(3.0), Some(4.0), None, Some(6.0), Some(7.0)];

        let (percent_k, percent_d) = stochs(&high, &low, &close, 3, 2, 2);

        assert_eq!(percent_k, vec![None, None, None, None, None, None]);
        assert_eq!(percent_d, vec![None, None, None, None, None, None]);
    }
}
