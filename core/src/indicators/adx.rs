use crate::indicators::dmi::dmi;

pub fn adx(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    dmi_period: usize,
    adx_period: usize,
) -> Vec<Option<f64>> {
    let (plus_di, minus_di) = dmi(highs, lows, closes, dmi_period);
    let mut adx = Vec::with_capacity(plus_di.len());
    let mut dx_sum = 0.0;
    let mut adx_point = 0.0;

    for i in 0..plus_di.len() {
        let dx = match (plus_di[i], minus_di[i]) {
            (Some(plus), Some(minus)) if plus != 0.0 || minus != 0.0 => {
                (plus - minus).abs() / (plus + minus) * 100.0
            }
            _ => 0.0,
        };

        let initial_period = dmi_period + adx_period - 1;
        if i < initial_period {
            dx_sum += dx;
            adx.push(None);
        } else if i == initial_period {
            dx_sum += dx;
            adx_point = dx_sum / adx_period as f64;
            adx.push(Some(adx_point));
        } else {
            adx_point = (adx_point * (adx_period - 1) as f64 + dx) / adx_period as f64;
            adx.push(Some(adx_point));
        }
    }

    adx
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_adx() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");

            let result = adx(&highs, &lows, &closes, 14, 14);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/adx_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "ADX test failed for symbol {}.",
                symbol
            );
        }
    }
}
