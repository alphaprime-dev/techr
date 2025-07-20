pub fn psl(closes: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut psl = vec![None; closes.len()];
    let len = closes.len();

    if len < period + 1 || period <= 1 {
        return psl;
    }

    let mut count = 0;

    // Count initial positive price changes
    for i in 1..period {
        if closes[i] > closes[i - 1] {
            count += 1;
        }
    }

    // Calculate PSL for the rest of the series
    for i in period..len {
        // Add current price change to the count
        if closes[i] > closes[i - 1] {
            count += 1;
        }

        // Calculate PSL value
        let psl_value = (count as f64 / period as f64) * 100.0;
        psl[i] = Some(psl_value);

        // Remove oldest price change from the count
        if closes[i - period + 1] > closes[i - period] {
            count -= 1;
        }
    }

    psl
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_psl() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = psl(&close, 12);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/psl_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "PSL test failed for symbol {}.",
                symbol
            );
        }
    }
}
