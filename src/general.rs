use anyhow::{Error, Result};
use poise::{command, samples::register_application_commands_buttons, Context};

use crate::Data;

#[command(slash_command, prefix_command)]
pub async fn ping(context: Context<'_, Data, Error>) -> Result<()> {
    let ping = context.ping().await;
    context
        .reply(format!("Pong! {}ms", ping.as_millis()))
        .await?;
    Ok(())
}

#[command(slash_command, prefix_command, owners_only)]
pub async fn register(context: Context<'_, Data, Error>) -> Result<()> {
    register_application_commands_buttons(context).await?;
    Ok(())
}
