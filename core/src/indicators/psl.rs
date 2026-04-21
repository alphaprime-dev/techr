use crate::utils::rolling_sum_strict;

pub fn psl(closes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut psl = vec![None; closes.len()];
    let len = closes.len();

    if len < period + 1 || period <= 1 {
        return psl;
    }

    let changes = closes
        .windows(2)
        .map(|window| match (window[0], window[1]) {
            (Some(prev), Some(current)) => Some(if current > prev { 1.0 } else { 0.0 }),
            _ => None,
        })
        .collect::<Vec<_>>();

    let sums = rolling_sum_strict(&changes, period);
    for i in period..len {
        if let Some(sum) = sums[i - 1] {
            psl[i] = Some((sum / period as f64) * 100.0);
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
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
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
    fn test_psl_with_gap_invalidates_change_window() {
        let close = vec![Some(1.0), Some(2.0), Some(3.0), None, Some(4.0), Some(5.0)];

        let result = psl(&close, 2);

        assert_eq!(result, vec![None, None, Some(100.0), None, None, None]);
    }
}
