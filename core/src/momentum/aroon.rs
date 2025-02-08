use crate::utils::{find_max, find_min};

pub fn aroon(highs: &[f64], lows: &[f64], period: usize) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let mut aroon_up = vec![None; highs.len()];
    let mut aroon_down = vec![None; lows.len()];

    if highs.len() < period {
        return (aroon_up, aroon_down);
    }

    for i in period..highs.len() {
        let high_slice = &highs[i - period..=i];
        let low_slice = &lows[i - period..=i];
        let max_high = find_max(high_slice);
        let min_low = find_min(low_slice);

        let max_index = high_slice.iter().rposition(|&x| x == max_high).unwrap();
        let min_index = low_slice.iter().rposition(|&x| x == min_low).unwrap();

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
}
