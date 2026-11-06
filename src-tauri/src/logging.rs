//! Log macros. stdout/stderr end up in OS logs (journald, unified log), so:
//! never log secrets, message content, search terms, ids, tokens or paths.
//! Log fixed messages, counts and booleans only.

use std::fmt::Arguments;
use std::io::Write;

// Unlike println!, never panics when stdout is closed.
pub(crate) fn write_stdout(args: Arguments<'_>) {
    let _ = writeln!(std::io::stdout().lock(), "{args}");
}

pub(crate) fn write_stderr(args: Arguments<'_>) {
    let _ = writeln!(std::io::stderr().lock(), "{args}");
}

/// Dev builds only (`tauri dev`): removed from every `tauri build` (release profile).
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::logging::write_stdout(format_args!($($arg)*));
        }
    };
}

/// Always printed, including in production.
macro_rules! info_log {
    ($($arg:tt)*) => {
        $crate::logging::write_stdout(format_args!($($arg)*))
    };
}

/// Always printed to stderr, including in production.
macro_rules! error_log {
    ($($arg:tt)*) => {
        $crate::logging::write_stderr(format_args!($($arg)*))
    };
}
