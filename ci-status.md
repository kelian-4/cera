# CI Status

Commit: f68edd2f19b117be03985c076797ac272ca06e3e
Branche: dev
Date: 2026-10-07 09:30:07 UTC
Run: https://github.com/kelian-4/cera/actions/runs/37600994694

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
[1m[92m    Checking[0m nix v0.27.1
[1m[92m    Checking[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 1.44s
```

## Log complet - Build
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m   Compiling[0m nix v0.27.1
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name nix --edition=2021 /home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nix-0.27.1/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="fs"' --cfg 'feature="ioctl"' --cfg 'feature="poll"' --cfg 'feature="process"' --cfg 'feature="signal"' --cfg 'feature="term"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("acct", "aio", "default", "dir", "env", "event", "feature", "fs", "hostname", "inotify", "ioctl", "kmod", "memoffset", "mman", "mount", "mqueue", "net", "personality", "pin-utils", "poll", "process", "pthread", "ptrace", "quota", "reboot", "resource", "sched", "signal", "socket", "term", "time", "ucontext", "uio", "user", "zerocopy"))' -C metadata=0a77cb1ac4700467 -C extra-filename=-671e6b86e316e608 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rmeta --extern cfg_if=/home/runner/work/cera/cera/target/debug/deps/libcfg_if-e202c1210b381f60.rmeta --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rmeta --cap-lints allow`
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=789b2fb61ad4a37f -C extra-filename=-2b70397db7a339b1 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rmeta --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rmeta --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rmeta`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=212ac5ff117fb7ae -C extra-filename=-d0ba6b0fc148147d --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 2.01s
```

## Log complet - Tests
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m       Fresh[0m nix v0.27.1
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=ebe8ffbcfca6fd5a -C extra-filename=-8baea2e795c850cf --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=81c1988cab904c4a -C extra-filename=-db7687ae4651fef9 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m    Finished[0m `test` profile [unoptimized + debuginfo] target(s) in 1.09s
[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-8baea2e795c850cf`

running 109 tests
test cli::tests::test_diff_requires_two_files ... ok
test cli::tests::test_diff_takes_two_or_more_files ... ok
test cli::tests::test_double_dash_ends_options ... ok
test cli::tests::test_explorer_dir_can_come_before_the_flag ... ok
test cli::tests::test_explorer_with_dir ... ok
test cli::tests::test_explorer_with_dir_and_extra_files ... ok
test cli::tests::test_explorer_without_dir_uses_current_directory ... ok
test cli::tests::test_find_requires_exactly_one_name ... ok
test cli::tests::test_find_takes_one_name ... ok
test cli::tests::test_grep_pattern_starting_with_dash_needs_double_dash ... ok
test cli::tests::test_grep_requires_a_pattern ... ok
test cli::tests::test_grep_takes_pattern_then_paths ... ok
test cli::tests::test_grep_takes_pattern_without_paths ... ok
test cli::tests::test_help_after_double_dash_is_a_plain_argument ... ok
test cli::tests::test_help_flags ... ok
test cli::tests::test_help_takes_priority_over_other_arguments ... ok
test cli::tests::test_modes_are_mutually_exclusive ... ok
test cli::tests::test_no_args_opens_editor_without_files ... ok
test cli::tests::test_plain_files_open_in_editor ... ok
test cli::tests::test_show_collects_all_files ... ok
test cli::tests::test_show_and_explorer_together_is_an_error ... ok
test cli::tests::test_show_without_file_is_an_error ... ok
test cli::tests::test_single_dash_is_a_filename_not_an_option ... ok
test cli::tests::test_unknown_option_is_an_error ... ok
test editor::buffer_manager::tests::test_multiple_buffers ... ok
test editor::buffer_manager::tests::test_open_file_reuses_existing_buffer_instead_of_duplicating ... ok
test editor::undo_stack::tests::test_coalescing ... ok
test editor::undo_stack::tests::test_coalescing_breaks_on_word_boundary ... ok
test editor::clipboard::tests::test_clipboard_fallback_roundtrip ... ok
test i18n::tests::test_all_embedded_languages_parse_successfully ... ok
test i18n::tests::test_all_languages_have_the_same_keys_as_fallback ... ok
test i18n::tests::test_detect_system_language_parses_locale_format ... ok
test i18n::tests::test_load_falls_back_to_english_for_unknown_language ... ok
test i18n::tests::test_load_explicit_language ... ok
test i18n::tests::test_missing_key_returns_key_itself ... ok
test io::config::tests::test_config_defaults_on_missing_file ... ok
test i18n::tests::test_t1_replaces_placeholder ... ok
test io::config::tests::test_config_plugins_enabled_can_be_emptied ... ok
test io::config::tests::test_config_loads_real_file ... ok
test io::config::tests::test_config_reads_plugins_section ... ok
test io::config::tests::test_default_config_path_ends_with_config_json ... ok
test io::config::tests::test_ensure_config_file_exists_creates_file_with_parent_dir ... ok
test io::config::tests::test_ensure_config_file_exists_never_overwrites_existing_file ... ok
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
test plugin::manager::bundled_plugin_tests::test_pretty_cat_renders_file_with_line_numbers ... ok
test plugin::manager::bundled_plugin_tests::test_file_explorer_lists_directory_and_opens_file ... ok
test plugin::manager::tests::test_poll_incoming_never_blocks_when_no_data ... ok
test plugin::manager::tests::test_spawn_nonexistent_command_fails ... ok
test plugin::protocol::tests::test_json_escape_handles_quotes_and_backslashes ... ok
test plugin::protocol::tests::test_outgoing_focus_message_shape ... ok
test plugin::protocol::tests::test_outgoing_init_message_shape_fullscreen ... ok
test plugin::protocol::tests::test_outgoing_init_message_shape_panel ... ok
test plugin::protocol::tests::test_outgoing_key_message_shape ... ok
test plugin::protocol::tests::test_parse_close_message ... ok
test plugin::protocol::tests::test_parse_hex_color_with_and_without_hash ... ok
test plugin::protocol::tests::test_parse_invalid_json_returns_none ... ok
test plugin::protocol::tests::test_parse_open_file_message ... ok
test plugin::protocol::tests::test_parse_render_message ... ok
test plugin::protocol::tests::test_parse_render_message_with_panel_width ... ok
test plugin::protocol::tests::test_parse_status_message ... ok
test plugin::protocol::tests::test_parse_unknown_type_is_unknown_not_error ... ok
test plugin::registry::tests::test_command_with_slash_resolved_relative_to_plugin_dir ... ok
test plugin::registry::tests::test_doc_text_prefers_help_then_falls_back_to_description ... ok
test plugin::registry::tests::test_load_enabled_plugins_skips_missing_ones ... ok
test plugin::registry::tests::test_load_manifest_basic_fields ... ok
test plugin::registry::tests::test_load_manifest_missing_returns_none ... ok
test plugin::registry::tests::test_load_manifest_without_command_returns_none ... ok
test syntax::tests::test_syntax_selection ... ok
test syntax::tests::test_update_syntax_keyword ... ok
test view::colors::tests::test_ansi256_white ... ok
test view::colors::tests::test_true_color_sequence ... ok
test view::help_text::tests::test_cli_help_lists_options_and_plugin_docs ... ok
test view::help_text::tests::test_cli_help_without_plugins_has_no_plugin_section ... ok
test view::help_text::tests::test_full_documentation_has_no_plugin_category_without_plugins ... ok
test view::help_text::tests::test_full_documentation_lists_plugin_docs ... ok
test view::screen_renderer::tests::test_custom_cursor_at_end_when_line_is_full ... ok
test view::screen_renderer::tests::test_custom_cursor_at_end_with_room ... ok
test view::screen_renderer::tests::test_custom_cursor_in_middle_of_text ... ok
test view::screen_renderer::tests::test_no_cursor_leaves_text_untouched ... ok
test view::screen_renderer::tests::test_wrap_message_breaks_on_word_boundary ... ok
test view::screen_renderer::tests::test_wrap_message_fits_on_one_line ... ok
test view::screen_renderer::tests::test_wrap_message_hard_breaks_long_word ... ok
test view::screen_renderer::tests::test_wrap_message_zero_width ... ok
test plugin::manager::tests::test_spawn_and_roundtrip_with_cat ... ok

test result: ok. 109 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-db7687ae4651fef9`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m   Doc-tests[0m cera
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name cera --test src/lib.rs --test-run-directory /home/runner/work/cera/cera --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib -L dependency=/home/runner/work/cera/cera/target/debug/deps -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' --error-format human`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
