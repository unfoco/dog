#[macro_export]
macro_rules! log_mem {
    ($template:expr, $($arg:expr),*) => {
        ctx.log_mem(
            format!($template, $($arg),*)
        ).await?;
    };
}

#[macro_export]
macro_rules! log_sys {
    ($template:expr, $($arg:expr),*) => {
        ctx.log_sys(
            format!($template, $($arg),*)
        ).await?;
    };
}
