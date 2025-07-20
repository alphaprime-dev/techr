use crate::indicators::ema::ema;

pub fn massi(
    highs: &[f64],
    lows: &[f64],
    period_ema: usize,
    period_sum: usize,
    period_signal: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = highs.len();
    let mut mass = vec![None; len];
    let mut signal = vec![None; len];

    if len < 2 * (period_ema - 1) + (period_sum - 1) + 1 {
        return (mass, signal);
    }

    let high_low_diffs: Vec<f64> = highs.iter().zip(lows.iter()).map(|(h, l)| h - l).collect();
    let s_ema = ema(&high_low_diffs, period_ema);
    let s_ema_filtered: Vec<f64> = s_ema.iter().filter_map(|&x| x).collect();
    let offset: usize = period_ema - 1;
    let d_ema = ema(&s_ema_filtered, period_ema);

    let mut ema_ratio = Vec::with_capacity(d_ema.len());
    for i in offset..d_ema.len() + offset {
        if let (Some(s), Some(d)) = (s_ema[i], d_ema[i - offset]) {
            ema_ratio.push(s / d);
        }
    }

    let mut ratio_sum = 0.0;
    for i in 0..ema_ratio.len() {
        ratio_sum += ema_ratio[i];
        if i >= period_sum - 1 {
            mass[i + 2 * offset] = Some(ratio_sum);
            ratio_sum -= ema_ratio[i - (period_sum - 1)];
        }
    }

    let mass_values: Vec<f64> = mass.iter().filter_map(|&x| x).collect();
    let signal_ema = ema(&mass_values, period_signal);
    let signal_offset = len - signal_ema.len();
    for (i, &s) in signal_ema.iter().enumerate() {
        signal[i + signal_offset] = s;
    }

    (mass, signal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_massi() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");

            let (mass, signal) = massi(&highs, &lows, 9, 25, 9);

            let expected_mass = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/massi_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/massi_signal_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(mass, 8),
                round_vec(expected_mass, 8),
                "MASSI mass test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(signal, 8),
                round_vec(expected_signal, 8),
                "MASSI signal test failed for symbol {}.",
                symbol
            );
        }
    }
}
