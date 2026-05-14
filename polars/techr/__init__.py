from pathlib import Path
from typing import Any

import polars as pl
from polars.plugins import register_plugin_function

from .types import IntoExpr

LIB = Path(__file__).resolve().parent

__all__ = [
    "ad",
    "adx",
    "adxr",
    "aroon_down",
    "aroon_up",
    "aroonosc",
    "atr",
    "bband_lower",
    "bband_middle",
    "bband_upper",
    "cci",
    "cci_line",
    "cci_signal",
    "cmf",
    "co",
    "cv",
    "disparity",
    "dmi_minus",
    "dmi_plus",
    "efi",
    "ema",
    "env_lower",
    "env_middle",
    "env_upper",
    "eom_line",
    "eom_signal",
    "erbear",
    "erbull",
    "ichimoku_base_line",
    "ichimoku_conversion_line",
    "ichimoku_lagging_span",
    "ichimoku_leading_span_a",
    "ichimoku_leading_span_b",
    "macd",
    "macd_hist",
    "macd_histogram",
    "macd_line",
    "macd_signal",
    "massi_line",
    "massi_signal",
    "mfi",
    "mom",
    "nvi_line",
    "nvi_signal",
    "obv_line",
    "obv_signal",
    "pchan_lower",
    "pchan_middle",
    "pchan_upper",
    "ppo_histogram",
    "ppo_line",
    "ppo_signal",
    "psar",
    "psl",
    "pvi_line",
    "pvi_signal",
    "pvo_histogram",
    "pvo_line",
    "pvo_signal",
    "roc",
    "rsi",
    "rsi_line",
    "rsi_signal",
    "sma",
    "sonar_line",
    "sonar_signal",
    "stoch_percent_d",
    "stoch_percent_k",
    "stochf_percent_d",
    "stochf_percent_k",
    "stochrsi_percent_d",
    "stochrsi_percent_k",
    "trix_line",
    "trix_signal",
    "ultosc",
    "vr",
    "willr",
    "wma",
]


def _register(
    function_name: str,
    args: list[IntoExpr],
    kwargs: dict[str, Any],
) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        function_name=function_name,
        args=args,
        kwargs=kwargs or None,
        is_elementwise=False,
    )


