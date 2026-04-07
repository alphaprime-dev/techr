use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use techr::{
    bband as techr_bband, disparity as techr_disparity, ema as techr_ema, macd as techr_macd,
    sma as techr_sma, stochf as techr_stochf, stochs as techr_stochs, wma as techr_wma,
};

#[derive(Deserialize)]
struct PeriodKwargs {
    period: u32,
}

#[derive(Deserialize)]
struct BBandKwargs {
    period: u32,
    sigma: f64,
}

#[derive(Deserialize)]
struct FastSlowKwargs {
    fast_period: u32,
    slow_period: u32,
}

#[derive(Deserialize)]
struct FastSlowSignalKwargs {
    fast_period: u32,
    slow_period: u32,
    signal_period: u32,
}

#[derive(Deserialize)]
struct StochFKwargs {
    fastk_period: u32,
    fastd_period: u32,
}

#[derive(Deserialize)]
struct StochKwargs {
    fastk_period: u32,
    slowk_period: u32,
    slowd_period: u32,
}

#[derive(Deserialize)]
struct IchimokuLeadingSpanAKwargs {
    base_line_period: u32,
    conversion_line_period: u32,
}

fn series_to_f64_vec(series: &Series) -> PolarsResult<Vec<f64>> {
    let casted = series.cast(&DataType::Float64)?;
    Ok(casted.f64()?.to_vec_null_aware().left().unwrap())
}

fn option_vec_to_series(values: Vec<Option<f64>>) -> Series {
    values.into_iter().collect()
}

fn shift(values: Vec<Option<f64>>, periods: isize) -> Vec<Option<f64>> {
    let len = values.len();
    let mut shifted = vec![None; len];

    for (idx, value) in values.into_iter().enumerate() {
        let target = idx as isize + periods;
        if (0..len as isize).contains(&target) {
            shifted[target as usize] = value;
        }
    }

    shifted
}

fn rolling_midpoint(highs: &[f64], lows: &[f64], period: usize) -> Vec<Option<f64>> {
    let len = highs.len();
    let mut result = vec![None; len];

    if len < period {
        return result;
    }

    for i in (period - 1)..len {
        let max_high = highs[i + 1 - period..=i]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let min_low = lows[i + 1 - period..=i]
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        result[i] = Some((max_high + min_low) / 2.0);
    }

    result
}

fn mean_optional(lhs: Vec<Option<f64>>, rhs: Vec<Option<f64>>) -> Vec<Option<f64>> {
    lhs.into_iter()
        .zip(rhs)
        .map(|(left, right)| match (left, right) {
            (Some(left), Some(right)) => Some((left + right) / 2.0),
            _ => None,
        })
        .collect()
}

#[polars_expr(output_type=Float64)]
fn sma(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let out = techr_sma(&input, kwargs.period as usize);
    Ok(option_vec_to_series(out))
}

#[polars_expr(output_type=Float64)]
fn wma(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let out = techr_wma(&input, kwargs.period as usize);
    Ok(option_vec_to_series(out))
}

#[polars_expr(output_type=Float64)]
fn ema(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let out = techr_ema(&input, kwargs.period as usize);
    Ok(option_vec_to_series(out))
}

#[polars_expr(output_type=Float64)]
fn disparity(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let out = techr_disparity(&input, kwargs.period as usize);
    Ok(option_vec_to_series(out))
}

#[polars_expr(output_type=Float64)]
fn macd(inputs: &[Series], kwargs: FastSlowKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (macd_line, _, _) = techr_macd(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        9,
    );
    Ok(option_vec_to_series(macd_line))
}

#[polars_expr(output_type=Float64)]
fn macd_signal(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (_, signal_line, _) = techr_macd(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    );
    Ok(option_vec_to_series(signal_line))
}

#[polars_expr(output_type=Float64)]
fn macd_hist(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (_, _, histogram) = techr_macd(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    );
    Ok(option_vec_to_series(histogram))
}

#[polars_expr(output_type=Float64)]
fn bband_middle(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (_, middle, _) = techr_bband(&input, kwargs.period as usize, None);
    Ok(option_vec_to_series(middle))
}

#[polars_expr(output_type=Float64)]
fn bband_lower(inputs: &[Series], kwargs: BBandKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (_, _, lower) = techr_bband(&input, kwargs.period as usize, Some(kwargs.sigma));
    Ok(option_vec_to_series(lower))
}

#[polars_expr(output_type=Float64)]
fn bband_upper(inputs: &[Series], kwargs: BBandKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (upper, _, _) = techr_bband(&input, kwargs.period as usize, Some(kwargs.sigma));
    Ok(option_vec_to_series(upper))
}

#[polars_expr(output_type=Float64)]
fn stochf_percent_k(inputs: &[Series], kwargs: StochFKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let (percent_k, _) = techr_stochf(
        &highs,
        &lows,
        &closes,
        kwargs.fastk_period as usize,
        kwargs.fastd_period as usize,
    );
    Ok(option_vec_to_series(percent_k))
}

#[polars_expr(output_type=Float64)]
fn stochf_percent_d(inputs: &[Series], kwargs: StochFKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let (_, percent_d) = techr_stochf(
        &highs,
        &lows,
        &closes,
        kwargs.fastk_period as usize,
        kwargs.fastd_period as usize,
    );
    Ok(option_vec_to_series(percent_d))
}

#[polars_expr(output_type=Float64)]
fn stoch_percent_k(inputs: &[Series], kwargs: StochKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let (percent_k, _) = techr_stochs(
        &highs,
        &lows,
        &closes,
        kwargs.fastk_period as usize,
        kwargs.slowk_period as usize,
        kwargs.slowd_period as usize,
    );
    Ok(option_vec_to_series(percent_k))
}

#[polars_expr(output_type=Float64)]
fn stoch_percent_d(inputs: &[Series], kwargs: StochKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let (_, percent_d) = techr_stochs(
        &highs,
        &lows,
        &closes,
        kwargs.fastk_period as usize,
        kwargs.slowk_period as usize,
        kwargs.slowd_period as usize,
    );
    Ok(option_vec_to_series(percent_d))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_base_line(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(rolling_midpoint(
        &highs,
        &lows,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_conversion_line(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(rolling_midpoint(
        &highs,
        &lows,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_leading_span_a(
    inputs: &[Series],
    kwargs: IchimokuLeadingSpanAKwargs,
) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let base_line = rolling_midpoint(&highs, &lows, kwargs.base_line_period as usize);
    let conversion_line = rolling_midpoint(&highs, &lows, kwargs.conversion_line_period as usize);
    let span = mean_optional(base_line, conversion_line);
    Ok(option_vec_to_series(shift(span, -26)))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_leading_span_b(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let span = rolling_midpoint(&highs, &lows, kwargs.period as usize);
    Ok(option_vec_to_series(shift(span, -26)))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_lagging_span(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let span = closes.into_iter().map(Some).collect::<Vec<_>>();
    Ok(option_vec_to_series(shift(span, kwargs.period as isize)))
}
