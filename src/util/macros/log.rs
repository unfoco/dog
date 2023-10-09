#[macro_export]
macro_rules! log_mem {
    ($ctx:expr, $template:expr, $($arg:expr),*) => {
        {
            use crate::util::traits::ExtendContext;

            $ctx.log_mem(
                format!($template, $($arg),*)
            ).await?;
        }
    };
}

#[macro_export]
macro_rules! log_sys {
    ($ctx:expr, $template:expr, $($arg:expr),*) => {
        {
            use crate::util::traits::ExtendContext;

            $ctx.log_sys(
                format!($template, $($arg),*)
            ).await?;
        }
    };
}
