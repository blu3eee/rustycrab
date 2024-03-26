Permission-missing = Thiếu quyền sử dụng lệnh

# Lỗi thông thường
commonError-noUserInfo = Không thể tìm thấy thông tin { $user }
commonError-noBotGuildConfig = Không tìm thấy settings của server
commonError-noTag = Tag ít nhất một người
commonError-tag1 = Tag một người

# Utilities
interaction-denied = Bạn không đủ quyền.
requested-user = Được yêu cầu bởi @{ $username }
invalid-image-url = Link/ảnh không hợp lệ

# Commands 
command-error = Đã có lỗi xảy ra khi xử lí lệnh.
command-guildonly = Lệnh chỉ được sử dụng ở server.

# Lệnh của chủ sở hữu Bot
command-botowner = Quản lý quyền BotOwner
command-botowner-addSuccess = Đã thêm quyền BotOwner cho { $user }
command-botowner-removeSuccess = Đã xóa quyền  BotOwner của { $user }
command-botowner-alreadyOwner = { $user } đã có quyền BotOwner
command-botowner-notOwner = { $user } không có quyền BotOwner.
command-botowner-listEmpty = Trống...
command-botowner-listTitle = Người dùng có quyền BotOwner

# Lệnh quản trị Bot
command-botadmin = Quản lý quyền BotAdmin
command-botadmin-addSuccess = Đã thêm quyền BotAdmin cho { $user }
command-botadmin-removeSuccess = Đã xóa quyền BotAdmin của { $user }
command-botadmin-alreadyAdmin = { $user } đã có quyền BotAdmin
command-botadmin-notAdmin = { $user } không có quyền BotAdmin.
command-botadmin-listEmpty = Trống...
command-botadmin-listTitle = Người dùng có quyền BotAdmin

# Lệnh ngôn ngữ
command-language = Thay đổi ngôn ngữ của bot
command-language-invalidLocale = Ngôn ngữ không hợp lệ. Ngôn ngữ có sẵn: { $locales }
command-language-localeChanged = Ngôn ngữ của bot đã chuyển sang `{ $locale }`

# Lệnh Avatar
command-avatar = Kiểm tra hình đại diện của người dùng

# Lệnh toán
command-math = Tính các phép toán cơ bản
command-math-invalid = Phép học không hợp lệ

# Lệnh prefix
command-prefix = Thay đổi prefix máy chủ của bot
command-prefix-invalid = prefix không hợp lệ, prefix không được để trống
command-prefix-success = prefix được cập nhật thành công. Prefix: `{ $prefix }`
command-prefix-failed = thất bại khi cập nhật prefix
# Lệnh check banner
command-banner = Kiểm tra banner người dùng

# Lệnh giúp đỡ
command-help = hiển thị bot hoặc lệnh trợ giúp

# Lệnh ping
command-ping = Kiểm tra thời gian phản hồi trung bình của API và Bot

# Lệnh Snipe
command-snipe = Snipe các tin nhắn đã xóa
command-snipe-invalid-position = Không tìm thấy tin nhắn đã xóa tại vị trí được chỉ định.

# Lệnh Ban
command-ban = Ban một hoặc nhiều member
command-ban-success = Đã ban { $user } khỏi server.
command-ban-fail = Xảy ra lỗi khi ban { $user } khỏi server 
command-ban-admin = Không thể ban { $user } do người dùng có quyền admin.

# Lệnh Unban
command-unban = Unban người dùng
command-unban-success=Người dùng không bị ban { $user }
command-unban-failed = Đã xảy ra lỗi khi cố gắng unban { $user }
command-unban-notfound= Người dùng { $user } không bị ban

# Lệnh Kick
command-kick = Đá một hoặc nhiều thành viên
command-kick-success=Đã đá { $user } ra khỏi máy chủ.
command-kick-failed = Đã xảy ra lỗi khi cố gắng kick { $user } khỏi máy chủ.
command-kick-admin = Không thể kick người dùng { $user } với quyền Admin

# Lệnh Timeout
command-timeout = Mute một hoặc nhiều thành viên
command-timeout-success = Đã mute { $user } trong { $duration } giây.
command-timeout-fail = Đã xảy ra lỗi khi cố gắng mute { $user } trong { $duration } giây.
command-timeout-admin = Không thể mute người dùng { $user } với quyền Admin

# Lệnh Untimeout
command-untimeout = Unmute một hoặc nhiều thành viên
command-untimeout-success = Đã unmute { $user }.
command-untimeout-fail = Đã xảy ra lỗi khi cố unmute { $user }.
command-untimeout-notfound = { $user } hiện không bị mute.

# Role Command
command-role = Thêm hoặc bỏ role được chỉ định từ một hoặc nhiều thành viên
command-role-add-success = Đã thêm role { $role } cho { $user } thành công.
command-role-add-failed = Không thể thêm role { $role } cho { $user }. Lỗi: { $err }
command-role-remove-success = Đã bỏ role { $role } từ { $user } thành công.
command-role-remove-failed = Không thể bỏ role { $role } từ { $user }. Lỗi: { $err }
command-role-no-perm = Bot không có quyền quản lý role { $role }.

