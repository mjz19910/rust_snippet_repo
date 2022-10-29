```rust
core::ptr::drop_in_place<
	&mut rustc_demangle::SizeLimitedFmtAdapter<
		&mut core::fmt::Formatter
	>
>.is_at(0x55555558a1e0 as fn(...));
core::ptr::drop_in_place<
	&mut std::io::Write::write_fmt::Adapter<
		alloc::vec::Vec<u8, alloc::alloc::Global>
	>
>.is_at(0x555555563f40 as fn(...));

sym!(
	"_ZN4core3ptr52drop_in_place$LT$std..thread..local..AccessError$GT$17hb8695687ffc4574dE.llvm.18418849756054565257"
).is_at(0x555555562660);


{ drop_in_place: 0x55555558a1e0, size_of: 0x00, align_of: 0x01, vtable_fns: [0x555555590460] }
{ drop_in_place: 0x55555558a1e0, size_of: 0x00, align_of: 0x01, vtable_fns: [0x555555599860] }
{ drop_in_place: 0x55555558a1e0, size_of: 0x00, align_of: 0x01, vtable_fns: [0x55555558a1c0] }
{ drop_in_place: 0x555555563f40, size_of: 0x00, align_of: 0x01, vtable_fns: [0x555555563800] }
{ drop_in_place: 0x555555563f40, size_of: 0x00, align_of: 0x01, vtable_fns: [0x55555557f010, 0x55555557f070] }
{ drop_in_place: 0x555555563f40, size_of: 0x00, align_of: 0x01, vtable_fns: [0x55555557de10] }
{ drop_in_place: 0x555555563f40, size_of: 0x00, align_of: 0x01, vtable_fns: [0x555555594370] }
{ drop_in_place: 0x555555563f40, size_of: 0x00, align_of: 0x01, vtable_fns: [0x5555555767c0] }
{ drop_in_place: 0x555555562660, size_of: 0x00, align_of: 0x01, vtable_fns: [0x5555555767c0] }
{ drop_in_place: 0x55555555d580, size_of: 0x00, align_of: 0x01, vtable_fns: [0x555555594370] }
{ drop_in_place: 0x5555555940a0, size_of: 0x00, align_of: 0x01, vtable_fns: [0x555555594360] }

```
