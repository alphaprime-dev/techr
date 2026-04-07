import json
from pathlib import Path
from typing import Callable

import polars as pl
import polars_techr as ta
import pytest

ROOT = Path(__file__).resolve().parents[2]
DATA_DIR = ROOT / "data"
EXPECTED_DIR = DATA_DIR / "expected"
SYMBOLS = ("TSLA", "005930")


def load_ohlcv(symbol: str) -> pl.DataFrame:
    rows = json.loads((DATA_DIR / f"{symbol}.json").read_text())
    return pl.DataFrame(
        {
            "open": [row[1] for row in rows],
            "high": [row[2] for row in rows],
            "low": [row[3] for row in rows],
            "close": [row[4] for row in rows],
            "volume": [row[5] for row in rows],
        },
        schema={
            "open": pl.Float64,
            "high": pl.Float64,
            "low": pl.Float64,
            "close": pl.Float64,
            "volume": pl.Float64,
        },
    )


def load_expected(name: str, symbol: str) -> list[float | None]:
    return json.loads((EXPECTED_DIR / f"{name}_{symbol}.json").read_text())


def round_values(values: list[float | None], digits: int = 8) -> list[float | None]:
    rounded: list[float | None] = []
    for value in values:
        rounded.append(None if value is None else round(value, digits))
    return rounded


def assert_values_close(
    actual: list[float | None],
    expected: list[float | None],
    digits: int = 8,
) -> None:
    assert len(actual) == len(expected)
    tolerance = 10 ** (-digits)

    for actual_value, expected_value in zip(actual, expected):
        if actual_value is None or expected_value is None:
            assert actual_value is expected_value
            continue
        assert abs(actual_value - expected_value) <= tolerance


def select_expr(df: pl.DataFrame, expr: pl.Expr, alias: str, lazy: bool) -> pl.Series:
    query = df.lazy() if lazy else df
    result = query.select(expr.alias(alias))
    if lazy:
        result = result.collect()
    return result.get_column(alias)


def rolling_midpoint(highs: list[float], lows: list[float], period: int) -> list[float | None]:
    result: list[float | None] = [None] * len(highs)
    if len(highs) < period:
        return result

    for idx in range(period - 1, len(highs)):
        window_highs = highs[idx + 1 - period : idx + 1]
        window_lows = lows[idx + 1 - period : idx + 1]
        result[idx] = (max(window_highs) + min(window_lows)) / 2
    return result


def shift(values: list[float | None], periods: int) -> list[float | None]:
    shifted: list[float | None] = [None] * len(values)
    for idx, value in enumerate(values):
        target = idx + periods
        if 0 <= target < len(values):
            shifted[target] = value
    return shifted


def ichimoku_expected_base_line(df: pl.DataFrame, period: int) -> list[float | None]:
    return rolling_midpoint(df["high"].to_list(), df["low"].to_list(), period)


def ichimoku_expected_leading_span_a(
    df: pl.DataFrame,
    base_line_period: int,
    conversion_line_period: int,
) -> list[float | None]:
    base_line = rolling_midpoint(df["high"].to_list(), df["low"].to_list(), base_line_period)
    conversion_line = rolling_midpoint(
        df["high"].to_list(),
        df["low"].to_list(),
        conversion_line_period,
    )
    span = [
        None if base is None or conversion is None else (base + conversion) / 2
        for base, conversion in zip(base_line, conversion_line)
    ]
    return shift(span, -26)


def ichimoku_expected_leading_span_b(df: pl.DataFrame, period: int) -> list[float | None]:
    return shift(rolling_midpoint(df["high"].to_list(), df["low"].to_list(), period), -26)


def ichimoku_expected_lagging_span(df: pl.DataFrame, period: int) -> list[float | None]:
    return shift(df["close"].to_list(), period)


SeriesExprBuilder = Callable[[], pl.Expr]

SERIES_CASES: list[tuple[str, SeriesExprBuilder, str]] = [
    ("sma", lambda: ta.sma(pl.col("close"), period=20), "sma"),
    ("wma", lambda: ta.wma(pl.col("close"), period=20), "wma"),
    ("ema", lambda: ta.ema(pl.col("close"), period=20), "ema"),
    ("disparity", lambda: ta.disparity(pl.col("close"), period=20), "disparity"),
    (
        "macd",
        lambda: ta.macd(pl.col("close"), fast_period=12, slow_period=26),
        "macd_line",
    ),
    (
        "macd_signal",
        lambda: ta.macd_signal(
            pl.col("close"),
            fast_period=12,
            slow_period=26,
            signal_period=9,
        ),
        "macd_signal",
    ),
    (
        "macd_hist",
        lambda: ta.macd_hist(
            pl.col("close"),
            fast_period=12,
            slow_period=26,
            signal_period=9,
        ),
        "macd_histogram",
    ),
    ("bband_middle", lambda: ta.bband_middle(pl.col("close"), period=20), "sma"),
    (
        "bband_lower",
        lambda: ta.bband_lower(pl.col("close"), period=20, sigma=2),
        "bband_lower",
    ),
    (
        "bband_upper",
        lambda: ta.bband_upper(pl.col("close"), period=20, sigma=2),
        "bband_upper",
    ),
    (
        "stochf_percent_k",
        lambda: ta.stochf_percent_k(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            fastk_period=14,
            fastd_period=3,
        ),
        "stochf_K",
    ),
    (
        "stochf_percent_d",
        lambda: ta.stochf_percent_d(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            fastk_period=14,
            fastd_period=3,
        ),
        "stochf_D",
    ),
    (
        "stoch_percent_k",
        lambda: ta.stoch_percent_k(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            fastk_period=14,
            slowk_period=3,
            slowd_period=3,
        ),
        "stochs_K",
    ),
    (
        "stoch_percent_d",
        lambda: ta.stoch_percent_d(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            fastk_period=14,
            slowk_period=3,
            slowd_period=3,
        ),
        "stochs_D",
    ),
]

