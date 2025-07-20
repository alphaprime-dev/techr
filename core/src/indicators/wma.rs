pub fn wma(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];

    if data.len() < period {
        return result;
    }

    let weight_sum = (period * (period + 1)) / 2;
    let mut weighted_sum = 0.0;

    // Initialize the first period
    for i in 0..period {
        weighted_sum += data[i] * (i + 1) as f64;
    }

    for i in period - 1..data.len() {
        result[i] = Some(weighted_sum / weight_sum as f64);
        if i + 1 < data.len() {
            weighted_sum = weighted_sum + data[i + 1] * period as f64
                - data[i + 1 - period..=i].iter().sum::<f64>();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_wma() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = wma(&input, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/wma_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                expected,
                "WMA test failed for symbol {}.",
                symbol
            );
        }
    }
}
