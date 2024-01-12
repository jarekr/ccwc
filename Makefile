
EXEPATH=./target/debug/ccwc

all:
	echo "run test"

.PHONY: build
build:
	cargo build
	@date

.PHONY: test
test: build test_suite
	@date

.PHONY: test_suite
test_suite: test_c test_l test_c_l test_none 

test_c:
	$(EXEPATH) -c test.txt
	@echo

test_c_l:
	$(EXEPATH) -c -l test.txt
	@echo

test_l:
	$(EXEPATH) -l test.txt
	@echo

test_none:
	$(EXEPATH) test.txt
	@echo
