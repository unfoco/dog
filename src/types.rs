use crate::config::Config;

pub type CommandVec = Vec<poise::Command<Data, Error>>;

pub type Framework = poise::Framework<Data, Error>;
pub type FrameworkOptions = poise::FrameworkOptions<Data, Error>;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type AppContext<'a> = poise::ApplicationContext<'a, Data, Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;

pub struct Data {
    pub config: Config
}
