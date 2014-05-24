.PHONY: all clean-all time-all

PROBLEMS := $(patsubst %.rs,%,$(wildcard p*.rs))

all: $(PROBLEMS)

p%: p%.rs euler/mod.rs
	rustc -O $<

time-all: $(addprefix time-,$(PROBLEMS))

time-p%: p%
	@time ./$<

clean-all:
	rm -f $(PROBLEMS)
