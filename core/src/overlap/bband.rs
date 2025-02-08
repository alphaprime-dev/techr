use crate::overlap::sma;

use crate::utils::round_scalar;

pub fn bband(
    data: &[f64],
    period: usize,
    multiplier: Option<f64>,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let center = sma(data, period);
    let (upper_band, lower_band) = bband_bands(data, period, multiplier);
    (upper_band, center, lower_band)
}

fn bband_bands(
    data: &[f64],
    period: usize,
    multiplier: Option<f64>,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let mut upper_band = vec![None; data.len()];
    let mut lower_band = vec![None; data.len()];
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let multiplier = multiplier.unwrap_or(2.0);

    if data.len() < period {
        return (upper_band, lower_band);
    }

    for i in 0..data.len() {
        sum += data[i];
        sum_sq += data[i] * data[i];

        if i >= period {
            sum -= data[i - period];
            sum_sq -= data[i - period] * data[i - period];
        }

        if i >= period - 1 {
            let mean = sum / period as f64;
            let variance = (sum_sq / period as f64) - (mean * mean);
            let stddev = variance.sqrt();
            let deviation = multiplier * stddev;
            upper_band[i] = Some(round_scalar(mean + deviation, 8));
            lower_band[i] = Some(round_scalar(mean - deviation, 8));
        }
    }

    (upper_band, lower_band)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_bband() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let (upper, middle, lower) = bband(&input, 20, None);

            let expected_upper = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/bband_upper_{}.json",
                symbol
            ));
            let expected_middle = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sma_{}.json",
                symbol
            ));
            let expected_lower = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/bband_lower_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(upper, 8),
                expected_upper,
                "BBAND upper test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(middle, 8),
                expected_middle,
                "BBAND middle test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(lower, 8),
                expected_lower,
                "BBAND lower test failed for symbol {}.",
                symbol
            );
        }
    }
}
