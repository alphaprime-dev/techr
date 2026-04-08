# polars-techr

`polars-techr` exposes `techr` indicators as Polars expression plugins.

## Installation

```bash
uv add polars-techr
```

## Supported indicators

- `sma`, `wma`, `ema`, `disparity`
- `macd`, `macd_signal`, `macd_hist`
- `bband_middle`, `bband_lower`, `bband_upper`
- `stochf_percent_k`, `stochf_percent_d`
- `stoch_percent_k`, `stoch_percent_d`
- `ichimoku_base_line`, `ichimoku_conversion_line`
- `ichimoku_leading_span_a`, `ichimoku_leading_span_b`, `ichimoku_lagging_span`

## Usage

```python
import polars as pl
import polars_techr as ta

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
2. Optionally build release artifacts locally for a final preflight check.
3. Optionally run the `Polars Release` workflow manually to publish the current ref to TestPyPI.
4. Create and push a `polars-vX.Y.Z` tag to publish to PyPI.

Before the first release, configure Trusted Publishers for both PyPI and TestPyPI on `alphaprime-dev/techr`.
