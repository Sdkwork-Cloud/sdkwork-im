/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';

enum ErrorCode {
  /// @brief Token 无效。 <br>
  ///        进房时使用的 Token 无效或过期失效。需要用户重新获取 Token，并调用 `updateToken` 方法更新 Token。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_INVALID_TOKEN(-1000),

  /// @brief 加入房间错误。 <br>
  /// 进房时发生未知错误导致加入房间失败。需要用户重新加入房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_JOIN_ROOM(-1001),

  /// @brief 没有发布音视频流权限。 <br>
  ///        用户在所在房间中发布音视频流失败，失败原因为用户没有发布流的权限。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  ERROR_CODE_NO_PUBLISH_PERMISSION(-1002),

  /// @brief 没有订阅音视频流权限。 <br>
  ///        用户订阅所在房间中的音视频流失败，失败原因为用户没有订阅流的权限。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  ERROR_CODE_NO_SUBSCRIBE_PERMISSION(-1003),

  /// @brief 相同用户 ID 的用户加入本房间，当前用户被踢出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_DUPLICATE_LOGIN(-1004),

  /// @brief App ID 参数异常。 <br>
  ///        创建引擎时传入的 App ID 参数为空。
  ///
  ERROR_CODE_APP_ID_NULL(-1005),

  /// @brief 服务端调用 OpenAPI 将当前用户踢出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_KICKED_OUT(-1006),

  /// @brief 当调用 `createRoom` ，如果 roomId 非法，会返回 null，并抛出该错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_ROOM_ID_ILLEGAL(-1007),

  /// @brief Token 过期。调用 `joinRoom` 使用新的 Token 重新加入房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_TOKEN_EXPIRED(-1009),

  /// @brief 调用 `updateToken` 传入的 Token 无效。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_UPDATE_TOKEN_WITH_INVALID_TOKEN(-1010),

  /// @brief 服务端调用 OpenAPI 解散房间，所有用户被移出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_ROOM_DISMISS(-1011),

  /// @hidden internal use only
  /// @brief 加入房间错误。 <br>
  ///        调用 `joinRoom` 方法时, LICENSE 计费账号未使用 LICENSE_AUTHENTICATE SDK，加入房间错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_JOIN_ROOM_WITHOUT_LICENSE_AUTHENTICATE_SDK(-1012),

  /// @brief 通话回路检测已经存在同样 roomId 的房间了。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_ROOM_ALREADY_EXIST(-1013),

  /// @brief 加入多个房间时使用了不同的 uid。 <br>
  ///        同一个引擎实例中，用户需使用同一个 uid 加入不同的房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_USER_ID_DIFFERENT(-1014),

  /// @hidden internal use only
  /// @brief 服务端 license 过期，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_SERVER_LICENSE_EXPIRE(-1017),

  /// @hidden internal use only
  /// @brief 超过服务端 license 许可的并发量上限，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_EXCEEDS_THE_UPPER_LIMIT(-1018),

  /// @hidden internal use only
  /// @brief license 参数错误，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_LICENSE_PARAMETER_ERROR(-1019),

  /// @hidden internal use only
  /// @brief license 证书路径错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_LICENSE_FILE_PATH_ERROR(-1020),

  /// @hidden internal use only
  /// @brief license 证书不合法。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_LICENSE_ILLEGAL(-1021),

  /// @hidden internal use only
  /// @brief license 证书已经过期，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_LICENSE_EXPIRED(-1022),

  /// @hidden internal use only
  /// @brief license 证书内容不匹配。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_LICENSE_INFORMATION_NOT_MATCH(-1023),

  /// @hidden internal use only
  /// @brief license 当前证书与缓存证书不匹配。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_LICENSE_NOT_MATCH_WITH_CACHE(-1024),

  /// @brief 房间被封禁。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_JOIN_ROOM_ROOM_FORBIDDEN(-1025),

  /// @brief 用户被封禁。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_JOIN_ROOM_USER_FORBIDDEN(-1026),

  /// @hidden internal use only
  /// @brief license 计费方法没有加载成功。可能是因为 license 相关插件未正确集成。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ERROR_CODE_JOIN_ROOM_LICENSE_FUNCTION_NOT_FOUND(-1027),

