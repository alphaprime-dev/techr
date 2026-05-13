struct EmaState {
    period: usize,
    alpha: f64,
    seeded_count: usize,
    seed_sum: f64,
    value: Option<f64>,
}

impl EmaState {
    fn new(period: usize) -> Self {
        let alpha = if period == 0 {
            0.0
        } else {
            2.0 / (period as f64 + 1.0)
        };

        Self {
            period,
            alpha,
            seeded_count: 0,
            seed_sum: 0.0,
            value: None,
        }
    }

    fn update(&mut self, item: Option<f64>) -> Option<f64> {
        if self.period == 0 {
            return None;
        }

        let Some(value) = item else {
            if self.value.is_none() {
                self.seeded_count = 0;
                self.seed_sum = 0.0;
            }
            return None;
        };

        if let Some(current) = self.value {
            let next = self.alpha * value + (1.0 - self.alpha) * current;
            self.value = Some(next);
            return Some(next);
        }

        self.seed_sum += value;
        self.seeded_count += 1;
        if self.seeded_count == self.period {
            let initial = self.seed_sum / self.period as f64;
            self.value = Some(initial);
            return Some(initial);
        }

        None
    }
}

pub fn trix(
    data: &[Option<f64>],
    period: usize,
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = data.len();
    let mut line = Vec::with_capacity(len);
    let mut signal = Vec::with_capacity(len);

    let mut ema = EmaState::new(period);
    let mut double_ema = EmaState::new(period);
    let mut triple_ema = EmaState::new(period);
    let mut signal_ema = EmaState::new(signal_period);
    let mut previous_triple_ema = None;

    for &value in data {
        let current_ema = ema.update(value);
        let current_double_ema = double_ema.update(current_ema);
        let current_triple_ema = triple_ema.update(current_double_ema);
        let current_line = match (current_triple_ema, previous_triple_ema) {
            (Some(current), Some(previous)) if previous != 0.0 => {
                Some((current - previous) * 100.0 / previous)
            }
            _ => None,
        };

        line.push(current_line);
        signal.push(signal_ema.update(current_line));
        previous_triple_ema = current_triple_ema;
    }

    (line, signal)
}

pub fn trix_line(data: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let len = data.len();
    let mut line = Vec::with_capacity(len);

    let mut ema = EmaState::new(period);
    let mut double_ema = EmaState::new(period);
    let mut triple_ema = EmaState::new(period);
    let mut previous_triple_ema = None;

    for &value in data {
        let current_ema = ema.update(value);
        let current_double_ema = double_ema.update(current_ema);
        let current_triple_ema = triple_ema.update(current_double_ema);
        let current_line = match (current_triple_ema, previous_triple_ema) {
            (Some(current), Some(previous)) if previous != 0.0 => {
                Some((current - previous) * 100.0 / previous)
            }
            _ => None,
        };

        line.push(current_line);
        previous_triple_ema = current_triple_ema;
    }

    line
}

pub fn trix_signal(data: &[Option<f64>], period: usize, signal_period: usize) -> Vec<Option<f64>> {
    let (_, signal) = trix(data, period, signal_period);
    signal
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::ema::ema;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_trix() {
        let test_cases = vec!["005930", "TSLA"];

        for symbol in test_cases {
            let input = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let (line, signal) = trix(&input, 12, 9);

            let expected_line = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/trix_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/trix_signal_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(line, 8),
                round_vec(expected_line, 8),
                "TRIX line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal, 8),
                round_vec(expected_signal, 8),
                "TRIX signal test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_trix_matches_composed_ema_across_gaps() {
        let input = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            None,
            Some(5.0),
            Some(6.0),
            Some(7.0),
            Some(8.0),
            Some(9.0),
        ];

        let line = trix_line(&input, 2);
        let single = ema(&input, 2);
        let double = ema(&single, 2);
        let triple = ema(&double, 2);
        let expected = triple
            .iter()
            .enumerate()
            .map(|(idx, &current)| {
                if idx == 0 {
                    return None;
                }

                match (current, triple[idx - 1]) {
                    (Some(current), Some(previous)) if previous != 0.0 => {
                        Some((current - previous) * 100.0 / previous)
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(round_vec(line, 8), round_vec(expected, 8));
    }

    #[test]
    fn test_trix_signal_follows_base_ema_contract_across_gaps() {
        let input = vec![
            Some(1.0),
            Some(2.0),
            Some(3.0),
            Some(4.0),
            None,
            Some(5.0),
            Some(6.0),
            Some(7.0),
            Some(8.0),
            Some(9.0),
        ];

        let (line, signal) = trix(&input, 2, 2);

        assert_eq!(signal, ema(&line, 2));
        assert_eq!(signal, trix_signal(&input, 2, 2));
    }

    #[test]
    fn test_trix_returns_none_when_previous_triple_ema_is_zero() {
        let input = vec![Some(0.0), Some(0.0), Some(1.0), Some(2.0)];

        let line = trix_line(&input, 1);

        assert_eq!(line, vec![None, None, None, Some(100.0)]);
    }
}
