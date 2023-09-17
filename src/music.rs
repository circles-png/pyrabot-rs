use anyhow::{Error, Result};
use poise::{command, Context};
use serenity::prelude::Mentionable;
use songbird::{
    get,
    input::{Input, Restartable},
};

use crate::Data;

#[command(slash_command, prefix_command)]
pub async fn join(context: Context<'_, Data, Error>) -> Result<()> {
    context.defer().await?;
    let Some(guild_id) = context.guild_id() else {
        context.reply("you're not in a guild!").await?;
        return Ok(());
    };
    let Some(guild) = context.guild() else {
        context.reply("you're not in a guild!").await?;
        return Ok(());
    };
    let channel_id = guild
        .voice_states
        .get(&context.author().id)
        .and_then(|voice_state| voice_state.channel_id);
    let Some(connect_to) = channel_id else {
        context.reply("you're not in a voice channel!").await?;
        return Ok(());
    };
    let Some(manager) = get(context.serenity_context()).await else {
        context.reply("no voice client!").await?;
        return Ok(());
    };
    let (_, success) = manager.join(guild_id, connect_to).await;

    if success.is_ok() {
        context
            .reply(format!("joined voice channel {}!", connect_to.mention()))
            .await?;
    } else {
        context
            .reply(format!(
                "failed to join voice channel {}!",
                connect_to.mention()
            ))
            .await?;
    }
    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn leave(context: Context<'_, Data, Error>) -> Result<()> {
    context.defer().await?;
    let Some(guild_id) = context.guild_id() else {
        context.reply("you're not in a guild!").await?;
        return Ok(());
    };
    let Some(manager) = get(context.serenity_context()).await else {
        context.reply("no voice client!").await?;
        return Ok(());
    };
    let has_handler = manager.get(guild_id).is_some();
    if has_handler {
        if manager.remove(guild_id).await.is_err() {
            context.reply("failed to leave voice channel!").await?;
            return Ok(());
        }
        context.reply("left voice channel!").await?;
    } else {
        context.reply("not in a voice channel!").await?;
    }
    Ok(())
}

#[command(slash_command, prefix_command)]
pub async fn play(context: Context<'_, Data, Error>, query: String) -> Result<()> {
    context.defer().await?;
    let Some(guild_id) = context.guild_id() else {
        context.reply("you're not in a guild!").await?;
        return Ok(());
    };
    let Some(manager) = get(context.serenity_context()).await else {
        context.reply("no voice client!").await?;
        return Ok(());
    };
    if let Some(handler_lock) = manager.get(guild_id) {
        let Ok(source) = Restartable::ytdl_search(&query, true).await else {
            context.reply("i couldn't start the source!").await?;
            return Ok(());
        };
        let input: Input = source.into();
        let metadata = input.metadata.clone();
        let mut handler = handler_lock.lock().await;
        let queue_length = handler.queue().len();
        handler.enqueue_source(input);
        drop(handler);
        context
            .reply(metadata.title.map_or_else(
                || "playing a song (couldn't get title!)".to_string(),
                |title| format!("playing `{}` (position {})!", title, queue_length),
            ))
            .await?;
    } else {
        context
            .reply("i'm not in a voice channel! use `join`")
            .await?;
    }
    Ok(())
}
