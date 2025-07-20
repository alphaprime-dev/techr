use crate::indicators::ad;

pub fn co(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    volumes: &[f64],
    period_short: usize,
    period_long: usize,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut co = vec![None; len];

    if len < period_long {
        return co;
    } else if period_long == period_short {
        return vec![Some(0.0); len];
    }

    let ad_values = ad(highs, lows, closes, volumes);
    let short_k = 2.0 / (period_short as f64 + 1.0);
    let long_k = 2.0 / (period_long as f64 + 1.0);

    if ad_values[0].is_none() {
        return co;
    }

    let mut short_ema = ad_values[0].unwrap();
    let mut long_ema = short_ema;

    for i in 1..len {
        if let Some(ad) = ad_values[i] {
            short_ema = ad * short_k + short_ema * (1.0 - short_k);
            long_ema = ad * long_k + long_ema * (1.0 - long_k);

            if i >= period_long - 1 {
                co[i] = Some(short_ema - long_ema);
            }
        }
    }

    co
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_co() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v");

            let co_result = co(&highs, &lows, &closes, &volumes, 3, 10);

            let expected_co = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/co_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(co_result, 4),
                round_vec(expected_co, 4),
                "CO test failed for symbol {}.",
                symbol
            );
        }
    }
}
