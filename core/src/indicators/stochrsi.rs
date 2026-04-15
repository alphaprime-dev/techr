use crate::indicators::rsi;
use crate::utils::{rolling_max_min, rolling_mean_strict};

pub fn stochrsi(
    closes: &[f64],
    period_rsi: usize,
    period_k: usize,
    period_d: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = closes.len();
    let mut percent_k = vec![None; len];

    if len < period_rsi + period_k || period_rsi <= 1 || period_k <= 1 {
        return (percent_k, vec![None; len]);
    }

    let rsi_values = rsi(closes, period_rsi);
    let rsi_values_with_nan: Vec<f64> = rsi_values
        .iter()
        .map(|value| value.unwrap_or(f64::NAN))
        .collect();
    let (rolling_max, rolling_min) =
        rolling_max_min(&rsi_values_with_nan, &rsi_values_with_nan, period_k);

    for i in (period_rsi + period_k - 1)..len {
        let valid_values: Vec<f64> = rsi_values[i + 1 - period_k..=i]
            .iter()
            .filter_map(|&x| x)
            .collect();

        if valid_values.len() != period_k {
            continue;
        }

        let (Some(rsi), Some(rsi_max), Some(rsi_min)) =
            (rsi_values[i], rolling_max[i], rolling_min[i])
        else {
            continue;
        };

        percent_k[i] = if rsi_max == rsi_min {
            None
        } else {
            Some(((rsi - rsi_min) / (rsi_max - rsi_min)) * 100.0)
        };
    }

    let percent_d = if period_d == 1 {
        percent_k.clone()
    } else {
        rolling_mean_strict(&percent_k, period_d)
    };

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
