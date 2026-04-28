use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use techr_core as core;
use techr_core::{
    bband_lower as techr_bband_lower, bband_middle as techr_bband_middle,
    bband_upper as techr_bband_upper, disparity as techr_disparity, ema as techr_ema,
    ichimoku_base_line as techr_ichimoku_base_line,
    ichimoku_conversion_line as techr_ichimoku_conversion_line,
    ichimoku_lagging_span as techr_ichimoku_lagging_span,
    ichimoku_leading_span_a as techr_ichimoku_leading_span_a,
    ichimoku_leading_span_b as techr_ichimoku_leading_span_b,
    macd_histogram as techr_macd_histogram, macd_line as techr_macd_line,
    macd_signal as techr_macd_signal, sma as techr_sma, stoch_percent_d as techr_stoch_percent_d,
    stoch_percent_k as techr_stoch_percent_k, stochf_percent_d as techr_stochf_percent_d,
    stochf_percent_k as techr_stochf_percent_k, wma as techr_wma,
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

#[derive(Deserialize)]
struct IchimokuLeadingSpanBKwargs {
    period: u32,
    base_line_period: u32,
}

#[derive(Deserialize)]
struct IchimokuLaggingSpanKwargs {
    base_line_period: u32,
}

#[derive(Deserialize)]
struct DmiAdxKwargs {
    dmi_period: u32,
    adx_period: u32,
}

#[derive(Deserialize)]
struct AdxrKwargs {
    dmi_period: u32,
    adx_period: u32,
    adxr_period: u32,
}

#[derive(Deserialize)]
struct EnvKwargs {
    period: u32,
    shift_percentage: f64,
}

#[derive(Deserialize)]
struct EomLineKwargs {
    period: u32,
    scale: f64,
}

#[derive(Deserialize)]
struct EomSignalKwargs {
    period: u32,
    signal_period: u32,
    scale: f64,
}

#[derive(Deserialize)]
struct MassiLineKwargs {
    period_ema: u32,
    period_sum: u32,
}

#[derive(Deserialize)]
struct MassiSignalKwargs {
    period_ema: u32,
    period_sum: u32,
    period_signal: u32,
}

#[derive(Deserialize)]
struct SignalKwargs {
    signal_period: u32,
}

#[derive(Deserialize)]
struct PeriodShortLongKwargs {
    period_short: u32,
    period_long: u32,
}

#[derive(Deserialize)]
struct PsarKwargs {
    increment: f64,
    initial_acceleration_factor: f64,
    max_acceleration_factor: f64,
}

#[derive(Deserialize)]
struct SonarLineKwargs {
    period: u32,
    step: u32,
}

#[derive(Deserialize)]
struct SonarSignalKwargs {
    period: u32,
    step: u32,
    signal_period: u32,
}

#[derive(Deserialize)]
struct StochRsiKwargs {
    period_rsi: u32,
    period_k: u32,
    period_d: u32,
}

#[derive(Deserialize)]
struct UltOscKwargs {
    period_short: u32,
    period_medium: u32,
    period_long: u32,
}

fn series_to_f64_vec(series: &Series) -> PolarsResult<Vec<Option<f64>>> {
    let casted = series.cast(&DataType::Float64)?;
    Ok(casted.f64()?.to_vec())
}

fn option_vec_to_series(values: Vec<Option<f64>>) -> Series {
    values.into_iter().collect()
}

fn truncate(values: Vec<Option<f64>>, len: usize) -> Vec<Option<f64>> {
    values.into_iter().take(len).collect()
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
    let macd_line = techr_macd_line(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
    );
    Ok(option_vec_to_series(macd_line))
}

#[polars_expr(output_type=Float64)]
fn macd_line(inputs: &[Series], kwargs: FastSlowKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(techr_macd_line(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn macd_signal(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let signal_line = techr_macd_signal(
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
    let histogram = techr_macd_histogram(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    );
    Ok(option_vec_to_series(histogram))
}

#[polars_expr(output_type=Float64)]
fn macd_histogram(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(techr_macd_histogram(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn bband_middle(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let middle = techr_bband_middle(&input, kwargs.period as usize);
    Ok(option_vec_to_series(middle))
}

#[polars_expr(output_type=Float64)]
fn bband_lower(inputs: &[Series], kwargs: BBandKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let lower = techr_bband_lower(&input, kwargs.period as usize, Some(kwargs.sigma));
    Ok(option_vec_to_series(lower))
}

#[polars_expr(output_type=Float64)]
fn bband_upper(inputs: &[Series], kwargs: BBandKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let upper = techr_bband_upper(&input, kwargs.period as usize, Some(kwargs.sigma));
    Ok(option_vec_to_series(upper))
}

#[polars_expr(output_type=Float64)]
fn stochf_percent_k(inputs: &[Series], kwargs: StochFKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let percent_k = techr_stochf_percent_k(&highs, &lows, &closes, kwargs.fastk_period as usize);
    Ok(option_vec_to_series(percent_k))
}

#[polars_expr(output_type=Float64)]
fn stochf_percent_d(inputs: &[Series], kwargs: StochFKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let percent_k = techr_stochf_percent_k(&highs, &lows, &closes, kwargs.fastk_period as usize);
    let percent_d = techr_stochf_percent_d(
        &percent_k,
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
    let percent_k = techr_stoch_percent_k(
        &highs,
        &lows,
        &closes,
        kwargs.fastk_period as usize,
        kwargs.slowk_period as usize,
    );
    Ok(option_vec_to_series(percent_k))
}

#[polars_expr(output_type=Float64)]
fn stoch_percent_d(inputs: &[Series], kwargs: StochKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let percent_d = techr_stoch_percent_d(
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
    Ok(option_vec_to_series(techr_ichimoku_base_line(
        &highs,
        &lows,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_conversion_line(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(techr_ichimoku_conversion_line(
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
    let len = highs.len();
    Ok(option_vec_to_series(truncate(
        techr_ichimoku_leading_span_a(
            &highs,
            &lows,
            kwargs.conversion_line_period as usize,
            kwargs.base_line_period as usize,
        ),
        len,
    )))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_leading_span_b(
    inputs: &[Series],
    kwargs: IchimokuLeadingSpanBKwargs,
) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let len = highs.len();
    Ok(option_vec_to_series(truncate(
        techr_ichimoku_leading_span_b(
            &highs,
            &lows,
            kwargs.period as usize,
            kwargs.base_line_period as usize,
        ),
        len,
    )))
}

#[polars_expr(output_type=Float64)]
fn ichimoku_lagging_span(
    inputs: &[Series],
    kwargs: IchimokuLaggingSpanKwargs,
) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(techr_ichimoku_lagging_span(
        &closes,
        kwargs.base_line_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn ad(inputs: &[Series]) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let volumes = series_to_f64_vec(&inputs[3])?;
    Ok(option_vec_to_series(core::ad(
        &highs, &lows, &closes, &volumes,
    )))
}

#[polars_expr(output_type=Float64)]
fn adx(inputs: &[Series], kwargs: DmiAdxKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::adx(
        &highs,
        &lows,
        &closes,
        kwargs.dmi_period as usize,
        kwargs.adx_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn adxr(inputs: &[Series], kwargs: AdxrKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::adxr(
        &highs,
        &lows,
        &closes,
        kwargs.dmi_period as usize,
        kwargs.adx_period as usize,
        kwargs.adxr_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn aroon_up(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let (up, _) = core::aroon(&highs, &lows, kwargs.period as usize);
    Ok(option_vec_to_series(up))
}

#[polars_expr(output_type=Float64)]
fn aroon_down(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let (_, down) = core::aroon(&highs, &lows, kwargs.period as usize);
    Ok(option_vec_to_series(down))
}

#[polars_expr(output_type=Float64)]
fn aroonosc(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::aroonosc(
        &highs,
        &lows,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn atr(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::atr(
        &highs,
        &lows,
        &closes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn cci(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::cci(
        &highs,
        &lows,
        &closes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn cmf(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let volumes = series_to_f64_vec(&inputs[3])?;
    Ok(option_vec_to_series(core::cmf(
        &highs,
        &lows,
        &closes,
        &volumes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn co(inputs: &[Series], kwargs: PeriodShortLongKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let volumes = series_to_f64_vec(&inputs[3])?;
    Ok(option_vec_to_series(core::co(
        &highs,
        &lows,
        &closes,
        &volumes,
        kwargs.period_short as usize,
        kwargs.period_long as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn cv(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::cv(
        &highs,
        &lows,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn dmi_plus(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let (plus, _) = core::dmi(&highs, &lows, &closes, kwargs.period as usize);
    Ok(option_vec_to_series(plus))
}

#[polars_expr(output_type=Float64)]
fn dmi_minus(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let (_, minus) = core::dmi(&highs, &lows, &closes, kwargs.period as usize);
    Ok(option_vec_to_series(minus))
}

#[polars_expr(output_type=Float64)]
fn efi(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::efi(
        &closes,
        &volumes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn env_upper(inputs: &[Series], kwargs: EnvKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (upper, _, _) = core::env(&input, kwargs.period as usize, kwargs.shift_percentage);
    Ok(option_vec_to_series(upper))
}

#[polars_expr(output_type=Float64)]
fn env_middle(inputs: &[Series], kwargs: EnvKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (_, middle, _) = core::env(&input, kwargs.period as usize, kwargs.shift_percentage);
    Ok(option_vec_to_series(middle))
}

#[polars_expr(output_type=Float64)]
fn env_lower(inputs: &[Series], kwargs: EnvKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    let (_, _, lower) = core::env(&input, kwargs.period as usize, kwargs.shift_percentage);
    Ok(option_vec_to_series(lower))
}

#[polars_expr(output_type=Float64)]
fn eom_line(inputs: &[Series], kwargs: EomLineKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let volumes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::eom_line(
        &highs,
        &lows,
        &volumes,
        kwargs.period as usize,
        kwargs.scale,
    )))
}

#[polars_expr(output_type=Float64)]
fn eom_signal(inputs: &[Series], kwargs: EomSignalKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let volumes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::eom_signal(
        &highs,
        &lows,
        &volumes,
        kwargs.period as usize,
        kwargs.signal_period as usize,
        kwargs.scale,
    )))
}

#[polars_expr(output_type=Float64)]
fn erbear(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let lows = series_to_f64_vec(&inputs[0])?;
    let closes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::erbear(
        &lows,
        &closes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn erbull(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let closes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::erbull(
        &highs,
        &closes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn massi_line(inputs: &[Series], kwargs: MassiLineKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::massi_line(
        &highs,
        &lows,
        kwargs.period_ema as usize,
        kwargs.period_sum as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn massi_signal(inputs: &[Series], kwargs: MassiSignalKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::massi_signal(
        &highs,
        &lows,
        kwargs.period_ema as usize,
        kwargs.period_sum as usize,
        kwargs.period_signal as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn mfi(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    let volumes = series_to_f64_vec(&inputs[3])?;
    Ok(option_vec_to_series(core::mfi(
        &highs,
        &lows,
        &closes,
        &volumes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn mom(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::mom(
        &closes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn nvi_line(inputs: &[Series]) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::nvi_line(&closes, &volumes)))
}

#[polars_expr(output_type=Float64)]
fn nvi_signal(inputs: &[Series], kwargs: SignalKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::nvi_signal(
        &closes,
        &volumes,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn obv_line(inputs: &[Series]) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::obv_line(&closes, &volumes)))
}

#[polars_expr(output_type=Float64)]
fn obv_signal(inputs: &[Series], kwargs: SignalKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::obv_signal(
        &closes,
        &volumes,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn pchan_upper(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let (upper, _, _) = core::pchan(&highs, &lows, kwargs.period as usize);
    Ok(option_vec_to_series(upper))
}

#[polars_expr(output_type=Float64)]
fn pchan_middle(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let (_, middle, _) = core::pchan(&highs, &lows, kwargs.period as usize);
    Ok(option_vec_to_series(middle))
}

#[polars_expr(output_type=Float64)]
fn pchan_lower(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let (_, _, lower) = core::pchan(&highs, &lows, kwargs.period as usize);
    Ok(option_vec_to_series(lower))
}

#[polars_expr(output_type=Float64)]
fn ppo_line(inputs: &[Series], kwargs: FastSlowKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::ppo_line(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn ppo_signal(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::ppo_signal(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn ppo_histogram(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::ppo_histogram(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn psar(inputs: &[Series], kwargs: PsarKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::psar(
        &highs,
        &lows,
        &closes,
        kwargs.increment,
        kwargs.initial_acceleration_factor,
        kwargs.max_acceleration_factor,
    )))
}

#[polars_expr(output_type=Float64)]
fn psl(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::psl(
        &closes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn pvi_line(inputs: &[Series]) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::pvi_line(&closes, &volumes)))
}

#[polars_expr(output_type=Float64)]
fn pvi_signal(inputs: &[Series], kwargs: SignalKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::pvi_signal(
        &closes,
        &volumes,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn pvo_line(inputs: &[Series], kwargs: FastSlowKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::pvo_line(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn pvo_signal(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::pvo_signal(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn pvo_histogram(inputs: &[Series], kwargs: FastSlowSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::pvo_histogram(
        &input,
        kwargs.fast_period as usize,
        kwargs.slow_period as usize,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn roc(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::roc(
        &closes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn rsi(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::rsi(
        &input,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn sonar_line(inputs: &[Series], kwargs: SonarLineKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::sonar_line(
        &input,
        kwargs.period as usize,
        kwargs.step as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn sonar_signal(inputs: &[Series], kwargs: SonarSignalKwargs) -> PolarsResult<Series> {
    let input = series_to_f64_vec(&inputs[0])?;
    Ok(option_vec_to_series(core::sonar_signal(
        &input,
        kwargs.period as usize,
        kwargs.step as usize,
        kwargs.signal_period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn stochrsi_percent_k(inputs: &[Series], kwargs: StochRsiKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let (percent_k, _) = core::stochrsi(
        &closes,
        kwargs.period_rsi as usize,
        kwargs.period_k as usize,
        kwargs.period_d as usize,
    );
    Ok(option_vec_to_series(percent_k))
}

#[polars_expr(output_type=Float64)]
fn stochrsi_percent_d(inputs: &[Series], kwargs: StochRsiKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let (_, percent_d) = core::stochrsi(
        &closes,
        kwargs.period_rsi as usize,
        kwargs.period_k as usize,
        kwargs.period_d as usize,
    );
    Ok(option_vec_to_series(percent_d))
}

#[polars_expr(output_type=Float64)]
fn ultosc(inputs: &[Series], kwargs: UltOscKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::ultosc(
        &highs,
        &lows,
        &closes,
        kwargs.period_short as usize,
        kwargs.period_medium as usize,
        kwargs.period_long as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn vr(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let closes = series_to_f64_vec(&inputs[0])?;
    let volumes = series_to_f64_vec(&inputs[1])?;
    Ok(option_vec_to_series(core::vr(
        &closes,
        &volumes,
        kwargs.period as usize,
    )))
}

#[polars_expr(output_type=Float64)]
fn willr(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let highs = series_to_f64_vec(&inputs[0])?;
    let lows = series_to_f64_vec(&inputs[1])?;
    let closes = series_to_f64_vec(&inputs[2])?;
    Ok(option_vec_to_series(core::willr(
        &highs,
        &lows,
        &closes,
        kwargs.period as usize,
    )))
}
