use crate::utils::calc_true_ranges;

pub fn ultosc(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period_short: usize,
    period_medium: usize,
    period_long: usize,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut ultosc = vec![None; len];

    if len != closes.len() || len != lows.len() || len < period_long + 1 {
        return ultosc;
    }

    let trs = calc_true_ranges(highs, lows, closes);
    let buying_pressures = get_buying_pressures(closes, lows);

    let mut long_denomi = 0.0;
    let mut long_nomi = 0.0;
    let mut medium_denomi = 0.0;
    let mut medium_nomi = 0.0;
    let mut short_denomi = 0.0;
    let mut short_nomi = 0.0;

    for i in 1..len {
        let bp = buying_pressures[i - 1];
        let tr = trs[i - 1];

        long_denomi += bp;
        long_nomi += tr;

        if i >= period_long - period_medium + 1 {
            medium_denomi += bp;
            medium_nomi += tr;
        }

        if i >= period_long - period_short + 1 {
            short_denomi += bp;
            short_nomi += tr;
        }

        if i >= period_long {
            let uo_point = ((long_denomi / long_nomi
                + 2.0 * (medium_denomi / medium_nomi)
                + 4.0 * (short_denomi / short_nomi))
                * 100.0)
                / 7.0;

            ultosc[i] = Some(uo_point);

            // Remove oldest values from each period
            long_denomi -= buying_pressures[i - period_long];
            long_nomi -= trs[i - period_long];
            medium_denomi -= buying_pressures[i - period_medium];
            medium_nomi -= trs[i - period_medium];
            short_denomi -= buying_pressures[i - period_short];
            short_nomi -= trs[i - period_short];
        }
    }

    ultosc
}

fn get_buying_pressures(closes: &[f64], lows: &[f64]) -> Vec<f64> {
    let len = closes.len();
    let mut buying_pressures = Vec::with_capacity(len - 1);

    for i in 1..len {
        let bp = closes[i] - lows[i].min(closes[i - 1]);
        buying_pressures.push(bp);
    }

    buying_pressures
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_ultosc() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let result = ultosc(&high, &low, &close, 7, 14, 28);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ultosc_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "ULTOSC test failed for symbol {}.",
                symbol
            );
        }
    }
}
