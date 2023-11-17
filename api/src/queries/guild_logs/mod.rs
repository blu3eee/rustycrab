pub mod log_setting_queries;
pub mod action_log_queries;

use enum_primitive_derive::Primitive;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum DiscordLogsCategories {
    Message = 0,
    Member = 1,
    Role = 2,
    Channel = 3,
    Emoji = 4,
    Voice = 5,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum MessageEvents {
    MessageDelete = 0,
    MessageBulkDelete = 1,
    MessageEdit = 2,
    GuildInvites = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum MemberEvents {
    MemberJoin = 0,
    MemberLeave = 1,
    MemberRoleAdd = 2,
    MemberRoleRemove = 3,
    MemberTimeout = 4,
    MemberNicknameChange = 5,
    MemberKick = 6,
    MemberBan = 7,
    MemberUnban = 8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum RoleEvents {
    RoleCreate = 0,
    RoleDelete = 1,
    RoleUpdate = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum ChannelEvents {
    ChannelCreate = 0,
    ChannelDelete = 1,
    ChannelUpdate = 2,
    ThreadCreate = 3,
    ThreadDelete = 4,
    ThreadUpdate = 5,
    ThreadMembersUpdate = 6,
    ThreadMemberUpdate = 7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum EmojiEvents {
    EmojiCreate = 0,
    EmojiDelete = 1,
    EmojiNameChange = 2,
    StickerCreate = 3,
    StickerDelete = 4,
    StickerUpdate = 5,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum VoiceEvents {
    VoiceJoin = 0,
    VoiceLeave = 1,
    VoiceMove = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
pub enum BotLogs {
    ContextCommand = 0,
    SlashCommand = 1,
}
