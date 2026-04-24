use std::collections::VecDeque;

pub fn round_scalar(value: f64, decimal_places: u32) -> f64 {
    let factor = 10.0f64.powi(decimal_places as i32);
    (value * factor).round() / factor
}

pub fn round_vec(vec: Vec<Option<f64>>, decimal_places: u32) -> Vec<Option<f64>> {
    vec.iter()
        .map(|&x| {
            x.map(|y| {
                let factor = 10.0f64.powi(decimal_places as i32);
                (y * factor).round() / factor
            })
        })
        .collect()
}

pub fn calc_mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    let count = data.len();
    sum / count as f64
}

pub fn find_max(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

pub fn find_min(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::INFINITY, f64::min)
}

/// Computes rolling max/min values over paired slices using a monotonic queue.
///
/// NaN inputs are skipped. If a full window contains no finite values for one side,
/// that side returns `None` for the window.
pub fn rolling_max_min(
    highs: &[f64],
    lows: &[f64],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = highs.len();
    let mut rolling_max = vec![None; len];
    let mut rolling_min = vec![None; len];

    if len < period || period == 0 {
        return (rolling_max, rolling_min);
    }

    let mut max_deque = VecDeque::with_capacity(period);
    let mut min_deque = VecDeque::with_capacity(period);

    for i in 0..len {
        push_max_index(&mut max_deque, highs, i);
        push_min_index(&mut min_deque, lows, i);

        if i + 1 < period {
            continue;
        }

        let earliest_idx = i + 1 - period;
        evict_expired(&mut max_deque, earliest_idx);
        evict_expired(&mut min_deque, earliest_idx);

        rolling_max[i] = max_deque.front().map(|&idx| highs[idx]);
        rolling_min[i] = min_deque.front().map(|&idx| lows[idx]);
    }

    (rolling_max, rolling_min)
}

/// Computes rolling argmax/argmin source indices over paired slices using a monotonic queue.
///
/// Returned indices refer to the original input slices. NaN inputs are skipped, so a
/// window with no finite values on one side returns `None` for that side.
pub fn rolling_argmax_argmin(
    highs: &[f64],
    lows: &[f64],
    period: usize,
) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let len = highs.len();
    let mut max_indices = vec![None; len];
    let mut min_indices = vec![None; len];

    if len < period || period == 0 {
        return (max_indices, min_indices);
    }

    let mut max_deque = VecDeque::with_capacity(period);
    let mut min_deque = VecDeque::with_capacity(period);

    for i in 0..len {
        push_max_index(&mut max_deque, highs, i);
        push_min_index(&mut min_deque, lows, i);

        if i + 1 < period {
            continue;
        }

        let earliest_idx = i + 1 - period;
        evict_expired(&mut max_deque, earliest_idx);
        evict_expired(&mut min_deque, earliest_idx);

        max_indices[i] = max_deque.front().copied();
        min_indices[i] = min_deque.front().copied();
    }

    (max_indices, min_indices)
}

/// Computes the midpoint of the rolling high/low channel for each full window.
pub fn rolling_midpoint(highs: &[f64], lows: &[f64], period: usize) -> Vec<Option<f64>> {
    let (rolling_max, rolling_min) = rolling_max_min(highs, lows, period);
    rolling_max
        .into_iter()
        .zip(rolling_min)
        .map(|(max_high, min_low)| match (max_high, min_low) {
            (Some(max_high), Some(min_low)) => Some((max_high + min_low) / 2.0),
            _ => None,
        })
        .collect()
}

fn rolling_mean_strict_impl(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = data.len();
    let mut means = vec![None; len];

    if len < period || period == 0 {
        return means;
    }

    let mut sum = 0.0;
    let mut valid_count = 0usize;

    for i in 0..len {
        if let Some(value) = data[i] {
            sum += value;
            valid_count += 1;
        }

        if i >= period {
            if let Some(value) = data[i - period] {
                sum -= value;
                valid_count -= 1;
            }
        }

        if i >= period - 1 && valid_count == period {
            means[i] = Some(sum / period as f64);
        }
    }

    means
}

/// Computes a rolling mean that only emits a value when the full window contains valid values.
pub fn rolling_mean_strict(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    rolling_mean_strict_impl(data, period)
}

