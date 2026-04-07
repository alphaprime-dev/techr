.DEFAULT_GOAL := all

# using pip install cargo (via maturin via pip) doesn't get the tty handle
# so doesn't render color without some help
export CARGO_TERM_COLOR=$(shell (test -t 0 && echo "always") || echo "auto")

.PHONY: build-dev-polars
build-dev-polars:
	@rm -f polars/polars_techr/*.so
	cd polars && uv run maturin develop --uv


.PHONY: build-prod-polars
build-prod-polars:
	@rm -f polars/polars_techr/*.so
	cd polars && uv run maturin build --release


.PHONY: test-core
test-core:
	cd core && cargo test

.PHONY: test-polars
test-polars:
	cd polars && uv run pytest

.PHONY: test
test: test-core test-polars
