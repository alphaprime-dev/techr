from __future__ import annotations

from pathlib import Path

import polars as pl
import polars_techr as ta


def main() -> int:
    expr = ta.sma(pl.col("x"), period=2)
    if not isinstance(expr, pl.Expr):
        raise TypeError("polars-techr did not return a Polars expression")

    print(f"loaded {Path(ta.__file__).resolve()}")
    print(expr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
