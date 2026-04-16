import polars as pl
from typing import TypeAlias
from polars.datatypes import DataType, DataTypeClass

IntoExpr: TypeAlias = pl.Expr | str | pl.Series
PolarsDataType: TypeAlias = DataType | DataTypeClass
