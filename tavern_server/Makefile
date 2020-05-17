ENVFILE=.env
include ${ENVFILE}
export

all:
	@echo "Makefile needs your attention"

test:
	cargo test

test-db:
	env | sort
	export RUST_TEST_THREADS=1
	cargo test --features db-test
