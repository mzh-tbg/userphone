use poise::serenity_prelude as serenity;

use crate::data::{Data, Error};

pub async fn guild_removal(
    http: &serenity::Http,
    incomplete: &serenity::UnavailableGuild,
    guild: &Option<serenity::Guild>,
    data: &Data
) -> Result<(), Error> {

    {
        let mut lock = data.wait_line.lock();

        if let Some(pos) = data.wait_line.lock()
            .iter()
            .position(|v| v.guild_id == incomplete.id)
        {
            lock.remove(pos);
        }
    }

    let Some(guild) = guild.as_ref() else { return Err("Could not get guild".into()) };

    for channels in guild.channels.iter() {
        if data.current_calls.contains_key(channels.0 ) {
            let Some(value) = data.current_calls.remove(channels.0)
                else { continue; };

            value.1
                .linked_channel
                .send_message(
                    http, 
                    serenity::CreateMessage::new()
                        .content("<:phoneing:1366919461965402152>")
                ).await?;

        }
    }

    Ok(())
} 