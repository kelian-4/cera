# CI Status

Commit: 213245f74c8df5886a6f759026549af48b35c6ad
Branche: dev
Date: 2026-09-15 04:34:07 UTC
Run: https://github.com/kelian-4/cera/actions/runs/34929306540

| Etape | Resultat |
|---|---|
| Formatage (fmt) | success |
| Clippy | success |
| Build | success |
| Tests | success |

## Log complet - Formatage (fmt)
```
```

## Log complet - Clippy
```
[1m[92m    Updating[0m crates.io index
[1m[92m Downloading[0m crates ...
[1m[92m  Downloaded[0m cfg-if v1.0.4
[1m[92m  Downloaded[0m bitflags v2.13.2
[1m[92m  Downloaded[0m nix v0.27.1
[1m[92m  Downloaded[0m libc v0.2.189
[1m[92m   Compiling[0m libc v0.2.189
[1m[92m    Checking[0m cfg-if v1.0.4
[1m[92m    Checking[0m bitflags v2.13.2
[1m[92m    Checking[0m nix v0.27.1
[1m[92m    Checking[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[91merror[0m[1m: this `if` can be collapsed into the outer `match`[0m
   [1m[94m--> [0msrc/editor/core.rs:213:17
    [1m[94m|[0m
[1m[94m213[0m [1m[94m|[0m [1m[91m/[0m                 if cursor.cy < ob.buffer.num_rows() {
[1m[94m214[0m [1m[94m|[0m [1m[91m|[0m                     cursor.cy += 1;
[1m[94m215[0m [1m[94m|[0m [1m[91m|[0m                 }
    [1m[94m|[0m [1m[91m|_________________^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#collapsible_match
    [1m[94m= [0m[1mnote[0m: `-D clippy::collapsible-match` implied by `-D warnings`
    [1m[94m= [0m[1mhelp[0m: to override `-D warnings` add `#[allow(clippy::collapsible_match)]`
