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


def assert_values_close(
    actual: list[float | None],
    expected: list[float | None],
    abs_tol: float = 1e-8,
) -> None:
    assert len(actual) == len(expected)

    for actual_value, expected_value in zip(actual, expected):
        if actual_value is None or expected_value is None:
            assert actual_value is expected_value
            continue
        assert actual_value == pytest.approx(expected_value, abs=abs_tol)


def select_expr(df: pl.DataFrame, expr: pl.Expr, alias: str, lazy: bool) -> pl.Series:
    query = df.lazy() if lazy else df
    result = query.select(expr.alias(alias))
    if lazy:
        result = result.collect()
    return result.get_column(alias)


SeriesExprBuilder = Callable[[], pl.Expr]

# Indicators whose expected fixture length matches the input row count.
CORE_EXPECTED_CASES: list[tuple[str, SeriesExprBuilder, str]] = [
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
    (
        "ichimoku_base_line",
        lambda: ta.ichimoku_base_line(pl.col("high"), pl.col("low"), period=26),
        "ichimoku_base_line",
    ),
    (
        "ichimoku_conversion_line",
        lambda: ta.ichimoku_conversion_line(pl.col("high"), pl.col("low"), period=9),
        "ichimoku_conversion_line",
    ),
    (
        "ichimoku_lagging_span",
        lambda: ta.ichimoku_lagging_span(pl.col("close"), period=26),
        "ichimoku_lagging_span",
    ),
]

# Leading spans are forward-projected in core fixtures, so Polars compares a
# truncated prefix that matches the input row count.
TRUNCATED_CORE_EXPECTED_CASES: list[tuple[str, SeriesExprBuilder, str]] = [
    (
        "ichimoku_leading_span_a",
        lambda: ta.ichimoku_leading_span_a(
            pl.col("high"),
            pl.col("low"),
            base_line_period=26,
            conversion_line_period=9,
        ),
        "ichimoku_leading_span_a",
    ),
    (
        "ichimoku_leading_span_b",
        lambda: ta.ichimoku_leading_span_b(pl.col("high"), pl.col("low"), period=52),
        "ichimoku_leading_span_b",
    ),
]


@pytest.mark.parametrize("symbol", SYMBOLS)
@pytest.mark.parametrize("lazy", [False, True])
@pytest.mark.parametrize(("name", "expr_builder", "expected_name"), CORE_EXPECTED_CASES)
def test_indicator_matches_core_expected(
    symbol: str,
    lazy: bool,
    name: str,
    expr_builder: SeriesExprBuilder,
    expected_name: str,
) -> None:
    """Match core expected fixtures for non-truncated indicator outputs."""
    # given
    df = load_ohlcv(symbol)
    expected = load_expected(expected_name, symbol)

    # when
    result = select_expr(df, expr_builder(), name, lazy)

    # then
    assert_values_close(result.to_list(), expected)


@pytest.mark.parametrize("symbol", SYMBOLS)
@pytest.mark.parametrize("lazy", [False, True])
@pytest.mark.parametrize(
    ("name", "expr_builder", "expected_name"),
    TRUNCATED_CORE_EXPECTED_CASES,
)
def test_indicator_matches_truncated_core_expected(
    symbol: str,
    lazy: bool,
    name: str,
    expr_builder: SeriesExprBuilder,
    expected_name: str,
) -> None:
    """Truncate longer core fixtures to the Polars output height before comparing."""
    # given
    df = load_ohlcv(symbol)
    expected = load_expected(expected_name, symbol)

    # when
    result = select_expr(df, expr_builder(), name, lazy)

    # then
    assert len(expected) > df.height
    expected = expected[: df.height]
    assert_values_close(result.to_list(), expected)


def test_single_input_integer_columns_are_cast_to_float() -> None:
    """Cast integer single-input columns to float before indicator evaluation."""
    # given
    df = pl.DataFrame({"close": [1, 2, 3, 4, 5]})

    # when
    result = df.select(ta.sma(pl.col("close"), period=2).alias("sma")).get_column("sma")

    # then
    assert_values_close(result.to_list(), [None, 1.5, 2.5, 3.5, 4.5])


def test_multi_input_integer_columns_are_cast_to_float() -> None:
    """Cast integer multi-input columns to float before indicator evaluation."""
    # given
    df = pl.DataFrame(
        {
            "high": [11, 12, 13, 14, 15],
            "low": [1, 2, 3, 4, 5],
            "close": [6, 7, 8, 9, 10],
        }
    )

    # when
    result = df.select(
        ta.stochf_percent_k(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            fastk_period=3,
            fastd_period=2,
        ).alias("value")
    ).get_column("value")

    # then
    assert_values_close(result.to_list(), [
        None,
        None,
        58.33333333,
        58.33333333,
        58.33333333,
    ])
