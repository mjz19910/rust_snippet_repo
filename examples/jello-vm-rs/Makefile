.PHONY: run test

run:
	cargo run -q --example jello-vm-rs Main.class

test:
	java test.general 2>&1 >test/general.java.out
	cargo run -q --example jello-vm-rs test/general.class 2>&1 >test/general.rs.out
	diff test/general.java.out test/general.rs.out \
	 && echo "[general] success" \
	 || echo "[general] fail"
