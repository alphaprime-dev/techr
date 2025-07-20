pub fn psar(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    increment: f64,
    initial_acceleration_factor: f64,
    max_acceleration_factor: f64,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut psar = vec![None; len];

    if len != lows.len() || len != closes.len() || len < 3 {
        return psar;
    }

    let mut direction = (closes[1] > closes[0]) as usize;
    let mut extreme_point = if direction == 1 { highs[1] } else { lows[1] };
    let mut current_psar = if direction == 1 { lows[0] } else { highs[0] };
    let mut acceleration_factor = initial_acceleration_factor;

    for i in 2..len {
        current_psar += acceleration_factor * (extreme_point - current_psar);

        if i >= 3 {
            current_psar = if direction == 1 {
                current_psar.min(lows[i - 1].min(lows[i - 2]))
            } else {
                current_psar.max(highs[i - 1].max(highs[i - 2]))
            };
        }

        let is_direction_changed = if direction == 1 {
            lows[i] < current_psar
        } else {
            highs[i] > current_psar
        };

        if is_direction_changed {
            direction = 1 - direction;
            current_psar = extreme_point;
            extreme_point = if direction == 1 { highs[i] } else { lows[i] };
            acceleration_factor = initial_acceleration_factor;
        } else if (direction == 1 && highs[i] > extreme_point)
            || (direction == 0 && lows[i] < extreme_point)
        {
            extreme_point = if direction == 1 { highs[i] } else { lows[i] };
            acceleration_factor = (acceleration_factor + increment).min(max_acceleration_factor);
        }

        psar[i] = Some(current_psar);
    }

    psar
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_psar() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = psar(&high, &low, &close, 0.02, 0.02, 0.2);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/psar_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "PSAR test failed for symbol {}.",
                symbol
            );
        }
    }
}
