# CI Status

Commit: 13898aa1aefb47cb5163caf550a3e69b0553da77
Branche: main
Date: 2026-10-13 08:42:23 UTC
Run: https://github.com/kelian-4/cera/actions/runs/38370993248

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
[1m[92m    Checking[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
```

## Log complet - Build
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m       Fresh[0m nix v0.27.1
[1m[92m       Dirty[0m cera v0.1.0 (/home/runner/work/cera/cera): the file `src/syntax/cpp_syntax.rs` has changed (1791880895.721609734s, 165h 24m 17s after last build at 1791285438.416052262s)
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=789b2fb61ad4a37f -C extra-filename=-2b70397db7a339b1 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rmeta --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rmeta --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rmeta`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=212ac5ff117fb7ae -C extra-filename=-d0ba6b0fc148147d --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 1.23s
```

## Log complet - Tests
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m nix v0.27.1
[1m[92m       Dirty[0m cera v0.1.0 (/home/runner/work/cera/cera): the file `src/syntax/php_syntax.rs` has changed (1791880895.723003387s, 165h 24m 16s after last build at 1791285439.306107336s)
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=ebe8ffbcfca6fd5a -C extra-filename=-8baea2e795c850cf --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=81c1988cab904c4a -C extra-filename=-db7687ae4651fef9 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name paste --edition=2021 tests/paste.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=ee24e6cac58fb3f1 -C extra-filename=-dc502ba479745c0a --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name plugin_languages --edition=2021 tests/plugin_languages.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=7249014d0a5fe652 -C extra-filename=-b4dae101d3ea0655 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name escape_key --edition=2021 tests/escape_key.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=091ce9a0c3660142 -C extra-filename=-3e07fb81419e900d --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name pretty_cat_languages --edition=2021 tests/pretty_cat_languages.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=02fad8208b3ce53e -C extra-filename=-01f818e69756a6f1 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m    Finished[0m `test` profile [unoptimized + debuginfo] target(s) in 1.69s
[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-8baea2e795c850cf`

running 147 tests
test cli::tests::test_diff_requires_two_files ... ok
test cli::tests::test_double_dash_ends_options ... ok
test cli::tests::test_diff_takes_two_or_more_files ... ok
test cli::tests::test_explorer_with_dir_and_extra_files ... ok
test cli::tests::test_explorer_dir_can_come_before_the_flag ... ok
test cli::tests::test_explorer_with_dir ... ok
test cli::tests::test_explorer_without_dir_uses_current_directory ... ok
test cli::tests::test_find_requires_exactly_one_name ... ok
test cli::tests::test_find_takes_one_name ... ok
test cli::tests::test_grep_requires_a_pattern ... ok
test cli::tests::test_grep_takes_pattern_then_paths ... ok
test cli::tests::test_help_after_double_dash_is_a_plain_argument ... ok
test cli::tests::test_grep_pattern_starting_with_dash_needs_double_dash ... ok
test cli::tests::test_grep_takes_pattern_without_paths ... ok
test cli::tests::test_help_flags ... ok
test cli::tests::test_help_takes_priority_over_other_arguments ... ok
test cli::tests::test_plain_files_open_in_editor ... ok
test cli::tests::test_show_and_explorer_together_is_an_error ... ok
test cli::tests::test_modes_are_mutually_exclusive ... ok
test cli::tests::test_no_args_opens_editor_without_files ... ok
test cli::tests::test_show_collects_all_files ... ok
test cli::tests::test_unknown_option_is_an_error ... ok
test cli::tests::test_show_without_file_is_an_error ... ok
test cli::tests::test_single_dash_is_a_filename_not_an_option ... ok
test editor::buffer_manager::tests::test_multiple_buffers ... ok
test editor::core::paste_text_tests::test_normalize_drops_control_characters_that_could_trigger_commands ... ok
test editor::core::paste_text_tests::test_normalize_unifies_every_kind_of_line_ending ... ok
test editor::core::paste_text_tests::test_single_line_paste_drops_control_characters ... ok
test editor::buffer_manager::tests::test_open_file_reuses_existing_buffer_instead_of_duplicating ... ok
test editor::core::paste_text_tests::test_single_line_paste_stops_at_the_first_line_break ... ok
test editor::core::paste_text_tests::test_normalize_keeps_tabs_and_unicode ... ok
test editor::undo_stack::tests::test_coalescing ... ok
test editor::undo_stack::tests::test_coalescing_breaks_on_word_boundary ... ok
test i18n::tests::test_all_embedded_languages_parse_successfully ... ok
test i18n::tests::test_detect_system_language_parses_locale_format ... ok
test i18n::tests::test_all_languages_have_the_same_keys_as_fallback ... ok
test i18n::tests::test_load_explicit_language ... ok
test i18n::tests::test_load_falls_back_to_english_for_unknown_language ... ok
test editor::clipboard::tests::test_clipboard_fallback_roundtrip ... ok
test io::config::tests::test_config_defaults_on_missing_file ... ok
test io::config::tests::test_config_loads_real_file ... ok
test io::config::tests::test_config_loads_syntax_class_colors ... ok
test io::config::tests::test_config_plugins_enabled_can_be_emptied ... ok
test io::config::tests::test_default_config_json_lists_every_syntax_color ... ok
test io::config::tests::test_default_config_path_ends_with_config_json ... ok
test io::config::tests::test_config_reads_plugins_section ... ok
test i18n::tests::test_missing_key_returns_key_itself ... ok
test io::config::tests::test_ensure_config_file_exists_creates_file_with_parent_dir ... ok
test i18n::tests::test_t1_replaces_placeholder ... ok
test io::config::tests::test_ensure_config_file_exists_never_overwrites_existing_file ... ok
test io::file_io::tests::test_open_existing_file ... ok
test io::json::tests::test_json_invalid ... ok
test io::file_io::tests::test_open_nonexistent_file_creates_empty_buffer ... ok
test io::file_io::tests::test_save_writes_to_disk ... ok
test model::buffer::tests::test_dirty_flag ... ok
test io::json::tests::test_json_nested_object ... ok
test model::buffer::tests::test_delete_char_merges_rows ... ok
test io::json::tests::test_json_parsing ... ok
test model::buffer::tests::test_insert_newline_splits_row ... ok
test model::buffer::tests::test_new_buffer_has_one_empty_line ... ok
test model::buffer::tests::test_set_lines_empty_still_has_one_line ... ok
test model::buffer::tests::test_to_string_content ... ok
test model::row::tests::test_cx_rx_roundtrip ... ok
test model::row::tests::test_insert_delete ... ok
test model::row::tests::test_tab_expansion ... ok
test model::row::tests::test_utf8_insert_delete ... ok
test model::selection::tests::test_selection_range ... ok
test model::utf8::tests::test_char_index_for_byte_offset ... ok
test model::utf8::tests::test_find_pattern_advances_from_offset ... ok
test model::utf8::tests::test_byte_offset_for_char_index ... ok
test model::utf8::tests::test_codepoint_count ... ok
test model::utf8::tests::test_display_width_cjk ... ok
test model::utf8::tests::test_find_pattern_matches_as_substring ... ok
test model::utf8::tests::test_substr_by_char ... ok
test model::utf8::tests::test_find_pattern_matches_inside_larger_word ... ok
test model::utf8::tests::test_rfind_pattern_before ... ok
test plugin::manager::bundled_plugin_tests::test_pretty_cat_renders_file_with_line_numbers ... ok
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
test syntax::language_samples::every_language_sample_is_highlighted_as_expected ... ok
test plugin::manager::bundled_plugin_tests::test_file_explorer_lists_directory_and_opens_file ... ok
test syntax::parity_tests::pretty_cat_colors_every_language_sample_like_the_editor ... ok
test syntax::parity_tests::pretty_cat_language_data_matches_the_editor ... ok
test syntax::tests::test_case_insensitive_keywords ... ok
test syntax::tests::test_case_sensitive_languages_do_not_match_other_cases ... ok
test syntax::tests::test_comment_marker_inside_string_is_not_a_comment ... ok
test syntax::tests::test_default_theme_gives_every_class_its_own_color ... ok
test syntax::tests::test_digits_inside_identifiers_are_not_numbers ... ok
test syntax::tests::test_embedded_language_state_survives_update_syntax_cascade ... ok
test syntax::tests::test_embedded_regions_do_not_nest_beyond_one_level ... ok
test syntax::tests::test_every_alias_resolves_to_a_real_language ... ok
test syntax::tests::test_every_language_is_well_formed ... ok
test syntax::tests::test_html_script_and_style_are_colored_with_their_own_language ... ok
test syntax::tests::test_json_keys_and_values_use_different_colors ... ok
test syntax::tests::test_keywords_inside_identifiers_are_not_highlighted ... ok
test syntax::tests::test_markdown_fence_with_unknown_language_stays_a_string ... ok
test syntax::tests::test_numbers_hex_binary_float_exponent_and_suffix ... ok
test syntax::tests::test_requested_languages_are_present ... ok
test syntax::tests::test_rust_lifetime_is_not_a_string_but_char_literal_is ... ok
test syntax::tests::test_shell_dollar_hash_is_not_a_comment ... ok
test syntax::tests::test_string_escapes_are_highlighted ... ok
test syntax::tests::test_string_that_is_not_html_or_sql_stays_a_plain_string ... ok
test syntax::tests::test_syntax_selection ... ok
test syntax::tests::test_transpose_tick_is_not_a_string_in_matlab ... ok
test syntax::tests::test_unterminated_string_does_not_leak_to_next_row ... ok
test plugin::manager::tests::test_poll_incoming_never_blocks_when_no_data ... ok
test syntax::tests::test_update_syntax_keyword ... ok
test syntax::tests::test_update_syntax_propagates_multiline_comment_state ... ok
test syntax::tests::test_update_syntax_propagates_multiline_string_state ... ok
test view::colors::tests::test_ansi256_white ... ok
test view::colors::tests::test_true_color_sequence ... ok
test view::help_text::tests::test_clamp_doc_scroll_never_passes_the_last_page ... ok
test view::help_text::tests::test_cli_help_lists_options_and_plugin_docs ... ok
test view::help_text::tests::test_cli_help_without_plugins_has_no_plugin_section ... ok
test view::help_text::tests::test_doc_visible_rows_leaves_title_and_last_row ... ok
test view::help_text::tests::test_flatten_documentation_counts_header_entries_and_blank ... ok
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
test syntax::tests::test_update_syntax_cascade_is_iterative_and_does_not_overflow_the_stack ... ok
test syntax::parity_tests::pretty_cat_colors_the_projects_own_files_like_the_editor ... ok

test result: ok. 147 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.42s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-db7687ae4651fef9`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/escape_key-3e07fb81419e900d`

running 1 test
test escape_is_handled_immediately_and_does_not_swallow_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.52s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/paste-dc502ba479745c0a`

running 1 test
test multiline_paste_in_the_middle_of_a_file_is_inserted_as_one_block ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 23.11s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/plugin_languages-b4dae101d3ea0655`

running 5 tests
test plugin_written_in_python_speaks_the_protocol ... ok
test plugin_written_in_javascript_speaks_the_protocol ... ok
test plugin_written_in_c_speaks_the_protocol ... ok
test plugin_written_in_rust_speaks_the_protocol ... ok
test plugin_written_in_cpp_speaks_the_protocol ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.85s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/pretty_cat_languages-01f818e69756a6f1`

running 4 tests
test cat_mode_reports_a_missing_file_and_continues ... ok
test cat_mode_keeps_unknown_file_types_uncolored ... ok
test interactive_mode_sends_colored_segments_and_closes_on_q ... ok
test cat_mode_colors_each_language_with_the_editor_palette ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

[1m[92m   Doc-tests[0m cera
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name cera --test src/lib.rs --test-run-directory /home/runner/work/cera/cera --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib -L dependency=/home/runner/work/cera/cera/target/debug/deps -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' --error-format human`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
