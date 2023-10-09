#[macro_export]
macro_rules! log_mem {
    ($template:expr, $($arg:expr),*) => {
        ctx.log_mem(
            format!($template, $($arg),*)
        ).await?;
    };
}

#[macro_export]
macro_rules! log_mem_with_embed {
    ($template:expr, $($arg:expr),*; $closure:expr) => {
        ctx.log_mem_with_embed(
            format!($template, $($arg),*),
            $closure,
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

#[macro_export]
macro_rules! log_sys_with_embed {
    ($template:expr, $($arg:expr),*; $closure:expr) => {
        ctx.log_sys_with_embed(
            format!($template, $($arg),*),
            $closure,
        ).await?;
    };
}
