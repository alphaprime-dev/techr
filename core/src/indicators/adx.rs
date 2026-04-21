use crate::indicators::dmi::dmi;

pub fn adx(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    dmi_period: usize,
    adx_period: usize,
) -> Vec<Option<f64>> {
    let (plus_di, minus_di) = dmi(highs, lows, closes, dmi_period);
    let mut adx = vec![None; plus_di.len()];
    let mut dx_sum = 0.0;
    let mut seeded = 0usize;
    let mut adx_point = None;

    if adx_period == 0 {
        return adx;
    }

    for i in 0..plus_di.len() {
        let Some(dx) = (match (plus_di[i], minus_di[i]) {
            (Some(plus), Some(minus)) if plus != 0.0 || minus != 0.0 => {
                Some((plus - minus).abs() / (plus + minus) * 100.0)
            }
            (Some(_), Some(_)) => Some(0.0),
            _ => None,
        }) else {
            continue;
        };

        if let Some(current_adx) = adx_point {
            let next_adx = (current_adx * (adx_period - 1) as f64 + dx) / adx_period as f64;
            adx_point = Some(next_adx);
            adx[i] = Some(next_adx);
        } else {
            dx_sum += dx;
            seeded += 1;
            if seeded == adx_period {
                let initial_adx = dx_sum / adx_period as f64;
                adx_point = Some(initial_adx);
                adx[i] = Some(initial_adx);
            }
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

            let highs = highs.into_iter().map(Some).collect::<Vec<_>>();
            let lows = lows.into_iter().map(Some).collect::<Vec<_>>();
            let closes = closes.into_iter().map(Some).collect::<Vec<_>>();

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
