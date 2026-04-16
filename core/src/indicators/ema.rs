pub fn ema(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];

    if data.len() < period {
        return result;
    }

    let alpha = 2.0 / (period as f64 + 1.0);
    let mut ema = data[..period].iter().sum::<f64>() / period as f64;

    result[period - 1] = Some(ema);

    for i in period..data.len() {
        ema = alpha * data[i] + (1.0 - alpha) * ema;
        result[i] = Some(ema);
    }

    result
}

pub(crate) fn ema_aligned(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];
    let Some(first_valid_idx) = data.iter().position(|value| value.is_some()) else {
        return result;
    };

    let valid_len = data.len() - first_valid_idx;
    if period == 0 || valid_len < period {
        return result;
    }

    let first_signal_idx = first_valid_idx + period - 1;
    let alpha = 2.0 / (period as f64 + 1.0);
    let mut ema = mean_window(data, first_valid_idx, period);

    result[first_signal_idx] = Some(ema);

    for (idx, value) in data.iter().enumerate().skip(first_signal_idx + 1) {
        let value = value.expect("aligned EMA input must be contiguous after the first value");
        ema = alpha * value + (1.0 - alpha) * ema;
        result[idx] = Some(ema);
    }

    result
}

fn mean_window(data: &[Option<f64>], start_idx: usize, period: usize) -> f64 {
    data[start_idx..start_idx + period]
        .iter()
        .map(|value| value.expect("initial EMA window must be fully populated"))
        .sum::<f64>()
        / period as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_ema() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = ema(&input, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ema_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "EMA test failed for symbol {}.",
                symbol
            );
        }
    }
}
