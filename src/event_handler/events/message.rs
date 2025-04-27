use poise::serenity_prelude as serenity;
use crate::data::{Data, Error};
use std::time::{Duration, Instant};


pub async fn message(
    http: &serenity::Http,
    new_message: &serenity::Message,
    data: &Data,
) -> Result<(), Error> {

    let command_names = vec!["~ring", "~skip", "~call", "~c", "~phone", "~connect", "~s", "~nc", "~newcall"];

    let channel_id = &new_message.channel_id;
    let author = &new_message.author;

    let message_content = new_message.content.clone();

    if author.bot || message_content.is_empty()
        || command_names.contains(&message_content.as_str()) {
        return Ok(())
    }
    
    let mut builder = serenity::ExecuteWebhook::new()
        .username(&author.name)
        .avatar_url(author.face())
        .content(&new_message.content);

    if let Some(v) = &new_message.referenced_message {
        if !v.content.is_empty() {  
            builder = builder.embed(
                serenity::CreateEmbed::new()
                    .author(
                        serenity::CreateEmbedAuthor::new(format!("- Replying to {}", &v.author.name))
                            .icon_url(v.author.face())
                    )
                    .colour(serenity::Colour::BLURPLE)
                    .field("", &v.content, false)
            )
        }
    }

    let info = match data.current_calls.get(channel_id) {
        Some(reference) if reference.start_time - Instant::now() > Duration::from_secs(36000) => {
            reference.linked_channel
                .send_message(http, serenity::CreateMessage::new()
                    .content("I think it's time for you guys to get some rest.")
                ).await?;
            
            false
        }

        Some(reference) => {
            match reference.linked_webhook.execute(http, true, builder).await {
                Ok(_) => return Ok(()),
                Err(error) => match error {
                    serenity::Error::Http(_) => true,
                    _ => return Ok(())
                }
            }
        }
        None => return Ok(())
    };

    data.current_calls.remove(channel_id);

    if info {
        let webhook = channel_id.create_webhook(
            http, 
            serenity::CreateWebhook::new("ArchaeasPhone")
        ).await?;   

        data.webhooks.insert(*channel_id, webhook);
    }

    Ok(())
}