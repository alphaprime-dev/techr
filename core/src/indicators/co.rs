use crate::indicators::ad;

pub fn co(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
    volumes: &[Option<f64>],
    period_short: usize,
    period_long: usize,
) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut co = vec![None; len];

    if len != lows.len()
        || len != closes.len()
        || len != volumes.len()
        || period_short == 0
        || period_long == 0
    {
        return co;
    }

    let ad_values = ad(highs, lows, closes, volumes);
    let short_k = 2.0 / (period_short as f64 + 1.0);
    let long_k = 2.0 / (period_long as f64 + 1.0);
    let mut short_ema = None;
    let mut long_ema = None;
    let mut seeded_rows = 0usize;
    let mut output_seeded = false;

    for i in 0..len {
        let Some(ad) = ad_values[i] else {
            if !output_seeded {
                short_ema = None;
                long_ema = None;
                seeded_rows = 0;
            }
            continue;
        };

        let next_short = short_ema.map_or(ad, |prev| ad * short_k + prev * (1.0 - short_k));
        let next_long = long_ema.map_or(ad, |prev| ad * long_k + prev * (1.0 - long_k));
        short_ema = Some(next_short);
        long_ema = Some(next_long);

        if output_seeded {
            co[i] = Some(next_short - next_long);
            continue;
        }

        seeded_rows += 1;
        if seeded_rows == period_long {
            output_seeded = true;
            co[i] = Some(next_short - next_long);
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
            let volumes = testutils::load_data(&format!("../data/{}.json", symbol), "v")
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

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

    #[test]
    fn test_co_requires_contiguous_seed_window_and_resumes_after_gap() {
        let highs = vec![
            Some(10.0),
            Some(12.0),
            None,
            Some(14.0),
            Some(16.0),
            Some(18.0),
            None,
            Some(20.0),
        ];
        let lows = vec![
            Some(8.0),
            Some(10.0),
            None,
            Some(12.0),
            Some(14.0),
            Some(16.0),
            None,
            Some(18.0),
        ];
        let closes = vec![
            Some(9.0),
            Some(11.0),
            None,
            Some(13.0),
            Some(15.0),
            Some(17.0),
            None,
            Some(19.0),
        ];
        let volumes = vec![
            Some(100.0),
            Some(100.0),
            None,
            Some(100.0),
            Some(100.0),
            Some(100.0),
            None,
            Some(100.0),
        ];

        let result = co(&highs, &lows, &closes, &volumes, 2, 3);

        assert_eq!(
            result,
            vec![None, None, None, None, None, Some(0.0), None, Some(0.0)]
        );
    }
}
