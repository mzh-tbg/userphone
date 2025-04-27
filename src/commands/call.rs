use crate::{commands::get_webhook, data::{Context, Data, Error, UserphoneLink, WaitLine}};
use poise::serenity_prelude as serenity;


#[poise::command(
    prefix_command,
    slash_command,
    guild_only = true,
    user_cooldown = 15,
    track_edits,
    aliases("call", "c", "phone", "connect")
)]
pub async fn ring(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Could not get guild")?;
    let initial = ctx.say("Please wait until another party has answered the call...").await?;

    let userphone = &ctx.data();
    let channel_id = ctx.channel_id();

    let webhook = get_webhook(ctx.http(), userphone, channel_id).await?;

    if userphone.current_calls.contains_key(&channel_id) {
        initial.edit(ctx, poise::CreateReply::default().content("You're already in a call!")).await?;
        return Ok(());
    }

    let initial_message = initial.message().await?.into_owned();
    let content = initial_message.content.clone();
    
    let mut message = match queue_handling(userphone, guild_id,  webhook, channel_id, initial_message).await {
        Ok(v) => v,
        Err(e) => {
            if format!("{}", e) == content {
                return Ok(())
            } else {
                return Err(e)
            }
        }
    };
  
    message.edit(
        ctx, 
        serenity::EditMessage::default()
            .content("A party has picked up the call.. Please be nice and respectful!")
    ).await?;

    initial.edit(
        ctx, 
        poise::CreateReply::default()
            .content("A party has picked up the call.. Please be nice and respectful!")
    ).await?;
    

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    guild_only = true,
    user_cooldown = 15,
    track_edits,
    aliases("s", "nc", "newcall")
)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {

    let guild_id = ctx.guild_id().ok_or("Could not get guild")?;
    let channel_id = ctx.channel_id();

    let userphone = &ctx.data();

    let http = ctx.http();

    let linked_channel = userphone.current_calls.get(&channel_id)
        .map(|v| v.linked_channel)
        .ok_or("You are not currently inside a call..")?;
    
    userphone.current_calls.remove(&linked_channel);
    userphone.current_calls.remove(&channel_id);

    linked_channel.send_message(
        http, 
        serenity::CreateMessage::new()
            .content("The other party has skipped you :( ...")
    ).await?;

    let reply = ctx.say("Skipping the call..").await?;

    let initial_message = reply.message().await?.into_owned();
    let content = initial_message.content.clone();

    let webhook = get_webhook(http, userphone, channel_id).await?;
    
    let mut message = match queue_handling(userphone, guild_id,  webhook, channel_id, initial_message).await {
        Ok(v) => v,
        Err(e) => {
            if format!("{}", e) == content {
                return Ok(())
            } else {
                return Err(e)
            }
        }
    };

    message.edit(
        http, 
        serenity::EditMessage::default()
            .content("A party has picked up the call.. Please be nice and respectful!")
    ).await?;

    reply.edit(
        ctx, 
        poise::CreateReply::default()
            .content("A party has picked up the call.. Please be nice and respectful!")
    ).await?;

    Ok(())
}


pub async fn queue_handling(
    userphone: &Data, 
    guild_id: serenity::GuildId, 
    webhook: serenity::Webhook, 
    current_channel: serenity::ChannelId,
    original_message: serenity::Message,
) -> Result<serenity::Message, Error> {

    let mut queue = userphone.wait_line.lock();
    
    let index = match queue.iter().enumerate()
        .find(|(_, w)| w.guild_id != guild_id)
    {
        Some(v) => v.0,
        None => {

            let content = original_message.content.clone();

            queue.push(WaitLine {
                guild_id,
                current_channel,
                current_webhook: webhook,
                original_message,
            });

            return Err(content.into())
        }
    };

    let waiting = queue.remove(index);

    let time = std::time::Instant::now();

    userphone.current_calls.insert(
        current_channel, 
        UserphoneLink {
            linked_channel: waiting.current_channel,
            linked_webhook: waiting.current_webhook,
            start_time: time,
        }
    );

    userphone.current_calls.insert(
        waiting.current_channel,
        UserphoneLink {
            linked_channel: current_channel,
            linked_webhook: webhook,
            start_time: time,
        }
    );

    Ok(waiting.original_message)
}