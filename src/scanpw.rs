// This module was taken and adapted from the `scanpw` crate:
//
// https://forge.typ3.tech/charles/scanpw
//

/*
Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::io::{stdout, Write};

use crossterm::{
  cursor::MoveToNextLine,
  event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
  execute,
  style::Print,
  terminal,
};

/// Reads a password from standard input
///
/// Invocations of [`scanpw`] expand to an expression retuning a [`String`] that
/// contains a line of input from `stdin`. It can be invoked with arguments
/// identical to those of [`print`], and if so, those arguments will be used
/// to generate a prompt on the standard output. Input will begin on the same
/// line that the prompt ends, if any. If no arguments are provided, input will
/// start where the cursor is, which is likely to be on its own empty line.
///
/// # Panics
///
/// This macro will panic if there are IO errors on the standard input or
/// output.
#[macro_export] macro_rules! scanpw {
  // Manually set echo mode, with prompt
  ( $prompt:expr ) => {{
    print!("{}", $prompt);
    use ::std::io::Write;
    ::std::io::stdout().flush().unwrap();

    scanpw::try_scanpw().unwrap()
  }};
}

/// Attempts to read a password from standard input
///
/// The result is either a [`String`] or a [`crossterm::ErrorKind`]. Input
/// begins wherever the cursor was before calling this function, which is
/// likely to be on its own empty line.
pub fn try_scanpw() -> crossterm::Result<String> {
  // Enter raw mode so we can control character echoing
  terminal::enable_raw_mode()?;

  // The password
  let mut pw = String::new();

  loop {
    if let Event::Key(k) = event::read()? {
      match k {
        // Normal character input
        KeyEvent {
          code: KeyCode::Char(c),
          modifiers,
        } if modifiers.is_empty() => {
          pw.push(c);
        }

        // Password input completed
        KeyEvent {
          code: KeyCode::Enter,
          ..
        } => {
          execute!(stdout(), MoveToNextLine(1))?;
          break;
        }

        // Handle backspace
        KeyEvent {
          code: KeyCode::Backspace,
          ..
        } => {
          // Delete the character from the password
          pw.pop();
        }

        // Pass Ctrl+C through as a signal like normal
        KeyEvent {
          code: KeyCode::Char('c'),
          modifiers,
        } if modifiers == KeyModifiers::CONTROL => {
          execute!(stdout(), Print("^C"),)?;
          terminal::disable_raw_mode()?;
          die();
        }

        _ => (),
      }
    }
  }

  // Reset the terminal back to normal
  terminal::disable_raw_mode()?;

  Ok(pw)
}

fn die() {
 use nix::sys::signal::{raise, Signal::SIGINT};
 raise(SIGINT).unwrap();
}

