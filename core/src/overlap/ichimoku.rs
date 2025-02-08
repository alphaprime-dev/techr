use crate::utils::{find_max, find_min};

pub fn ichimoku(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    tenkan_period: usize,
    kijun_period: usize,
    senkou_b_period: usize,
) -> (
    Vec<Option<f64>>, // Tenkan-sen
    Vec<Option<f64>>, // Kijun-sen
    Vec<Option<f64>>, // Chikou Span
    Vec<Option<f64>>, // Senkou Span A
    Vec<Option<f64>>, // Senkou Span B
) {
    let len = highs.len();
    let mut tenkan = vec![None; len];
    let mut kijun = vec![None; len];
    let mut chikou = vec![None; len];
    let mut senkou_a = vec![None; len + kijun_period - 1];
    let mut senkou_b = vec![None; len + kijun_period - 1];

    for i in 0..len {
        if i >= tenkan_period - 1 {
            let max_high = find_max(&highs[i + 1 - tenkan_period..=i]);
            let min_low = find_min(&lows[i + 1 - tenkan_period..=i]);
            tenkan[i] = Some((max_high + min_low) / 2.0);
        }

        if i >= kijun_period - 1 {
            let max_high = find_max(&highs[i + 1 - kijun_period..=i]);
            let min_low = find_min(&lows[i + 1 - kijun_period..=i]);
            kijun[i] = Some((max_high + min_low) / 2.0);
            chikou[i + 1 - kijun_period] = Some(closes[i]);
        }

        if let (Some(t), Some(k)) = (tenkan[i], kijun[i]) {
            senkou_a[i + kijun_period - 1] = Some((t + k) / 2.0);
        }

        if i >= senkou_b_period - 1 {
            let max_high = find_max(&highs[i + 1 - senkou_b_period..=i]);
            let min_low = find_min(&lows[i + 1 - senkou_b_period..=i]);
            senkou_b[i + kijun_period - 1] = Some((max_high + min_low) / 2.0);
        }
    }

    (tenkan, kijun, chikou, senkou_a, senkou_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{round_vec, testutils};

    #[test]
    fn test_ichimoku() {
        let test_cases = vec!["005930", "TSLA"];
        for symbol in test_cases {
            let high = testutils::load_data(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data(&format!("../data/{}.json", symbol), "c");

            let (tenkan, kijun, chikou, senkou_a, senkou_b) =
                ichimoku(&high, &low, &close, 9, 26, 52);

            let expected_tenkan = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_tenkan_{}.json",
                symbol
            ));
            let expected_kijun = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_kijun_{}.json",
                symbol
            ));
            let expected_chikou = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_chikou_{}.json",
                symbol
            ));
            let expected_senkou_a = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_senkou_A_{}.json",
                symbol
            ));
            let expected_senkou_b = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_senkou_B_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(tenkan, 8),
                round_vec(expected_tenkan, 8),
                "Ichimoku Tenkan-sen test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(kijun, 8),
                round_vec(expected_kijun, 8),
                "Ichimoku Kijun-sen test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(chikou, 8),
                round_vec(expected_chikou, 8),
                "Ichimoku Chikou Span test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(senkou_a, 8),
                round_vec(expected_senkou_a, 8),
                "Ichimoku Senkou Span A test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(senkou_b, 8),
                round_vec(expected_senkou_b, 8),
                "Ichimoku Senkou Span B test failed for symbol {}.",
                symbol
            );
        }
    }
}
