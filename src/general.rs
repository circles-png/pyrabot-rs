use anyhow::{Error, Result};
use poise::{command, Context};

use crate::Data;

#[command(slash_command, prefix_command)]
pub async fn ping(context: Context<'_, Data, Error>) -> Result<()> {
    let ping = context.ping().await;
    context
        .reply(format!("Pong! {}ms", ping.as_millis()))
        .await?;
    Ok(())
}