  /// @brief 订阅音视频流失败，订阅音视频流总数超过上限。 <br>
  ///        游戏场景下为了保证音视频通话的性能和质量，服务器会限制用户订阅的音视频流的总数。当用户订阅的音视频流总数已达上限时，继续订阅更多流时会失败，同时用户会收到此错误通知。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  ERROR_CODE_OVER_SUBSCRIBE_LIMIT(-1070),

  /// @hidden for internal use only
  ///
  ERROR_CODE_LOAD_SO_LIB(-1072),

  /// @brief 发布流失败，发布流总数超过上限。 <br>
  ///        RTC 系统会限制单个房间内发布的总流数，总流数包括视频流、音频流和屏幕流。如果房间内发布流数已达上限时，本地用户再向房间中发布流时会失败，同时会收到此错误通知。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  ERROR_CODE_OVER_STREAM_PUBLISH_LIMIT(-1080),

  /// @brief 服务端异常状态导致退出房间。 通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。 <br>
  ///        SDK与信令服务器断开，并不再自动重连，可联系技术支持。  <br>
  ///
  ERROR_CODE_ABNORMAL_SERVER_STATUS(-1084),

  /// @hidden for internal use only
  /// @brief 在一路流推多房间的场景下，在至少有两个房间在发布同一路流时，其中一个房间取消发布失败，此时需要业务方重试或者由业务方通知用户重试取消发布。
  ///
  ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED(-1085),

  /// @hidden for internal use only
  /// @brief 指定服务区域时传入错误参数。
  ///
  ERROR_CODE_WRONG_AREA_CODE(-1086),

  /// @deprecated since 3.52, use ERROR_CODE_OVER_STREAM_PUBLISH_LIMIT(-1080) instead
  /// @brief 发布屏幕流失败，发布流总数超过上限。 <br>
  ///        RTC 系统会限制单个房间内发布的总流数，总流数包括视频流、音频流和屏幕流。如果房间内发布流数已达上限时，本地用户再向房间中发布流时会失败，同时会收到此错误通知。
  ///
  ERROR_CODE_OVER_SCREEN_PUBLISH_LIMIT(-1081),

  /// @deprecated since 3.52, use ERROR_CODE_OVER_STREAM_PUBLISH_LIMIT(-1080) instead
  /// @brief 发布视频流总数超过上限。 <br>
  ///        RTC 系统会限制单个房间内发布的视频流数。如果房间内发布视频流数已达上限时，本地用户再向房间中发布视频流时会失败，同时会收到此错误通知。
  ///
  ERROR_CODE_OVER_VIDEO_PUBLISH_LIMIT(-1082),

  /// @deprecated since 3.60, use INVALID_UID_REPEATED(0) carried by onAVSyncEvent{@link #IRTCRoomEventHandler#onAVSyncEvent} instead.
  /// @brief 音视频同步失败。 <br>
  ///        当前音频源已与其他视频源关联同步关系。 <br>
  ///        单个音频源不支持与多个视频源同时同步。 <br>
  ///        通过 onStreamStateChanged 回调。
  ///
  ERROR_CODE_INVALID_AUDIO_SYNC_USERID_REPEATED(-1083);

  final dynamic $value;
  const ErrorCode([this.$value]);
}

enum TranscodingError {
  /// @brief 推流成功。
  ///
  TRANSCODING_ERROR_OK(0),

  /// @brief 推流参数错误。你必须更新合流参数并重试。
  ///
  TRANSCODING_ERROR_INVALID_ARGUMENT(1),

  /// @brief 和 RTC 服务端建立连接失败。会自动重连
  ///
  TRANSCODING_ERROR_SUBSCRIBE(2),

  /// @brief 合流服务中间过程存在错误，建议重试。
  ///
  TRANSCODING_ERROR_PROCESSING(3),

  /// @brief 推流失败，可以等待服务端重新推流。
  ///
  TRANSCODING_ERROR_PUBLISH(4);

  final dynamic $value;
  const TranscodingError([this.$value]);
}

