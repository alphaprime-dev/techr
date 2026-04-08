# polars-techr

Python wrapper for `techr` indicators on top of Polars plugins.

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

- Standalone Ichimoku line functions use `period` when they need a single period.
- `ichimoku_leading_span_a` uses `base_line_period` and `conversion_line_period` because it combines two lines.
- Polars plugins keep the output row-aligned with the input, so `ichimoku_leading_span_a` and `ichimoku_leading_span_b` truncate the forward-projected tail from the core result.
