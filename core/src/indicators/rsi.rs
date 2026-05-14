use crate::indicators::ema::ema;

pub fn rsi(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut rsi = vec![None; data.len()];

    if data.len() < period || period <= 1 {
        return rsi;
    }

    let mut total_up = 0.0;
    let mut total_down = 0.0;
    let mut avg_up = None;
    let mut avg_down = None;
    let mut seeded_changes = 0usize;

    for i in 1..data.len() {
        let (Some(current), Some(prev)) = (data[i], data[i - 1]) else {
            if avg_up.is_none() {
                total_up = 0.0;
                total_down = 0.0;
                seeded_changes = 0;
            }
            continue;
        };

        let change = current - prev;

        if let (Some(current_avg_up), Some(current_avg_down)) = (avg_up, avg_down) {
            let up = change.max(0.0);
            let down = change.min(0.0).abs();
            let next_avg_up = (current_avg_up * (period - 1) as f64 + up) / period as f64;
            let next_avg_down = (current_avg_down * (period - 1) as f64 + down) / period as f64;
            avg_up = Some(next_avg_up);
            avg_down = Some(next_avg_down);

            let rsi_point = if next_avg_down == 0.0 {
                100.0
            } else if next_avg_up == 0.0 {
                0.0
            } else {
                (next_avg_up / (next_avg_up + next_avg_down)) * 100.0
            };

            rsi[i] = Some(rsi_point);
            continue;
        }

        if change > 0.0 {
            total_up += change;
        } else {
            total_down += change.abs();
        }

        seeded_changes += 1;
        if seeded_changes == period - 1 {
            avg_up = Some(total_up / (period - 1) as f64);
            avg_down = Some(total_down / (period - 1) as f64);
        }
    }

    rsi
}

pub fn rsi_signal(data: &[Option<f64>], period: usize, signal_period: usize) -> Vec<Option<f64>> {
    let line = rsi(data, period);
    ema(&line, signal_period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_rsi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let result = rsi(&input, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/rsi_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                expected,
                "RSI test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_rsi_with_interior_gap_resumes_from_prior_state() {
        let aligned = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(2.0),
            None,
            Some(4.0),
            Some(5.0),
        ];

        let result = rsi(&aligned, 3);

        assert_eq!(
            result,
            vec![
                None,
                None,
                None,
                Some(66.66666666666666),
                None,
                None,
                Some(77.77777777777779),
            ]
        );
    }

    #[test]
    fn test_rsi_requires_contiguous_seed_window() {
        let aligned = vec![
            Some(1.0),
            Some(2.0),
            None,
            Some(3.0),
            Some(4.0),
            Some(5.0),
            Some(4.0),
        ];

        let result = rsi(&aligned, 3);

        assert_eq!(
            result,
            vec![None, None, None, None, None, None, Some(66.66666666666666)]
        );
    }

    #[test]
    fn test_rsi_signal_follows_base_ema_contract() {
        let aligned = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(2.0),
            None,
            Some(4.0),
            Some(5.0),
            Some(6.0),
        ];

        let line = rsi(&aligned, 3);
        let signal = rsi_signal(&aligned, 3, 2);

        assert_eq!(signal, ema(&line, 2));
    }
}
