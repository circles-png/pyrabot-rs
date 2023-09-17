#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![feature(async_closure)]

mod general;
mod music;

use anyhow::Result;
use dotenv_codegen::dotenv;
use general::ping;
use music::{join, leave, play};
use poise::{
    samples::{register_globally, register_in_guild},
    serenity_prelude::GuildId,
    Framework, FrameworkOptions,
};
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;

use std::vec;

#[derive(Debug)]
pub struct Data;

#[tokio::main]
async fn main() -> Result<()> {
    Framework::builder()
        .options(FrameworkOptions {
            commands: vec![ping(), join(), leave(), play()],
            on_error: |error| {
                Box::pin(async move {
                    dbg!(error);
                })
            },
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
        .client_settings(SerenityInit::register_songbird)
        .run()
        .await?;
    Ok(())
}
