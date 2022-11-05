.PHONY: run test build_java

run:
	cargo run -q --example jello-vm-rs Main.class

tests := general nop_def print_double print_float print_int print_long print_str

../../target/debug/examples/jello-vm-rs: *.rs
	cargo build --example jello-vm-rs

exec_tests := $(foreach t,$(tests),\
../../target/debug/examples/jello-vm-rs test/$(t).class 2>&1 >test/$(t).rs.out; \
diff test/$(t).java.out test/$(t).rs.out && echo "[$(t)] success" || echo "[$(t)] fail";\
)

java_req_out:=$(foreach t,$(tests),test/$(t).java.out)

test: $(java_req_out)
	$(exec_tests)

build_test_out: $(foreach t,$(tests),test/$(t).class)
	$(foreach t,$(tests),java test.$(t) 2>&1 >test/$(t).java.out;)

build_java: $(foreach t,$(tests),test/$(t).class)

$(foreach t,$(tests),test/$(t).class): $(foreach t,$(tests),test/$(t).java)
	javac $^