enum RoomMessageSendResult {
  /// @brief 发送成功
  ///
  ROOM_MESSAGE_SEND_RESULT_SUCCESS(200),

  /// @brief 超过 QPS 限制
  ///
  ROOM_MESSAGE_SEND_RESULT_EXCEED_QPS(5),

  /// @brief 发送失败。消息发送方没有加入房间。
  ///
  ROOM_MESSAGE_SEND_RESULT_NOT_JOIN(100),

  /// @brief 发送失败。连接未完成初始化。
  ///
  ROOM_MESSAGE_SEND_RESULT_INIT(101),

  /// @brief 发送失败。没有可用的数据传输通道连接
  ///
  ROOM_MESSAGE_SEND_RESULT_NO_CONNECTION(102),

  /// @brief 发送失败。消息超过最大长度 64KB。
  ///
  ROOM_MESSAGE_SEND_RESULT_EXCEED_MAX_LENGTH(103),

  /// @brief 发送失败。未知错误
  ///
  ROOM_MESSAGE_SEND_RESULT_UNKNOWN(1000);

  final dynamic $value;
  const RoomMessageSendResult([this.$value]);
}

enum UserMessageSendResult {
  /// @brief 发送消息成功。
  ///
  USER_MESSAGE_SEND_RESULT_SUCCESS(0),

  /// @brief 消息发送失败。发送超时。
  ///
  USER_MESSAGE_SEND_RESULT_TIMEOUT(1),

  /// @brief 消息发送失败。连接断开，消息未发出。
  ///
  USER_MESSAGE_SEND_RESULT_BROKEN(2),

  /// @brief 消息发送失败。找不到接收方。
  ///
  USER_MESSAGE_SEND_RESULT_NO_RECEIVER(3),

  /// @brief 消息发送失败。远端用户没有登录或进房。
  ///
  USER_MESSAGE_SEND_RESULT_NO_RELAY_PATH(4),

  /// @brief 消息发送失败。超过 QPS 限制。
  ///
  USER_MESSAGE_SEND_RESULT_EXCEED_QPS(5),

  /// @brief 消息发送失败。消息发送方没有加入房间。
  ///
  USER_MESSAGE_SEND_RESULT_NOT_JOIN(100),

  /// @brief 消息发送失败。连接未完成初始化。
  ///
  USER_MESSAGE_SEND_RESULT_INIT(101),

  /// @brief 消息发送失败。没有可用的数据传输通道连接。
  ///
  USER_MESSAGE_SEND_RESULT_NO_CONNECTION(102),

  /// @brief 消息发送失败。消息超过最大长度 (64 KB)。
  ///
  USER_MESSAGE_SEND_RESULT_EXCEED_MAX_LENGTH(103),

  /// @brief 消息发送失败。接收方用户 ID 为空。
  ///
  USER_MESSAGE_SEND_RESULT_EMPTY_USER(104),

  /// @brief 消息发送失败。房间外或应用服务器消息发送方没有登录。
  ///
  USER_MESSAGE_SEND_RESULT_NOT_LOGIN(105),

  /// @brief 消息发送失败。发送消息给业务方服务器之前没有设置参数。
  ///
  USER_MESSAGE_SEND_RESULT_SERVER_PARAMS_NOT_SET(106),

  /// @brief 消息发送失败。未知错误。
  ///
  USER_MESSAGE_SEND_RESULT_UNKNOWN(1000),

  USER_MESSAGE_SEND_RESULT_E2BS_SEND_FAILED(17),

  USER_MESSAGE_SEND_RESULT_E2BS_RETURN_FAILED(18);

  final dynamic $value;
  const UserMessageSendResult([this.$value]);
}

enum MixedStreamTaskErrorCode {
  /// @brief 推流成功。
  ///
  OK(0),

  /// @hidden currently not available
  /// @brief 预留错误码，未启用
  ///
  BASE(1090),

  /// @brief 任务处理超时，请检查网络状态并重试。
  ///
  TIMEOUT(1091),

  /// @brief 服务端检测到错误的推流参数。
  ///
  INVALID_PARAM_BY_SERVER(1092),

  /// @brief 对流的订阅超时
  ///
  SUB_TIMEOUT_BY_SERVER(1093),

