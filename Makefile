.PHONY: all clean-all time-all

PROBLEMS := $(patsubst %.rs,%,$(wildcard p*.rs))

all: $(PROBLEMS)

p%: p%.rs euler/lib.rs
	# TODO: this is a hack where the euler crate must be made already
	# and all dependencies from the problems must be in its dependencies
	rustc -O -L euler/target -L euler/target/deps $<

time-all: $(addprefix time-,$(PROBLEMS))

time-p%: p%
	@time ./$<

clean-all:
	rm -f $(PROBLEMS)