def sma(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("sma", [expr], {"period": period})


def wma(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("wma", [expr], {"period": period})


def ema(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("ema", [expr], {"period": period})


def disparity(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("disparity", [expr], {"period": period})


def macd(expr: IntoExpr, *, fast_period: int, slow_period: int) -> pl.Expr:
    return _register(
        "macd",
        [expr],
        {"fast_period": fast_period, "slow_period": slow_period},
    )


def macd_line(expr: IntoExpr, *, fast_period: int, slow_period: int) -> pl.Expr:
    return _register(
        "macd_line",
        [expr],
        {"fast_period": fast_period, "slow_period": slow_period},
    )


def macd_signal(
    expr: IntoExpr,
    *,
    fast_period: int,
    slow_period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "macd_signal",
        [expr],
        {
            "fast_period": fast_period,
            "slow_period": slow_period,
            "signal_period": signal_period,
        },
    )


def macd_hist(
    expr: IntoExpr,
    *,
    fast_period: int,
    slow_period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "macd_hist",
        [expr],
        {
            "fast_period": fast_period,
            "slow_period": slow_period,
            "signal_period": signal_period,
        },
    )


def macd_histogram(
    expr: IntoExpr,
    *,
    fast_period: int,
    slow_period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "macd_histogram",
        [expr],
        {
            "fast_period": fast_period,
            "slow_period": slow_period,
            "signal_period": signal_period,
        },
    )


def bband_middle(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("bband_middle", [expr], {"period": period})


def bband_lower(expr: IntoExpr, *, period: int, sigma: float) -> pl.Expr:
    return _register("bband_lower", [expr], {"period": period, "sigma": sigma})


def bband_upper(expr: IntoExpr, *, period: int, sigma: float) -> pl.Expr:
    return _register("bband_upper", [expr], {"period": period, "sigma": sigma})


def stochf_percent_k(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    fastk_period: int,
    fastd_period: int,
) -> pl.Expr:
    return _register(
        "stochf_percent_k",
        [high, low, close],
        {"fastk_period": fastk_period, "fastd_period": fastd_period},
    )


def stochf_percent_d(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    fastk_period: int,
    fastd_period: int,
) -> pl.Expr:
    return _register(
        "stochf_percent_d",
        [high, low, close],
        {"fastk_period": fastk_period, "fastd_period": fastd_period},
    )


def stoch_percent_k(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    fastk_period: int,
    slowk_period: int,
    slowd_period: int,
) -> pl.Expr:
    return _register(
        "stoch_percent_k",
        [high, low, close],
        {
            "fastk_period": fastk_period,
            "slowk_period": slowk_period,
            "slowd_period": slowd_period,
        },
    )


def stoch_percent_d(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    fastk_period: int,
    slowk_period: int,
    slowd_period: int,
) -> pl.Expr:
    return _register(
        "stoch_percent_d",
        [high, low, close],
        {
            "fastk_period": fastk_period,
            "slowk_period": slowk_period,
            "slowd_period": slowd_period,
        },
    )


def ichimoku_base_line(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    """Return the Ichimoku base line for a single rolling period."""
    return _register("ichimoku_base_line", [high, low], {"period": period})


def ichimoku_conversion_line(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    """Return the Ichimoku conversion line for a single rolling period."""
    return _register("ichimoku_conversion_line", [high, low], {"period": period})


def ichimoku_leading_span_a(
    high: IntoExpr,
    low: IntoExpr,
    *,
    base_line_period: int,
    conversion_line_period: int,
) -> pl.Expr:
    """Return leading span A, truncated to the input row count."""
    return _register(
        "ichimoku_leading_span_a",
        [high, low],
        {
            "base_line_period": base_line_period,
            "conversion_line_period": conversion_line_period,
        },
    )


def ichimoku_leading_span_b(
    high: IntoExpr,
    low: IntoExpr,
    *,
    period: int,
    base_line_period: int = 26,
) -> pl.Expr:
    """Return leading span B shifted by the base line period and truncated to the input row count."""
    return _register(
        "ichimoku_leading_span_b",
        [high, low],
        {"period": period, "base_line_period": base_line_period},
    )


def ichimoku_lagging_span(close: IntoExpr, *, base_line_period: int) -> pl.Expr:
    """Return the Ichimoku lagging span shifted by the base line period within the input row count."""
    return _register(
        "ichimoku_lagging_span",
        [close],
        {"base_line_period": base_line_period},
    )


def ad(high: IntoExpr, low: IntoExpr, close: IntoExpr, volume: IntoExpr) -> pl.Expr:
    return _register("ad", [high, low, close, volume], {})


def adx(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    dmi_period: int,
    adx_period: int,
) -> pl.Expr:
    return _register(
        "adx",
        [high, low, close],
        {"dmi_period": dmi_period, "adx_period": adx_period},
    )


def adxr(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    dmi_period: int,
    adx_period: int,
    adxr_period: int,
) -> pl.Expr:
    return _register(
        "adxr",
        [high, low, close],
        {
            "dmi_period": dmi_period,
            "adx_period": adx_period,
            "adxr_period": adxr_period,
        },
    )


def aroon_up(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("aroon_up", [high, low], {"period": period})


def aroon_down(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("aroon_down", [high, low], {"period": period})


def aroonosc(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("aroonosc", [high, low], {"period": period})


def atr(high: IntoExpr, low: IntoExpr, close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("atr", [high, low, close], {"period": period})


def cci(high: IntoExpr, low: IntoExpr, close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("cci", [high, low, close], {"period": period})


def cci_line(high: IntoExpr, low: IntoExpr, close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("cci_line", [high, low, close], {"period": period})


def cci_signal(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "cci_signal",
        [high, low, close],
        {"period": period, "signal_period": signal_period},
    )


def cmf(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    volume: IntoExpr,
    *,
    period: int,
) -> pl.Expr:
    return _register("cmf", [high, low, close, volume], {"period": period})


def co(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    volume: IntoExpr,
    *,
    period_short: int,
    period_long: int,
) -> pl.Expr:
    return _register(
        "co",
        [high, low, close, volume],
        {"period_short": period_short, "period_long": period_long},
    )


def cv(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("cv", [high, low], {"period": period})


def dmi_plus(high: IntoExpr, low: IntoExpr, close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("dmi_plus", [high, low, close], {"period": period})


def dmi_minus(
    high: IntoExpr, low: IntoExpr, close: IntoExpr, *, period: int
) -> pl.Expr:
    return _register("dmi_minus", [high, low, close], {"period": period})


def efi(close: IntoExpr, volume: IntoExpr, *, period: int) -> pl.Expr:
    return _register("efi", [close, volume], {"period": period})


def env_upper(
    expr: IntoExpr,
    *,
    period: int,
    shift_percentage: float,
) -> pl.Expr:
    return _register(
        "env_upper",
        [expr],
        {"period": period, "shift_percentage": shift_percentage},
    )


def env_middle(
    expr: IntoExpr,
    *,
    period: int,
    shift_percentage: float,
) -> pl.Expr:
    return _register(
        "env_middle",
        [expr],
        {"period": period, "shift_percentage": shift_percentage},
    )


def env_lower(
    expr: IntoExpr,
    *,
    period: int,
    shift_percentage: float,
) -> pl.Expr:
    return _register(
        "env_lower",
        [expr],
        {"period": period, "shift_percentage": shift_percentage},
    )


def eom_line(
    high: IntoExpr,
    low: IntoExpr,
    volume: IntoExpr,
    *,
    period: int,
    scale: float,
) -> pl.Expr:
    return _register(
        "eom_line",
        [high, low, volume],
        {"period": period, "scale": scale},
    )


def eom_signal(
    high: IntoExpr,
    low: IntoExpr,
    volume: IntoExpr,
    *,
    period: int,
    signal_period: int,
    scale: float,
) -> pl.Expr:
    return _register(
        "eom_signal",
        [high, low, volume],
        {"period": period, "signal_period": signal_period, "scale": scale},
    )


def erbear(low: IntoExpr, close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("erbear", [low, close], {"period": period})


def erbull(high: IntoExpr, close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("erbull", [high, close], {"period": period})


def massi_line(
    high: IntoExpr,
    low: IntoExpr,
    *,
    period_ema: int,
    period_sum: int,
) -> pl.Expr:
    return _register(
        "massi_line",
        [high, low],
        {"period_ema": period_ema, "period_sum": period_sum},
    )


def massi_signal(
    high: IntoExpr,
    low: IntoExpr,
    *,
    period_ema: int,
    period_sum: int,
    period_signal: int,
) -> pl.Expr:
    return _register(
        "massi_signal",
        [high, low],
        {
            "period_ema": period_ema,
            "period_sum": period_sum,
            "period_signal": period_signal,
        },
    )


def mfi(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    volume: IntoExpr,
    *,
    period: int,
) -> pl.Expr:
    return _register("mfi", [high, low, close, volume], {"period": period})


def mom(close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("mom", [close], {"period": period})


def nvi_line(close: IntoExpr, volume: IntoExpr) -> pl.Expr:
    return _register("nvi_line", [close, volume], {})


def nvi_signal(close: IntoExpr, volume: IntoExpr, *, signal_period: int) -> pl.Expr:
    return _register("nvi_signal", [close, volume], {"signal_period": signal_period})


def obv_line(close: IntoExpr, volume: IntoExpr) -> pl.Expr:
    return _register("obv_line", [close, volume], {})


def obv_signal(close: IntoExpr, volume: IntoExpr, *, signal_period: int) -> pl.Expr:
    return _register("obv_signal", [close, volume], {"signal_period": signal_period})


def pchan_upper(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("pchan_upper", [high, low], {"period": period})


def pchan_middle(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("pchan_middle", [high, low], {"period": period})


def pchan_lower(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("pchan_lower", [high, low], {"period": period})


def ppo_line(expr: IntoExpr, *, fast_period: int, slow_period: int) -> pl.Expr:
    return _register(
        "ppo_line",
        [expr],
        {"fast_period": fast_period, "slow_period": slow_period},
    )


def ppo_signal(
    expr: IntoExpr,
    *,
    fast_period: int,
    slow_period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "ppo_signal",
        [expr],
        {
            "fast_period": fast_period,
            "slow_period": slow_period,
            "signal_period": signal_period,
        },
    )


def ppo_histogram(
    expr: IntoExpr,
    *,
    fast_period: int,
    slow_period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "ppo_histogram",
        [expr],
        {
            "fast_period": fast_period,
            "slow_period": slow_period,
            "signal_period": signal_period,
        },
    )


def psar(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    increment: float,
    initial_acceleration_factor: float,
    max_acceleration_factor: float,
) -> pl.Expr:
    return _register(
        "psar",
        [high, low, close],
        {
            "increment": increment,
            "initial_acceleration_factor": initial_acceleration_factor,
            "max_acceleration_factor": max_acceleration_factor,
        },
    )


def psl(close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("psl", [close], {"period": period})


def pvi_line(close: IntoExpr, volume: IntoExpr) -> pl.Expr:
    return _register("pvi_line", [close, volume], {})


def pvi_signal(close: IntoExpr, volume: IntoExpr, *, signal_period: int) -> pl.Expr:
    return _register("pvi_signal", [close, volume], {"signal_period": signal_period})


def pvo_line(expr: IntoExpr, *, fast_period: int, slow_period: int) -> pl.Expr:
    return _register(
        "pvo_line",
        [expr],
        {"fast_period": fast_period, "slow_period": slow_period},
    )


def pvo_signal(
    expr: IntoExpr,
    *,
    fast_period: int,
    slow_period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "pvo_signal",
        [expr],
        {
            "fast_period": fast_period,
            "slow_period": slow_period,
            "signal_period": signal_period,
        },
    )


def pvo_histogram(
    expr: IntoExpr,
    *,
    fast_period: int,
    slow_period: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "pvo_histogram",
        [expr],
        {
            "fast_period": fast_period,
            "slow_period": slow_period,
            "signal_period": signal_period,
        },
    )


def roc(close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("roc", [close], {"period": period})


def rsi(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("rsi", [expr], {"period": period})


def rsi_line(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("rsi_line", [expr], {"period": period})


def rsi_signal(expr: IntoExpr, *, period: int, signal_period: int) -> pl.Expr:
    return _register(
        "rsi_signal",
        [expr],
        {"period": period, "signal_period": signal_period},
    )


def trix_line(expr: IntoExpr, *, period: int) -> pl.Expr:
    return _register("trix_line", [expr], {"period": period})


def trix_signal(expr: IntoExpr, *, period: int, signal_period: int) -> pl.Expr:
    return _register(
        "trix_signal",
        [expr],
        {"period": period, "signal_period": signal_period},
    )


def sonar_line(expr: IntoExpr, *, period: int, step: int) -> pl.Expr:
    return _register("sonar_line", [expr], {"period": period, "step": step})


def sonar_signal(
    expr: IntoExpr,
    *,
    period: int,
    step: int,
    signal_period: int,
) -> pl.Expr:
    return _register(
        "sonar_signal",
        [expr],
        {"period": period, "step": step, "signal_period": signal_period},
    )


def stochrsi_percent_k(
    close: IntoExpr,
    *,
    period_rsi: int,
    period_k: int,
    period_d: int,
) -> pl.Expr:
    return _register(
        "stochrsi_percent_k",
        [close],
        {"period_rsi": period_rsi, "period_k": period_k, "period_d": period_d},
    )


def stochrsi_percent_d(
    close: IntoExpr,
    *,
    period_rsi: int,
    period_k: int,
    period_d: int,
) -> pl.Expr:
    return _register(
        "stochrsi_percent_d",
        [close],
        {"period_rsi": period_rsi, "period_k": period_k, "period_d": period_d},
    )


def ultosc(
    high: IntoExpr,
    low: IntoExpr,
    close: IntoExpr,
    *,
    period_short: int,
    period_medium: int,
    period_long: int,
) -> pl.Expr:
    return _register(
        "ultosc",
        [high, low, close],
        {
            "period_short": period_short,
            "period_medium": period_medium,
            "period_long": period_long,
        },
    )


def vr(close: IntoExpr, volume: IntoExpr, *, period: int) -> pl.Expr:
    return _register("vr", [close, volume], {"period": period})


def willr(high: IntoExpr, low: IntoExpr, close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("willr", [high, low, close], {"period": period})
