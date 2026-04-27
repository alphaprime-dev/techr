use crate::indicators::adx::adx;

pub fn adxr(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
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
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();
            let closes = testutils::load_data(&format!("../data/{}.json", symbol), "c")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

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

    #[test]
    fn test_adxr_requires_current_and_lagged_adx_after_gaps() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            Some(14.0),
            None,
            Some(15.0),
            Some(16.0),
            Some(18.0),
            None,
            Some(19.0),
            Some(20.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(9.0),
            Some(11.0),
            None,
            Some(13.0),
            Some(14.0),
            Some(15.0),
            None,
            Some(17.0),
            Some(18.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            Some(13.0),
            None,
            Some(14.0),
            Some(15.0),
            Some(17.0),
            None,
            Some(18.0),
            Some(19.0),
        ];

        let result = adxr(&highs, &lows, &closes, 2, 2, 4);

        assert_eq!(
            result,
            vec![
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(100.0)
            ]
        );
    }

    #[test]
    fn test_adxr_length_mismatch_fails_closed() {
        let highs = vec![Some(10.0), Some(12.0), Some(14.0)];
        let lows = vec![Some(8.0), Some(9.0)];
        let closes = vec![Some(9.0), Some(11.0), Some(13.0)];

        let result = adxr(&highs, &lows, &closes, 2, 2, 2);

        assert_eq!(result, vec![None, None, None]);
    }
}
