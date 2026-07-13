CSS_SRC := tailwind.css
CSS_OUT := public/styles/main.css

# Load .env (if present) and export its vars into the recipe environment, so
# `option_env!("GOOGLE_API_URL")` is baked into the WASM at compile time.
ifneq (,$(wildcard .env))
DOTENV := set -a; . ./.env; set +a;
endif

.PHONY: dev build css watch check clippy fmt test

css:
	npx @tailwindcss/cli -i $(CSS_SRC) -o $(CSS_OUT)

watch:
	npx @tailwindcss/cli -i $(CSS_SRC) -o $(CSS_OUT) --watch

dev: css
	$(DOTENV) npx @tailwindcss/cli -i $(CSS_SRC) -o $(CSS_OUT) --watch & \
	$(DOTENV) trunk serve

build: css
	$(DOTENV) trunk build --release

check:
	cargo check --target wasm32-unknown-unknown

clippy:
	cargo clippy --target wasm32-unknown-unknown -- -D clippy::correctness -D clippy::suspicious

fmt:
	cargo fmt --all --check

test:
	cargo test --lib