# AFK
command-afk = Set trạng thái AFK
command-afk-success = { $user }, trạng thái AFK của bạn đã được cập nhật.
afk-notification = [Server { $server }] Thông báo AFK: { $user } đã quay lại.
afk-is-afk = { $user }, `@{ $afk_name }` đang ở trạng thái AFK { $since } { $message }
afk-notifyme = Notify me
afk-is-back = { $user } không còn ở trạng thái AFK nữa
afk-notify-added = Bạn sẽ nhận được thông báo khi người dùng này trở lại
afk-notfound = Người dùng này không còn ở trạng thái AFK nữa.

# VOICE & MUSIC
music-note = Tính năng này chỉ chấp nhận URL từ SoundCloud, Youtube, Spotify, hoặc tìm kiếm trên Youtube. Kết quả tìm kiếm sẽ lấy video đầu tiên từ tìm kiếm Youtube và thêm vào hàng đợi
music-not-same-channel = Bạn cần ở trong cùng kênh giọng nói với bot để sử dụng lệnh này
music-nowplaying = Đang phát
music-not-playing = Tớ không phát nhạc nào
music-no-voice = Tớ không ở trong kênh giọng nói nào
music-user-novoice = Bạn cần ở trong kênh giọng nói để sử dụng lệnh
music-cannot-connect = Tớ không thể kết nối với kênh của bạn
music-error-track = Đã có lỗi xảy ra khi phát bài hát này
music-duration = Thời lượng
music-position-inqueue = Vị trí trong hàng đợi
music-content-credits = Credits
music-content-credits-soundcloud = Nghe trên Soundcloud
music-content-credits-youtube = Nghe trên Youtube
music-content-creator = Content creator
music-playlist-fetch-error = Lỗi khi lấy bài hát từ danh sách phát, vui lòng kiểm tra lại link hoặc chế độ riêng tư của video/danh sách phát.
music-loading-url = Đang tải bài hát

# Join Command
command-join = Yêu cầu bot tham gia kênh voice
command-join-nochannel = Bạn cần ở trong kênh voice để sử dụng lệnh
command-join-joined = Đã tham gia { $channel }
command-join-failed = Không thể tham gia { $channel }! Lỗi: { $err }

# Leave Command
command-leave = Yêu cầu bot rời kênh voice
command-leave-left = Đã rời kênh voice
command-leave-failed = Không thể rời kênh voice.

# Loop Command
command-loop = Lặp lại bài nhạc hoặc danh sách nhạc
command-loop-track = Đang lặp bài hát hiện tại
command-loop-track-failed = Không thể lặp bài hát hiện tại
command-loop-queue = Lặp toàn bộ hàng đợi
command-loop-invalid =  Invalid loop type. accepts: current/one/all/queue

# Pause Command
command-pause = Tạm dừng bài hát đang phát
command-pause-paused = Đã tạm dừng bài hát
command-pause-unpaused = Đã tiếp tục phát bài hát

# Play Command
command-play = Phát nhạc
command-play-invalid-url = Vui lòng cung cấp URL hoặc truy vấn tìm kiếm hợp lệ để phát
command-play-added-tracks = Đã thêm { $count } bài hát vào hàng đợi
command-play-added-track = Đã thêm bài hát

# Queue
music-queue-empty = Hàng đợi trống
music-queue-title = Hàng đợi: { $count } bài hát
music-queue-prev = Trang Trước
music-queue-next = Trang Sau

# Resume 
command-resume = Tiếp tục phát bài hát đã tạm dừng
command-resume-success = Đã tiếp tục phát bài hát
command-resume-failed = Không thể tiếp tục phát bài hát
command-resume-notpaused = Máy phát nhạc không bị tạm dừng

# Skip 
command-skip = Bỏ qua bài hát hiện tại
command-skip-skipped = { $title } đã bị bỏ qua
command-skip-author = Được bỏ qua bởi @{ $username }
command-skip-requested-by = Bài hát được yêu cầu bởi
command-skip-no-metadata = Đã bỏ qua một bài hát, không tìm thấy thông tin bài hát bị bỏ qua
command-skip-failed = Không thể bỏ qua bài hát hiện tại

# Skip to
command-skipto = Bỏ qua đến bài hát ở một vị trí cụ thể
command-skipto-nopos = Vui lòng cung cấp một vị trí hợp lệ để bỏ qua đến
command-skipto-invalid = Vị trí không hợp lệ: chỉ có { $count } bài hát trong hàng đợi
command-skipto-success = Đã bỏ qua đến bài hát ở vị trí { $position }

# Auto response
command-autores = Quản lí auto-responders
autores-existed = Auto-res với trigger `{ $trigger } đã existed.
autores-created = Auto-res vừa được tạo. Trigger: `{ $trigger }`
autores-create-failed = Lỗi khi tạo auto-res với trigger `{ $trigger }`
autores-deleted = Đã xoá auto-res với trigger: `{ $trigger }`
autores-delete-failed = Lỗi khi xoá auto-res với trigger `{ $trigger }`
autores-notfound = Không tìm thấy auto-res với trigger `{ $trigger }`
autores-updated = Đã cập nhật auto-res với trigger `{ $trigger }`
autores-update-failed = Lỗi khi cập nhật auto-res với trigger `{ $trigger }`
autores-limited = Server bạn đã đạt giới hạn 20 auto-res/server.