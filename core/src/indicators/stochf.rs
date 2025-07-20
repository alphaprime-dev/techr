use crate::utils::{calc_mean, find_max, find_min};

pub fn stochf(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period_k: usize,
    period_d: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = closes.len();
    let mut percent_k = vec![None; len];
    let mut percent_d = vec![None; len];

    if len < period_k {
        return (percent_k, percent_d);
    }

    for i in (period_k - 1)..len {
        let max_high = find_max(&highs[i + 1 - period_k..=i]);
        let min_low = find_min(&lows[i + 1 - period_k..=i]);

        let k = if max_high == min_low {
            None
        } else {
            Some(((closes[i] - min_low) / (max_high - min_low)) * 100.0)
        };

        percent_k[i] = k;

        if period_d == 1 {
            percent_d[i] = k;
        } else if i >= period_k - 1 + (period_d - 1) {
            let slice = &percent_k[i + 1 - period_d..=i];
            let valid_values: Vec<f64> = slice.iter().filter_map(|&x| x).collect();
            let d = if valid_values.len() == period_d {
                Some(calc_mean(&valid_values))
            } else {
                None
            };
            percent_d[i] = d;
        }
    }

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
