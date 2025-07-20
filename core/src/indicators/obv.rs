use crate::indicators::ema::ema;

pub fn obv(
    data: &[f64],
    volumes: &[f64],
    signal_period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    let obv_line = calc_obv_line(data, volumes);
    let obv_signal = obv_signal_by_obvline(&obv_line, signal_period);

    (obv_line, obv_signal)
}

fn calc_obv_line(data: &[f64], volumes: &[f64]) -> Vec<Option<f64>> {
    let mut obv = vec![None; data.len()];

    let len = data.len();

    if len == 0 || len != volumes.len() {
        return obv;
    }

    let mut obv_point = volumes[0];
    obv[0] = Some(obv_point);

    for i in 1..len {
        let current_close = data[i];
        let prev_close = data[i - 1];
        let volume = volumes[i];

        let increment = if prev_close < current_close {
            volume
        } else if prev_close > current_close {
            -volume
        } else {
            0.0
        };

        obv_point += increment;
        obv[i] = Some(obv_point);
    }

    obv
}

fn obv_signal_by_obvline(obv_line: &[Option<f64>], signal_period: usize) -> Vec<Option<f64>> {
    let mut obv_signal = vec![None; obv_line.len()];

    let null_count = obv_line.iter().take_while(|&&x| x.is_none()).count();
    let obv_values = obv_line
        .iter()
        .skip(null_count)
        .filter_map(|&v| v)
        .collect::<Vec<f64>>();
    let ema_obv = ema(&obv_values, signal_period);

    for i in 0..ema_obv.len() {
        if let Some(ema_value) = ema_obv[i] {
            obv_signal[i + null_count] = Some(ema_value);
        }
    }

    obv_signal
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_obv() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v");

            let (line, signal) = obv(&close, &volume, 9);
            let expected_line = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/obv_line_{}.json",
                symbol
            ));
            let expected_signal = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/obv_signal_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(line, 4),
                round_vec(expected_line, 4),
                "OBV test failed for symbol {}.",
                symbol
            );

            assert_eq!(
                round_vec(signal, 4),
                round_vec(expected_signal, 4),
                "OBV test failed for symbol {}.",
                symbol
            );
        }
    }
}
