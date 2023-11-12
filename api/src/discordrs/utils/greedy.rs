use discord::model::{ User, UserId };

use crate::{ discordrs::client::DiscordClient, utilities::app_error::AppError };

fn extract_id(arg: &str) -> Option<u64> {
    // Check if the argument is in one of the mention formats: <@>, <@!>, <@&>, or <#>
    if
        (arg.starts_with("<@") || arg.starts_with("<#") || arg.starts_with("<@&")) &&
        arg.ends_with('>')
    {
        let trimmed = arg
            .trim_start_matches("<@")
            .trim_start_matches("<#")
            .trim_start_matches("<@&")
            .trim_end_matches('>')
            .trim_start_matches('!');
        trimmed.parse::<u64>().ok()
    } else if
        // Check if the argument is a raw numeric string
        arg.chars().all(char::is_numeric)
    {
        arg.parse::<u64>().ok()
    } else {
        None
    }
}

pub fn greedy_users<'a>(
    client: &'a DiscordClient,
    args: &'a [&'a str]
) -> (Vec<User>, &'a [&'a str]) {
    let mut users: Vec<User> = Vec::new();
    let mut last_index: usize = 0;

    for (index, &arg) in args.iter().enumerate() {
        match extract_id(arg) {
            Some(id) => {
                let user_id: UserId = UserId(id);
                // Assuming get_user is a synchronous function - adjust if it's async
                if let Ok(user) = client.discord.get_user(user_id) {
                    users.push(user);
                }
                last_index = index;
            }
            None => {
                break;
            } // Stop processing if the argument isn't a valid user ID
        }
    }

    let remaining_args: &[&str] = &args[last_index + 1..];
    (users, remaining_args)
}

pub fn greedy_user<'a>(
    client: &'a DiscordClient,
    args: &'a [&'a str]
) -> (Option<User>, &'a [&'a str]) {
    if let Some(&first_arg) = args.first() {
        if let Some(id) = extract_id(first_arg) {
            let user_id = UserId(id);
            // Assuming get_user is a synchronous function - adjust if it's async
            match client.discord.get_user(user_id) {
                Ok(user) => (Some(user), &args[1..]), // User found, return it with remaining args
                Err(_) => (None, args), // User not found, return original args
            }
        } else {
            (None, args) // First argument is not a valid user ID, return original args
        }
    } else {
        (None, args) // No arguments provided, return original args
    }
}

pub fn greedy_force_user<'a>(
    client: &'a DiscordClient,
    args: &'a [&'a str]
) -> Result<(User, &'a [&'a str]), AppError> {
    if let Some(&first_arg) = args.first() {
        if let Some(id) = extract_id(first_arg) {
            let user_id = UserId(id);
            // Assuming get_user is a synchronous function - adjust if it's async
            match client.discord.get_user(user_id) {
                Ok(user) => Ok((user, &args[1..])), // User found, return it with remaining args
                Err(_) => Err(AppError::not_found("User not found")), // User not found, return error
            }
        } else {
            Err(AppError::bad_request("invalid user id")) // First argument is not a valid user ID, return error
        }
    } else {
        Err(AppError::bad_request("invalid user id")) // No arguments provided, return error
    }
}

fn extract_emoji_id(arg: &str) -> Option<u64> {
    // Check if the argument is in the custom emoji format <:emojiname:emojiid> or <a:emojiname:emojiid>
    if (arg.starts_with("<:") || arg.starts_with("<a:")) && arg.ends_with('>') {
        arg.split(':').last()?.trim_end_matches('>').parse::<u64>().ok()
    } else if arg.chars().all(char::is_numeric) {
        // Check if the argument is a raw numeric string
        arg.parse::<u64>().ok()
    } else {
        None
    }
}

pub fn greedy_emoji<'a>(args: &'a [&'a str]) -> (Option<u64>, &'a [&'a str]) {
    if let Some(&first_arg) = args.first() {
        if let Some(emoji_id) = extract_emoji_id(first_arg) {
            return (Some(emoji_id), &args[1..]);
        }
    }
    (None, args)
}

pub fn greedy_emojis<'a>(args: &'a [&'a str]) -> (Vec<u64>, &'a [&'a str]) {
    let mut emoji_ids = Vec::new();
    let mut last_index = 0;

    for (index, &arg) in args.iter().enumerate() {
        if let Some(emoji_id) = extract_emoji_id(arg) {
            emoji_ids.push(emoji_id);
            last_index = index;
        } else {
            break;
        }
    }

    let remaining_args = &args[last_index + 1..];
    (emoji_ids, remaining_args)
}
