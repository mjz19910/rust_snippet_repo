.PHONY: run test

run:
	cargo run -q --example jello-vm-rs Main.class

tests := general nop_def print_double print_float print_int print_long print_str

exec_tests := $(foreach t,$(tests),\
../../target/debug/examples/jello-vm-rs test/$(t).class 2>&1 >test/$(t).rs.out; \
diff test/$(t).java.out test/$(t).rs.out && echo "[$(t)] success" || echo "[$(t)] fail";\
)

java_req:=$(foreach t,$(tests),test/$(t).java.out)

test: $(java_req)
	$(exec_tests)
