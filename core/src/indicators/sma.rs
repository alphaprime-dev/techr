pub fn sma(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut sma = vec![None; data.len()];
    let mut sum = 0.0;

    if data.len() < period {
        return sma;
    }

    for i in 0..data.len() {
        sum += data[i];
        if i >= period {
            sum -= data[i - period];
        }
        if i >= period - 1 {
            sma[i] = Some(sum / period as f64);
        }
    }

    sma
}

pub(crate) fn sma_aligned(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];
    let Some(first_valid_idx) = data.iter().position(|value| value.is_some()) else {
        return result;
    };

    let valid_len = data.len() - first_valid_idx;
    if period == 0 || valid_len < period {
        return result;
    }

    let mut sum = 0.0;
    for value in data.iter().skip(first_valid_idx).take(period) {
        sum += value.expect("initial SMA window must be fully populated");
    }

    let first_signal_idx = first_valid_idx + period - 1;
    result[first_signal_idx] = Some(sum / period as f64);

    for idx in (first_signal_idx + 1)..data.len() {
        let entering =
            data[idx].expect("aligned SMA input must be contiguous after the first value");
        let leaving =
            data[idx - period].expect("aligned SMA input must be contiguous after the first value");
        sum += entering - leaving;
        result[idx] = Some(sum / period as f64);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_sma() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = sma(&input, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/sma_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                expected,
                "SMA test failed for symbol {}.",
                symbol
            );
        }
    }
}
