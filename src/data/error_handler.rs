use std::fmt::{Debug, Display};

use colorful::Colorful;
use super::{Data, Error};
use poise::serenity_prelude::Timestamp;



pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            let builder = poise::CreateReply::default()
                .content(error.to_string())
                .ephemeral(true)
                .reply(true);

            let _ = ctx.send(builder).await;
            print_error(&ctx.command().name, error);
        },
        
        poise::FrameworkError::MissingUserPermissions { missing_permissions, ctx, .. } => {

            let mut message = String::new();
            
            for perm in missing_permissions.expect("No missing perms").iter() {
                message.push_str(format!("- `{perm}`\n").as_str());
            }

            let _ = ctx.reply(format!("You are missing the following permissions: {message}")).await;
            
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                print_error("Unknown", e);
            }
        },
    }
}


fn print_error<T, V>(command_name: V, error: T) 
    where 
        T: Debug,
        V: Display
{

    println!(
        "[{}] [events] [{}] ==> {} >> {:?} <<",
        Timestamp::now().format("%d/%m/%Y %H:%M:%S"),
        "error".color(colorful::Color::Red),
        command_name,
        error
    );

}