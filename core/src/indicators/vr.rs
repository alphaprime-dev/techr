/// Volume Ratio (VR)
pub fn vr(closes: &[f64], volumes: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len != volumes.len() || len < period + 1 {
        return result;
    }

    let mut up_volume = 0.0;
    let mut down_volume = 0.0;
    let mut same_volume = 0.0;

    // Initialize volumes for the first period
    for i in 1..period {
        update_volumes(
            closes[i] - closes[i - 1],
            volumes[i],
            &mut up_volume,
            &mut down_volume,
            &mut same_volume,
        );
    }

    // Calculate VR for each point after the initial period
    for i in period..len {
        update_volumes(
            closes[i] - closes[i - 1],
            volumes[i],
            &mut up_volume,
            &mut down_volume,
            &mut same_volume,
        );

        result[i] = Some(calculate_vr(up_volume, down_volume, same_volume));

        // Adjust volumes by removing the oldest value
        update_volumes(
            closes[i - period + 1] - closes[i - period],
            -volumes[i - period + 1],
            &mut up_volume,
            &mut down_volume,
            &mut same_volume,
        );
    }

    result
}

#[inline]
fn update_volumes(diff: f64, volume: f64, up: &mut f64, down: &mut f64, same: &mut f64) {
    if diff > 0.0 {
        *up += volume;
    } else if diff < 0.0 {
        *down += volume;
    } else {
        *same += volume;
    }
}

#[inline]
fn calculate_vr(up: f64, down: f64, same: f64) -> f64 {
    let denominator = down + same * 0.5;
    if denominator == 0.0 {
        100.0
    } else {
        ((up + same * 0.5) / denominator) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_vr() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v");
            let result = vr(&close, &volume, 20);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/vr_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "VR test failed for symbol {}.",
                symbol
            );
        }
    }
}
