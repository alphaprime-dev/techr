use crate::utils::{forward_shift, rolling_midpoint};

fn leading_span_a_from_lines(
    conversion_line: &[Option<f64>],
    base_line: &[Option<f64>],
    base_line_period: usize,
) -> Vec<Option<f64>> {
    let span = conversion_line
        .iter()
        .zip(base_line.iter())
        .map(|(conversion, base)| match (conversion, base) {
            (Some(conversion), Some(base)) => Some((conversion + base) / 2.0),
            _ => None,
        })
        .collect::<Vec<_>>();

    forward_shift(span, base_line_period)
}

pub fn ichimoku_conversion_line(
    highs: &[f64],
    lows: &[f64],
    conversion_line_period: usize,
) -> Vec<Option<f64>> {
    rolling_midpoint(highs, lows, conversion_line_period)
}

pub fn ichimoku_base_line(
    highs: &[f64],
    lows: &[f64],
    base_line_period: usize,
) -> Vec<Option<f64>> {
    rolling_midpoint(highs, lows, base_line_period)
}

pub fn ichimoku_lagging_span(closes: &[f64], base_line_period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut lagging_span = vec![None; len];

    if len < base_line_period {
        return lagging_span;
    }

    for i in (base_line_period - 1)..len {
        lagging_span[i + 1 - base_line_period] = Some(closes[i]);
    }

    lagging_span
}

pub fn ichimoku_leading_span_a(
    highs: &[f64],
    lows: &[f64],
    conversion_line_period: usize,
    base_line_period: usize,
) -> Vec<Option<f64>> {
    let conversion_line = ichimoku_conversion_line(highs, lows, conversion_line_period);
    let base_line = ichimoku_base_line(highs, lows, base_line_period);
    leading_span_a_from_lines(&conversion_line, &base_line, base_line_period)
}

pub fn ichimoku_leading_span_b(
    highs: &[f64],
    lows: &[f64],
    base_line_period: usize,
    leading_span_b_period: usize,
) -> Vec<Option<f64>> {
    forward_shift(
        rolling_midpoint(highs, lows, leading_span_b_period),
        base_line_period,
    )
}

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
    let conversion_line = ichimoku_conversion_line(highs, lows, conversion_line_period);
    let base_line = ichimoku_base_line(highs, lows, base_line_period);
    let lagging_span = ichimoku_lagging_span(closes, base_line_period);
    let leading_span_a = leading_span_a_from_lines(&conversion_line, &base_line, base_line_period);
    let leading_span_b =
        ichimoku_leading_span_b(highs, lows, base_line_period, leading_span_b_period);

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