  /// @brief 合流服务端内部错误。
  ///
  INVALID_STATE_BY_SERVER(1094),

  /// @brief 合流服务端推 CDN 失败。
  ///
  AUTHENTICATION_BY_CDN(1095),

  /// @brief 服务端未知错误。
  ///
  UNKNOWN_BY_SERVER(1096),

  /// @brief 服务端接收信令超时，请检查网络状态并重试。
  ///
  SIGNAL_REQUEST_TIMEOUT(1097),

  /// @brief 图片合流失败。
  ///
  MIX_IMAGE_FAIL(1098),

  /// @hidden internal use only
  /// @brief 缓存未同步。
  ///
  STREAM_SYNC_WORSE(1099),

  /// @brief 发布 WTN 流失败
  ///
  PUSH_WTN_FAILED(1195),

  /// @hidden for internal use only
  ///
  MAX(1199);

  final dynamic $value;
  const MixedStreamTaskErrorCode([this.$value]);
}

enum SubtitleErrorCode {
  /// @brief 客户端无法识别云端媒体处理发送的错误码。请联系技术支持。
  ///
  SUBTITLE_ERROR_CODE_UNKNOW(-1),

  /// @brief 字幕已开启。
  ///
  SUBTITLE_ERROR_CODE_SUCCESS(0),

  /// @brief 云端媒体处理内部出现错误，请联系技术支持。
  ///
  SUBTITLE_ERROR_CODE_POST_PROCESS_ERROR(1),

  /// @brief 第三方服务连接失败，请联系技术支持。
  ///
  SUBTITLE_ERROR_CODE_ASR_CONNECTION_ERROR(2),

  /// @brief 第三方服务内部出现错误，请联系技术支持。
  ///
  SUBTITLE_ERROR_CODE_ASR_SERVICE_ERROR(3),

  /// @brief 未进房导致调用`startSubtitle`失败。请加入房间后再调用此方法。
  ///
  SUBTITLE_ERROR_CODE_BEFORE_JOIN_ROOM(4),

  /// @brief 字幕已开启，无需重复调用 `startSubtitle`。
  ///
  SUBTITLE_ERROR_CODE_ALREADY_ON(5),

  /// @brief 你选择的目标语言目前暂不支持。
  ///
  SUBTITLE_ERROR_CODE_UNSUPPORTED_LANGUAGE(6),

  /// @brief 云端媒体处理超时未响应，请联系技术支持。
  ///
  SUBTITLE_ERROR_CODE_POST_PROCESS_TIMEOUT(7);

  final dynamic $value;
  const SubtitleErrorCode([this.$value]);
}

enum SingleStreamTaskEvent {
  /// @hidden for internal use only
  ///
  BASE(0),

  /// @brief 任务发起成功。
  ///
  START_SUCCESS(1),

  /// @brief 任务发起失败。
  ///
  START_FAILED(2),

  /// @brief 任务停止。
  ///
  STOP_SUCCESS(3),

  /// @brief 结束任务失败。
  ///
  STOP_FAILED(4),

  /// @brief Warning 事件。
  ///
  WARNING(5);

  final dynamic $value;
  const SingleStreamTaskEvent([this.$value]);
}

enum KTVErrorCode {
  /// @brief 成功。
  ///
  OK(0),

  /// @brief AppID 异常。
  ///
  APPID_INVALID(-3000),

  /// @brief 非法参数，传入的参数不正确。
  ///
  PARAS_INVALID(-3001),

  /// @brief 获取歌曲资源失败。
  ///
  GET_MUSIC_FAILED(-3002),

  /// @brief 获取歌词失败。
  ///
  GET_LYRIC_FAILED(-3003),

  /// @brief 歌曲下架。
  ///
  MUSIC_TAKEDOWN(-3004),

  /// @brief 歌曲文件下载失败。
  ///
  MUSIC_DOWNLOAD(-3005),

  /// @brief MIDI 文件下载失败。
  ///
  MIDI_DOWNLOAD_FAILED(-3006),

  /// @brief 系统繁忙。
  ///
  SYSTEM_BUSY(-3007),

  /// @brief 网络异常。
  ///
  NETWORK(-3008),

