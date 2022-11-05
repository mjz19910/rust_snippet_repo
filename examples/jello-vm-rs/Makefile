.PHONY: run test

run:
	cargo run -q --example jello-vm-rs Main.class

tests := general print_int print_float print_str print_long nop_def print_double

exec_tests := $(foreach t,$(tests),java test.$(t) 2>&1 >test/$(t).java.out; \
cargo run -q --example jello-vm-rs test/$(t).class 2>&1 >test/$(t).rs.out; \
diff test/$(t).java.out test/$(t).rs.out && echo "[$(t)] success" || echo "[$(t)] fail";\
)

test:
	$(exec_tests)
