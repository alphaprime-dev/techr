# techr

`techr` exposes technical indicators as Polars expression plugins.

## Installation

```bash
uv add techr
```

## Supported indicators

- Price/trend: `sma`, `wma`, `ema`, `disparity`, `mom`, `roc`, `rsi`, `rsi_line`, `rsi_signal`, `psl`
- Bands/channels: `bband_middle`, `bband_lower`, `bband_upper`, `env_upper`, `env_middle`, `env_lower`, `pchan_upper`, `pchan_middle`, `pchan_lower`
- Momentum/oscillators: `macd`, `macd_line`, `macd_signal`, `macd_hist`, `macd_histogram`, `ppo_line`, `ppo_signal`, `ppo_histogram`, `pvo_line`, `pvo_signal`, `pvo_histogram`, `sonar_line`, `sonar_signal`, `trix_line`, `trix_signal`, `stochf_percent_k`, `stochf_percent_d`, `stoch_percent_k`, `stoch_percent_d`, `stochrsi_percent_k`, `stochrsi_percent_d`
- High/low/volume indicators: `ad`, `adx`, `adxr`, `aroon_up`, `aroon_down`, `aroonosc`, `atr`, `cci`, `cci_line`, `cci_signal`, `cmf`, `co`, `cv`, `dmi_plus`, `dmi_minus`, `efi`, `eom_line`, `eom_signal`, `erbear`, `erbull`, `massi_line`, `massi_signal`, `mfi`, `nvi_line`, `nvi_signal`, `obv_line`, `obv_signal`, `psar`, `pvi_line`, `pvi_signal`, `ultosc`, `vr`, `willr`
- Ichimoku: `ichimoku_base_line`, `ichimoku_conversion_line`, `ichimoku_leading_span_a`, `ichimoku_leading_span_b`, `ichimoku_lagging_span`

## Usage

```python
import polars as pl
import techr as ta

df = pl.DataFrame(
    {
        "high": [11.0, 12.0, 13.0],
        "low": [9.0, 10.0, 11.0],
        "close": [10.0, 11.0, 12.0],
    }
)

result = df.select(
    ta.sma(pl.col("close"), period=2).alias("sma_2"),
    ta.macd(pl.col("close"), fast_period=12, slow_period=26).alias("macd"),
    ta.stochf_percent_k(
        pl.col("high"),
        pl.col("low"),
        pl.col("close"),
        fastk_period=14,
        fastd_period=3,
    ).alias("stochf_k"),
)
```

Null input values are accepted. Indicators preserve row alignment and emit nulls according to the core rolling-window and seed recovery rules.

## Ichimoku Notes

- Standalone Ichimoku rolling-window lines such as `ichimoku_base_line` and `ichimoku_conversion_line` use `period`.
- `ichimoku_leading_span_a` uses `base_line_period` and `conversion_line_period` because it combines two lines.
- `ichimoku_leading_span_b` uses `period` for the rolling window and `base_line_period` for the forward displacement. The Python wrapper defaults `base_line_period` to `26`.
- `ichimoku_lagging_span` uses `base_line_period` for its backward displacement.
- Polars plugins keep the output row-aligned with the input, so `ichimoku_leading_span_a` and `ichimoku_leading_span_b` truncate the forward-projected tail from the core result.

## Development

```bash
cd polars
uv sync --group dev
uv run maturin develop --uv
uv run pytest
```

Build distributable artifacts locally with:

```bash
cd polars
uv run maturin build --release --sdist --out dist
uv run python scripts/check_artifacts.py dist
```

## Release

1. Update the version in `Cargo.toml`.
2. Merge the version bump to `main`.
3. `Polars Release` builds the artifacts and publishes them to PyPI.
4. After the PyPI publish succeeds, the workflow creates the matching `polars-vX.Y.Z` GitHub Release.

If the matching GitHub Release already exists, the workflow exits without publishing a duplicate release.

Before the first release, configure a Trusted Publisher for PyPI on `alphaprime-dev/techr`.
