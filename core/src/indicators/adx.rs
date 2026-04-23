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
            if adx_point.is_none() {
                dx_sum = 0.0;
                seeded = 0;
            }
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

    #[test]
    fn test_adx_requires_contiguous_seed_window_and_resumes_after_gap() {
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

        let result = adx(&highs, &lows, &closes, 2, 2);

        assert_eq!(
            result,
            vec![
                None,
                None,
                None,
                None,
                None,
                None,
                Some(100.0),
                None,
                None,
                Some(100.0)
            ]
        );
    }

    #[test]
    fn test_adx_length_mismatch_fails_closed() {
        let highs = vec![Some(10.0), Some(12.0), Some(14.0)];
        let lows = vec![Some(8.0), Some(9.0)];
        let closes = vec![Some(9.0), Some(11.0), Some(13.0)];

        let result = adx(&highs, &lows, &closes, 2, 2);

        assert_eq!(result, vec![None, None, None]);
    }
}
