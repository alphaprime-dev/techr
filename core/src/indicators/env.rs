use crate::indicators::sma::sma;

pub fn env(
    data: &[Option<f64>],
    period: usize,
    shift_percentage: f64,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = data.len();
    let mut env_upper = vec![None; len];
    let mut env_lower = vec![None; len];
    let sma_values = sma(data, period);

    if len < period || period == 0 {
        return (env_upper, sma_values, env_lower);
    }

    for i in 0..len {
        if let Some(sma_value) = sma_values[i] {
            env_upper[i] = Some(sma_value * (1.0 + shift_percentage / 100.0));
            env_lower[i] = Some(sma_value * (1.0 - shift_percentage / 100.0));
        }
    }

    (env_upper, sma_values, env_lower)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{round_vec, testutils};

    #[test]
    fn test_env() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let result = env(&input, 20, 10.0);

            let (env_upper, sma_values, env_lower) = result;

            let expected_upper: Vec<Option<f64>> =
                testutils::load_expected(&format!("../data/expected/env_upper_{}.json", symbol));
            let expected_lower: Vec<Option<f64>> =
                testutils::load_expected(&format!("../data/expected/env_lower_{}.json", symbol));
            let expected_middle = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sma_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(env_upper, 8),
                round_vec(expected_upper, 8),
                "ENV upper test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(sma_values, 8),
                round_vec(expected_middle, 8),
                "ENV middle (SMA) test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(env_lower, 8),
                round_vec(expected_lower, 8),
                "ENV lower test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_env_with_interior_gap_invalidates_window() {
        let input = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];

        let (upper, middle, lower) = env(&input, 2, 10.0);

        assert_eq!(middle, vec![None, Some(1.5), None, None, Some(4.5)]);
        assert_eq!(
            round_vec(upper, 8),
            vec![None, Some(1.65), None, None, Some(4.95)]
        );
        assert_eq!(
            round_vec(lower, 8),
            vec![None, Some(1.35), None, None, Some(4.05)]
        );
    }

    #[test]
    fn test_env_full_window_invalidation() {
        let input = vec![None, Some(2.0), None, Some(4.0)];

        let (upper, middle, lower) = env(&input, 2, 10.0);

        assert_eq!(upper, vec![None, None, None, None]);
        assert_eq!(middle, vec![None, None, None, None]);
        assert_eq!(lower, vec![None, None, None, None]);
    }
}
