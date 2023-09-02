//! Provides log macros.

/// Logs a message.
///
/// This macro will generically log with the specified [`Level`] and `format!`
/// based argument list.
///
/// # Examples
///
/// ```
/// use spdlog::{log, Level};
///
/// # let app_events = spdlog::default_logger();
/// let data = (42, "Forty-two");
/// let private_data = "private";
///
/// log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
/// log!(logger: app_events, Level::Warn, "App warning: {}, {}, {}",
///     data.0, data.1, private_data);
/// ```
///
/// [`Level`]: crate::Level
#[macro_export]
macro_rules! log {
    (logger: $logger:expr, $level:expr, $($arg:tt)+) => ({
        let logger = &$logger;
        const LEVEL: $crate::Level = $level;
        const SHOULD_LOG: bool = $crate::STATIC_LEVEL_FILTER.__compare_const(LEVEL);
        if SHOULD_LOG && logger.should_log(LEVEL) {
            $crate::__log(logger, LEVEL, $crate::source_location_current!(), format_args!($($arg)+));
        }
    });
    ($level:expr, $($arg:tt)+) => ($crate::log!(logger: $crate::default_logger(), $level, $($arg)+))
}

/// Logs a message at the critical level.
///
/// # Examples
///
/// ```
/// use spdlog::critical;
///
/// # let app_events = spdlog::default_logger();
/// let (left, right) = (true, false);
///
/// critical!("Runtime assertion failed. Left: `{}`, Right: `{}`", left, right);
/// critical!(logger: app_events, "Runtime assertion failed. Left: `{}`, Right: `{}`", left, right);
/// ```
#[macro_export]
macro_rules! critical {
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Critical, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Critical, $($arg)+)
    )
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```
/// use spdlog::error;
///
/// # let app_events = spdlog::default_logger();
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {} on port {}", err_info, port);
/// error!(logger: app_events, "App Error: {}, Port: {}", err_info, port);
/// ```
#[macro_export]
macro_rules! error {
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Error, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Error, $($arg)+)
    )
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```
/// use spdlog::warn;
///
/// # let input_events = spdlog::default_logger();
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {}!", warn_description);
/// warn!(logger: input_events, "App received warning: {}", warn_description);
/// ```
#[macro_export]
macro_rules! warn {
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Warn, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Warn, $($arg)+)
    )
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```
/// use spdlog::info;
///
/// # struct Connection { port: u32, speed: f32 }
/// # let connection_events = spdlog::default_logger();
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
/// info!(logger: connection_events, "Successfull connection, port: {}, speed: {}",
///       conn_info.port, conn_info.speed);
/// ```
#[macro_export]
macro_rules! info {
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Info, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Info, $($arg)+)
    )
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```
/// use spdlog::debug;
///
/// # struct Position { x: f32, y: f32 }
/// # let app_events = spdlog::default_logger();
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// debug!(logger: app_events, "New position: x: {}, y: {}", pos.x, pos.y);
/// ```
#[macro_export]
macro_rules! debug {
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Debug, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Debug, $($arg)+)
    )
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```
/// use spdlog::trace;
///
/// # struct Position { x: f32, y: f32 }
/// # let app_events = spdlog::default_logger();
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// trace!(logger: app_events, "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// ```
#[macro_export]
macro_rules! trace {
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Trace, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Trace, $($arg)+)
    )
}
