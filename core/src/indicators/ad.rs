use crate::utils::calc_clv;

pub fn ad(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
) -> Vec<Option<f64>> {
    let mut ad = vec![None; highs.len()];

    let len = highs.len();

    if len == 0 || len != lows.len() || len != closes.len() || len != volumes.len() {
        return ad;
    }

    let mut ad_point = 0.0;
    for i in 0..len {
        let (Some(high), Some(low), Some(close), Some(volume)) =
            (highs[i], lows[i], closes[i], volumes[i])
        else {
            continue;
        };

        ad_point += calc_clv(high, low, close) * volume;
        ad[i] = Some(ad_point);
    }

    ad
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_ad() {
        let test_cases = vec!["005930"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let volume = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

            let result = ad(&high, &low, &close, &volume);
            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ad_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 6),
                round_vec(expected, 6),
                "AD test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_ad_with_gap_preserves_running_total() {
        let highs = vec![Some(10.0), Some(12.0), None, Some(14.0)];
        let lows = vec![Some(8.0), Some(10.0), None, Some(12.0)];
        let closes = vec![Some(9.0), Some(11.0), None, Some(13.0)];
        let volumes = vec![Some(100.0), Some(100.0), Some(100.0), Some(100.0)];

        let result = ad(&highs, &lows, &closes, &volumes);

        assert_eq!(result, vec![Some(0.0), Some(0.0), None, Some(0.0)]);
    }
}
