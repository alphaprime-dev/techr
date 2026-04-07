mod expressions;
use pyo3::types::PyModule;
use pyo3::{pymodule, Bound, PyResult};
use pyo3_polars::PolarsAllocator;

#[pymodule]
fn _polars_techr(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}

#[global_allocator]
static ALLOC: PolarsAllocator = PolarsAllocator::new();
