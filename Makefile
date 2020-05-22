ENVFILE=.env
include ${ENVFILE}
export

# Do not run tests on tavern_derive directly: panic=abort is not supported
CARGO_TEST_FLAGS=-p tavern_server -p tavern_pathfinder
CARGO_INCREMENTAL=0
CARGO_VERBOSE=false

ifeq (${CARGO_NIGHTLY}, test)
	CARGO_VERBOSE_FLAG=--verbose
	CARGO_COMMAND=rustup run nightly cargo
	RUSTFLAGS=-Cpanic=abort -Zpanic_abort_tests -Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Z macro-backtrace
else
	CARGO_COMMAND=rustup run stable cargo
	CARGO_VERBOSE_FLAG=
	RUSTFLAGS=
endif

all: test test-db

rustup:
	@if ! which rustup &> /dev/null; then\
		echo "Rustup is not available and is required. Press enter to install, or Ctrl-C to exit.";\
		read unused;\
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;\
	fi

rustup-nightly: rustup
	@if [ -z "$(shell rustup toolchain list | grep "nightly-x86_64-unknown-linux-")" ]; then\
		echo "Installing nightly toolchain";\
		rustup toolchain install nightly;\
	fi

test: rustup-nightly
	${CARGO_COMMAND} test ${CARGO_TEST_FLAGS}

test-db: rustup-nightly
	export RUST_TEST_THREADS=1
	${CARGO_COMMAND} test ${CARGO_TEST_FLAGS} --all-features
