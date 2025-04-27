use dashmap::DashMap;
use parking_lot::Mutex;
use poise::serenity_prelude as serenity;

pub mod error_handler;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

#[derive(Default)]
pub struct Data {

    pub current_calls: DashMap<serenity::ChannelId, UserphoneLink>,
    pub wait_line: Mutex<Vec<WaitLine>>,
    pub webhooks: DashMap<serenity::ChannelId, serenity::Webhook> // store channel -> webhooks so no requesting it each time

}

#[derive(Clone)]
pub struct UserphoneLink {
    pub linked_channel: serenity::ChannelId,
    pub linked_webhook: serenity::Webhook,
    pub start_time: std::time::Instant,
}

pub struct WaitLine {
    pub guild_id: serenity::GuildId,
    pub current_channel: serenity::ChannelId,
    pub current_webhook: serenity::Webhook,
    pub original_message: serenity::Message
}