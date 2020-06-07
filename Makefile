ENVFILE=.env
include ${ENVFILE}
export

# Do not run tests on tavern_derive directly: panic=abort is not supported
CARGO_TEST_FLAGS=
CARGO_INCREMENTAL=0
CARGO_VERBOSE=false
CARGO_COVERAGE=false

ifeq (${CARGO_NIGHTLY}, true)
	CARGO_VERBOSE_FLAG=--verbose
	CARGO_COMMAND=rustup run nightly cargo
	RUSTUP_TARGET=rustup-nightly
	RUSTFLAGS=-Z macro-backtrace --cfg nightly --cfg procmacro2_semver_exempt
else
	CARGO_COMMAND=rustup run stable cargo
	CARGO_VERBOSE_FLAG=
	RUSTUP_TARGET=rustup-stable
	RUSTFLAGS=
endif

ifeq (${CARGO_COVERAGE}, true)
	RUSTFLAGS=${RUSTFLAGS} -Cpanic=abort -Zpanic_abort_tests -Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off
endif

all: test-db

postgres:
	@if which systemctl &> /dev/null && ! systemctl is-active postgresql; then\
		echo "Starting postgresql";\
		sudo systemctl start postgresql;\
	fi

# The double sudo is not an error: it prompts for root privileges to run the command as user
# postgres, rather than asking for user postgres' password.

init-dev-db: postgres
	sudo sudo -u postgres psql \
		-c "CREATE DATABASE tavern_test;" \
		-c "CREATE USER ${TAVERN_DB_USER} WITH PASSWORD '${TAVERN_DB_PASS}'" \
		-c "GRANT ALL PRIVILEGES ON ${TAVERN_DB_NAME} TO ${TAVERN_DB_USER};"

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

rustup-stable: rustup
	@if [ -z "$(shell rustup toolchain list | grep "stable-x86_64-unknown-linux-")" ]; then\
		echo "Installing stable toolchain";\
		rustup toolchain install stable;\
	fi

test: ${RUSTUP_TARGET}
	${CARGO_COMMAND} test --manifest-path tavern_server/Cargo.toml ${CARGO_TEST_FLAGS}

test-db: export RUST_TEST_THREADS = 1
test-db: ${RUSTUP_TARGET} postgres
	${CARGO_COMMAND} test ${CARGO_TEST_FLAGS} --all-features

clean: ${RUSTUP_TARGET}
	${CARGO_COMMAND} clean

run: ${RUSTUP_TARGET} postgres
	${CARGO_COMMAND} run
