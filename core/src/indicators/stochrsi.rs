use crate::indicators::rsi;
use crate::utils::{calc_mean, find_max, find_min};

pub fn stochrsi(
    closes: &[f64],
    period_rsi: usize,
    period_k: usize,
    period_d: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = closes.len();
    let mut percent_k = vec![None; len];
    let mut percent_d = vec![None; len];

    if len < period_rsi + period_k || period_rsi <= 1 || period_k <= 1 {
        return (percent_k, percent_d);
    }

    let rsi_values = rsi(closes, period_rsi);

    for i in (period_rsi + period_k - 1)..len {
        let slice = &rsi_values[i + 1 - period_k..=i];
        let valid_values: Vec<f64> = slice.iter().filter_map(|&x| x).collect();

        if valid_values.len() == period_k {
            let rsi_max = find_max(&valid_values);
            let rsi_min = find_min(&valid_values);

            let k = if rsi_max == rsi_min {
                None
            } else {
                rsi_values[i].map(|rsi| ((rsi - rsi_min) / (rsi_max - rsi_min)) * 100.0)
            };

            percent_k[i] = k;

            if period_d == 1 {
                percent_d[i] = k;
            } else if i >= period_rsi + period_k - 1 + (period_d - 1) {
                let d_slice = &percent_k[i + 1 - period_d..=i];
                let valid_d_values: Vec<f64> = d_slice.iter().filter_map(|&x| x).collect();
                let d = if valid_d_values.len() == period_d {
                    Some(calc_mean(&valid_d_values))
                } else {
                    None
                };
                percent_d[i] = d;
            }
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
    fn test_stochrsi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");

            let (percent_k, percent_d) = stochrsi(&closes, 14, 14, 3);

            let expected_k = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochrsi_K_{}.json",
                symbol
            ));
            let expected_d = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/stochrsi_D_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(percent_k, 4),
                round_vec(expected_k, 4),
                "StochRSI %K test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(percent_d, 4),
                round_vec(expected_d, 4),
                "StochRSI %D test failed for symbol {}.",
                symbol
            );
        }
    }
}
