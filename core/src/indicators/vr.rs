/// Volume Ratio (VR)
pub fn vr(closes: &[Option<f64>], volumes: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut result = vec![None; len];

    if len != volumes.len() || len < period + 1 {
        return result;
    }

    let mut up_values = vec![0.0; len];
    let mut down_values = vec![0.0; len];
    let mut same_values = vec![0.0; len];
    let mut valid = vec![false; len];
    let mut up_sum = 0.0;
    let mut down_sum = 0.0;
    let mut same_sum = 0.0;
    let mut valid_count = 0usize;

    for i in 1..len {
        let (Some(close), Some(prev_close), Some(volume)) = (closes[i], closes[i - 1], volumes[i])
        else {
            if i >= period {
                up_sum -= up_values[i - period];
                down_sum -= down_values[i - period];
                same_sum -= same_values[i - period];
                valid_count -= valid[i - period] as usize;
            }
            continue;
        };

        if close > prev_close {
            up_values[i] = volume;
        } else if close < prev_close {
            down_values[i] = volume;
        } else {
            same_values[i] = volume;
        }
        valid[i] = true;
        up_sum += up_values[i];
        down_sum += down_values[i];
        same_sum += same_values[i];
        valid_count += 1;

        if i >= period {
            up_sum -= up_values[i - period];
            down_sum -= down_values[i - period];
            same_sum -= same_values[i - period];
            valid_count -= valid[i - period] as usize;
        }

        if valid_count == period {
            result[i] = Some(calculate_vr(up_sum, down_sum, same_sum));
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