ICHIMOKU_CASES = [
    (
        "ichimoku_base_line",
        lambda: ta.ichimoku_base_line(pl.col("high"), pl.col("low"), period=26),
        lambda df: ichimoku_expected_base_line(df, 26),
    ),
    (
        "ichimoku_conversion_line",
        lambda: ta.ichimoku_conversion_line(pl.col("high"), pl.col("low"), period=9),
        lambda df: ichimoku_expected_base_line(df, 9),
    ),
    (
        "ichimoku_leading_span_a",
        lambda: ta.ichimoku_leading_span_a(
            pl.col("high"),
            pl.col("low"),
            base_line_period=26,
            conversion_line_period=9,
        ),
        lambda df: ichimoku_expected_leading_span_a(df, 26, 9),
    ),
    (
        "ichimoku_leading_span_b",
        lambda: ta.ichimoku_leading_span_b(pl.col("high"), pl.col("low"), period=52),
        lambda df: ichimoku_expected_leading_span_b(df, 52),
    ),
    (
        "ichimoku_lagging_span",
        lambda: ta.ichimoku_lagging_span(pl.col("close"), period=26),
        lambda df: ichimoku_expected_lagging_span(df, 26),
    ),
]


def test_public_api_exports() -> None:
    assert ta.__version__
    for name in (
        "sma",
        "wma",
        "ema",
        "disparity",
        "macd",
        "macd_signal",
        "macd_hist",
        "bband_middle",
        "bband_lower",
        "bband_upper",
        "stochf_percent_k",
        "stochf_percent_d",
        "stoch_percent_k",
        "stoch_percent_d",
        "ichimoku_base_line",
        "ichimoku_conversion_line",
        "ichimoku_leading_span_a",
        "ichimoku_leading_span_b",
        "ichimoku_lagging_span",
    ):
        assert hasattr(ta, name)


@pytest.mark.parametrize("symbol", SYMBOLS)
@pytest.mark.parametrize("lazy", [False, True])
@pytest.mark.parametrize(("name", "expr_builder", "expected_name"), SERIES_CASES)
def test_indicator_matches_core_expected(
    symbol: str,
    lazy: bool,
    name: str,
    expr_builder: SeriesExprBuilder,
    expected_name: str,
) -> None:
    df = load_ohlcv(symbol)
    result = select_expr(df, expr_builder(), name, lazy)
    expected = load_expected(expected_name, symbol)
    assert_values_close(round_values(result.to_list()), round_values(expected))


@pytest.mark.parametrize("symbol", SYMBOLS)
@pytest.mark.parametrize("lazy", [False, True])
@pytest.mark.parametrize(("name", "expr_builder", "expected_builder"), ICHIMOKU_CASES)
def test_ichimoku_matches_alphata_semantics(
    symbol: str,
    lazy: bool,
    name: str,
    expr_builder: SeriesExprBuilder,
    expected_builder: Callable[[pl.DataFrame], list[float | None]],
) -> None:
    df = load_ohlcv(symbol)
    result = select_expr(df, expr_builder(), name, lazy)
    expected = expected_builder(df)
    assert_values_close(round_values(result.to_list()), round_values(expected))


def test_single_input_integer_columns_are_cast_to_float() -> None:
    df = pl.DataFrame({"close": [1, 2, 3, 4, 5]})
    result = df.select(ta.sma(pl.col("close"), period=2).alias("sma")).get_column("sma")
    assert result.to_list() == [None, 1.5, 2.5, 3.5, 4.5]


def test_multi_input_integer_columns_are_cast_to_float() -> None:
    df = pl.DataFrame(
        {
            "high": [11, 12, 13, 14, 15],
            "low": [1, 2, 3, 4, 5],
            "close": [6, 7, 8, 9, 10],
        }
    )
    result = df.select(
        ta.stochf_percent_k(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            fastk_period=3,
            fastd_period=2,
        ).alias("value")
    ).get_column("value")
    assert round_values(result.to_list()) == [
        None,
        None,
        58.33333333,
        58.33333333,
        58.33333333,
    ]
