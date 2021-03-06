use serenity::client::Context;
use serenity::framework::standard::help_commands;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::Args;
use serenity::framework::standard::HelpOptions;
use serenity::framework::standard::{CommandGroup, CommandResult};
use serenity::model::prelude::{Message, UserId};

use std::collections::HashSet;

#[help]
#[no_help_available_text(
    "**Error**: I was unable to find any information on this command, \
    usually indicating that this command does not exist or does not have \
    any help available for said command. Please try again later, or try \
    searching for a different command instead."
)]
fn help(
    context: &mut Context,
    message: &Message,
    arguments: Args,
    options: &'static HelpOptions,
    command_groups: &[&'static CommandGroup],
    bot_owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, message, arguments, &options, command_groups, bot_owners)
}
