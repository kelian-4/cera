# CI Status

Commit: ecbb495fb5f5ceccf28776aad1791b766aaff86d
Branche: dev
Date: 2026-10-29 08:21:58 UTC
Run: https://github.com/kelian-4/cera/actions/runs/40841479143

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
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 1.25s
```

## Log complet - Build
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m nix v0.27.1
[1m[92m       Dirty[0m cera v0.1.0 (/home/runner/work/cera/cera): the file `src/syntax/css_syntax.rs` has changed (1793262058.676358521s, 101h 6m 25s after last build at 1792898073.326929534s)
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=789b2fb61ad4a37f -C extra-filename=-2b70397db7a339b1 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rmeta --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rmeta --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rmeta`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=212ac5ff117fb7ae -C extra-filename=-d0ba6b0fc148147d --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 1.54s
```

## Log complet - Tests
```
[1m[92m       Fresh[0m libc v0.2.189
[1m[92m       Fresh[0m bitflags v2.13.2
[1m[92m       Fresh[0m cfg-if v1.0.4
[1m[92m       Fresh[0m nix v0.27.1
[1m[92m       Dirty[0m cera v0.1.0 (/home/runner/work/cera/cera): the file `src/io/file_io.rs` has changed (1793262058.675417895s, 101h 6m 24s after last build at 1792898074.548544821s)
[1m[92m   Compiling[0m cera v0.1.0 (/home/runner/work/cera/cera)
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=ebe8ffbcfca6fd5a -C extra-filename=-8baea2e795c850cf --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name escape_key --edition=2021 tests/escape_key.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=091ce9a0c3660142 -C extra-filename=-3e07fb81419e900d --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name pretty_cat_languages --edition=2021 tests/pretty_cat_languages.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=02fad8208b3ce53e -C extra-filename=-01f818e69756a6f1 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name bundled_plugins --edition=2021 tests/bundled_plugins.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=cfd67f1e3e2ae4bd -C extra-filename=-f5741d0d8772507a --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name plugin_languages --edition=2021 tests/plugin_languages.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=7249014d0a5fe652 -C extra-filename=-b4dae101d3ea0655 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name version --edition=2021 tests/version.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=1d4f3664bd20b2b8 -C extra-filename=-16fefda16938d94d --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name paste --edition=2021 tests/paste.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=ee24e6cac58fb3f1 -C extra-filename=-dc502ba479745c0a --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name cera --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=81c1988cab904c4a -C extra-filename=-db7687ae4651fef9 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name arrow_down --edition=2021 tests/arrow_down.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=9546585211e9a5bf -C extra-filename=-17121a83f103d1c3 --out-dir /home/runner/work/cera/cera/target/debug/deps -L dependency=/home/runner/work/cera/cera/target/debug/deps --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib`
[1m[92m    Finished[0m `test` profile [unoptimized + debuginfo] target(s) in 2.43s
[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-8baea2e795c850cf`

running 236 tests
test cli::tests::test_diff_takes_two_or_more_files ... ok
test cli::tests::test_diff_requires_two_files ... ok
test cli::tests::test_double_dash_ends_options ... ok
test cli::tests::test_explorer_dir_can_come_before_the_flag ... ok
test cli::tests::test_explorer_with_dir ... ok
test cli::tests::test_explorer_with_dir_and_extra_files ... ok
test cli::tests::test_explorer_without_dir_uses_current_directory ... ok
test cli::tests::test_find_takes_one_name ... ok
test cli::tests::test_find_requires_exactly_one_name ... ok
test cli::tests::test_grep_takes_pattern_then_paths ... ok
test cli::tests::test_grep_pattern_starting_with_dash_needs_double_dash ... ok
test cli::tests::test_grep_requires_a_pattern ... ok
test cli::tests::test_help_after_double_dash_is_a_plain_argument ... ok
test cli::tests::test_grep_takes_pattern_without_paths ... ok
test cli::tests::test_help_flags ... ok
test cli::tests::test_help_takes_priority_over_other_arguments ... ok
test cli::tests::test_modes_are_mutually_exclusive ... ok
test cli::tests::test_help_takes_priority_over_version ... ok
test cli::tests::test_no_args_opens_editor_without_files ... ok
test cli::tests::test_plain_files_open_in_editor ... ok
test cli::tests::test_show_collects_all_files ... ok
test cli::tests::test_show_without_file_is_an_error ... ok
test cli::tests::test_unknown_option_is_an_error ... ok
test cli::tests::test_single_dash_is_a_filename_not_an_option ... ok
test cli::tests::test_version_after_double_dash_is_a_plain_argument ... ok
test cli::tests::test_version_flag ... ok
test cli::tests::test_version_takes_priority_over_other_arguments ... ok
test editor::buffer_manager::tests::test_multiple_buffers ... ok
test editor::buffer_manager::tests::test_open_file_reuses_existing_buffer_instead_of_duplicating ... ok
test editor::buffer_manager::tests::test_open_unreadable_file_leaves_no_buffer_behind ... ok
test editor::core::paste_text_tests::test_normalize_keeps_tabs_and_unicode ... ok
test cli::tests::test_show_and_explorer_together_is_an_error ... ok
test editor::core::paste_text_tests::test_normalize_unifies_every_kind_of_line_ending ... ok
test editor::core::paste_text_tests::test_single_line_paste_drops_control_characters ... ok
test editor::undo_stack::tests::test_coalescing ... ok
test editor::core::paste_text_tests::test_single_line_paste_stops_at_the_first_line_break ... ok
test editor::undo_stack::tests::test_coalescing_breaks_on_word_boundary ... ok
test format::tests::blank_lines_are_collapsed_and_edges_removed ... ok
test format::tests::braceless_if_indents_one_statement ... ok
test editor::core::paste_text_tests::test_normalize_drops_control_characters_that_could_trigger_commands ... ok
test format::tests::block_comments_follow_their_opener ... ok
test format::tests::brackets_in_strings_and_comments_are_ignored ... ok
test format::tests::continuation_after_operator ... ok
test format::tests::c_aligned_arguments_keep_their_alignment ... ok
test format::tests::closing_line_with_several_brackets_returns_to_opener_level ... ok
test format::tests::css_is_reindented ... ok
test format::tests::designated_initializers_are_not_chains ... ok
test format::tests::go_uses_tabs ... ok
test format::tests::else_stays_on_the_level_of_its_if ... ok
test format::tests::html_comments_move_with_their_opener ... ok
test format::tests::html_multiline_tag_attributes ... ok
test format::tests::html_nesting_and_void_elements ... ok
test editor::clipboard::tests::test_clipboard_fallback_roundtrip ... ok
test format::tests::html_with_unclosed_optional_tags_is_left_alone ... ok
test format::tests::increment_is_not_a_continuation ... ok
test format::tests::include_directive_is_not_an_open_string ... ok
test format::tests::html_pre_script_and_style_are_untouched ... ok
test format::tests::json_is_reindented ... ok
test format::tests::jsx_is_left_alone ... ok
test format::tests::method_chain_lines_are_indented ... ok
test format::tests::multiline_strings_are_untouched ... ok
test format::tests::markdown_and_diff_are_not_formatted ... ok
test format::tests::lua_functions_and_loops ... ok
test format::tests::preprocessor_stays_in_column_zero ... ok
test format::tests::python_comment_follows_next_statement ... ok
test format::tests::output_is_stable_when_formatted_twice ... ok
test format::tests::python_continuation_lines_keep_their_alignment ... ok
test format::tests::python_keeps_two_blank_lines_and_docstrings ... ok
test format::tests::python_normalizes_to_four_spaces ... ok
test format::tests::ruby_blocks_and_middles ... ok
test format::tests::ruby_heredoc_body_is_untouched ... ok
test format::tests::ruby_modifier_if_does_not_open_a_block ... ok
test format::tests::shell_words_that_look_like_keywords_are_ignored ... ok
test format::tests::rust_reindents_blocks ... ok
test format::tests::shell_blocks_and_case ... ok
test format::tests::unbalanced_file_only_gets_hygiene ... ok
test format::tests::switch_labels_and_bodies ... ok
test format::tests::xml_self_closing_and_declaration ... ok
test format::tests::yaml_normalizes_to_two_spaces ... ok
test format::tests::yaml_block_scalars_move_with_their_key ... ok
test i18n::tests::test_detect_system_language_parses_locale_format ... ok
test i18n::tests::test_load_explicit_language ... ok
test i18n::tests::test_all_embedded_languages_parse_successfully ... ok
test i18n::tests::test_all_languages_have_the_same_keys_as_fallback ... ok
test io::config::tests::test_config_defaults_on_missing_file ... ok
test i18n::tests::test_load_falls_back_to_english_for_unknown_language ... ok
test io::config::tests::test_config_loads_real_file ... ok
test i18n::tests::test_missing_key_returns_key_itself ... ok
test io::config::tests::test_config_loads_syntax_class_colors ... ok
test io::config::tests::test_config_plugins_enabled_can_be_emptied ... ok
test io::config::tests::test_config_reads_plugins_section ... ok
test io::config::tests::test_default_config_json_lists_every_syntax_color ... ok
test io::config::tests::test_default_config_path_ends_with_config_json ... ok
test io::config::tests::test_ensure_config_does_not_install_plugins_when_config_exists ... ok
test io::config::tests::test_ensure_config_file_exists_never_overwrites_existing_file ... ok
test i18n::tests::test_t1_replaces_placeholder ... ok
test io::config::tests::test_ensure_config_file_exists_creates_file_with_parent_dir ... ok
test io::config::tests::test_ensure_config_without_bundled_plugins_creates_no_plugins_dir ... ok
test io::config::tests::test_format_on_save_and_indent_guides_options ... ok
test io::config::tests::test_ensure_config_installs_bundled_plugins_next_to_new_config ... ok
test io::file_io::tests::test_open_directory_is_refused ... ok
test io::file_io::tests::test_open_existing_file ... ok
test io::file_io::tests::test_open_non_utf8_file_is_refused_and_not_overwritten ... ok
test io::file_io::tests::test_open_nonexistent_file_creates_empty_buffer ... ok
test io::json::tests::test_json_invalid ... ok
test io::file_io::tests::test_save_writes_to_disk ... ok
test io::json::tests::test_json_nested_object ... ok
test io::json::tests::test_json_parsing ... ok
test model::buffer::tests::test_dirty_flag ... ok
test model::buffer::tests::test_delete_char_merges_rows ... ok
test model::buffer::tests::test_new_buffer_has_one_empty_line ... ok
test model::buffer::tests::test_insert_newline_splits_row ... ok
test model::buffer::tests::test_set_lines_empty_still_has_one_line ... ok
test model::buffer::tests::test_to_string_content ... ok
test model::row::tests::test_insert_delete ... ok
test io::config::tests::test_ensure_config_updates_installed_plugins_when_the_version_changes ... ok
test model::row::tests::test_cx_rx_roundtrip ... ok
test model::row::tests::test_utf8_insert_delete ... ok
test model::row::tests::test_tab_expansion ... ok
test model::selection::tests::test_selection_range ... ok
test model::utf8::tests::test_byte_offset_for_char_index ... ok
test model::utf8::tests::test_char_index_for_byte_offset ... ok
test model::utf8::tests::test_codepoint_count ... ok
test model::utf8::tests::test_find_pattern_advances_from_offset ... ok
test model::utf8::tests::test_display_width_cjk ... ok
test model::utf8::tests::test_find_pattern_matches_as_substring ... ok
test model::utf8::tests::test_find_pattern_matches_inside_larger_word ... ok
test model::utf8::tests::test_rfind_pattern_before ... ok
test model::utf8::tests::test_substr_by_char ... ok
test plugin::install::tests::test_bundled_directory_found_next_to_binary ... ok
test plugin::install::tests::test_bundled_directory_found_in_share_layout ... ok
test plugin::install::tests::test_bundled_directory_absent ... ok
test plugin::install::tests::test_install_makes_copied_files_writable ... ok
test plugin::install::tests::test_install_keeps_custom_plugins ... ok
test plugin::install::tests::test_install_copies_plugins_recursively ... ok
test plugin::install::tests::test_install_with_missing_source_does_nothing ... ok
test plugin::install::tests::test_install_writes_the_version_marker ... ok
test plugin::install::tests::test_install_replaces_an_existing_bundled_plugin ... ok
test plugin::install::tests::test_update_does_nothing_when_the_version_is_the_same ... ok
test plugin::install::tests::test_install_skips_entries_without_manifest ... ok
test plugin::install::tests::test_update_never_creates_a_missing_plugins_directory ... ok
test plugin::install::tests::test_update_handles_plugins_installed_without_a_marker ... ok
test plugin::install::tests::test_update_replaces_plugins_when_the_version_changes ... ok
test plugin::manager::bundled_plugin_tests::test_pretty_cat_renders_file_with_line_numbers ... ok
test plugin::manager::tests::test_spawn_nonexistent_command_fails ... ok
test plugin::protocol::tests::test_incoming_text_cannot_carry_terminal_escape_sequences ... ok
test plugin::protocol::tests::test_json_escape_handles_quotes_and_backslashes ... ok
test plugin::protocol::tests::test_open_file_path_is_not_altered ... ok
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
test view::indent_guides::tests::active_guide_covers_only_its_column_and_rows ... ok
test view::indent_guides::tests::active_guide_in_body_is_the_innermost_block ... ok
test view::indent_guides::tests::active_guide_on_header_is_the_block_it_opens ... ok
test view::indent_guides::tests::active_guide_spans_blank_rows ... ok
test view::indent_guides::tests::blank_row_after_header_keeps_the_guide ... ok
test view::indent_guides::tests::blank_row_at_block_end_drops_to_next_indent ... ok
test view::indent_guides::tests::blank_rows_at_edges_have_no_guides ... ok
test view::indent_guides::tests::comment_continuation_rows_inherit_opener_indent ... ok
test view::indent_guides::tests::comment_text_does_not_skew_detection ... ok
test view::indent_guides::tests::detects_four_spaces ... ok
test view::indent_guides::tests::detects_tabs_as_tab_stop ... ok
test view::indent_guides::tests::detects_two_spaces ... ok
test view::indent_guides::tests::falls_back_to_language_default ... ok
test view::indent_guides::tests::guide_columns_cover_complete_levels_only ... ok
test view::indent_guides::tests::no_active_guide_at_top_level ... ok
test view::sanitize::tests::test_del_and_c1_controls_are_neutralized ... ok
test view::sanitize::tests::test_escape_sequences_are_neutralized ... ok
test view::sanitize::tests::test_newline_and_carriage_return_are_neutralized ... ok
test view::sanitize::tests::test_plain_text_is_unchanged ... ok
test view::sanitize::tests::test_tab_is_kept ... ok
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

test result: ok. 236 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.66s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/cera-db7687ae4651fef9`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/arrow_down-17121a83f103d1c3`

running 1 test
test arrow_down_on_last_line_appends_a_line ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.25s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/bundled_plugins-f5741d0d8772507a`

running 5 tests
test existing_config_keeps_its_plugins_untouched ... ok
test binary_without_bundled_plugins_creates_no_plugins_directory ... ok
test first_run_installs_bundled_plugins_and_enables_them ... ok
test new_version_updates_installed_plugins ... ok
test same_version_keeps_local_changes ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/escape_key-3e07fb81419e900d`

running 1 test
test escape_is_handled_immediately_and_does_not_swallow_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.51s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/paste-dc502ba479745c0a`

running 1 test
test multiline_paste_in_the_middle_of_a_file_is_inserted_as_one_block ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 23.31s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/plugin_languages-b4dae101d3ea0655`

running 5 tests
test plugin_written_in_python_speaks_the_protocol ... ok
test plugin_written_in_c_speaks_the_protocol ... ok
test plugin_written_in_rust_speaks_the_protocol ... ok
test plugin_written_in_cpp_speaks_the_protocol ... ok
test plugin_written_in_javascript_speaks_the_protocol ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.03s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/pretty_cat_languages-01f818e69756a6f1`

running 4 tests
test cat_mode_keeps_unknown_file_types_uncolored ... ok
test cat_mode_reports_a_missing_file_and_continues ... ok
test interactive_mode_sends_colored_segments_and_closes_on_q ... ok
test cat_mode_colors_each_language_with_the_editor_palette ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s

[1m[92m     Running[0m `/home/runner/work/cera/cera/target/debug/deps/version-16fefda16938d94d`

running 4 tests
test short_version_prints_name_and_version ... ok
test version_prints_name_and_version ... ok
test version_does_not_create_the_config ... ok
test version_wins_over_other_options ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m   Doc-tests[0m cera
[1m[92m     Running[0m `/home/runner/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustdoc --edition=2021 --crate-type lib --color always --crate-name cera --test src/lib.rs --test-run-directory /home/runner/work/cera/cera --extern bitflags=/home/runner/work/cera/cera/target/debug/deps/libbitflags-86956476ac6de8d5.rlib --extern cera=/home/runner/work/cera/cera/target/debug/deps/libcera-2b70397db7a339b1.rlib --extern libc=/home/runner/work/cera/cera/target/debug/deps/liblibc-225b59a4e9bd79b1.rlib --extern nix=/home/runner/work/cera/cera/target/debug/deps/libnix-671e6b86e316e608.rlib -L dependency=/home/runner/work/cera/cera/target/debug/deps -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' --error-format human`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