[1m[96mhelp[0m: collapse nested if block
    [1m[94m|[0m
[1m[94m212[0m [92m~ [0m            terminal::ARROW_DOWN
[1m[94m213[0m [92m~ [0m                if cursor.cy < ob.buffer.num_rows() [92m=> [0m{
[1m[94m214[0m [1m[94m|[0m                     cursor.cy += 1;
[1m[94m215[0m [92m~ [0m                }
    [1m[94m|[0m

[1m[91merror[0m[1m: manual implementation of `.is_multiple_of()`[0m
   [1m[94m--> [0msrc/model/row.rs:165:23
    [1m[94m|[0m
[1m[94m165[0m [1m[94m|[0m                 while new_col % tab_stop() != 0 {
    [1m[94m|[0m                       [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mhelp: replace with: `!new_col.is_multiple_of(tab_stop())`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#manual_is_multiple_of
    [1m[94m= [0m[1mnote[0m: `-D clippy::manual-is-multiple-of` implied by `-D warnings`
    [1m[94m= [0m[1mhelp[0m: to override `-D warnings` add `#[allow(clippy::manual_is_multiple_of)]`

[1m[91merror[0m[1m: this can be `std::io::Error::other(_)`[0m
  [1m[94m--> [0msrc/terminal/mod.rs:69:26
   [1m[94m|[0m
[1m[94m69[0m [1m[94m|[0m             .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
   [1m[94m|[0m                          [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#io_other_error
   [1m[94m= [0m[1mnote[0m: `-D clippy::io-other-error` implied by `-D warnings`
   [1m[94m= [0m[1mhelp[0m: to override `-D warnings` add `#[allow(clippy::io_other_error)]`
[1m[96mhelp[0m: use `std::io::Error::other`
   [1m[94m|[0m
[1m[94m69[0m [91m- [0m            .map_err(|e| io::Error::[91mnew[0m([91mio::ErrorKind::Other, [0me))?;
[1m[94m69[0m [92m+ [0m            .map_err(|e| io::Error::[92mother[0m(e))?;
   [1m[94m|[0m

[1m[91merror[0m[1m: this can be `std::io::Error::other(_)`[0m
   [1m[94m--> [0msrc/terminal/mod.rs:101:26
    [1m[94m|[0m
[1m[94m101[0m [1m[94m|[0m             .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    [1m[94m|[0m                          [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#io_other_error
[1m[96mhelp[0m: use `std::io::Error::other`
    [1m[94m|[0m
[1m[94m101[0m [91m- [0m            .map_err(|e| io::Error::[91mnew[0m([91mio::ErrorKind::Other, [0me))?;
[1m[94m101[0m [92m+ [0m            .map_err(|e| io::Error::[92mother[0m(e))?;
    [1m[94m|[0m

[1m[91merror[0m: could not compile `cera` (lib) due to 4 previous errors
[1m[33mwarning[0m: build failed, waiting for other jobs to finish...
[1m[91merror[0m: could not compile `cera` (lib test) due to 4 previous errors
```

## Log complet - Build
```
[1m[92m   Compiling[0m libc v0.2.189
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name libc --edition=2021 /home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.189/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --allow=unused_qualifications '--warn=clippy::unnecessary_semicolon' '--allow=clippy::unnecessary_cast' '--warn=clippy::ptr_as_ptr' '--allow=clippy::non_minimal_cfg' '--allow=clippy::missing_safety_doc' '--warn=clippy::map_unwrap_or' '--warn=clippy::manual_assert' '--allow=clippy::identity_op' '--warn=clippy::explicit_iter_loop' --cfg 'feature="default"' --cfg 'feature="extra_traits"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("align", "const-extern-fn", "default", "extra_traits", "rustc-dep-of-std", "rustc-std-workspace-core", "std", "use_std"))' -C metadata=b9e858c46794286a -C extra-filename=-225b59a4e9bd79b1 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --cap-lints allow --cfg freebsd12 --check-cfg 'cfg(emscripten_old_stat_abi)' --check-cfg 'cfg(espidf_picolibc)' --check-cfg 'cfg(espidf_time32)' --check-cfg 'cfg(freebsd10)' --check-cfg 'cfg(freebsd11)' --check-cfg 'cfg(freebsd12)' --check-cfg 'cfg(freebsd13)' --check-cfg 'cfg(freebsd14)' --check-cfg 'cfg(freebsd15)' --check-cfg 'cfg(gnu_file_offset_bits64)' --check-cfg 'cfg(gnu_time_bits64)' --check-cfg 'cfg(libc_deny_warnings)' --check-cfg 'cfg(linux_time_bits64)' --check-cfg 'cfg(musl_v1_2_3)' --check-cfg 'cfg(musl32_time64)' --check-cfg 'cfg(musl_redir_time64)' --check-cfg 'cfg(vxworks_lt_25_09)' --check-cfg 'cfg(libc_pauthtest)' --check-cfg 'cfg(target_os,values("switch","aix","ohos","hurd","rtems","visionos","nuttx","cygwin","qurt","qnx"))' --check-cfg 'cfg(target_env,values("illumos","wasi","aix","ohos","nto71_iosock"))' --check-cfg 'cfg(target_arch,values("loongarch64","mips32r6","mips64r6","csky"))'`
[1m[92m   Compiling[0m cfg-if v1.0.4
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cfg_if --edition=2018 /home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cfg-if-1.0.4/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("core", "rustc-dep-of-std"))' -C metadata=fd47af220b565fac -C extra-filename=-e202c1210b381f60 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --cap-lints allow`
[1m[92m   Compiling[0m bitflags v2.13.2
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name bitflags --edition=2021 /home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.13.2/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("arbitrary", "bytemuck", "example_generated", "serde", "serde_core", "std"))' -C metadata=8c1a3eb8f2630e06 -C extra-filename=-86956476ac6de8d5 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --cap-lints allow`
[1m[92m   Compiling[0m nix v0.27.1
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name nix --edition=2021 /home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nix-0.27.1/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="ioctl"' --cfg 'feature="poll"' --cfg 'feature="process"' --cfg 'feature="signal"' --cfg 'feature="term"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("acct", "aio", "default", "dir", "env", "event", "feature", "fs", "hostname", "inotify", "ioctl", "kmod", "memoffset", "mman", "mount", "mqueue", "net", "personality", "pin-utils", "poll", "process", "pthread", "ptrace", "quota", "reboot", "resource", "sched", "signal", "socket", "term", "time", "ucontext", "uio", "user", "zerocopy"))' -C metadata=29e502236352d808 -C extra-filename=-61091f233f82b102 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rmeta --extern cfg_if=/home/runner/work/cera/cera/target/debug/deps/libcfg_if-e202c1210b381f60.rmeta --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rmeta --cap-lints allow`
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=7684b1cb7fc3ad41 -C extra-filename=-265f65eb0fbef32c --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rmeta --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rmeta --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rmeta`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=0373c7d690e4aeca -C extra-filename=-4ea6f987a5324741 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-265f65eb0fbef32c.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rlib`
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 2.45s
```

## Log complet - Tests
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m nix v0.27.1
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=8b06f7f737320b48 -C extra-filename=-24fe9b87231282b6 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-265f65eb0fbef32c.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=acaf246eb6f4080d -C extra-filename=-61406aa377921b4c --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rlib`
[1m[92m    Finished[0m `test` profile [unoptimized + debuginfo] target(s) in 0.70s
[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-61406aa377921b4c`

running 29 tests
test editor::buffer_manager::tests::test_multiple_buffers ... ok
test editor::undo_stack::tests::test_coalescing ... ok
test io::config::tests::test_config_defaults_on_missing_file ... ok
test io::config::tests::test_config_loads_real_file ... ok
test io::file_io::tests::test_open_existing_file ... ok
test io::file_io::tests::test_open_nonexistent_file_creates_empty_buffer ... ok
test io::file_io::tests::test_save_writes_to_disk ... ok
test io::json::tests::test_json_invalid ... ok
test io::json::tests::test_json_nested_object ... ok
test io::json::tests::test_json_parsing ... ok
test model::buffer::tests::test_delete_char_merges_rows ... ok
test model::buffer::tests::test_dirty_flag ... ok
test model::buffer::tests::test_insert_newline_splits_row ... ok
test model::buffer::tests::test_to_string_content ... ok
test model::row::tests::test_cx_rx_roundtrip ... ok
test model::row::tests::test_insert_delete ... ok
test model::row::tests::test_tab_expansion ... ok
test model::row::tests::test_utf8_insert_delete ... ok
test model::selection::tests::test_selection_range ... ok
test model::utf8::tests::test_byte_offset_for_char_index ... ok
test model::utf8::tests::test_char_index_for_byte_offset ... ok
test model::utf8::tests::test_codepoint_count ... ok
test model::utf8::tests::test_display_width_cjk ... ok
test model::utf8::tests::test_substr_by_char ... ok
test syntax::tests::test_syntax_selection ... ok
test syntax::tests::test_update_syntax_keyword ... ok
test view::colors::tests::test_ansi256_white ... ok
test view::colors::tests::test_true_color_sequence ... ok
test editor::clipboard::tests::test_clipboard_fallback_roundtrip ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-24fe9b87231282b6`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m   Doc-tests[0m cera
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name cera --test src/lib.rs --test-run-directory /home/runner/work/cera/cera --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-265f65eb0fbef32c.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rlib -L dependency=/home/runner/work/cera/cera/target/debug/deps -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' --error-format human`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
