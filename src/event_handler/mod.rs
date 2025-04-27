use colorful::Colorful;
use poise::serenity_prelude::{self as serenity, CacheHttp, Timestamp};

use crate::data::{
    Data,
    Error
};

mod events;


pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error>
{

    println!(
        "[{}] [events] [{}] ==> {}",
        Timestamp::now().format("%d/%m/%Y %H:%M:%S"),
        "event-handler".color(colorful::Color::BlueViolet),
        event.snake_case_name()
    );
    let http = ctx.http();


    match event {
        serenity::FullEvent::Message { new_message } => {
            events::message::message(http, new_message, data).await?;
        }
        _ => ()
    }


    Ok(())
}