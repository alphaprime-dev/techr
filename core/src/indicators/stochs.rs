use crate::utils::{calc_mean, find_max, find_min};

pub fn stochs(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowd_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = closes.len();
    let mut percent_k = vec![None; len];
    let mut percent_d = vec![None; len];

    if len < fastk_period {
        return (percent_k, percent_d);
    }

    let mut raw_k = vec![None; len];
    for i in (fastk_period - 1)..len {
        let max_high = find_max(&highs[i + 1 - fastk_period..=i]);
        let min_low = find_min(&lows[i + 1 - fastk_period..=i]);

        raw_k[i] = if max_high == min_low {
            None
        } else {
            Some(((closes[i] - min_low) / (max_high - min_low)) * 100.0)
        };
    }
    for i in (fastk_period + slowk_period - 2)..len {
        let slice = &raw_k[i + 1 - slowk_period..=i];
        let valid_values: Vec<f64> = slice.iter().filter_map(|&x| x).collect();
        percent_k[i] = if valid_values.len() == slowk_period {
            Some(calc_mean(&valid_values))
        } else {
            None
        };
    }

    for i in (fastk_period + slowk_period + slowd_period - 3)..len {
        let slice = &percent_k[i + 1 - slowd_period..=i];
        let valid_values: Vec<f64> = slice.iter().filter_map(|&x| x).collect();
        percent_d[i] = if valid_values.len() == slowd_period {
            Some(calc_mean(&valid_values))
        } else {
            None
        };
    }

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
