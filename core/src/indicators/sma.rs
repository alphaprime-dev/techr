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
