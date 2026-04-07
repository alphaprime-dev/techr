from pathlib import Path
from typing import Any

import polars as pl
from polars.plugins import register_plugin_function

from polars_techr._polars_techr import __version__
from polars_techr.types import IntoExpr

LIB = Path(__file__).parent

__all__ = [
    "__version__",
    "bband_lower",
    "bband_middle",
    "bband_upper",
    "disparity",
    "ema",
    "ichimoku_base_line",
    "ichimoku_conversion_line",
    "ichimoku_lagging_span",
    "ichimoku_leading_span_a",
    "ichimoku_leading_span_b",
    "macd",
    "macd_hist",
    "macd_signal",
    "sma",
    "stoch_percent_d",
    "stoch_percent_k",
    "stochf_percent_d",
    "stochf_percent_k",
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
        kwargs=kwargs,
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
    return _register("ichimoku_base_line", [high, low], {"period": period})


def ichimoku_conversion_line(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("ichimoku_conversion_line", [high, low], {"period": period})


def ichimoku_leading_span_a(
    high: IntoExpr,
    low: IntoExpr,
    *,
    base_line_period: int,
    conversion_line_period: int,
) -> pl.Expr:
    return _register(
        "ichimoku_leading_span_a",
        [high, low],
        {
            "base_line_period": base_line_period,
            "conversion_line_period": conversion_line_period,
        },
    )


def ichimoku_leading_span_b(high: IntoExpr, low: IntoExpr, *, period: int) -> pl.Expr:
    return _register("ichimoku_leading_span_b", [high, low], {"period": period})


def ichimoku_lagging_span(close: IntoExpr, *, period: int) -> pl.Expr:
    return _register("ichimoku_lagging_span", [close], {"period": period})