/// Computes a rolling weighted mean that only emits a value when the full window is valid.
fn rolling_weighted_mean_strict_impl(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = data.len();
    let mut result = vec![None; len];

    if len < period || period == 0 {
        return result;
    }

    let weight_sum = (period * (period + 1)) as f64 / 2.0;
    let mut sum = 0.0;
    let mut weighted_sum = 0.0;
    let mut valid_count = 0usize;

    for i in 0..period {
        if let Some(value) = data[i] {
            sum += value;
            weighted_sum += value * (i + 1) as f64;
            valid_count += 1;
        }
    }

    if valid_count == period {
        result[period - 1] = Some(weighted_sum / weight_sum);
    }

    for end in period..len {
        let entering = data[end].unwrap_or(0.0);
        let leaving = data[end - period].unwrap_or(0.0);

        weighted_sum = weighted_sum - sum + entering * period as f64;
        sum += entering - leaving;

        if data[end].is_some() {
            valid_count += 1;
        }
        if data[end - period].is_some() {
            valid_count -= 1;
        }

        if valid_count == period {
            result[end] = Some(weighted_sum / weight_sum);
        }
    }

    result
}

/// Computes a rolling weighted mean that only emits a value when the full window is valid.
pub fn rolling_weighted_mean_strict(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    rolling_weighted_mean_strict_impl(data, period)
}

/// Computes rolling mean and standard deviation for fully valid windows only.
fn rolling_mean_stddev_strict_impl(
    data: &[Option<f64>],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = data.len();
    let mut means = vec![None; len];
    let mut stddevs = vec![None; len];

    if len < period || period == 0 {
        return (means, stddevs);
    }

    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut valid_count = 0usize;

    for i in 0..len {
        if let Some(value) = data[i] {
            sum += value;
            sum_sq += value * value;
            valid_count += 1;
        }

        if i >= period {
            if let Some(value) = data[i - period] {
                sum -= value;
                sum_sq -= value * value;
                valid_count -= 1;
            }
        }

        if i >= period - 1 && valid_count == period {
            let mean = sum / period as f64;
            let variance = (sum_sq / period as f64) - (mean * mean);
            means[i] = Some(mean);
            stddevs[i] = Some(variance.max(0.0).sqrt());
        }
    }

    (means, stddevs)
}

/// Computes rolling mean and standard deviation for fully valid windows only.
pub fn rolling_mean_stddev_strict(
    data: &[Option<f64>],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    rolling_mean_stddev_strict_impl(data, period)
}

fn push_max_index(deque: &mut VecDeque<usize>, data: &[f64], idx: usize) {
    if data[idx].is_nan() {
        return;
    }

    while let Some(&back) = deque.back() {
        if data[back] <= data[idx] {
            deque.pop_back();
        } else {
            break;
        }
    }
    deque.push_back(idx);
}

fn push_min_index(deque: &mut VecDeque<usize>, data: &[f64], idx: usize) {
    if data[idx].is_nan() {
        return;
    }

    while let Some(&back) = deque.back() {
        if data[back] >= data[idx] {
            deque.pop_back();
        } else {
            break;
        }
    }
    deque.push_back(idx);
}

fn evict_expired(deque: &mut VecDeque<usize>, earliest_idx: usize) {
    while let Some(&front) = deque.front() {
        if front < earliest_idx {
            deque.pop_front();
        } else {
            break;
        }
    }
}

pub fn forward_shift<T>(values: Vec<Option<T>>, period: usize) -> Vec<Option<T>> {
    let mut shifted: Vec<Option<T>> = (0..values.len() + period - 1).map(|_| None).collect();

    for (idx, value) in values.into_iter().enumerate() {
        if let Some(value) = value {
            shifted[idx + period - 1] = Some(value);
        }
    }

    shifted
}

pub fn calc_clv(high: f64, low: f64, close: f64) -> f64 {
    if high == low {
        0.0
    } else {
        ((close - low) - (high - close)) / (high - low)
    }
}

pub fn calc_true_ranges(highs: &[f64], lows: &[f64], closes: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(highs.len() - 1);

    for i in 1..highs.len() {
        let high = highs[i];
        let low = lows[i];
        let prev_close = closes[i - 1];
        result.push(calc_tr(high, low, prev_close));
    }

    result
}

