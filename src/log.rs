#[macro_export]
#[allow(unused)]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => ({
        ::lapce_plugin::PLUGIN_RPC.window_log_message(
            $lvl,
            format!($($arg)+),
        )?;
    });
}

#[macro_export]
#[allow(unused)]
macro_rules! error {
    ($($arg:tt)+) => {
        crate::log!(::lapce_plugin::psp_types::lsp_types::MessageType::ERROR, $($arg)+)
    };
}

#[macro_export]
#[allow(unused)]
macro_rules! warn {
    ($($arg:tt)+) => {
        crate::log!(::lapce_plugin::psp_types::lsp_types::MessageType::WARN, $($arg)+)
    };
}

#[macro_export]
#[allow(unused)]
macro_rules! debug {
    ($($arg:tt)+) => {
        crate::log!(::lapce_plugin::psp_types::lsp_types::MessageType::DEBUG, $($arg)+)
    };
}

#[macro_export]
#[allow(unused)]
macro_rules! info {
    ($($arg:tt)+) => {
        crate::log!(::lapce_plugin::psp_types::lsp_types::MessageType::INFO, $($arg)+)
    };
}
