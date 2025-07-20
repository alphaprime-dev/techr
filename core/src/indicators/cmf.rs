use crate::utils::calc_clv;

pub fn cmf(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    volumes: &[f64],
    period: usize,
) -> Vec<Option<f64>> {
    let mut cmf = vec![None; highs.len()];
    let len = highs.len();

    if len != lows.len()
        || len != closes.len()
        || len != volumes.len()
        || len < period
        || period <= 1
    {
        return cmf;
    }

    let mut sum_money_flow_volume = 0.0;
    let mut sum_volume = 0.0;

    for i in 0..period {
        let current_money_flow_volume = calc_clv(highs[i], lows[i], closes[i]) * volumes[i];
        sum_money_flow_volume += current_money_flow_volume;
        sum_volume += volumes[i];
    }

    let cmf_point = if sum_volume == 0.0 {
        None
    } else {
        Some(sum_money_flow_volume / sum_volume)
    };
    cmf[period - 1] = cmf_point;

    for i in period..len {
        let oldest_money_flow_volume =
            calc_clv(highs[i - period], lows[i - period], closes[i - period]) * volumes[i - period];
        let current_money_flow_volume = calc_clv(highs[i], lows[i], closes[i]) * volumes[i];

        sum_money_flow_volume -= oldest_money_flow_volume;
        sum_money_flow_volume += current_money_flow_volume;
        sum_volume -= volumes[i - period];
        sum_volume += volumes[i];

        let cmf_point = if sum_volume == 0.0 {
            Some(0.0)
        } else {
            Some(sum_money_flow_volume / sum_volume)
        };
        cmf[i] = cmf_point;
    }

    cmf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_cmf() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v");
            let result = cmf(&high, &low, &close, &volume, 21);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/cmf_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "CMF test failed for symbol {}.",
                symbol
            );
        }
    }
}
