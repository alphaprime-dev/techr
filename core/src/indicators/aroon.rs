use crate::utils::rolling_argmax_argmin;

pub fn aroon(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let mut aroon_up = vec![None; highs.len()];
    let mut aroon_down = vec![None; lows.len()];

    if highs.len() != lows.len() || highs.len() < period {
        return (aroon_up, aroon_down);
    }

    let window = period + 1;
    let (max_indices, min_indices) = rolling_argmax_argmin(highs, lows, window);

    for i in period..highs.len() {
        let window_start = i - period;
        let (Some(max_index), Some(min_index)) = (max_indices[i], min_indices[i]) else {
            continue;
        };
        let max_index = max_index - window_start;
        let min_index = min_index - window_start;

        let aroon_up_point = (max_index as f64 * 100.0) / period as f64;
        let aroon_down_point = (min_index as f64 * 100.0) / period as f64;

        aroon_up[i] = Some(aroon_up_point);
        aroon_down[i] = Some(aroon_down_point);
    }

    (aroon_up, aroon_down)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_aroon() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let highs = highs.into_iter().map(Some).collect::<Vec<_>>();
            let lows = lows.into_iter().map(Some).collect::<Vec<_>>();
            let (aroon_up, aroon_down) = aroon(&highs, &lows, 25);

            let expected_up = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/aroon_up_{}.json",
                symbol
            ));
            let expected_down = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/aroon_down_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(aroon_up, 8),
                round_vec(expected_up, 8),
                "Aroon Up test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(aroon_down, 8),
                round_vec(expected_down, 8),
                "Aroon Down test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_aroon_with_gap_invalidates_window() {
        let highs = vec![Some(1.0), Some(5.0), None, Some(2.0)];
        let lows = vec![Some(4.0), Some(1.0), None, Some(3.0)];

        let (up, down) = aroon(&highs, &lows, 2);

        assert_eq!(up, vec![None, None, None, None]);
        assert_eq!(down, vec![None, None, None, None]);
    }
}
