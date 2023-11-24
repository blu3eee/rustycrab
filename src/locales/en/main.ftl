permission-missing = Missing permission(s) to use the command

# Common Errors
commonError-noUserInfo = Can't find { $user } information.
commonError-noBotGuildConfig = Not Guild Configuration found.
commonError-noTag = Tag at least one person.
commonError-tag1 = Tag one person

# Bot Owner Commands
command-botowner = Manage BotOwner permission
command-botowner-addSuccess = Added BotOwner permission for { $user }
command-botowner-removeSuccess = Removed { $user } BotOwner permission
command-botowner-alreadyOwner = { $user } already has BotOwner permission
command-botowner-notOwner = { $user } doesn't have BotOwner permission.
command-botowner-listEmpty = empty...
command-botowner-listTitle = Users with BotOwner permission

# Bot Admin Commands
command-botadmin = Manage BotAdmin permission
command-botadmin-addSuccess = Added BotAdmin permission for { $user }
command-botadmin-removeSuccess = Removed { $user } BotAdmin permission
command-botadmin-alreadyAdmin = { $user } already has BotAdmin permission
command-botadmin-notAdmin = { $user } doesn't have BotAdmin permission.
command-botadmin-listEmpty = empty...
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
command-ban-success = Banned { $user } from the server.
command-ban-fail = An error happened when trying to ban { $user } from the server.
command-ban-admin = Cannot ban { $user } with Admin permission.

# Unban Command
command-unban = unban a user
command-unban-success = Unbanned user { $user }
command-unban-fail = An error happened when trying to unban { $user }
command-unban-notfound = User { $user } is not banned

# Kick Command
command-kick = Kick one or more members
command-kick-success = Kicked { $user } out of the server.
command-kick-fail = An error occurred when trying to kick { $user } from the server.
command-kick-admin = Cannot kick { $user } with Admin permission.

# Timeout Command
command-timeout = Timeout one or more members
command-timeout-success = Timed out { $user } for { $duration } minutes.
command-timeout-fail = An error happened when trying to timeout { $user } for { $duration } minutes.
command-timeout-admin = Cannot timeout { $user } with Admin permission.

# Untimeout Command
command-untimeout = Remove timeout from one or more members
command-untimeout-success = Removed timeout from { $user }.
command-untimeout-fail = An error happened when trying to remove timeout from { $user }.
command-untimeout-notfound = { $user } is not currently timed out.

# Role Command
command-role = Add or remove a specified role from one or more members
command-role-add-success = Added role { $role } to { $user } successfully.
command-role-add-failed = Failed to add role { $role } to { $user }. Error: { $err }
command-role-remove-success = Removed role { $role } from { $user } successfully.
command-role-remove-failed = Failed to remove role { $role } from { $user }. Error: { $err }
command-role-no-perm = Bot does not have permission to manage role { $role }.

# AFK
command-afk = set AFK status
command-afk-success = { $user }, your AFK status has been updated. 
afk-notification = [Server { $server }] AFK Notification: { $user } is back.
afk-is-afk = { $user }, `@{ $afk_name }` is AFK { $since } { $message }
afk-notifyme = Notify me
afk-is-back = { $user } is no longer afk
afk-notify-added = You will be notified when this user is back
afk-notfound = This user is no longer AFK