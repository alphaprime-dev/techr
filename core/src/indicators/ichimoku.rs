use crate::utils::{find_max, find_min};

pub fn ichimoku(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    conversion_line_period: usize,
    base_line_period: usize,
    leading_span_b_period: usize,
) -> (
    Vec<Option<f64>>, // Conversion line
    Vec<Option<f64>>, // Base line
    Vec<Option<f64>>, // Lagging span
    Vec<Option<f64>>, // Leading span A
    Vec<Option<f64>>, // Leading span B
) {
    let len = highs.len();
    let mut conversion_line = vec![None; len];
    let mut base_line = vec![None; len];
    let mut lagging_span = vec![None; len];
    let mut leading_span_a = vec![None; len + base_line_period - 1];
    let mut leading_span_b = vec![None; len + base_line_period - 1];

    for i in 0..len {
        if i >= conversion_line_period - 1 {
            let max_high = find_max(&highs[i + 1 - conversion_line_period..=i]);
            let min_low = find_min(&lows[i + 1 - conversion_line_period..=i]);
            conversion_line[i] = Some((max_high + min_low) / 2.0);
        }

        if i >= base_line_period - 1 {
            let max_high = find_max(&highs[i + 1 - base_line_period..=i]);
            let min_low = find_min(&lows[i + 1 - base_line_period..=i]);
            base_line[i] = Some((max_high + min_low) / 2.0);
            lagging_span[i + 1 - base_line_period] = Some(closes[i]);
        }

        if let (Some(conversion), Some(base)) = (conversion_line[i], base_line[i]) {
            leading_span_a[i + base_line_period - 1] = Some((conversion + base) / 2.0);
        }

        if i >= leading_span_b_period - 1 {
            let max_high = find_max(&highs[i + 1 - leading_span_b_period..=i]);
            let min_low = find_min(&lows[i + 1 - leading_span_b_period..=i]);
            leading_span_b[i + base_line_period - 1] = Some((max_high + min_low) / 2.0);
        }
    }

    (
        conversion_line,
        base_line,
        lagging_span,
        leading_span_a,
        leading_span_b,
    )
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

            let (conversion_line, base_line, lagging_span, leading_span_a, leading_span_b) =
                ichimoku(&high, &low, &close, 9, 26, 52);

            let expected_conversion_line = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_conversion_line_{}.json",
                symbol
            ));
            let expected_base_line = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_base_line_{}.json",
                symbol
            ));
            let expected_lagging_span = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_lagging_span_{}.json",
                symbol
            ));
            let expected_leading_span_a = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_leading_span_a_{}.json",
                symbol
            ));
            let expected_leading_span_b = testutils::load_expected::<Option<f64>>(&format!(
                "../data/expected/ichimoku_leading_span_b_{}.json",
                symbol
            ));

            assert_eq!(
                round_vec(conversion_line, 8),
                round_vec(expected_conversion_line, 8),
                "Ichimoku conversion line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(base_line, 8),
                round_vec(expected_base_line, 8),
                "Ichimoku base line test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(lagging_span, 8),
                round_vec(expected_lagging_span, 8),
                "Ichimoku lagging span test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(leading_span_a, 8),
                round_vec(expected_leading_span_a, 8),
                "Ichimoku leading span A test failed for symbol {}.",
                symbol
            );
            assert_eq!(
                round_vec(leading_span_b, 8),
                round_vec(expected_leading_span_b, 8),
                "Ichimoku leading span B test failed for symbol {}.",
                symbol
            );
        }
    }
}
