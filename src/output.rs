use supports_color::{Stream, on};

/// Check if stdout supports colors.
#[inline(always)]
pub fn color_enabled() -> bool {
    static COLOR_SUPPORTED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *COLOR_SUPPORTED.get_or_init(|| on(Stream::Stdout).is_some())
}

/// Check if stderr supports colors.
pub fn color_enabled_err() -> bool {
    static COLOR_SUPPORTED_ERR: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *COLOR_SUPPORTED_ERR.get_or_init(|| on(Stream::Stderr).is_some())
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        if *$crate::LOG_LEVEL.get().unwrap() <= $crate::LogLevelList::Error {
            if $crate::output::color_enabled() {
                println!("{} {}", "[success]".green().bold(), format!($($arg)*));
            } else {
                println!("[success] {}", format!($($arg)*));
            }
        }
    }};
}

#[macro_export]
macro_rules! failure {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        if *$crate::LOG_LEVEL.get().unwrap() <= $crate::LogLevelList::Fatal {
            if $crate::output::color_enabled_err() {
                eprintln!("{} {}", "[error]  ".red().bold(), format!($($arg)*));
            } else {
                eprintln!("[error]   {}", format!($($arg)*));
            }
        }
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        if *$crate::LOG_LEVEL.get().unwrap() <= $crate::LogLevelList::Warn {
            if $crate::output::color_enabled_err() {
                eprintln!("{} {}", "[warning]".yellow().bold(), format!($($arg)*));
            } else {
                eprintln!("[warning] {}", format!($($arg)*));
            }
        }
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        if *$crate::LOG_LEVEL.get().unwrap() <= $crate::LogLevelList::Info {
            if $crate::output::color_enabled() {
                println!("{} {}", "[info]   ".bright_blue().bold(), format!($($arg)*));
            } else {
                println!("[info]    {}", format!($($arg)*));
            }
        }
    }};
}

#[macro_export]
macro_rules! status {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        if *$crate::LOG_LEVEL.get().unwrap() <= $crate::LogLevelList::Debug {
            if $crate::output::color_enabled() {
                println!("{} {}", "[status] ".magenta().bold(), format!($($arg)*));
            } else {
                println!("[status]  {}", format!($($arg)*));
            }
        }
    }};
}
