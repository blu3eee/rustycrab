use std::error::Error;

use rustycrab_model::{ color::ColorResolvables, error::BoxedError, music::PlayerLoopState };

use twilight_model::{
    gateway::payload::incoming::VoiceStateUpdate,
    id::{ marker::GuildMarker, Id },
};

use crate::twilightrs::{
    discord_client::{ DiscordClient, MessageContent },
    messages::DiscordEmbed,
};

use self::update_type::{ get_update_type, VoiceUpdateType };

mod update_type;

pub async fn handle_voice_state_update(
    client: DiscordClient,
    update: &Box<VoiceStateUpdate>
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let new_state = &update.0;
    // println!("{:?}", new_state);

    // Extract guild_id from the new state
    let guild_id = match new_state.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Ok(());
        }
    };

    // Determine the type of voice state update
    let update_type = {
        // Before the async block
        let mut voice_cached = client.voice_states_cached.write().unwrap();
        let old_state = voice_cached
            .get(&guild_id)
            .and_then(|guild_map| guild_map.get(&new_state.user_id))
            .cloned();

        // Update the cached voice state
        voice_cached
            .entry(guild_id)
            .or_insert_with(Default::default)
            .insert(new_state.user_id, new_state.clone());

        get_update_type(&old_state, &new_state)
    };

    let bot_channel_id = client.get_bot_vc_channel_id(guild_id).await?;

    // Handle different update types
    match update_type {
        VoiceUpdateType::Join(channel_id) => {
            println!("{} joined channel {:?}", new_state.user_id, channel_id);
            if bot_channel_id.is_some() && client.get_vc_member_count(bot_channel_id.unwrap()) > 1 {
                let _ = resume_on_user_join(&client, guild_id).await;
            }
        }
        VoiceUpdateType::Leave(channel_id) => {
            // Handle user leaving a channel
            println!("{} left channel {:?}", new_state.user_id, channel_id);
            if bot_channel_id.is_some() && client.get_vc_member_count(bot_channel_id.unwrap()) == 1 {
                let _ = pause_player_on_user_left(&client, guild_id).await;
            }
            if new_state.user_id == client.get_bot().await?.id {
                client.voice_music_manager.set_loop_state(guild_id, PlayerLoopState::NoLoop);
                client.voice_music_manager.clear_waiting_queue(guild_id);
                let _ = client.voice_music_manager.songbird.remove(guild_id).await;
            }
        }
        VoiceUpdateType::ChannelSwitch(old_channel_id, new_channel_id) => {
            println!(
                "{} switched channel {:?} -> {:?}",
                new_state.user_id,
                old_channel_id,
                new_channel_id
            );

            if bot_channel_id.is_some() {
                if new_state.user_id == client.get_bot().await?.id {
                    let trackqueue = client.voice_music_manager.get_play_queue(guild_id);
                    let _ = trackqueue.pause();
                    // // remove
                    // let _ = client.voice_music_manager.songbird.remove(guild_id).await;
                    // // join again with new state
                    let _ = client.voice_music_manager.songbird.join(
                        guild_id,
                        new_channel_id
                    ).await;
                    let _ = trackqueue.resume();
                    // let track_handle = client.fetch_trackhandle(guild_id, None).await;
                    // println!("trackhandle {:?}", track_handle);
                    // if let Ok(track_handle) = track_handle {
                    //     let _ = track_handle.stop();
                    //     // println!("switching songbird to new channel");
                    //     let _ = client.voice_music_manager.songbird.join(
                    //         guild_id,
                    //         new_channel_id
                    //     ).await;
                    //     // println!("switching result {:?}", result);
                    //     // let _ = track_handle.play();
                    // } else {
                    //     eprintln!("cant find track handle");
                    // }
                }
                // if client.get_vc_member_count(bot_channel_id.unwrap()) == 1 {
                //     println!("pausing");
                //     let _ = client.voice_music_manager.songbird.leave(guild_id).await;
                // } else {
                //     println!("resuming");
                //     let _ = resume_on_user_join(&client, guild_id).await;
                // }
            }

            // Handle user switching channels
        }
        VoiceUpdateType::UpdateWithinChannel(channel_id) => {
            // Handle updates within a channel (mute/deafen)
            println!("{} voice state updated {:?}", update.user_id, channel_id);
        }
    }
    println!("finished handling voice state");
    Ok(())
}

async fn pause_player_on_user_left(
    client: &DiscordClient,
    guild_id: Id<GuildMarker>
) -> Result<(), BoxedError> {
    let (player_channel_id, _) = client.voice_music_manager.get_player_ids(guild_id);

    if client.voice_music_manager.pause_player(guild_id).await? == true {
        if let Some(player_channel_id) = player_channel_id {
            let _ = client.send_message(
                player_channel_id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        author_name: Some(format!("Music player paused")),
                        author_icon_url: Some(client.voice_music_manager.spinning_disk.clone()),
                        description: Some(format!("No one left in the channel")),
                        color: Some(ColorResolvables::Yellow.as_u32()),
                        ..Default::default()
                    }]
                )
            ).await;
        }
    }

    Ok(())
}

async fn resume_on_user_join(
    client: &DiscordClient,
    guild_id: Id<GuildMarker>
) -> Result<(), BoxedError> {
    let (player_channel_id, _) = client.voice_music_manager.get_player_ids(guild_id);
    if client.voice_music_manager.resume_player(guild_id).await? == true {
        if let Some(player_channel_id) = player_channel_id {
            let _ = client.send_message(
                player_channel_id,
                MessageContent::DiscordEmbeds(
                    vec![DiscordEmbed {
                        author_name: Some(format!("Music player resumed")),
                        author_icon_url: Some(client.voice_music_manager.spinning_disk.clone()),
                        color: Some(ColorResolvables::Green.as_u32()),
                        ..Default::default()
                    }]
                )
            ).await;
        }
    }
    Ok(())
}
