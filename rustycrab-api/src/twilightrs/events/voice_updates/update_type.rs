use twilight_model::id::Id;
use twilight_model::voice::VoiceState;
use twilight_model::id::marker::ChannelMarker;

#[derive(Debug, Clone)]
pub enum VoiceUpdateType {
    Join(Id<ChannelMarker>),
    Leave(Option<Id<ChannelMarker>>),
    ChannelSwitch(Id<ChannelMarker>, Id<ChannelMarker>),
    UpdateWithinChannel(Id<ChannelMarker>), // For mute/deafen updates
}

pub fn get_update_type(old_state: &Option<VoiceState>, new_state: &VoiceState) -> VoiceUpdateType {
    match old_state {
        Some(old_state) => {
            if new_state.channel_id.is_none() {
                VoiceUpdateType::Leave(old_state.channel_id.clone())
            } else if let Some(old_channel_id) = old_state.channel_id {
                let new_channel_id = new_state.channel_id.unwrap();
                if old_channel_id != new_channel_id {
                    VoiceUpdateType::ChannelSwitch(old_channel_id, new_channel_id)
                } else {
                    VoiceUpdateType::UpdateWithinChannel(new_channel_id)
                }
            } else {
                VoiceUpdateType::Join(new_state.channel_id.unwrap())
            }
        }
        None => {
            if new_state.channel_id.is_some() {
                VoiceUpdateType::Join(new_state.channel_id.unwrap())
            } else {
                VoiceUpdateType::Leave(None)
            }
        }
    }
}
