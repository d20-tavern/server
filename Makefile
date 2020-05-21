ENVFILE=.env
include ${ENVFILE}
export

# Do not run tests on tavern_derive directly: panic=abort is not supported
CARGO_TEST_FLAGS=-p tavern_server -p tavern_pathfinder
CARGO_INCREMENTAL=0
RUSTFLAGS=-Cpanic=abort -Zpanic_abort_tests -Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zmacro-backtrace
CARGO_VERBOSE=false

ifeq (${CARGO_VERBOSE}, true)
CARGO_VERBOSE_FLAG=--verbose
else
CARGO_VERBOSE_FLAG=
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
	rustup run nightly cargo test ${CARGO_TEST_FLAGS} ${CARGO_VERBOSE_FLAG}

test-db: rustup-nightly
	export RUST_TEST_THREADS=1
	rustup run nightly cargo test ${CARGO_TEST_FLAGS} ${CARGO_VERBOSE_FLAG} --all-features

clean: rustup-nightly
	rustup run nightly cargo clean
