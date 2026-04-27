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
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period: usize,
) -> Vec<Option<f64>> {
    rolling_midpoint(highs, lows, period)
}

pub fn ichimoku_base_line(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period: usize,
) -> Vec<Option<f64>> {
    rolling_midpoint(highs, lows, period)
}

pub fn ichimoku_lagging_span(closes: &[Option<f64>], base_line_period: usize) -> Vec<Option<f64>> {
    let len = closes.len();
    let mut lagging_span = vec![None; len];

    if base_line_period == 0 || len < base_line_period {
        return lagging_span;
    }

    for i in (base_line_period - 1)..len {
        lagging_span[i + 1 - base_line_period] = closes[i];
    }

    lagging_span
}

pub fn ichimoku_leading_span_a(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    conversion_line_period: usize,
    base_line_period: usize,
) -> Vec<Option<f64>> {
    let conversion_line = ichimoku_conversion_line(highs, lows, conversion_line_period);
    let base_line = ichimoku_base_line(highs, lows, base_line_period);
    leading_span_a_from_lines(&conversion_line, &base_line, base_line_period)
}

pub fn ichimoku_leading_span_b(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    period: usize,
    base_line_period: usize,
) -> Vec<Option<f64>> {
    forward_shift(rolling_midpoint(highs, lows, period), base_line_period)
}

pub fn ichimoku(
    highs: &[Option<f64>],
    lows: &[Option<f64>],
    closes: &[Option<f64>],
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
    if len != lows.len() || len != closes.len() {
        return (
            vec![None; len],
            vec![None; len],
            vec![None; len],
            vec![None; len],
            vec![None; len],
        );
    }

    let conversion_line = ichimoku_conversion_line(highs, lows, conversion_line_period);
    let base_line = ichimoku_base_line(highs, lows, base_line_period);
    let lagging_span = ichimoku_lagging_span(closes, base_line_period);
    let leading_span_a = leading_span_a_from_lines(&conversion_line, &base_line, base_line_period);
    let leading_span_b =
        ichimoku_leading_span_b(highs, lows, leading_span_b_period, base_line_period);

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
            let high = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "h");
            let low = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "l");
            let close = testutils::load_data_nullable(&format!("../data/{}.json", symbol), "c");

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

    #[test]
    fn test_ichimoku_gap_invalidates_extrema_windows_and_preserves_lagging_alignment() {
        let highs = vec![Some(5.0), Some(7.0), None, Some(10.0), Some(12.0)];
        let lows = vec![Some(1.0), Some(3.0), None, Some(6.0), Some(8.0)];
        let closes = vec![Some(4.0), Some(5.0), None, Some(8.0), Some(11.0)];

        let (conversion, base, lagging, leading_a, leading_b) =
            ichimoku(&highs, &lows, &closes, 2, 2, 2);

        assert_eq!(conversion, vec![None, Some(4.0), None, None, Some(9.0)]);
        assert_eq!(base, vec![None, Some(4.0), None, None, Some(9.0)]);
        assert_eq!(lagging, vec![Some(5.0), None, Some(8.0), Some(11.0), None]);
        assert_eq!(
            leading_a,
            vec![None, None, Some(4.0), None, None, Some(9.0)]
        );
        assert_eq!(
            leading_b,
            vec![None, None, Some(4.0), None, None, Some(9.0)]
        );
    }
}
