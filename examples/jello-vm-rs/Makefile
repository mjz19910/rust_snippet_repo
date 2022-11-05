.PHONY: run test

run:
	cargo run -q --example jello-vm-rs Main.class

test:
	java test.general 2>&1 >test/general.java.out
	cargo run -q --example jello-vm-rs test/general.class 2>&1 >test/general.rs.out
	diff test/general.java.out test/general.rs.out \
	&& echo "[general] success" \
	|| echo "[general] fail"
	java test.print_int 2>&1 >test/print_int.java.out
	cargo run -q --example jello-vm-rs test/print_int.class 2>&1 >test/print_int.rs.out
	diff test/print_int.java.out test/print_int.rs.out \
	&& echo "[print_int] success" \
	|| echo "[print_int] fail"
