use crate::utils::rolling_sum_strict;

/// Volume Ratio (VR)
pub fn vr(closes: &[Option<f64>], volumes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len != volumes.len() || len < period + 1 {
        return result;
    }

    let mut up = vec![None; len];
    let mut down = vec![None; len];
    let mut same = vec![None; len];

    for i in 1..len {
        let (Some(close), Some(prev_close), Some(volume)) = (closes[i], closes[i - 1], volumes[i])
        else {
            continue;
        };

        if close > prev_close {
            up[i] = Some(volume);
            down[i] = Some(0.0);
            same[i] = Some(0.0);
        } else if close < prev_close {
            up[i] = Some(0.0);
            down[i] = Some(volume);
            same[i] = Some(0.0);
        } else {
            up[i] = Some(0.0);
            down[i] = Some(0.0);
            same[i] = Some(volume);
        }
    }

    let up_sum = rolling_sum_strict(&up, period);
    let down_sum = rolling_sum_strict(&down, period);
    let same_sum = rolling_sum_strict(&same, period);

    for i in 0..len {
        if let (Some(up_volume), Some(down_volume), Some(same_volume)) =
            (up_sum[i], down_sum[i], same_sum[i])
        {
            result[i] = Some(calculate_vr(up_volume, down_volume, same_volume));
        }
    }

    result
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
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
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

    #[test]
    fn test_vr_gap_invalidates_window_until_full_pairwise_window_returns() {
        let closes = vec![
            Some(10.0),
            Some(11.0),
            Some(10.0),
            None,
            Some(10.0),
            Some(11.0),
        ];
        let volumes = vec![Some(1.0), Some(3.0), Some(2.0), None, Some(4.0), Some(5.0)];

        let result = vr(&closes, &volumes, 2);

        assert_eq!(result, vec![None, None, Some(150.0), None, None, None]);
    }
}
