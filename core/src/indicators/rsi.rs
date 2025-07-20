pub fn rsi(data: &[f64], period: usize) -> Vec<Option<f64>> {
    let mut rsi = vec![None; data.len()];

    if data.len() < period {
        return rsi;
    }

    let mut total_up = 0.0;
    let mut total_down = 0.0;
    let mut avg_up;
    let mut avg_down;

    for i in 1..period {
        let change = data[i] - data[i - 1];
        if change > 0.0 {
            total_up += change;
        } else {
            total_down += change.abs();
        }
    }

    avg_up = total_up / (period - 1) as f64;
    avg_down = total_down / (period - 1) as f64;

    for i in period..data.len() {
        let change = data[i] - data[i - 1];
        let up = change.max(0.0);
        let down = change.min(0.0).abs();
        avg_up = (avg_up * (period - 1) as f64 + up) / period as f64;
        avg_down = (avg_down * (period - 1) as f64 + down) / period as f64;

        let rsi_point = if avg_down == 0.0 {
            100.0
        } else if avg_up == 0.0 {
            0.0
        } else {
            (avg_up / (avg_up + avg_down)) * 100.0
        };

        rsi[i] = Some(rsi_point);
    }

    rsi
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_rsi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = rsi(&input, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/rsi_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                expected,
                "RSI test failed for symbol {}.",
                symbol
            );
        }
    }
}
