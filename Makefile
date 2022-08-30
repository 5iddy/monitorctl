.PHONY: watch 

PROJECT_NAME=monitorctl

test:
	cargo test --release

docs:
	cargo --no-deps -rp ${PROJECT_NAME}

tidy:
	cargo fmt -p ${PROJECT_NAME}

watch:
	cargo watch -qw src/ -x test -x "doc --no-deps -q -p ${PROJECT_NAME}" -x "fmt -qp ${PROJECT_NAME}" -x "clippy"

build:
	cargo build --profile release --package ${PROJECT_NAME}

install:
	cargo install --profile release --bin ${PROJECT_NAME} --path .