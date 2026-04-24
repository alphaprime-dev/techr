use crate::utils::rolling_sum_strict;

pub fn psl(closes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len < period + 1 || period <= 1 {
        return result;
    }

    let mut positive_changes = vec![None; len];
    for i in 1..len {
        if let (Some(current), Some(previous)) = (closes[i], closes[i - 1]) {
            positive_changes[i] = Some(if current > previous { 1.0 } else { 0.0 });
        }
    }

    let counts = rolling_sum_strict(&positive_changes, period);
    for i in 0..len {
        if let Some(count) = counts[i] {
            result[i] = Some((count / period as f64) * 100.0);
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
    fn test_psl() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");
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

    #[test]
    fn test_psl_gap_invalidates_until_full_change_window_recovers() {
        let closes = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            None,
            Some(5.0),
            Some(4.0),
            Some(6.0),
        ];

        let result = psl(&closes, 2);

        assert_eq!(
            result,
            vec![None, None, Some(100.0), None, None, None, Some(50.0)]
        );
    }
}