fn calc_tr(high: f64, low: f64, prev_close: f64) -> f64 {
    let th = high.max(prev_close);
    let tl = low.min(prev_close);
    th - tl
}

pub fn wilders_smoothing(data: &[f64], period: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(data.len() - period + 1);
    let mut partial_sum: f64 = data.iter().take(period - 1).sum();

    for i in period - 1..data.len() {
        partial_sum = partial_sum - (partial_sum / period as f64) + data[i];
        result.push(partial_sum);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        let test_cases = vec![
            (1.23456, 2, 1.23),
            (7.891011, 2, 7.89),
            (12.345678, 2, 12.35),
            (3.14159, 3, 3.142),
            (100.0, 1, 100.0),
            (0.001, 1, 0.0),
        ];

        for (input, decimal_places, expected) in test_cases {
            let result = round_scalar(input, decimal_places);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_round_vec() {
        let test_cases = vec![
            (
                vec![Some(1.23456), Some(7.891011), None, Some(12.345678)], // input
                2,                                                          // decimal_places
                vec![Some(1.23), Some(7.89), None, Some(12.35)],            // expected
            ),
            (
                vec![Some(3.14159), Some(2.71828), Some(1.41421)],
                3,
                vec![Some(3.142), Some(2.718), Some(1.414)],
            ),
            (
                vec![Some(100.0), Some(0.001), Some(10.0)],
                1,
                vec![Some(100.0), Some(0.0), Some(10.0)],
            ),
        ];

        for (input, decimal_places, expected) in test_cases {
            let result = round_vec(input, decimal_places);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_calc_mean() {
        let result = calc_mean(&vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_find_max() {
        let result = find_max(&vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_find_min() {
        let result = find_min(&vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_rolling_midpoint() {
        let highs = vec![10.0, 12.0, 14.0, 16.0, 18.0];
        let lows = vec![4.0, 6.0, 8.0, 10.0, 12.0];

        let result = rolling_midpoint(&highs, &lows, 3);

        assert_eq!(result, vec![None, None, Some(9.0), Some(11.0), Some(13.0)]);
    }

    #[test]
    fn test_rolling_max_min() {
        let highs = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let lows = vec![5.0, 2.0, 3.0, 1.0, 4.0];

        let (rolling_max, rolling_min) = rolling_max_min(&highs, &lows, 3);

        assert_eq!(
            rolling_max,
            vec![None, None, Some(3.0), Some(5.0), Some(5.0)]
        );
        assert_eq!(
            rolling_min,
            vec![None, None, Some(2.0), Some(1.0), Some(1.0)]
        );
    }

    #[test]
    fn test_rolling_argmax_argmin_prefers_latest_duplicate() {
        let highs = vec![1.0, 5.0, 5.0, 2.0];
        let lows = vec![4.0, 1.0, 1.0, 3.0];

        let (max_indices, min_indices) = rolling_argmax_argmin(&highs, &lows, 3);

        assert_eq!(max_indices, vec![None, None, Some(2), Some(2)]);
        assert_eq!(min_indices, vec![None, None, Some(2), Some(2)]);
    }

    #[test]
    fn test_rolling_max_min_ignores_nan_when_finite_values_exist() {
        let highs = vec![1.0, f64::NAN, 3.0];
        let lows = vec![5.0, f64::NAN, 2.0];

        let (rolling_max, rolling_min) = rolling_max_min(&highs, &lows, 2);

        assert_eq!(rolling_max, vec![None, Some(1.0), Some(3.0)]);
        assert_eq!(rolling_min, vec![None, Some(5.0), Some(2.0)]);
    }

    #[test]
    fn test_rolling_argmax_argmin_ignore_nan_when_finite_values_exist() {
        let highs = vec![1.0, f64::NAN, 5.0];
        let lows = vec![4.0, f64::NAN, 1.0];

        let (max_indices, min_indices) = rolling_argmax_argmin(&highs, &lows, 2);

        assert_eq!(max_indices, vec![None, Some(0), Some(2)]);
        assert_eq!(min_indices, vec![None, Some(0), Some(2)]);
    }

    #[test]
    fn test_rolling_max_min_all_nan_window_returns_none() {
        let highs = vec![f64::NAN, f64::NAN, f64::NAN];
        let lows = vec![f64::NAN, f64::NAN, f64::NAN];

        let (rolling_max, rolling_min) = rolling_max_min(&highs, &lows, 2);
        let (max_indices, min_indices) = rolling_argmax_argmin(&highs, &lows, 2);

        assert_eq!(rolling_max, vec![None, None, None]);
        assert_eq!(rolling_min, vec![None, None, None]);
        assert_eq!(max_indices, vec![None, None, None]);
        assert_eq!(min_indices, vec![None, None, None]);
    }

    #[test]
    fn test_rolling_mean_strict() {
        let data = vec![Some(1.0), Some(3.0), None, Some(5.0), Some(7.0)];

        let means = rolling_mean_strict(&data, 2);

        assert_eq!(means, vec![None, Some(2.0), None, None, Some(6.0)]);
    }

    #[test]
    fn test_rolling_weighted_mean_strict() {
        let data = vec![None, Some(1.0), Some(2.0), None, Some(4.0)];

        let means = rolling_weighted_mean_strict(&data, 2);

        assert_eq!(means, vec![None, None, Some(5.0 / 3.0), None, None]);
    }

    #[test]
    fn test_rolling_mean_stddev_strict() {
        let data = vec![Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)];

        let (means, stddevs) = rolling_mean_stddev_strict(&data, 2);

        assert_eq!(means, vec![None, Some(1.5), None, None, Some(4.5)]);
        assert_eq!(stddevs, vec![None, Some(0.5), None, None, Some(0.5)]);
    }

    #[test]
    fn test_forward_shift() {
        let values = vec![Some(1.0), None, Some(3.0)];

        let result = forward_shift(values, 2);

        assert_eq!(result, vec![None, Some(1.0), None, Some(3.0)]);
    }

    #[test]
    fn test_calc_clv() {
        let test_cases = vec![
            (1.0, 2.0, 3.0, -3.0),
            (1.0, 2.0, 1.0, 1.0),
            (1.0, 1.0, 1.0, 0.0),
        ];
        for (high, low, close, expected) in test_cases {
            let result = calc_clv(high, low, close);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_calc_true_ranges() {
        let highs = vec![
            10.0, 12.0, 11.5, 13.0, 14.5, 13.5, 15.0, 16.0, 15.5, 17.0, 18.0, 17.5, 19.0, 20.0,
            19.5, 21.0, 22.0, 21.5, 23.0, 24.0, 23.5,
        ];
        let lows = vec![
            9.0, 10.5, 10.0, 11.5, 13.0, 12.0, 13.5, 14.5, 14.0, 15.5, 16.5, 16.0, 17.5, 18.5,
            18.0, 19.5, 20.5, 20.0, 21.5, 22.5, 22.0,
        ];
        let closes = vec![
            9.5, 11.5, 10.5, 12.5, 14.0, 13.0, 14.5, 15.5, 15.0, 16.5, 17.5, 17.0, 18.5, 19.5,
            19.0, 20.5, 21.5, 21.0, 22.5, 23.5, 23.0,
        ];

        let expected = vec![
            2.5, 1.5, 2.5, 2.0, 2.0, 2.0, 1.5, 1.5, 2.0, 1.5, 1.5, 2.0, 1.5, 1.5, 2.0, 1.5, 1.5,
            2.0, 1.5, 1.5,
        ];

        let result = calc_true_ranges(&highs, &lows, &closes);
        assert_eq!(result, expected, "Failed for dynamic input");
    }

    #[test]
    fn test_calc_wilders_smoothing() {
        // Using extended data for a more robust test case.
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let period = 3;
        let result = wilders_smoothing(&data, period);

        let expected = vec![
            Some(5.0),
            Some(22.0 / 3.0),
            Some(89.0 / 9.0),
            Some(340.0 / 27.0),
            Some(1247.0 / 81.0),
        ];
        assert_eq!(
            round_vec(result.into_iter().map(Some).collect(), 8),
            round_vec(expected, 8)
        );
    }
}
