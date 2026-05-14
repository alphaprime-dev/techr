import json
from pathlib import Path
from typing import Callable, cast

import polars as pl
import pytest
import techr as ta

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
    if lazy:
        result = cast(pl.DataFrame, df.lazy().select(expr.alias(alias)).collect())
        return result.get_column(alias)
    return df.select(expr.alias(alias)).get_column(alias)


SeriesExprBuilder = Callable[[], pl.Expr]
CASE_TOLERANCES = {
    "ad": 1e-4,
    "co": 1e-4,
    "stochrsi_percent_d": 1e-4,
    "stochrsi_percent_k": 1e-4,
}

# Indicators whose expected fixture length matches the input row count.
CORE_EXPECTED_CASES: list[tuple[str, SeriesExprBuilder, str]] = [
    (
        "ad",
        lambda: ta.ad(pl.col("high"), pl.col("low"), pl.col("close"), pl.col("volume")),
        "ad",
    ),
    (
        "adx",
        lambda: ta.adx(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            dmi_period=14,
            adx_period=14,
        ),
        "adx",
    ),
    (
        "adxr",
        lambda: ta.adxr(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            dmi_period=14,
            adx_period=14,
            adxr_period=14,
        ),
        "adxr",
    ),
    (
        "aroon_up",
        lambda: ta.aroon_up(pl.col("high"), pl.col("low"), period=25),
        "aroon_up",
    ),
    (
        "aroon_down",
        lambda: ta.aroon_down(pl.col("high"), pl.col("low"), period=25),
        "aroon_down",
    ),
    (
        "aroonosc",
        lambda: ta.aroonosc(pl.col("high"), pl.col("low"), period=25),
        "aroonosc",
    ),
    (
        "atr",
        lambda: ta.atr(pl.col("high"), pl.col("low"), pl.col("close"), period=20),
        "atr",
    ),
    ("sma", lambda: ta.sma(pl.col("close"), period=20), "sma"),
    ("wma", lambda: ta.wma(pl.col("close"), period=20), "wma"),
    ("ema", lambda: ta.ema(pl.col("close"), period=20), "ema"),
    ("disparity", lambda: ta.disparity(pl.col("close"), period=20), "disparity"),
    (
        "cci",
        lambda: ta.cci(pl.col("high"), pl.col("low"), pl.col("close"), period=20),
        "cci",
    ),
    (
        "cmf",
        lambda: ta.cmf(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            pl.col("volume"),
            period=21,
        ),
        "cmf",
    ),
    (
        "co",
        lambda: ta.co(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            pl.col("volume"),
            period_short=3,
            period_long=10,
        ),
        "co",
    ),
    ("cv", lambda: ta.cv(pl.col("high"), pl.col("low"), period=10), "cv"),
    (
        "dmi_plus",
        lambda: ta.dmi_plus(pl.col("high"), pl.col("low"), pl.col("close"), period=14),
        "dmi_plus",
    ),
    (
        "dmi_minus",
        lambda: ta.dmi_minus(pl.col("high"), pl.col("low"), pl.col("close"), period=14),
        "dmi_minus",
    ),
    (
        "efi",
        lambda: ta.efi(pl.col("close"), pl.col("volume"), period=14),
        "efi",
    ),
    (
        "env_upper",
        lambda: ta.env_upper(pl.col("close"), period=20, shift_percentage=10.0),
        "env_upper",
    ),
    (
        "env_middle",
        lambda: ta.env_middle(pl.col("close"), period=20, shift_percentage=10.0),
        "sma",
    ),
    (
        "env_lower",
        lambda: ta.env_lower(pl.col("close"), period=20, shift_percentage=10.0),
        "env_lower",
    ),
    (
        "eom_line",
        lambda: ta.eom_line(
            pl.col("high"),
            pl.col("low"),
            pl.col("volume"),
            period=14,
            scale=10000.0,
        ),
        "eom_line",
    ),
    (
        "eom_signal",
        lambda: ta.eom_signal(
            pl.col("high"),
            pl.col("low"),
            pl.col("volume"),
            period=14,
            signal_period=3,
            scale=10000.0,
        ),
        "eom_signal",
    ),
    (
        "erbear",
        lambda: ta.erbear(pl.col("low"), pl.col("close"), period=13),
        "erbear",
    ),
    (
        "erbull",
        lambda: ta.erbull(pl.col("high"), pl.col("close"), period=13),
        "erbull",
    ),
    (
        "macd",
        lambda: ta.macd(pl.col("close"), fast_period=12, slow_period=26),
        "macd_line",
    ),
    (
        "macd_line",
        lambda: ta.macd_line(pl.col("close"), fast_period=12, slow_period=26),
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
    (
        "macd_histogram",
        lambda: ta.macd_histogram(
            pl.col("close"),
            fast_period=12,
            slow_period=26,
            signal_period=9,
        ),
        "macd_histogram",
    ),
    (
        "massi_line",
        lambda: ta.massi_line(
            pl.col("high"), pl.col("low"), period_ema=9, period_sum=25
        ),
        "massi_line",
    ),
    (
        "massi_signal",
        lambda: ta.massi_signal(
            pl.col("high"),
            pl.col("low"),
            period_ema=9,
            period_sum=25,
            period_signal=9,
        ),
        "massi_signal",
    ),
    (
        "mfi",
        lambda: ta.mfi(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            pl.col("volume"),
            period=14,
        ),
        "mfi",
    ),
    ("mom", lambda: ta.mom(pl.col("close"), period=10), "mom"),
    (
        "nvi_line",
        lambda: ta.nvi_line(pl.col("close"), pl.col("volume")),
        "nvi_line",
    ),
    (
        "nvi_signal",
        lambda: ta.nvi_signal(pl.col("close"), pl.col("volume"), signal_period=255),
        "nvi_signal",
    ),
    (
        "obv_line",
        lambda: ta.obv_line(pl.col("close"), pl.col("volume")),
        "obv_line",
    ),
    (
        "obv_signal",
        lambda: ta.obv_signal(pl.col("close"), pl.col("volume"), signal_period=9),
        "obv_signal",
    ),
    (
        "pchan_upper",
        lambda: ta.pchan_upper(pl.col("high"), pl.col("low"), period=20),
        "pchan_upper",
    ),
    (
        "pchan_middle",
        lambda: ta.pchan_middle(pl.col("high"), pl.col("low"), period=20),
        "pchan_middle",
    ),
    (
        "pchan_lower",
        lambda: ta.pchan_lower(pl.col("high"), pl.col("low"), period=20),
        "pchan_lower",
    ),
    (
        "ppo_line",
        lambda: ta.ppo_line(pl.col("close"), fast_period=12, slow_period=26),
        "ppo_line",
    ),
    (
        "ppo_signal",
        lambda: ta.ppo_signal(
            pl.col("close"),
            fast_period=12,
            slow_period=26,
            signal_period=9,
        ),
        "ppo_signal",
    ),
    (
        "ppo_histogram",
        lambda: ta.ppo_histogram(
            pl.col("close"),
            fast_period=12,
            slow_period=26,
            signal_period=9,
        ),
        "ppo_histogram",
    ),
    (
        "psar",
        lambda: ta.psar(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            increment=0.02,
            initial_acceleration_factor=0.02,
            max_acceleration_factor=0.2,
        ),
        "psar",
    ),
    ("psl", lambda: ta.psl(pl.col("close"), period=12), "psl"),
    (
        "pvi_line",
        lambda: ta.pvi_line(pl.col("close"), pl.col("volume")),
        "pvi_line",
    ),
    (
        "pvi_signal",
        lambda: ta.pvi_signal(pl.col("close"), pl.col("volume"), signal_period=255),
        "pvi_signal",
    ),
    (
        "pvo_line",
        lambda: ta.pvo_line(pl.col("volume"), fast_period=12, slow_period=26),
        "pvo_line",
    ),
    (
        "pvo_signal",
        lambda: ta.pvo_signal(
            pl.col("volume"),
            fast_period=12,
            slow_period=26,
            signal_period=9,
        ),
        "pvo_signal",
    ),
    (
        "pvo_histogram",
        lambda: ta.pvo_histogram(
            pl.col("volume"),
            fast_period=12,
            slow_period=26,
            signal_period=9,
        ),
        "pvo_histogram",
    ),
    ("roc", lambda: ta.roc(pl.col("close"), period=20), "roc"),
    ("rsi", lambda: ta.rsi(pl.col("close"), period=14), "rsi"),
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
        lambda: ta.ichimoku_lagging_span(pl.col("close"), base_line_period=26),
        "ichimoku_lagging_span",
    ),
    (
        "sonar_line",
        lambda: ta.sonar_line(pl.col("close"), period=9, step=6),
        "sonar_line",
    ),
    (
        "sonar_signal",
        lambda: ta.sonar_signal(pl.col("close"), period=9, step=6, signal_period=5),
        "sonar_signal",
    ),
    ("trix_line", lambda: ta.trix_line(pl.col("close"), period=12), "trix_line"),
    (
        "trix_signal",
        lambda: ta.trix_signal(pl.col("close"), period=12, signal_period=9),
        "trix_signal",
    ),
    (
        "stochrsi_percent_k",
        lambda: ta.stochrsi_percent_k(
            pl.col("close"), period_rsi=14, period_k=14, period_d=3
        ),
        "stochrsi_K",
    ),
    (
        "stochrsi_percent_d",
        lambda: ta.stochrsi_percent_d(
            pl.col("close"), period_rsi=14, period_k=14, period_d=3
        ),
        "stochrsi_D",
    ),
    (
        "ultosc",
        lambda: ta.ultosc(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            period_short=7,
            period_medium=14,
            period_long=28,
        ),
        "ultosc",
    ),
    (
        "vr",
        lambda: ta.vr(pl.col("close"), pl.col("volume"), period=20),
        "vr",
    ),
    (
        "willr",
        lambda: ta.willr(pl.col("high"), pl.col("low"), pl.col("close"), period=14),
        "willr",
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
        lambda: ta.ichimoku_leading_span_b(
            pl.col("high"),
            pl.col("low"),
            period=52,
            base_line_period=26,
        ),
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
    assert_values_close(
        result.to_list(),
        expected,
        abs_tol=CASE_TOLERANCES.get(name, 1e-8),
    )


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
    assert_values_close(
        result.to_list(),
        [
            None,
            None,
            58.33333333,
            58.33333333,
            58.33333333,
        ],
    )


@pytest.mark.parametrize("lazy", [False, True])
def test_single_input_null_values_follow_core_gap_semantics(lazy: bool) -> None:
    """Accept null values for single-input indicators."""
    # given
    df = pl.DataFrame({"close": [1.0, None, 3.0, 4.0]})

    # when
    result = select_expr(df, ta.sma(pl.col("close"), period=2), "sma", lazy)

    # then
    assert_values_close(result.to_list(), [None, None, None, 3.5])


@pytest.mark.parametrize("lazy", [False, True])
def test_rsi_signal_matches_ema_of_rsi(lazy: bool) -> None:
    """RSI signal is the EMA of the RSI line."""
    # given
    df = load_ohlcv("TSLA")

    # when
    signal = select_expr(
        df,
        ta.rsi_signal(pl.col("close"), period=14, signal_period=9),
        "signal",
        lazy,
    )
    expected = select_expr(
        df,
        ta.ema(ta.rsi(pl.col("close"), period=14), period=9),
        "expected",
        lazy,
    )

    # then
    assert_values_close(signal.to_list(), expected.to_list())


@pytest.mark.parametrize("lazy", [False, True])
def test_cci_signal_matches_ema_of_cci(lazy: bool) -> None:
    """CCI signal is the EMA of the CCI line."""
    # given
    df = load_ohlcv("TSLA")

    # when
    signal = select_expr(
        df,
        ta.cci_signal(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            period=20,
            signal_period=9,
        ),
        "signal",
        lazy,
    )
    expected = select_expr(
        df,
        ta.ema(
            ta.cci(pl.col("high"), pl.col("low"), pl.col("close"), period=20),
            period=9,
        ),
        "expected",
        lazy,
    )

    # then
    assert_values_close(signal.to_list(), expected.to_list())


@pytest.mark.parametrize("lazy", [False, True])
def test_multi_input_null_values_follow_core_gap_semantics(lazy: bool) -> None:
    """Accept null values for multi-input indicators."""
    # given
    df = pl.DataFrame(
        {
            "high": [5.0, 7.0, None, 10.0, 12.0],
            "low": [1.0, 3.0, None, 6.0, 8.0],
            "close": [4.0, 5.0, None, 8.0, 11.0],
        }
    )

    # when
    result = select_expr(
        df,
        ta.stochf_percent_k(
            pl.col("high"),
            pl.col("low"),
            pl.col("close"),
            fastk_period=2,
            fastd_period=2,
        ),
        "value",
        lazy,
    )

    # then
    assert_values_close(
        result.to_list(),
        [None, 66.66666666666666, None, None, 83.33333333333334],
    )
