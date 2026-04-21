use crate::indicators::aroon;

pub fn aroonosc(highs: &[Option<f64>], lows: &[Option<f64>], period: usize) -> Vec<Option<f64>> {
    let mut aroonosc = vec![None; highs.len()];

    if highs.len() != lows.len() || highs.len() < period {
        return aroonosc;
    }

    let (aroon_ups, aroon_downs) = aroon(highs, lows, period);

    for i in period..highs.len() {
        if let (Some(up), Some(down)) = (aroon_ups[i], aroon_downs[i]) {
            aroonosc[i] = Some(up - down);
        }
    }

    aroonosc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use crate::utils::round_vec;

    #[test]
    fn test_aroonosc() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let highs = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let lows = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let highs = highs.into_iter().map(Some).collect::<Vec<_>>();
            let lows = lows.into_iter().map(Some).collect::<Vec<_>>();
            let result = aroonosc(&highs, &lows, 25);

            let expected = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/aroonosc_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(result, 8),
                round_vec(expected, 8),
                "AROONOSC test failed for symbol {}.",
                symbol
            );
        }
    }

    #[test]
    fn test_aroonosc_mismatched_lengths_fail_closed() {
        let highs = vec![Some(1.0), Some(2.0), Some(3.0)];
        let lows = vec![Some(1.0), Some(2.0)];

        let result = aroonosc(&highs, &lows, 2);

        assert_eq!(result, vec![None, None, None]);
    }
}
