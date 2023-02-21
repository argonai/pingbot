use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

pub fn run(_options: &[CommandDataOption]) -> String {
    let option = _options.get(0).expect("expected user option").resolved.as_ref().expect("expected user object");
    if let CommandDataOptionValue::User(user, _member) = option {
        format!("{}'s id is {}", user.tag(), user.id)
    } else {
        "Please provide a valid user".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("profile").description("Get all user info").create_option(|option|{
        option
            .name("user")
            .description("User whose profile you want")
            .kind(CommandOptionType::User).required(true)
    })
}
