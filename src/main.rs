#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]

use anyhow::{Error, Result};
use dotenv_codegen::dotenv;
use poise::{
    command,
    samples::{register_globally, register_in_guild},
    serenity_prelude::GuildId,
    Context, Framework, FrameworkOptions,
};
use serenity::prelude::GatewayIntents;

use std::vec;

#[derive(Debug)]
struct Data;

#[command(slash_command, prefix_command)]
async fn ping(context: Context<'_, Data, Error>) -> Result<()> {
    let ping = context.ping().await;
    context.reply(format!("Pong! {}ms", ping.as_millis())).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    Framework::builder()
        .options(FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .token(dotenv!("BOT_TOKEN"))
        .intents(GatewayIntents::all())
        .setup(|context, _ready, framework| {
            Box::pin(async move {
                println!("Ready!");
                register_globally(context, &framework.options().commands).await?;
                register_in_guild(
                    context,
                    &framework.options().commands,
                    GuildId(dotenv!("PRIMARY_GUILD").parse()?),
                )
                .await?;
                Ok(Data)
            })
        })
        .run()
        .await?;
    Ok(())
}
