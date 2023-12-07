permission-missing = Missing permission(s) to use the command

# Common Errors
commonError-noUserInfo = Can't find { $user } information
commonError-noBotGuildConfig = Not Guild Configuration found
commonError-noTag = Tag at least one person
commonError-tag1 = Tag one permission

# Utilities
interaction-denied = You're not allowed to use this.
requested-user = Requested by @{ $username }
invalid-image-url = Invalid image url

# Command1s 
command-error = An error happened when trying to process the command
command-guildonly = Command not used in a guild
command-invalid = Invalid command

# Bot Owner Commands
command-botowner = Manage BotOwner permission
command-botowner-addSuccess = Added BotOwner permission for { $user }
command-botowner-removeSuccess = Removed { $user } BotOwner permission
command-botowner-alreadyOwner = { $user } already has BotOwner permission
command-botowner-notOwner = { $user } doesn't have BotOwner permission
command-botowner-listEmpty = Empty...
command-botowner-listTitle = Users with BotOwner permission

# Bot Admin Commands
command-botadmin = Manage BotAdmin permission
command-botadmin-addSuccess = Added BotAdmin permission for { $user }
command-botadmin-removeSuccess = Removed { $user } BotAdmin permission
command-botadmin-alreadyAdmin = { $user } already has BotAdmin permission
command-botadmin-notAdmin = { $user } doesn't have BotAdmin permission
command-botadmin-listEmpty = Empty...
command-botadmin-listTitle = Users with BotAdmin permission

# Language Command
command-language = Change bot's language
command-language-invalidLocale = Invalid language. Available languages: { $locales }
command-language-localeChanged = Bot's language switched to `{ $locale }`

# Avatar Command
command-avatar = Check user avatar

# Math Command
command-math = Calculate basic math expressions
command-math-invalid = Invalid Math Expression

# Prefix Command
command-prefix = Change bot's server prefix
command-prefix-invalid = Invalid prefix, prefix can't be empty
command-prefix-success = Prefix updated successfully. Prefix: `{ $prefix }`

# Banner Command
command-banner = Check user banner

# Help Command 
command-help = display bot or command help

# Ping Command
command-ping = Check API and Bot average response time

# Snipe Command
command-snipe = Snipe deleted messages
command-snipe-invalid-position = No deleted message found at the specified position.

# Ban Command
command-ban = Ban one or more members
command-ban-success = Banned { $user } from the server
command-ban-fail = An error happened when trying to ban { $user } from the server.
command-ban-admin = Cannot ban { $user } with Admin permission

# Unban Command
command-unban = unban a user
command-unban-success = Unbanned user { $user }
command-unban-fail = An error happened when trying to unban { $user }
command-unban-notfound = User { $user } is not banned

# Kick Command
command-kick = Kick one or more members
command-kick-success = Kicked { $user } out of the server
command-kick-fail = An error occurred when trying to kick { $user } from the server
command-kick-admin = Cannot kick { $user } with Admin permission

# Timeout Command
command-timeout = Timeout one or more members
command-timeout-success = Timed out { $user } for { $duration } minutes
command-timeout-fail = An error happened when trying to timeout { $user } for { $duration } minutes
command-timeout-admin = Cannot timeout { $user } with Admin permission

# Untimeout Command
command-untimeout = Remove timeout from one or more members
command-untimeout-success = Removed timeout from { $user }.
command-untimeout-fail = An error happened when trying to remove timeout from { $user }.
command-untimeout-notfound = { $user } is not currently timed out.

# Role Command
command-role = Add or remove a specified role from one or more members
command-role-add-success = Added role { $role } to { $user } successfully
command-role-add-failed = Failed to add role { $role } to { $user }. Error: { $err }
command-role-remove-success = Removed role { $role } from { $user } successfully
command-role-remove-failed = Failed to remove role { $role } from { $user }. Error: { $err }
command-role-no-perm = Bot does not have permission to manage role { $role }

