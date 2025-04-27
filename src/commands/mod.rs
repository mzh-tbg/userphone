use poise::serenity_prelude as serenity;
use crate::data::{Data, Error};

mod call;
mod hangup;

pub fn get_commands() -> Vec<crate::data::Command> 
{
    vec![
        call::ring(),
        call::skip(),
        hangup::hangup()
    ]
}


pub async fn get_webhook(http: &serenity::Http, userphone: &Data, channel_id: serenity::ChannelId) -> Result<serenity::Webhook, Error> {
    match userphone.webhooks.get(&channel_id) {
        Some(webhook) => Ok(webhook.to_owned()),
        None => {
            let new_webhook = channel_id
                .webhooks(http)
                .await?
                .into_iter() 
                .find(|v| v.name == Some("ArchaeasPhone".to_owned()));
            
            let webhook = match new_webhook {
                Some(v) => v,
                None => channel_id
                    .create_webhook(http, serenity::CreateWebhook::new("ArchaeasPhone"))
                    .await?,
            };
        
            userphone.webhooks.insert(channel_id, webhook.clone());
            Ok(webhook)
        }
    }
}