# The binary name.
BINARY ?= deminer

OK_COLOR=\033[32;01m
NO_COLOR=\033[0m
MAKE_COLOR=\033[36m%-20s\033[0m

IGNORE_COVERAGE_FOR="src/main.rs"

all: build lint test ## build application, run linters and tests

build: ## build application
	@echo "$(OK_COLOR)==> Building application$(NO_COLOR)"
	cargo build

run: ## run application
	@echo "$(OK_COLOR)==> Running application$(NO_COLOR)"
	cargo run

fmt: ## format code
	@echo "$(OK_COLOR)==> Formatting$(NO_COLOR)"
	cargo fmt

test: ## run tests
	@echo "$(OK_COLOR)==> Running tests$(NO_COLOR)"
	RUSTFLAGS="-Cinstrument-coverage" cargo test

test-cover-all: test  ## run tests with code coverage and show html report in browser
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --ignore $(IGNORE_COVERAGE_FOR) -o ./target/debug/coverage/
	open ./target/debug/coverage/index.html

lint: ## run linters
	@echo "$(OK_COLOR)==> Linting$(NO_COLOR)"
	cargo clippy

clean: ## cleans-up artifacts
	@echo "$(OK_COLOR)==> Cleaning up$(NO_COLOR)"
	@rm -rf ./target
	@rm -rf ./*.profraw

help: ## show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(MAKE_COLOR) %s\n", $$1, $$2}'

# To avoid unintended conflicts with file names, always add to .PHONY
# unless there is a reason not to.
# https://www.gnu.org/software/make/manual/html_node/Phony-Targets.html
.PHONY: all build run fmt
.PHONY: lint test
.PHONY: test-cover-all
.PHONY: clean help
