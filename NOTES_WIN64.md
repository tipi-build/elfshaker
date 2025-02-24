Required to build:
==================


- tipi build with https://github.com/nxxm/nxxm-src/pull/1488
- `$env:CXXFLAGS="-MD"` and `$env:CFLAGS="-MD"`
- build with `tipi . -t vs-16-2019-win64-cxx17 -C Release` or `tipi . -t vs-16-2019-win64-cxx17 -C RelWithDebInfo`



Currently the cxx-bridge test still fails on  

```
Running 1 test case...
Creating it in "C:\\Users\\yannic\\AppData\\Local\\Temp\\elfshkr-test\\b135-ab44-8e83-70fd/elfshaker_data"
There are so many num_cpus : 16
Error : Ok(())
There are so many num_cpus : 16
create_snapshot README.md
Options extracted false true false 32
A: 0
D: 1
M: 0
Options extracted false true false 32
A: 1
D: 0
M: 0
Options extracted false false false 0
A: 0
D: 1
M: 0
Packing loose\init 1
Packing loose\myrevision-hash 1
Compressing objects...
There are opts.num_workers 12
Compressing objects... 100%
Options extracted true true false 32
A: 1
D: 0
M: 0
thread '<unnamed>' panicked at src\status.rs:134:78:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0:     0x7ff781965d81 - std::backtrace_rs::backtrace::dbghelp64::trace
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:91
   1:     0x7ff781965d81 - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff781965d81 - std::sys::backtrace::_print_fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:66
   3:     0x7ff781965d81 - std::sys::backtrace::impl$0::print::impl$0::fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:39
   4:     0x7ff78192201a - core::fmt::rt::Argument::fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\fmt\rt.rs:177
   5:     0x7ff78192201a - core::fmt::write
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\fmt\mod.rs:1186
   6:     0x7ff781959457 - std::io::Write::write_fmt<std::sys::pal::windows::stdio::Stderr>
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\io\mod.rs:1839
   7:     0x7ff781965ba5 - std::sys::backtrace::BacktraceLock::print
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:42
   8:     0x7ff7819695bd - std::panicking::default_hook::closure$1
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:268
   9:     0x7ff7819692e1 - std::panicking::default_hook
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:295
  10:     0x7ff781969e94 - std::panicking::rust_panic_with_hook
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:801
  11:     0x7ff781969c82 - std::panicking::begin_panic_handler::closure$0
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:667
  12:     0x7ff781966b2f - std::sys::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:170
  13:     0x7ff7819697de - std::panicking::begin_panic_handler
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:665
  14:     0x7ff7820121d1 - core::panicking::panic_fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\panicking.rs:74
  15:     0x7ff78201229d - core::panicking::panic
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\panicking.rs:148
  16:     0x7ff78201211e - core::option::unwrap_failed
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\option.rs:2012
  17:     0x7ff781a509e4 - <alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<E>>::from::hf83fe4c2a0b9ae5d
  18:     0x7ff781c5d961 - std::sys::backtrace::__rust_begin_short_backtrace::hc56c96c4816e39e4
  19:     0x7ff781c61618 - std::thread::Builder::spawn_unchecked::ha405f803e609aa01
  20:     0x7ff7819806bd - alloc::boxed::impl$48::call_once
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/alloc\src\boxed.rs:2454
  21:     0x7ff7819806bd - alloc::boxed::impl$48::call_once
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/alloc\src\boxed.rs:2454
  22:     0x7ff7819806bd - std::sys::pal::windows::thread::impl$0::new::thread_start
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\pal\windows\thread.rs:55
  23:     0x7ffca47ce8d7 - BaseThreadInitThunk
  24:     0x7ffca575bf2c - RtlUserThreadStart
thread '<unnamed>' panicked at src\status.rs:265:10:
unable to fetch sorted file list from worker thread: RecvError
stack backtrace:
   0:     0x7ff781965d81 - std::backtrace_rs::backtrace::dbghelp64::trace
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:91
   1:     0x7ff781965d81 - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff781965d81 - std::sys::backtrace::_print_fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:66
   3:     0x7ff781965d81 - std::sys::backtrace::impl$0::print::impl$0::fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:39
   4:     0x7ff78192201a - core::fmt::rt::Argument::fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\fmt\rt.rs:177
   5:     0x7ff78192201a - core::fmt::write
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\fmt\mod.rs:1186
   6:     0x7ff781959457 - std::io::Write::write_fmt<std::sys::pal::windows::stdio::Stderr>
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\io\mod.rs:1839
   7:     0x7ff781965ba5 - std::sys::backtrace::BacktraceLock::print
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:42
   8:     0x7ff7819695bd - std::panicking::default_hook::closure$1
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:268
   9:     0x7ff7819692e1 - std::panicking::default_hook
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:295
  10:     0x7ff781969e94 - std::panicking::rust_panic_with_hook
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:801
  11:     0x7ff781969cb9 - std::panicking::begin_panic_handler::closure$0
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:674
  12:     0x7ff781966b2f - std::sys::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:170
  13:     0x7ff7819697de - std::panicking::begin_panic_handler
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:665
  14:     0x7ff7820121d1 - core::panicking::panic_fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\panicking.rs:74
  15:     0x7ff782012830 - core::result::unwrap_failed
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\result.rs:1700
  16:     0x7ff781a6d081 - elfshaker::status::get_app::h4a71c23c2e9d49ad
  17:     0x7ff781a68207 - elfshaker::status::do_status::h97640b418d7a8971
  18:     0x7ff781a4f0ae - elfshaker::status::hc0fe4351882f29fa
  19:     0x7ff7818cc262 - elfshaker$cxxbridge1$status
  20:     0x7ff781edb147 - elfshaker::status
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\corrosion_generated\cxxbridge\elfshaker-cxxbridge\src\lib.cpp:936
  21:     0x7ff781ed1cad - store_with_separate_worktree_smoke_test::test_method
                               at C:\.tipi\vK.w\40999a5-elfshaker\tests\test_cxxbridge.cpp:84
  22:     0x7ff781ed014c - store_with_separate_worktree_smoke_test_invoker
                               at C:\.tipi\vK.w\40999a5-elfshaker\tests\test_cxxbridge.cpp:14
  23:     0x7ff781eba91a - boost::function0<void>::operator()
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\function\function_template.hpp:763
  24:     0x7ff781eba91a - boost::detail::forward::operator()
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1388
  25:     0x7ff781eba91a - boost::detail::function::function_obj_invoker0<boost::detail::forward,int>::invoke
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\function\function_template.hpp:137
  26:     0x7ff781ead76c - boost::detail::do_invoke
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:301
  27:     0x7ff781ead76c - boost::execution_monitor::catch_signals
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1219
  28:     0x7ff781eb3fb9 - boost::execution_monitor::execute
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1301
  29:     0x7ff781ed8bfd - boost::execution_monitor::vexecute
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1397
  30:     0x7ff781eb4158 - boost::unit_test::unit_test_monitor_t::execute_and_translate
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\unit_test_monitor.ipp:49
  31:     0x7ff781eb4ba9 - boost::unit_test::framework::state::execute_test_tree
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\framework.ipp:815
  32:     0x7ff781eb46e5 - boost::unit_test::framework::state::execute_test_tree
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\framework.ipp:740
  33:     0x7ff781ecc32a - boost::unit_test::framework::run
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\framework.ipp:1722
  34:     0x7ff781ed71d6 - boost::unit_test::unit_test_main
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\unit_test_main.ipp:250
  35:     0x7ff78200584c - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  36:     0x7ff78200584c - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  37:     0x7ffca47ce8d7 - BaseThreadInitThunk
  38:     0x7ffca575bf2c - RtlUserThreadStart
thread '<unnamed>' panicked at D:\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cxx-1.0.140\src\unwind.rs:37:9:
panic in ffi function elfshaker::bridge::status, aborting.
stack backtrace:
   0:     0x7ff781965d81 - std::backtrace_rs::backtrace::dbghelp64::trace
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:91
   1:     0x7ff781965d81 - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff781965d81 - std::sys::backtrace::_print_fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:66
   3:     0x7ff781965d81 - std::sys::backtrace::impl$0::print::impl$0::fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:39
   4:     0x7ff78192201a - core::fmt::rt::Argument::fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\fmt\rt.rs:177
   5:     0x7ff78192201a - core::fmt::write
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\fmt\mod.rs:1186
   6:     0x7ff781959457 - std::io::Write::write_fmt<std::sys::pal::windows::stdio::Stderr>
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\io\mod.rs:1839
   7:     0x7ff781965ba5 - std::sys::backtrace::BacktraceLock::print
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:42
   8:     0x7ff7819695bd - std::panicking::default_hook::closure$1
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:268
   9:     0x7ff7819692e1 - std::panicking::default_hook
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:295
  10:     0x7ff781969e94 - std::panicking::rust_panic_with_hook
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:801
  11:     0x7ff781969cb9 - std::panicking::begin_panic_handler::closure$0
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:674
  12:     0x7ff781966b2f - std::sys::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:170
  13:     0x7ff7819697de - std::panicking::begin_panic_handler
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:665
  14:     0x7ff7820121d1 - core::panicking::panic_fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\panicking.rs:74
  15:     0x7ff78200c897 - <cxx::unwind::Guard as core::ops::drop::Drop>::drop::h261ce0c3e3659666
  16:     0x7ff7818cc2d8 - elfshaker$cxxbridge1$status
  17:     0x7ffc8382f540 - _CxxFrameHandler3
  18:     0x7ffc83823a86 - is_exception_typeof
  19:     0x7ffc83822c60 - is_exception_typeof
  20:     0x7ffc8382f351 - _CxxFrameHandler3
  21:     0x7ffca5803f8f - _chkstk
  22:     0x7ffca56b4d22 - RtlUnwindEx
  23:     0x7ffc8382eeda - _C_specific_handler
  24:     0x7ffc838218e5 - is_exception_typeof
  25:     0x7ffc83821d00 - is_exception_typeof
  26:     0x7ffc83822d90 - is_exception_typeof
  27:     0x7ffc8382f351 - _CxxFrameHandler3
  28:     0x7ffca5803f0f - _chkstk
  29:     0x7ffca56b3b78 - RtlWow64GetCurrentCpuArea
  30:     0x7ffca56b2b86 - RtlRaiseException
  31:     0x7ffca2b0bb0a - RaiseException
  32:     0x7ffc83825267 - CxxThrowException
  33:     0x7ff781b01040 - panic_unwind::imp::panic
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/panic_unwind\src\seh.rs:352
  34:     0x7ff781b01040 - panic_unwind::__rust_start_panic
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/panic_unwind\src\lib.rs:99
  35:     0x7ff78196a2a5 - std::panicking::rust_panic
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:862
  36:     0x7ff781969f0c - std::panicking::rust_panic_with_hook
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:826
  37:     0x7ff781969cb9 - std::panicking::begin_panic_handler::closure$0
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:674
  38:     0x7ff781966b2f - std::sys::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\sys\backtrace.rs:170
  39:     0x7ff7819697de - std::panicking::begin_panic_handler
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/std\src\panicking.rs:665
  40:     0x7ff7820121d1 - core::panicking::panic_fmt
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\panicking.rs:74
  41:     0x7ff782012830 - core::result::unwrap_failed
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf\library/core\src\result.rs:1700
  42:     0x7ff781a6d081 - elfshaker::status::get_app::h4a71c23c2e9d49ad
  43:     0x7ff781a68207 - elfshaker::status::do_status::h97640b418d7a8971
  44:     0x7ff781a4f0ae - elfshaker::status::hc0fe4351882f29fa
  45:     0x7ff7818cc262 - elfshaker$cxxbridge1$status
  46:     0x7ff781edb147 - elfshaker::status
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\corrosion_generated\cxxbridge\elfshaker-cxxbridge\src\lib.cpp:936
  47:     0x7ff781ed1cad - store_with_separate_worktree_smoke_test::test_method
                               at C:\.tipi\vK.w\40999a5-elfshaker\tests\test_cxxbridge.cpp:84
  48:     0x7ff781ed014c - store_with_separate_worktree_smoke_test_invoker
                               at C:\.tipi\vK.w\40999a5-elfshaker\tests\test_cxxbridge.cpp:14
  49:     0x7ff781eba91a - boost::function0<void>::operator()
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\function\function_template.hpp:763
  50:     0x7ff781eba91a - boost::detail::forward::operator()
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1388
  51:     0x7ff781eba91a - boost::detail::function::function_obj_invoker0<boost::detail::forward,int>::invoke
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\function\function_template.hpp:137
  52:     0x7ff781ead76c - boost::detail::do_invoke
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:301
  53:     0x7ff781ead76c - boost::execution_monitor::catch_signals
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1219
  54:     0x7ff781eb3fb9 - boost::execution_monitor::execute
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1301
  55:     0x7ff781ed8bfd - boost::execution_monitor::vexecute
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\execution_monitor.ipp:1397
  56:     0x7ff781eb4158 - boost::unit_test::unit_test_monitor_t::execute_and_translate
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\unit_test_monitor.ipp:49
  57:     0x7ff781eb4ba9 - boost::unit_test::framework::state::execute_test_tree
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\framework.ipp:815
  58:     0x7ff781eb46e5 - boost::unit_test::framework::state::execute_test_tree
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\framework.ipp:740
  59:     0x7ff781ecc32a - boost::unit_test::framework::run
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\framework.ipp:1722
  60:     0x7ff781ed71d6 - boost::unit_test::unit_test_main
                               at C:\.tipi\vK.w\40999a5-elfshaker.b\eaf2664\bin\_deps\boost-src\build\vs-16-2019-win64-cxx17\installed\include\boost-1_80\boost\test\impl\unit_test_main.ipp:250
  61:     0x7ff78200584c - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  62:     0x7ff78200584c - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  63:     0x7ffca47ce8d7 - BaseThreadInitThunk
  64:     0x7ffca575bf2c - RtlUserThreadStart
```