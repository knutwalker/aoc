# https://tech.davis-hansson.com/p/make/
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

APP := aoc2022

CARGOFLAGS ?=

# generate release build
all: build
build: target/release/$(APP)

# clean build output
clean: .cargoinstalled
> cargo clean

# update the readme
readme: README.md

.PHONY: all build clean readme

### build targets

target/release/%: .cargoinstalled Cargo.toml Cargo.lock src/lib.rs src/bin/%/*.rs src/bin/%/input/*.txt
> cargo build $(CARGOFLAGS) --bin $* --release

%_bench.jsonld: target/release/%
> cargo bench --quiet --bin $* -- -Z unstable-options --format json > $@

%_bench.md: %_bench.jsonld cargo_bench_filter.jq
> jq -r -s -f cargo_bench_filter.jq $< > $@
> markdown-table-formatter $@

.PRECIOUS: target/release/% %_bench.jsonld

README.md: README.tpl.md aoc2020_bench.md aoc2021_bench.md aoc2022_bench.md
> m4 $< > $@

.cargoinstalled:
> @if ! command -v cargo 2> /dev/null
> @then
>   @echo "Cargo is not installed. Please visit 'https://rustup.rs/' and follow their instructions, or try to run 'curl --proto \"=https\" --tlsv1.2 -sSf https://sh.rustup.rs | sh'"
>   @exit 1
> @fi
> touch .cargoinstalled

# Day specific targets

# Download inputs

i%:
> curl --cookie "session=$$(cat .sessioncookie)" "https://adventofcode.com/2022/day/$*/input" > src/bin/$(APP)/input/day$*.txt
> bat src/bin/$(APP)/input/day$*.txt

# Generate source file
d%:
> m4 -D day=day$* day.rs.tpl > src/bin/$(APP)/day$*.rs
> code src/bin/$(APP)/day$*.rs

# Run tests

ex%:
> cargo watch -x 'test --bin $(APP) -- day$*::tests::test_ex --nocapture'

run%:
> cargo watch -x 'test --release --bin $(APP) -- day$*::tests::test --exact --nocapture'

t%:
> cargo watch -x 'test --release --bin $(APP) -- day$*::tests::test --nocapture'

# Run benchmarks

b%:
> cargo bench --bin $(APP) day$*::tests::bench
