use std::{fmt::Display, sync::Arc, time::Duration};
use colorful::Colorful;
use commands::get_commands;
use dotenv::dotenv;

use poise::serenity_prelude::{self as serenity, Timestamp};

use data::{
    Data,
    Command
};

mod commands;
mod data;
mod event_handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let commands: Vec<Command> = get_commands();
    
    let options = poise::FrameworkOptions {
        commands,
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler::event_handler(ctx, event, framework, data))
        },
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(Duration::from_secs(
                3600,
            )))),
            ignore_bots: true,
            mention_as_prefix: true,
            ..Default::default()
        },

        on_error: |error| Box::pin(data::error_handler::on_error(error)),

        pre_command: |ctx| {
            Box::pin(async move {
                print("pre-command", &ctx.command().qualified_name);
            })
        },

        post_command: |ctx| {
            Box::pin(async move {
                print("post-command", &ctx.command().qualified_name);
            })
        },

        skip_checks_for_owners: false,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
 
                Ok(Data::default())
            })
        })
        .options(options)
        .build();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("Missing the DISCORD_TOKEN env");
    let intents = serenity::GatewayIntents::all();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}


fn print<V>(time: &str, command_name: V) 
    where 
        V: Display
{

    println!(
        "[{}] [events] [{}] ==> {} >> {} <<",
        Timestamp::now().format("%d/%m/%Y %H:%M:%S"),
        "main".color(colorful::Color::LightPink1),
        time,
        command_name
    );
 
}

