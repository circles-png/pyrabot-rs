#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![feature(async_closure)]

mod general;
mod music;

use anyhow::Result;
use dotenv_codegen::dotenv;
use general::{ping, register};
use music::{join, leave, play};
use poise::{Framework, FrameworkOptions};
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;

use std::vec;

#[derive(Debug)]
pub struct Data;

#[tokio::main]
async fn main() -> Result<()> {
    Framework::builder()
        .options(FrameworkOptions {
            commands: vec![ping(), join(), leave(), play(), register()],
            on_error: |error| {
                Box::pin(async move {
                    dbg!(error);
                })
            },
            ..Default::default()
        })
        .token(dotenv!("BOT_TOKEN"))
        .intents(GatewayIntents::all())
        .setup(|_, _, _| {
            Box::pin(async move {
                println!("Ready!");
                Ok(Data)
            })
        })
        .client_settings(SerenityInit::register_songbird)
        .run()
        .await?;
    Ok(())
}