  /// @brief KTV 功能未加入房间。
  ///
  NOT_JOIN_ROOM(-3009),

  /// @brief 解析数据失败。
  ///
  PARSE_DATA(-3010),

  /// @brief 已在下载中。
  ///
  DOWNLOADING(-3012),

  /// @brief 内部错误，联系技术支持人员。
  ///
  INTERNAL_DOMAIN(-3013),

  /// @brief 下载失败，磁盘空间不足。清除缓存后重试。
  ///
  INSUFFICIENT_DISK_SPACE(-3014),

  /// @brief 下载失败，音乐文件解密失败，联系技术支持人员。
  ///
  MUSIC_DECRYPTION_FAILED(-3015),

  /// @brief 下载失败，音乐文件重命名失败，请重试。
  ///
  FILE_RENAME_FAILED(-3016),

  /// @brief 下载失败，下载超时，请重试。
  ///
  DOWNLOAD_TIMEOUT(-3017),

  /// @brief 清除缓存失败，可能原因是文件被占用或者系统异常，请重试。
  ///
  CLEAR_CACHE_FAILED(-3018),

  /// @brief 取消下载。
  ///
  DOWNLOAD_CANCELED(-3019),

  /// @hidden
  /// @deprecated 从 353 开始。
  /// @brief 下载失败。
  ///
  DOWNLOAD(-3011);

  final dynamic $value;
  const KTVErrorCode([this.$value]);
}

enum SingleStreamTaskErrorCode {
  /// @brief 推流成功。
  ///
  OK(0),

  /// @hidden currently not available
  /// @brief 预留错误码，未启用
  ///
  BASE(1090),

  /// @brief 服务端合流错误
  ///
  UNKNOWN_BY_SERVER(1091),

  /// @brief 任务处理超时，请检查网络状态并重试。
  ///
  SIGNAL_REQUEST_TIMEOUT(1092),

  /// @brief 服务端检测任务参数不合法
  ///
  INVALID_PARAM_BY_SERVER(1093),

  /// @brief 转推任务在目标房间的用户ID被踢出目标房间
  ///
  REMOTE_KICKED(1094),

  /// @brief 转推任务加入目标房间失败
  ///
  JOIN_DEST_ROOM_FAIED(1095),

  /// @brief 转推任务在源房间拉流超时
  ///
  RECEIVE_SRC_STREAM_TIMEOUT(1096),

  /// @brief 音视频编码转推任务不支持
  ///
  NOT_SURPORT_CODEC(1097);

  final dynamic $value;
  const SingleStreamTaskErrorCode([this.$value]);
}

enum PublicStreamErrorCode {
  /// @brief 发布或订阅成功。
  ///
  ERROR_CODE_SUCCESS(0),

  /// @brief WTN 流的参数异常，请修改参数后重试。
  ///
  ERROR_CODE_PUSH_PARAM_ERROR(1191),

  /// @brief 服务端状态异常，将自动重试。
  ///
  ERROR_CODE_PUSH_STATE_ERROR(1192),

  /// @brief 内部错误，不可恢复，请重试。
  ///
  ERROR_CODE_PUSH_INTERNAL_ERROR(1193),

  /// @brief 发布失败，将自动重试，请关注重试结果。
  ///
  ERROR_CODE_PUSH_ERROR(1195),

  /// @brief 发布失败，10 s 后会重试，重试 3 次后自动停止。
  ///
  ERROR_CODE_PUSH_TIMEOUT(1196),

  /// @brief 订阅失败，发布端未开始发布流。
  ///
  ERROR_CODE_PULL_NO_PUSH_STREAM(1300);

  final dynamic $value;
  const PublicStreamErrorCode([this.$value]);
}

enum WarningCode {
  /// @brief 发布音视频流失败。 <br>
  ///        当你在所在房间中发布音视频流时，由于服务器错误导致发布失败。SDK 会自动重试发布。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  WARNING_CODE_PUBLISH_STREAM_FAILED(-2002),

  /// @brief 订阅音视频流失败。 <br>
  ///        当前房间中找不到订阅的音视频流导致订阅失败。SDK 会自动重试订阅，若仍订阅失败则建议你退出重试。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  WARNING_CODE_SUBSCRIBE_STREAM_FAILED404(-2003),

