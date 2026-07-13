CSS_SRC := tailwind.css
CSS_OUT := public/styles/main.css

.PHONY: dev build css watch check clippy fmt test

css:
	npx @tailwindcss/cli -i $(CSS_SRC) -o $(CSS_OUT)

watch:
	npx @tailwindcss/cli -i $(CSS_SRC) -o $(CSS_OUT) --watch

dev: css
	npx @tailwindcss/cli -i $(CSS_SRC) -o $(CSS_OUT) --watch & trunk serve

build: css
	trunk build --release

check:
	cargo check --target wasm32-unknown-unknown

clippy:
	cargo clippy --target wasm32-unknown-unknown -- -D clippy::correctness -D clippy::suspicious

fmt:
	cargo fmt --all --check

test:
	cargo test --lib
