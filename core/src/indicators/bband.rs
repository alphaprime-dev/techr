use crate::indicators::sma::sma;
use crate::utils::{rolling_mean_stddev_strict, round_scalar};

pub fn bband(
    data: &[Option<f64>],
    period: usize,
    sigma: Option<f64>,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let center = sma(data, period);
    let (upper_band, lower_band) = bband_bands(data, period, sigma);
    (upper_band, center, lower_band)
}

pub fn bband_middle(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    sma(data, period)
}

pub fn bband_upper(data: &[Option<f64>], period: usize, sigma: Option<f64>) -> Vec<Option<f64>> {
    let (upper_band, _) = bband_bands(data, period, sigma);
    upper_band
}

pub fn bband_lower(data: &[Option<f64>], period: usize, sigma: Option<f64>) -> Vec<Option<f64>> {
    let (_, lower_band) = bband_bands(data, period, sigma);
    lower_band
}

fn bband_bands(
    data: &[Option<f64>],
    period: usize,
    sigma: Option<f64>,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let sigma = sigma.unwrap_or(2.0);
    let (means, stddevs) = rolling_mean_stddev_strict(data, period);

    means
        .into_iter()
        .zip(stddevs)
        .map(|(mean, stddev)| match (mean, stddev) {
            (Some(mean), Some(stddev)) => {
                let deviation = sigma * stddev;
                (
                    Some(round_scalar(mean + deviation, 8)),
                    Some(round_scalar(mean - deviation, 8)),
                )
            }
            _ => (None, None),
        })
        .unzip()
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
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
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

    #[test]
    fn test_bband_with_interior_gap_invalidates_full_window() {
        let input = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];

        let (upper, middle, lower) = bband(&input, 2, Some(2.0));

        assert_eq!(middle, vec![None, Some(1.5), None, None, Some(4.5)]);
        assert_eq!(upper, vec![None, Some(2.5), None, None, Some(5.5)]);
        assert_eq!(lower, vec![None, Some(0.5), None, None, Some(3.5)]);
    }

    #[test]
    fn test_bband_full_window_invalidation() {
        let input = vec![None, Some(2.0), None, Some(4.0)];

        let (upper, middle, lower) = bband(&input, 2, None);

        assert_eq!(upper, vec![None, None, None, None]);
        assert_eq!(middle, vec![None, None, None, None]);
        assert_eq!(lower, vec![None, None, None, None]);
    }
}
