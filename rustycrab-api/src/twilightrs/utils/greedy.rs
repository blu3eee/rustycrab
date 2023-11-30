use twilight_http::Client as HttpClient;
use twilight_model::{
    user::User,
    id::{ Id, marker::{ UserMarker, ChannelMarker } },
    channel::Channel,
};

use crate::utilities::app_error::AppError;

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

pub async fn greedy_users<'a>(
    http: &'a HttpClient,
    args: &'a [&'a str]
) -> (Vec<User>, &'a [&'a str]) {
    let mut users: Vec<User> = Vec::new();
    let mut last_index: usize = 0;

    for (index, &arg) in args.iter().enumerate() {
        if let Some(id) = extract_id(arg) {
            let user_id: Id<UserMarker> = Id::new(id);
            // Retrieve the user asynchronously
            if let Ok(user) = http.user(user_id).await {
                if let Ok(user_response) = user.model().await {
                    users.push(user_response);
                    last_index = index;
                }
            }
        } else {
            break;
        }
    }

    let remaining_args: &[&str] = &args[last_index + 1..];
    (users, remaining_args)
}

pub async fn greedy_user<'a>(
    http: &'a HttpClient,
    args: &'a [&'a str]
) -> (Option<User>, &'a [&'a str]) {
    if let Some(&first_arg) = args.first() {
        if let Some(id) = extract_id(first_arg) {
            let user_id: Id<UserMarker> = Id::new(id);
            // Retrieve the user asynchronously
            match http.user(user_id).await {
                Ok(response) =>
                    match response.model().await {
                        Ok(user) => {
                            return (Some(user), &args[1..]);
                        }
                        Err(_) => (),
                    }
                Err(_) => (),
            }
        }
    }
    (None, args)
}

pub async fn greedy_force_user<'a>(
    http: &'a HttpClient,
    args: &'a [&'a str]
) -> Result<(User, &'a [&'a str]), AppError> {
    if let Some(&first_arg) = args.first() {
        if let Some(id) = extract_id(first_arg) {
            let user_id: Id<UserMarker> = Id::new(id);
            match http.user(user_id).await {
                Ok(response) =>
                    match response.model().await {
                        Ok(user) => Ok((user, &args[1..])),
                        Err(_) => Err(AppError::not_found("User not found")),
                    }
                Err(_) => Err(AppError::bad_request("Invalid user ID")),
            }
        } else {
            Err(AppError::bad_request("Invalid user ID"))
        }
    } else {
        Err(AppError::bad_request("No user ID provided"))
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

// Function to parse a single channel mention or ID
pub async fn greedy_channel<'a>(
    http: &'a HttpClient,
    args: &'a [&'a str]
) -> (Option<Channel>, &'a [&'a str]) {
    if let Some(&first_arg) = args.first() {
        if let Some(id) = extract_id(first_arg) {
            let channel_id: Id<ChannelMarker> = Id::new(id);
            match http.channel(channel_id).await {
                Ok(response) =>
                    match response.model().await {
                        Ok(channel) => {
                            return (Some(channel), &args[1..]);
                        }
                        Err(_) => (),
                    }
                Err(_) => (),
            }
        }
    }
    (None, args)
}

// Function to parse multiple channel mentions or IDs
pub async fn greedy_channels<'a>(
    http: &'a HttpClient,
    args: &'a [&'a str]
) -> (Vec<Channel>, &'a [&'a str]) {
    let mut channels = Vec::new();
    let mut last_index = 0;

    for (index, &arg) in args.iter().enumerate() {
        if let Some(id) = extract_id(arg) {
            let channel_id: Id<ChannelMarker> = Id::new(id);
            if let Ok(response) = http.channel(channel_id).await {
                if let Ok(channel) = response.model().await {
                    channels.push(channel);
                    last_index = index;
                }
            }
        } else {
            break;
        }
    }

    let remaining_args = &args[last_index + 1..];
    (channels, remaining_args)
}
