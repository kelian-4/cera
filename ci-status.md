# CI Status

Commit: 84297e1915cdeeb50c1551849435e95f1d7502b7
Branche: main
Date: 2026-09-27 06:10:33 UTC
Run: https://github.com/kelian-4/cera/actions/runs/36299301045

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
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 2.93s
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
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 2.80s
```

## Log complet - Tests
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m nix v0.27.1
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=acaf246eb6f4080d -C extra-filename=-61406aa377921b4c --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=8b06f7f737320b48 -C extra-filename=-24fe9b87231282b6 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-265f65eb0fbef32c.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rlib`
[1m[92m    Finished[0m `test` profile [unoptimized + debuginfo] target(s) in 0.90s
[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-61406aa377921b4c`

running 54 tests
test editor::buffer_manager::tests::test_multiple_buffers ... ok
test editor::undo_stack::tests::test_coalescing ... ok
test editor::undo_stack::tests::test_coalescing_breaks_on_word_boundary ... ok
test i18n::tests::test_all_embedded_languages_parse_successfully ... ok
test editor::clipboard::tests::test_clipboard_fallback_roundtrip ... ok
test i18n::tests::test_detect_system_language_parses_locale_format ... ok
test i18n::tests::test_load_explicit_language ... ok
test i18n::tests::test_all_languages_have_the_same_keys_as_fallback ... ok
test i18n::tests::test_load_falls_back_to_english_for_unknown_language ... ok
test i18n::tests::test_missing_key_returns_key_itself ... ok
test io::config::tests::test_config_defaults_on_missing_file ... ok
test io::config::tests::test_config_loads_real_file ... ok
test io::config::tests::test_default_config_path_ends_with_config_json ... ok
test io::config::tests::test_ensure_config_file_exists_creates_file_with_parent_dir ... ok
test io::config::tests::test_ensure_config_file_exists_never_overwrites_existing_file ... ok
test i18n::tests::test_t1_replaces_placeholder ... ok
test io::file_io::tests::test_open_existing_file ... ok
test io::file_io::tests::test_open_nonexistent_file_creates_empty_buffer ... ok
test io::json::tests::test_json_invalid ... ok
test io::file_io::tests::test_save_writes_to_disk ... ok
test io::json::tests::test_json_nested_object ... ok
test io::json::tests::test_json_parsing ... ok
test model::buffer::tests::test_delete_char_merges_rows ... ok
test model::buffer::tests::test_dirty_flag ... ok
test model::buffer::tests::test_insert_newline_splits_row ... ok
test model::buffer::tests::test_new_buffer_has_one_empty_line ... ok
test model::buffer::tests::test_set_lines_empty_still_has_one_line ... ok
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
test model::utf8::tests::test_find_pattern_advances_from_offset ... ok
test model::utf8::tests::test_find_pattern_matches_as_substring ... ok
test model::utf8::tests::test_find_pattern_matches_inside_larger_word ... ok
test model::utf8::tests::test_rfind_pattern_before ... ok
test model::utf8::tests::test_substr_by_char ... ok
test syntax::tests::test_syntax_selection ... ok
test syntax::tests::test_update_syntax_keyword ... ok
test view::colors::tests::test_ansi256_white ... ok
test view::colors::tests::test_true_color_sequence ... ok
test view::screen_renderer::tests::test_custom_cursor_at_end_when_line_is_full ... ok
test view::screen_renderer::tests::test_custom_cursor_at_end_with_room ... ok
test view::screen_renderer::tests::test_custom_cursor_in_middle_of_text ... ok
test view::screen_renderer::tests::test_no_cursor_leaves_text_untouched ... ok
test view::screen_renderer::tests::test_wrap_message_breaks_on_word_boundary ... ok
test view::screen_renderer::tests::test_wrap_message_fits_on_one_line ... ok
test view::screen_renderer::tests::test_wrap_message_hard_breaks_long_word ... ok
test view::screen_renderer::tests::test_wrap_message_zero_width ... ok

test result: ok. 54 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-24fe9b87231282b6`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m   Doc-tests[0m cera
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name cera --test src/lib.rs --test-run-directory /home/runner/work/cera/cera --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-265f65eb0fbef32c.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-61091f233f82b102.rlib -L dependency=/home/runner/work/cera/cera/target/debug/deps -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' --error-format human`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
