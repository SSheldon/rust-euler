.PHONY: all clean-all time-all

PROBLEMS := $(patsubst src/bin/%.rs,%,$(wildcard src/bin/p*.rs))

all:
	cargo build --release

time-all: $(addprefix time-,$(PROBLEMS))

time-p%: all
	@time target/release/p$*

clean-all:
	cargo clean
