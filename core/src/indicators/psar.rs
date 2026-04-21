pub fn psar(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    increment: f64,
    initial_acceleration_factor: f64,
    max_acceleration_factor: f64,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut psar = vec![None; len];

    if len != lows.len() || len != closes.len() || len < 3 {
        return psar;
    }

    let mut direction = None;
    let mut extreme_point = None;
    let mut current_psar = None;
    let mut acceleration_factor = initial_acceleration_factor;
    let mut last_valid_highs = Vec::with_capacity(2);
    let mut last_valid_lows = Vec::with_capacity(2);
    let mut emitted_points = 0usize;

    for i in 1..len {
        if direction.is_none() {
            let (
                Some(prev_close),
                Some(close),
                Some(prev_low),
                Some(low),
                Some(prev_high),
                Some(high),
            ) = (
                closes[i - 1],
                closes[i],
                lows[i - 1],
                lows[i],
                highs[i - 1],
                highs[i],
            )
            else {
                continue;
            };

            let next_direction = (close > prev_close) as usize;
            direction = Some(next_direction);
            extreme_point = Some(if next_direction == 1 { high } else { low });
            current_psar = Some(if next_direction == 1 {
                prev_low
            } else {
                prev_high
            });
            last_valid_highs = vec![prev_high, high];
            last_valid_lows = vec![prev_low, low];
            continue;
        }

        let (
            Some(high),
            Some(low),
            Some(mut psar_point),
            Some(mut current_extreme),
            Some(mut current_direction),
        ) = (highs[i], lows[i], current_psar, extreme_point, direction)
        else {
            continue;
        };

        psar_point += acceleration_factor * (current_extreme - psar_point);

        if emitted_points > 0 && last_valid_highs.len() >= 2 && last_valid_lows.len() >= 2 {
            psar_point = if current_direction == 1 {
                psar_point.min(last_valid_lows[0].min(last_valid_lows[1]))
            } else {
                psar_point.max(last_valid_highs[0].max(last_valid_highs[1]))
            };
        }

        let is_direction_changed = if current_direction == 1 {
            low < psar_point
        } else {
            high > psar_point
        };

        if is_direction_changed {
            current_direction = 1 - current_direction;
            psar_point = current_extreme;
            current_extreme = if current_direction == 1 { high } else { low };
            acceleration_factor = initial_acceleration_factor;
        } else if (current_direction == 1 && high > current_extreme)
            || (current_direction == 0 && low < current_extreme)
        {
            current_extreme = if current_direction == 1 { high } else { low };
            acceleration_factor = (acceleration_factor + increment).min(max_acceleration_factor);
        }

        current_psar = Some(psar_point);
        extreme_point = Some(current_extreme);
        direction = Some(current_direction);
        psar[i] = Some(psar_point);
        emitted_points += 1;

        if last_valid_highs.len() == 2 {
            last_valid_highs.remove(0);
        }
        if last_valid_lows.len() == 2 {
            last_valid_lows.remove(0);
        }
        last_valid_highs.push(high);
        last_valid_lows.push(low);
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
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
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

    #[test]
    fn test_psar_gap_preserves_state_and_resumes_without_reseed() {
        let highs = vec![
            Some(10.0),
            Some(11.0),
            Some(12.0),
            None,
            Some(13.0),
            Some(14.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(9.0),
            Some(10.0),
            None,
            Some(11.0),
            Some(12.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(10.0),
            Some(11.0),
            None,
            Some(12.0),
            Some(13.0),
        ];

        let result = psar(&highs, &lows, &closes, 0.02, 0.02, 0.2);

        assert_eq!(
            result,
            vec![
                None,
                None,
                Some(8.06),
                None,
                Some(8.217600000000001),
                Some(8.504544000000001),
            ]
        );
    }
}
