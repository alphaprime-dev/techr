use crate::indicators::adx::adx;

pub fn adxr(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    dmi_period: usize,
    adx_period: usize,
    adxr_period: usize,
) -> Vec<Option<f64>> {
    let mut adxr = vec![None; highs.len()];

    let adx_values = adx(highs, lows, closes, dmi_period, adx_period);
    let initial_period = dmi_period + adx_period + adxr_period - 2;

    for i in initial_period..adxr.len() {
        if let (Some(current_adx), Some(past_adx)) =
            (adx_values[i], adx_values[i - adxr_period + 1])
        {
            let adxr_point = (current_adx + past_adx) / 2.0;
            adxr[i] = Some(adxr_point);
        }
    }

    adxr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_adxr() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");

            let result = adxr(&highs, &lows, &closes, 14, 14, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/adxr_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "ADXR test failed for symbol {}.",
                symbol
            );
        }
    }
}