  /// @brief 订阅音视频流失败。 <br>
  ///        当你订阅所在房间中的音视频流时，由于服务器错误导致订阅失败。SDK 会自动重试订阅。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  WARNING_CODE_SUBSCRIBE_STREAM_FAILED5XX(-2004),

  /// @hidden currently not available
  /// @brief 函数调用顺序错误。
  ///
  WARNING_CODE_INVOKE_ERROR(-2005),

  /// @hidden for internal use only
  /// @brief 调度异常，服务器返回的媒体服务器地址不可用。
  ///
  WARNING_CODE_INVALID_EXPECT_MEDIA_SERVER_ADDRESS(-2007),

  /// @brief 当调用 `setUserVisibility` 将自身可见性设置为 false 后，再尝试发布流会触发此警告。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  WARNING_CODE_PUBLISH_STREAM_FORBIDEN(-2009),

  /// @brief 发送自定义广播消息失败, 当前你未在房间中。
  ///
  WARNING_CODE_SEND_CUSTOM_MESSAGE(-2011),

  /// @brief 当房间内人数超过 500 人时，停止向房间内已有用户发送 `onUserJoined` 和 `onUserLeave` 回调，并通过广播提示房间内所有用户。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  WARNING_CODE_RECEIVE_USER_NOTIFY_STOP(-2013),

  /// @brief 用户已经在其他房间发布过流，或者用户正在发布 WTN 流。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  WARNING_CODE_USER_IN_PUBLISH(-2014),

  /// @brief 当前正在进行回路测试，该接口调用无效
  ///
  WARNING_CODE_IN_ECHO_TEST_MODE(-2017),

  /// @brief 摄像头权限异常，当前应用没有获取摄像头权限。
  ///
  WARNING_CODE_NO_CAMERA_PERMISSION(-5001),

  /// @brief 不支持在 publishScreenAudio 之后，调用 setScreenAudioSourceType{@link #RTCEngine#setScreenAudioSourceType} 设置屏幕音频采集类型
  ///
  WARNING_CODE_SET_SCREEN_AUDIO_SOURCE_TYPE_FAILED(-5009),

  /// @brief 不支持在 publishScreenAudio 之后，调用 setScreenAudioStreamIndex 设置屏幕音频共享发布类型
  ///
  WARNING_CODE_SET_SCREEN_STREAM_INDEX_FAILED(-5010),

  /// @brief 设置语音音高不合法
  ///
  WARNING_CODE_SET_SCREEN_STREAM_INVALID_VOICE_PITCH(-5011),

