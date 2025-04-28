use poise::serenity_prelude as serenity;
use crate::data::{Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    guild_only = true,
    user_cooldown = 15,
    track_edits,
    aliases("hg", "end")
)]
pub async fn hangup(ctx: Context<'_>) -> Result<(), Error> {

    let channel = ctx.channel_id();
    let userphone = ctx.data();
 
    match userphone.current_calls.get(&channel)
        .map(|v| v.linked_channel)
    {
        Some(linked_channel) => {
            userphone.current_calls.remove(&linked_channel);
            userphone.current_calls.remove(&channel);

            linked_channel.send_message(
                ctx.http(), 
                serenity::CreateMessage::new()
                    .content("The other party has ended the call :( ...")
            ).await?;

        },
        None => {
            let mut lock = userphone.wait_line.lock();

            let index = lock
                .iter()
                .enumerate()
                .position(|(_, l)| l.current_channel == channel)
                .ok_or("You are not currently in a call")?;

            lock.remove(index);
        }
    };


    ctx.send(
        poise::CreateReply::default()
            .content("You have ended the call")
    ).await?;

    Ok(())
}