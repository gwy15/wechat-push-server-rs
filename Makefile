CONFIG = config.default.toml config.ci.toml log4rs.yml
CONFIG_TARGETS = $(addprefix build/config/, $(CONFIG))
MIGRATIONS = $(shell find migrations -type f)
MIGRATIONS_TARGETS = $(addprefix build/, $(MIGRATIONS))

all: build.zip

build.zip: build/server-rs build/diesel build/docker-compose.yml build/Cargo.toml $(CONFIG_TARGETS) $(MIGRATIONS_TARGETS)
	zip build.zip -r build

build/server-rs: $(shell find src/ -type f -name '*.rs')
	mkdir -p build
	cargo build --release
	cp target/release/server-rs $@

build/diesel: ~/.cargo/bin/diesel
	mkdir -p build
	cp $^ $@

build/docker-compose.yml: docker-compose.yml
	mkdir -p build
	cp $^ $@

build/Cargo.toml: Cargo.toml
	cp $^ $@

$(CONFIG_TARGETS): build/config/%: config/%
	mkdir -p build/config
	cp $^ $@

$(MIGRATIONS_TARGETS): build/migrations/%: migrations/%
	dirname $@ | xargs mkdir -p
	cp $^ $@

.PHONY: all clean test

clean:
	rm -rf build build.zip

test: build/server-rs
	./tests/test.sh
