# TODO List

## High Priority
- [ ]  Clean up demos
- [ ]  Add additional popular crates

## Medium Priority
- [ ]  Document public APIs
- [ ]  More unit and integration tests
- [ ]  Ensure hyphens vs underscores are used correctly in rs-script command
- [ ]  Document where demo subdirectory is and how to install it
- [ ]  Simple demo https server
- [ ]  Get builder ignored tests working
- [ ]  Debug egui_code_editor.rss only showing env_logger if use egui::*; is included.
- [ ]  Test of test_build_state_pre_configure failing, also repl does not accept -- --nocapture.
- [ ]  Test all occurrences of termbg:: calls with clear_screen() or equiv and crossterm imports.
- [ ]  Investigate replacing shared::CargoManifest with cargo_toml crate.

## Low Priority
- [ ]  Consider history support for stdin.
- [ ]  Paste event in Windows slow or not happening?
- [ ]  How to insert line feed from keyboard to split line in reedline. (Supposedly shift+enter)
- [ ]  Decide if it's worth passing the wrapped syntax tree to gen_build_run from eval just to avoid re-parsing it for that specific use case.
- [ ]  "edit" crate - how to reconfigure editors dynamically - instructions unclear.
- [ ]  Clap aliases not working in REPL.
- [ ]  Work on demo/reedline_clap_repl_gemini.rs
- [ ]  Consider other Rust gui packages.
- [ ]  How to navigate reedline history entry by entry instead of line by line.


## Ideas / Future Enhancements
- [ ]  Consider supporting alternative TOML embedding keywords so we can run demo/regex_capture_toml.rs.
- [ ]  Option to cat files before delete.
- [ ]  WASM - is there a worthwhile one? - maybe Leptos if it doesn't need Node.js.