# AFK
command-afk = set AFK status
command-afk-success = { $user }, your AFK status has been updated. 
afk-notification = [Server { $server }] AFK Notification: { $user } is back.
afk-is-afk = { $user }, `@{ $afk_name }` is AFK { $since } { $message }
afk-notifyme = Notify me
afk-is-back = { $user } is no longer afk
afk-notify-added = You will be notified when this user is back
afk-notfound = This user is no longer AFK

# VOICE & MUSIC
music-note = This feature only accepts Youtube, Soundcloud, and Spotify. Search results will get the first video from youtube search and add it to the queue
music-not-same-channel = You need to be in the same voice channel as the bot to use this command
music-nowplaying = Now playing
music-not-playing = I'm not playing any music
music-no-voice = I'm not in any voice channel
music-user-novoice = You need to be in a voice channel to use the command
music-cannot-connect = I can't connect to your channel
music-error-track = An error happened while playing this track
music-duration = Duration
music-position-inqueue = Position in queue
music-content-credits = Credits
music-content-credits-soundcloud = Listen on Soundcloud
music-content-credits-youtube = Listen on Youtube
music-content-credits-spotify = Listen on Spotify
music-content-creator = Content creator
music-playlist-fetch-error = Error getting tracks from playlist, please check the link or the video/playlist availablity.
music-loading-url = Loading track(s)
music-playlist-found = Spotify Playlist

# Join Command
command-join = Ask the bot to join a voice channel
command-join-nochannel = You need to be in a voice channel to use the command
command-join-joined = Join { $channel }
command-join-failed = Failed to join { $channel }! Reason: { $err }

# Leave Command
command-leave = Nicely ask the bot to leave the channel
command-leave-left = Left the voice channel
command-leave-failed = Failed to leave the voice channel

# Loop Command
command-loop = Loop music
command-loop-track = Looping current track
command-loop-track-failed = Failed to loop current track
command-loop-queue = Looping entire queue
command-loop-invalid = Invalid loop type. accepts: current/one/all/queue

# Pause Command
command-pause = Pause the current playing track
command-pause-paused = Paused the track
command-pause-unpaused = Unpaused the track

# Play Command
command-play = Play music
command-play-invalid-url = Please provide a valid URL or search query to play
command-play-added-tracks = Added { $count } tracks to the queue
command-play-added-track = Added track

# Queue Command
music-queue-empty = Queue is empty
music-queue-title = Queue: { $count } songs
music-queue-prev = Prev Page
music-queue-next = Next Page

# Resume Command
command-resume = Resume the paused track
command-resume-success = Resumed the track
command-resume-failed = Failed to resume the track
command-resume-notpaused = Music player is not paused

# Skip Command
command-skip = Skip the current track
command-skip-skipped = { $title } has been skipped
command-skip-author = Skipped by @{ $username }
command-skip-requested-by = Track was requested by
command-skip-no-metadata = Skipped a track, no skipped track info found
command-skip-failed = Failed to skip current track

# Skip to
command-skipto = Skip to the track at a position
command-skipto-nopos = Please provide a valid position to skip to
command-skipto-invalid = Invalid position: only { $count } tracks in queue
command-skipto-success = Skipped to track at position { $position }

# Auto response
command-autores = Manage bot's auto-responses
autores-existed = Auto-response with trigger `{ $trigger } already existed.
autores-created = Auto-response created. Trigger: `{ $trigger }`
autores-create-failed = Failed to create auto-response with trigger `{ $trigger }`
autores-deleted = Auto-response deleted. Trigger: `{ $trigger }`
autores-delete-failed = Failed to delete auto-response with trigger `{ $trigger }`
autores-notfound = I can't find auto-response with trigger `{ $trigger }`
autores-updated = Updated auto-response with trigger `{ $trigger }`
autores-update-failed = Failed to update auto-response with trigger `{ $trigger }`