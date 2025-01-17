use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::tty::IsTty;
use mockall::Sequence;
use ratatui::style::{Color, Style, Stylize};
use rs_script::logging::Verbosity;
use rs_script::stdin::{apply_highlights, normalize_newlines, read_to_string, MockEventReader};
use rs_script::{edit_stdin, log, BuildRunError};
use std::io::{stdout, Write};
use std::process::{Command, Stdio};
use tui_textarea::TextArea;

#[test]
fn test_edit_stdin_submit() {
    // Check if the test is running in a terminal
    if !stdout().is_tty() {
        println!("Skipping test_edit_stdin_submit as it is not running in a terminal.");
        return;
    }

    let mut seq = Sequence::new();
    let mut mock_reader = MockEventReader::new();

    mock_reader
        .expect_read_event()
        .times(1)
        .in_sequence(&mut seq)
        .return_once(|| Ok(Event::Paste("Hello,\nworld".to_string())));

    mock_reader
        .expect_read_event()
        .times(1)
        .in_sequence(&mut seq)
        .return_once(|| {
            Ok(Event::Key(KeyEvent::new(
                KeyCode::Char('!'),
                KeyModifiers::NONE,
            )))
        });

    mock_reader
        .expect_read_event()
        .times(1)
        .in_sequence(&mut seq)
        .return_once(|| {
            Ok(Event::Key(KeyEvent::new(
                KeyCode::Char('d'),
                KeyModifiers::CONTROL,
            )))
        });

    let result = edit_stdin(mock_reader);

    log!(
        Verbosity::Normal,
        "\ntest_edit_stdin_submit result={result:#?}"
    );
    assert!(result.is_ok());
    let lines = result.unwrap();
    // Expecting a Vec with one entry: an empty string
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "Hello,");
    assert_eq!(lines[1], "world!");
}

#[test]
fn test_edit_stdin_quit() {
    // Check if the test is running in a terminal
    if !stdout().is_tty() {
        println!("Skipping test_edit_stdin_submit as it is not running in a terminal.");
        return;
    }
    let mut mock_reader = MockEventReader::new();

    mock_reader.expect_read_event().return_once(|| {
        Ok(Event::Key(KeyEvent::new(
            KeyCode::Char('q'),
            KeyModifiers::CONTROL,
        )))
    });

    let result = edit_stdin(mock_reader);

    assert!(result.is_err());
    assert!(matches!(
        result.err().unwrap().downcast_ref::<BuildRunError>(),
        Some(&BuildRunError::Cancelled)
    ));
}

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_read_to_string() {
    let string = r#"fn main() {{ println!("Hello, world!"); }}\n"#;
    let mut input = string.as_bytes();
    let result = read_to_string(&mut input).unwrap();
    assert_eq!(result, string);
}

#[test]
fn test_read_stdin() {
    init_logger();
    let string = "Hello, world!";
    let input = format!(
        r#"fn main() {{ println!("{string}"); }}
"#
    );
    println!("input={input}");

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--features=debug-logs")
        .arg("--")
        .arg("-q")
        .arg("-s")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("{}\n", string)
    );
}

#[test]
fn test_normalize_newlines() {
    let input = "Hello\r\nWorld\r!";
    let expected_output = "Hello\nWorld\n!";
    assert_eq!(normalize_newlines(input), expected_output);
}

#[test]
fn test_apply_highlights() {
    let mut textarea = TextArea::default();

    apply_highlights(true, &mut textarea);
    assert_eq!(
        textarea.selection_style(),
        Style::default().bg(Color::LightRed)
    );
    assert_eq!(textarea.cursor_style(), Style::default().on_yellow());
    assert_eq!(
        textarea.cursor_line_style(),
        Style::default().on_light_yellow()
    );

    apply_highlights(false, &mut textarea);
    assert_eq!(
        textarea.selection_style(),
        Style::default().bg(Color::Green)
    );
    assert_eq!(textarea.cursor_style(), Style::default().on_magenta());
    assert_eq!(
        textarea.cursor_line_style(),
        Style::default().on_dark_gray()
    );
}
