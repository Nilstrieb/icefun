# Rust compiler panic error report

While trying to upgrade rustc to the latest nightly, I stumbled across a Rust
compiler panic. This repo contains a minimum-reproducible test case
(well, reasonably compact).

From my brief investigation, it appears that adding the `warp::trace::request()`
combinator causes recent rustc nightly's to panic.

I've bisected the recent nightly versions; `nightly-2023-02-09` is the first
that starts panicking. Running with `nightly-2023-02-08` builds successfully.

To reproduce the bug, just run `cargo build`. Running `cargo check` doesn't
panic, though, since it seems to be codegen related.

The full error log:

```bash
rustc-warp-ice$ cargo build
   Compiling rustc-warp-ice v0.1.0
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1076:22: unexpected unsized tail: hyper::server::server::new_svc::State<hyper::server::conn::AddrStream, futures::future::Ready<std::result::Result<warp::filter::service::FilteredService<warp::trace::internal::WithTrace<[closure@warp::trace::request::{closure#0}], warp::filter::map::Map<warp::filter::FilterFn<[closure@warp::path::end::{closure#0}]>, [closure@src/main.rs:11:18: 11:20]>>>, std::convert::Infallible>>, warp::filter::service::FilteredService<warp::trace::internal::WithTrace<[closure@warp::trace::request::{closure#0}], warp::filter::map::Map<warp::filter::FilterFn<[closure@warp::path::end::{closure#0}]>, [closure@src/main.rs:11:18: 11:20]>>>, hyper::common::exec::Exec, hyper::server::server::NoopWatcher>

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/ef934d9b632b8ac00276558824664c104b92b5f0/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:        0x102f28ba8 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d8000cb115ab1b1
   1:        0x102f7883c - core::fmt::write::hb4a42c7efd8f1690
   2:        0x102f1c4c8 - std::io::Write::write_fmt::h4afe2735edabbc94
   3:        0x102f289bc - std::sys_common::backtrace::print::hff3831971f64cecb
   4:        0x102f2b4c0 - std::panicking::default_hook::{{closure}}::h71a6ecf250628b79
   5:        0x102f2b218 - std::panicking::default_hook::hca770bcc00460b96
   6:        0x10b25f0e8 - rustc_driver_impl[26e6f04b306098c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x102f2bbb8 - std::panicking::rust_panic_with_hook::h165d4369c18cfdb9
   8:        0x10f0401c0 - std[19d8fbcf6afc7ffd]::panicking::begin_panic::<rustc_errors[ea1af16506b93a34]::ExplicitBug>::{closure#0}
   9:        0x10f03ee14 - std[19d8fbcf6afc7ffd]::sys_common::backtrace::__rust_end_short_backtrace::<std[19d8fbcf6afc7ffd]::panicking::begin_panic<rustc_errors[ea1af16506b93a34]::ExplicitBug>::{closure#0}, !>
  10:        0x10f5b3088 - std[19d8fbcf6afc7ffd]::panicking::begin_panic::<rustc_errors[ea1af16506b93a34]::ExplicitBug>
  11:        0x10f090f84 - std[19d8fbcf6afc7ffd]::panic::panic_any::<rustc_errors[ea1af16506b93a34]::ExplicitBug>
  12:        0x10f090578 - <rustc_errors[ea1af16506b93a34]::HandlerInner>::bug::<&alloc[9c94e105aad12a13]::string::String>
  13:        0x10f0901c8 - <rustc_errors[ea1af16506b93a34]::Handler>::bug::<&alloc[9c94e105aad12a13]::string::String>
  14:        0x10f0be91c - rustc_middle[a501841c49e30a1f]::util::bug::opt_span_bug_fmt::<rustc_span[161be7aa2f1e880]::span_encoding::Span>::{closure#0}
  15:        0x10f0bae1c - rustc_middle[a501841c49e30a1f]::ty::context::tls::with_opt::<rustc_middle[a501841c49e30a1f]::util::bug::opt_span_bug_fmt<rustc_span[161be7aa2f1e880]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:        0x10f0bade8 - rustc_middle[a501841c49e30a1f]::ty::context::tls::with_context_opt::<rustc_middle[a501841c49e30a1f]::ty::context::tls::with_opt<rustc_middle[a501841c49e30a1f]::util::bug::opt_span_bug_fmt<rustc_span[161be7aa2f1e880]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:        0x10f0be88c - rustc_middle[a501841c49e30a1f]::util::bug::opt_span_bug_fmt::<rustc_span[161be7aa2f1e880]::span_encoding::Span>
  18:        0x10f5b7b90 - rustc_middle[a501841c49e30a1f]::util::bug::bug_fmt
  19:        0x10d7f58d8 - rustc_monomorphize[681161328a34c57f]::collector::find_vtable_types_for_unsizing
  20:        0x10d7f57e8 - rustc_monomorphize[681161328a34c57f]::collector::find_vtable_types_for_unsizing
  21:        0x10d7f3800 - <rustc_monomorphize[681161328a34c57f]::collector::MirNeighborCollector as rustc_middle[a501841c49e30a1f]::mir::visit::Visitor>::visit_rvalue
  22:        0x10d7f8f14 - rustc_monomorphize[681161328a34c57f]::collector::collect_neighbours
  23:        0x10d7f7844 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  24:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  25:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  26:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  27:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  28:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  29:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  30:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  31:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  32:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  33:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  34:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  35:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  36:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  37:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  38:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  39:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  40:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  41:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  42:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  43:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  44:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  45:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  46:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  47:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  48:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  49:        0x10d7f7c40 - rustc_monomorphize[681161328a34c57f]::collector::collect_items_rec
  50:        0x10d7fb980 - <core[84c79705706f622d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d1e121937e7265f2]::sync::par_for_each_in<alloc[9c94e105aad12a13]::vec::Vec<rustc_middle[a501841c49e30a1f]::mir::mono::MonoItem>, rustc_monomorphize[681161328a34c57f]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[84c79705706f622d]::ops::function::FnOnce<()>>::call_once
  51:        0x10d8073ac - <rustc_session[ff84896cf05d094f]::session::Session>::time::<(), rustc_monomorphize[681161328a34c57f]::collector::collect_crate_mono_items::{closure#1}>
  52:        0x10d7f601c - rustc_monomorphize[681161328a34c57f]::collector::collect_crate_mono_items
  53:        0x10d814230 - rustc_monomorphize[681161328a34c57f]::partitioning::collect_and_partition_mono_items
  54:        0x10e501720 - <rustc_query_system[ee0c09c3b3207d2e]::dep_graph::graph::DepGraph<rustc_middle[a501841c49e30a1f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[a501841c49e30a1f]::ty::context::TyCtxt, (), (&rustc_data_structures[d1e121937e7265f2]::unord::UnordSet<rustc_span[161be7aa2f1e880]::def_id::DefId>, &[rustc_middle[a501841c49e30a1f]::mir::mono::CodegenUnit])>
  55:        0x10e2e0010 - rustc_query_system[ee0c09c3b3207d2e]::query::plumbing::try_execute_query::<rustc_query_impl[5448ec55ad5312ac]::queries::collect_and_partition_mono_items, rustc_query_impl[5448ec55ad5312ac]::plumbing::QueryCtxt>
  56:        0x10e440700 - <rustc_query_impl[5448ec55ad5312ac]::Queries as rustc_middle[a501841c49e30a1f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  57:        0x10b374124 - rustc_codegen_ssa[5598ec4cf5527612]::base::codegen_crate::<rustc_codegen_llvm[f5898e0b00a25cd2]::LlvmCodegenBackend>
  58:        0x10b3a2834 - <rustc_codegen_llvm[f5898e0b00a25cd2]::LlvmCodegenBackend as rustc_codegen_ssa[5598ec4cf5527612]::traits::backend::CodegenBackend>::codegen_crate
  59:        0x10b2fb86c - <rustc_session[ff84896cf05d094f]::session::Session>::time::<alloc[9c94e105aad12a13]::boxed::Box<dyn core[84c79705706f622d]::any::Any>, rustc_interface[fd5e8e13dbefeeaf]::passes::start_codegen::{closure#0}>
  60:        0x10b2e9b10 - rustc_interface[fd5e8e13dbefeeaf]::passes::start_codegen
  61:        0x10b2e8e8c - <rustc_interface[fd5e8e13dbefeeaf]::passes::QueryContext>::enter::<<rustc_interface[fd5e8e13dbefeeaf]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[84c79705706f622d]::result::Result<alloc[9c94e105aad12a13]::boxed::Box<dyn core[84c79705706f622d]::any::Any>, rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>>
  62:        0x10b3097d0 - <rustc_interface[fd5e8e13dbefeeaf]::queries::Queries>::ongoing_codegen
  63:        0x10b2386fc - <rustc_interface[fd5e8e13dbefeeaf]::interface::Compiler>::enter::<rustc_driver_impl[26e6f04b306098c]::run_compiler::{closure#1}::{closure#2}, core[84c79705706f622d]::result::Result<core[84c79705706f622d]::option::Option<rustc_interface[fd5e8e13dbefeeaf]::queries::Linker>, rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>>
  64:        0x10b1dda34 - rustc_span[161be7aa2f1e880]::with_source_map::<core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>, rustc_interface[fd5e8e13dbefeeaf]::interface::run_compiler<core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>, rustc_driver_impl[26e6f04b306098c]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  65:        0x10b22eb90 - <scoped_tls[6ecc173de2707b37]::ScopedKey<rustc_span[161be7aa2f1e880]::SessionGlobals>>::set::<rustc_interface[fd5e8e13dbefeeaf]::interface::run_compiler<core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>, rustc_driver_impl[26e6f04b306098c]::run_compiler::{closure#1}>::{closure#0}, core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>>
  66:        0x10b1f4168 - std[19d8fbcf6afc7ffd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fd5e8e13dbefeeaf]::util::run_in_thread_pool_with_globals<rustc_interface[fd5e8e13dbefeeaf]::interface::run_compiler<core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>, rustc_driver_impl[26e6f04b306098c]::run_compiler::{closure#1}>::{closure#0}, core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>>
  67:        0x10b1e502c - <<std[19d8fbcf6afc7ffd]::thread::Builder>::spawn_unchecked_<rustc_interface[fd5e8e13dbefeeaf]::util::run_in_thread_pool_with_globals<rustc_interface[fd5e8e13dbefeeaf]::interface::run_compiler<core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>, rustc_driver_impl[26e6f04b306098c]::run_compiler::{closure#1}>::{closure#0}, core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[84c79705706f622d]::result::Result<(), rustc_errors[ea1af16506b93a34]::ErrorGuaranteed>>::{closure#1} as core[84c79705706f622d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  68:        0x102f3402c - std::sys::unix::thread::Thread::new::thread_start::h088a4f7b78481523
  69:        0x198e4a06c - __pthread_deallocate

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (ef934d9b6 2023-02-08) running on aarch64-apple-darwin

note: compiler flags: --crate-type bin -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `rustc-warp-ice`
```