  /// @brief 外部音频源新旧接口混用
  ///
  WARNING_CODE_INVALID_CALL_FOR_EXT_AUDIO(-5013),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) SDK 鉴权失效。联系技术支持人员。
  ///
  WARNING_CODE_INVALID_SAMI_APPKEY_OR_TOKEN(-7002),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 资源加载失败。传入正确的 DAT 路径，或联系技术支持人员。
  ///
  WARNING_CODE_INVALID_RESOURCE_PATH(-7003),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 库加载失败。使用正确的库，或联系技术支持人员。
  ///
  WARNING_CODE_LOAD_SAMI_LIBRARY_FAILED(-7004),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 不支持此音效。联系技术支持人员。
  ///
  WARNING_CODE_INVALID_SAMI_EFFECT_TYPE(-7005),

  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 获取房间信息失败警告
  /// @note SDK 获取房间信息失败（包含超时，返回非 200 的错误码），每隔两秒重试一次。 <br>
  ///        连续失败 5 次后，报该 warning，并继续重试。 <br>
  ///        建议提示用户：进入房间失败，请稍后再试
  ///
  WARNING_CODE_GET_ROOM_FAILED(-2000),

  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 自动订阅模式未关闭时，尝试开启手动订阅模式会触发此警告。 <br>
  ///        你需在进房前关闭自动订阅模式，再手动订阅音视频流。
  ///
  WARNING_CODE_SUBSCRIBE_STREAM_FORBIDEN(-2010),

  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 已存在同样 roomId 的房间。
  ///
  WARNING_CODE_ROOM_ID_ALREADY_EXIST(-2015),

  /// @brief 麦克风权限异常，当前应用没有获取麦克风权限。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_NOPERMISSION instead.
  ///
  WARNING_CODE_NO_MICROPHONE_PERMISSION(-5002),

  /// @brief 音频采集设备启动失败。 <br>
  ///        启动音频采集设备失败，当前设备可能被其他应用占用。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICEFAILURE instead.
  ///
  WARNING_CODE_RECODING_DEVICE_START_FAILED(-5003),

  /// @brief 音频播放设备启动失败警告。 <br>
  ///        可能由于系统资源不足，或参数错误。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICEFAILURE instead.
  ///
  WARNING_CODE_PLAYOUT_DEVICE_START_FAILED(-5004),

  /// @brief 无可用音频采集设备。 <br>
  ///        启动音频采集设备失败，请插入可用的音频采集设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICENOTFOUND instead.
  ///
  WARNING_CODE_NO_RECORDING_DEVICE(-5005),

  /// @brief 无可用音频播放设备。 <br>
  ///        启动音频播放设备失败，请插入可用的音频播放设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICENOTFOUND instead.
  ///
  WARNING_CODE_NO_PLAYOUT_DEVICE(-5006),

  /// @brief 当前音频设备没有采集到有效的声音数据，请检查更换音频采集设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceWarning{@link #MediaDeviceWarning}.MEDIA_DEVICE_WARNING_CAPTURE_SILENCE instead.
  ///
  WARNING_CODE_RECORDING_SILENCE(-5007),

  /// @brief 媒体设备误操作警告。 <br>
  ///        使用自定义采集时，不可调用内部采集开关，调用时将触发此警告。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceWarning{@link #MediaDeviceWarning}.MEDIA_DEVICE_WARNING_OPERATION_DENIED instead.
  ///
  WARNING_CODE_MEDIA_DEVICE_OPERATION_DENIED(-5008);

  final dynamic $value;
  const WarningCode([this.$value]);
}

enum LoginErrorCode {
  /// @brief 调用 `login` 方法登录成功。
  ///
  LOGIN_ERROR_CODE_SUCCESS(0),

  /// @brief 调用 `login` 方法时使用的 Token 无效或过期失效。需要用户重新获取 Token。
  ///
  LOGIN_ERROR_CODE_INVALID_TOKEN(-1000),

  /// @brief 登录错误。 <br>
  ///        调用 `login` 方法时发生未知错误导致登录失败，需要重新登录。
  ///
  LOGIN_ERROR_CODE_LOGIN_FAILED(-1001),

  /// @brief 调用 `login` 方法时传入的用户 ID 有问题。
  ///
  LOGIN_ERROR_CODE_INVALID_USER_ID(-1002),

  /// @brief 调用 `login` 登录时服务器错误。
  ///
  LOGIN_ERROR_CODE_SERVER_ERROR(-1003);

  final dynamic $value;
  const LoginErrorCode([this.$value]);
}

enum AudioRecordingErrorCode {
  /// @brief 录制正常
  ///
  AUDIO_RECORDING_ERROR_CODE_OK(0),

  /// @brief 没有文件写权限
  ///
  AUDIO_RECORDING_ERROR_CODE_NO_PERMISSION(-1),

  /// @brief 没有进入房间
  ///
  AUDIO_RECORDING_ERROR_CODE_NOT_IN_ROOM(-2),

  /// @brief 录制已经开始
  ///
  AUDIO_RECORDING_ERROR_CODE_ALREADY_STARTED(-3),

  /// @brief 录制还未开始
  ///
  AUDIO_RECORDING_ERROR_CODE_NOT_STARTED(-4),

  /// @brief 录制失败。文件格式不支持。
  ///
  AUDIO_RECORDING_ERROR_CODE_NOT_SUPPORT(-5),

  /// @brief 其他异常
  ///
  AUDIO_RECORDING_ERROR_CODE_OTHER(-6);

  final dynamic $value;
  const AudioRecordingErrorCode([this.$value]);
}
