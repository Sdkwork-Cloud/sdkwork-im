/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:io';
import 'dart:async';
import 'dart:typed_data';
import '../android/index.dart' as $p_a;
import '../ios/index.dart' as $p_i;

/** {zh}
        * @detail errorcode
* @brief 回调错误码。 <br>
*        SDK 内部遇到不可恢复的错误时，会通过 `onError` 回调通知用户。
        */
enum ErrorCode {
  /// @brief Token 无效。 <br>
  ///        进房时使用的 Token 无效或过期失效。需要用户重新获取 Token，并调用 `updateToken` 方法更新 Token。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  invalidToken(0),

  /// @brief 加入房间错误。 <br>
  /// 进房时发生未知错误导致加入房间失败。需要用户重新加入房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoom(1),

  /// @brief 没有发布音视频流权限。 <br>
  ///        用户在所在房间中发布音视频流失败，失败原因为用户没有发布流的权限。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  noPublishPermission(2),

  /// @brief 没有订阅音视频流权限。 <br>
  ///        用户订阅所在房间中的音视频流失败，失败原因为用户没有订阅流的权限。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  noSubscribePermission(3),

  /// @brief 相同用户 ID 的用户加入本房间，当前用户被踢出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  duplicateLogin(4),

  /// @platform android
  /// @brief App ID 参数异常。 <br>
  ///        创建引擎时传入的 App ID 参数为空。
  ///
  appIdNull(5),

  /// @brief 服务端调用 OpenAPI 将当前用户踢出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  kickedOut(6),

  /// @brief 当调用 `createRoom` ，如果 roomId 非法，会返回 null，并抛出该错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  roomIdIllegal(7),

  /// @brief Token 过期。调用 `joinRoom` 使用新的 Token 重新加入房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  tokenExpired(8),

  /// @brief 调用 `updateToken` 传入的 Token 无效。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  updateTokenWithInvalidToken(9),

  /// @brief 服务端调用 OpenAPI 解散房间，所有用户被移出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  roomDismiss(10),

  /// @hidden internal use only
  /// @brief 加入房间错误。 <br>
  ///        调用 `joinRoom` 方法时, LICENSE 计费账号未使用 LICENSE_AUTHENTICATE SDK，加入房间错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomWithoutLicenseAuthenticateSDK(11),

  /// @brief 通话回路检测已经存在同样 roomId 的房间了。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  roomAlreadyExist(12),

  /// @brief 加入多个房间时使用了不同的 uid。 <br>
  ///        同一个引擎实例中，用户需使用同一个 uid 加入不同的房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  userIdDifferent(13),

  /// @hidden internal use only
  /// @brief 服务端 license 过期，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomServerLicenseExpired(14),

  /// @hidden internal use only
  /// @brief 超过服务端 license 许可的并发量上限，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomExceedsTheUpperLimit(15),

  /// @hidden internal use only
  /// @brief license 参数错误，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomLicenseParameterError(16),

  /// @hidden internal use only
  /// @brief license 证书路径错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomLicenseFilePathError(17),

  /// @hidden internal use only
  /// @brief license 证书不合法。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomLicenseIllegal(18),

  /// @hidden internal use only
  /// @brief license 证书已经过期，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomLicenseExpired(19),

  /// @hidden internal use only
  /// @brief license 证书内容不匹配。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomLicenseInformationNotMatch(20),

  /// @hidden internal use only
  /// @brief license 当前证书与缓存证书不匹配。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomLicenseNotMatchWithCache(21),

  /// @brief 房间被封禁。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomRoomForbidden(22),

  /// @brief 用户被封禁。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomUserForbidden(23),

  /// @platform android
  /// @hidden internal use only
  /// @brief license 计费方法没有加载成功。可能是因为 license 相关插件未正确集成。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  joinRoomLicenseFunctionNotFound(24),

  /// @platform android
  /// @hidden for internal use only
  ///
  loadSOLib(25),

  /// @brief 发布流失败，发布流总数超过上限。 <br>
  ///        RTC 系统会限制单个房间内发布的总流数，总流数包括视频流、音频流和屏幕流。如果房间内发布流数已达上限时，本地用户再向房间中发布流时会失败，同时会收到此错误通知。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  overStreamPublishLimit(26),

  /// @brief 服务端异常状态导致退出房间。 通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。 <br>
  ///        SDK与信令服务器断开，并不再自动重连，可联系技术支持。  <br>
  ///
  abnormalServerStatus(27),

  /// @hidden for internal use only
  /// @brief 在一路流推多房间的场景下，在至少有两个房间在发布同一路流时，其中一个房间取消发布失败，此时需要业务方重试或者由业务方通知用户重试取消发布。
  ///
  ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED(28),

  /// @hidden for internal use only
  /// @brief 指定服务区域时传入错误参数。
  ///
  ERROR_CODE_WRONG_AREA_CODE(29),

  /// @brief 订阅音视频流失败，订阅音视频流总数超过上限。 <br>
  ///        游戏场景下为了保证音视频通话的性能和质量，服务器会限制用户订阅的音视频流的总数。当用户订阅的音视频流总数已达上限时，继续订阅更多流时会失败，同时用户会收到此错误通知。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  overStreamSubscribeLimit(30),

  /// @platform android
  /// @deprecated since 3.52, use ERROR_CODE_OVER_STREAM_PUBLISH_LIMIT(-1080) instead
  /// @brief 发布屏幕流失败，发布流总数超过上限。 <br>
  ///        RTC 系统会限制单个房间内发布的总流数，总流数包括视频流、音频流和屏幕流。如果房间内发布流数已达上限时，本地用户再向房间中发布流时会失败，同时会收到此错误通知。
  ///
  overScreenPublishLimit(31),

  /// @deprecated since 3.60, use INVALID_UID_REPEATED(0) carried by onAVSyncEvent{@link #IRTCRoomEventHandler#onAVSyncEvent} instead.
  /// @brief 音视频同步失败。 <br>
  ///        当前音频源已与其他视频源关联同步关系。 <br>
  ///        单个音频源不支持与多个视频源同时同步。 <br>
  ///        通过 onStreamStateChanged 回调。
  ///
  invalidAudioSyncUidRepeated(32),

  /// @deprecated since 3.52, use ERROR_CODE_OVER_STREAM_PUBLISH_LIMIT(-1080) instead
  /// @brief 发布视频流总数超过上限。 <br>
  ///        RTC 系统会限制单个房间内发布的视频流数。如果房间内发布视频流数已达上限时，本地用户再向房间中发布视频流时会失败，同时会收到此错误通知。
  ///
  overVideoPublishLimit(33),

  /// @platform ios
  /// @hidden for internal use only
  /// @brief notify deadlock
  ///
  ByteRTCErrorCodeDeadLockNotify(34);

  final dynamic $value;
  const ErrorCode([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 房间内群发消息结果
        */
enum RoomMessageSendResult {
  /// @brief 发送成功
  ///
  success(0),

  /// @brief 超过 QPS 限制
  ///
  exceedQPS(1),

  /// @brief 发送失败。消息发送方没有加入房间。
  ///
  notJoin(2),

  /// @brief 发送失败。连接未完成初始化。
  ///
  init(3),

  /// @platform ios
  /// @brief 发送超时，没有发送
  ///
  timeout(4),

  /// @platform ios
  /// @brief 通道断开，没有发送
  ///
  networkDisconnected(5),

  /// @brief 发送失败。没有可用的数据传输通道连接
  ///
  noConnection(6),

  /// @brief 发送失败。消息超过最大长度 64KB。
  ///
  exceedMaxLength(7),

  /// @brief 发送失败。未知错误
  ///
  unknown(8);

  final dynamic $value;
  const RoomMessageSendResult([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 发送消息结果，成功或失败，及失败原因
        */
enum UserMessageSendResult {
  /// @brief 发送消息成功。
  ///
  success(0),

  /// @brief 消息发送失败。发送超时。
  ///
  timeout(1),

  /// @brief 消息发送失败。连接断开，消息未发出。
  ///
  broken(2),

  /// @brief 消息发送失败。找不到接收方。
  ///
  noReceiver(3),

  /// @brief 消息发送失败。远端用户没有登录或进房。
  ///
  noRelayPath(4),

  /// @brief 消息发送失败。超过 QPS 限制。
  ///
  exceedQPS(5),

  e2BSSendFailed(6),

  e2BSReturnFailed(7),

  /// @brief 消息发送失败。消息发送方没有加入房间。
  ///
  notJoin(8),

  /// @brief 消息发送失败。连接未完成初始化。
  ///
  init(9),

  /// @brief 消息发送失败。没有可用的数据传输通道连接。
  ///
  noConnection(10),

  /// @brief 消息发送失败。消息超过最大长度 (64 KB)。
  ///
  exceedMaxLength(11),

  /// @brief 消息发送失败。接收方用户 ID 为空。
  ///
  emptyUser(12),

  /// @brief 消息发送失败。房间外或应用服务器消息发送方没有登录。
  ///
  notLogin(13),

  /// @brief 消息发送失败。发送消息给业务方服务器之前没有设置参数。
  ///
  serverParamsNotSet(14),

  /// @brief 消息发送失败。未知错误。
  ///
  unknown(15);

  final dynamic $value;
  const UserMessageSendResult([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 合流任务错误码
        */
enum MixedStreamTaskErrorCode {
  /// @brief 推流成功。
  ///
  ok(0),

  /// @hidden currently not available
  /// @brief 预留错误码，未启用
  ///
  base(1),

  /// @brief 任务处理超时，请检查网络状态并重试。
  ///
  timeout(2),

  /// @brief 服务端检测到错误的推流参数。
  ///
  invalid_param_by_server(3),

  /// @brief 对流的订阅超时
  ///
  sub_timeout_by_server(4),

  /// @brief 合流服务端内部错误。
  ///
  invalid_state_by_server(5),

  /// @brief 合流服务端推 CDN 失败。
  ///
  authentication_by_cdn(6),

  /// @brief 服务端未知错误。
  ///
  unknown_by_server(7),

  /// @brief 服务端接收信令超时，请检查网络状态并重试。
  ///
  signal_request_timeout(8),

  /// @brief 图片合流失败。
  ///
  mix_image_fail(9),

  /// @hidden internal use only
  /// @brief 缓存未同步。
  ///
  stream_sync_worse(10),

  /// @brief 发布 WTN 流失败
  ///
  push_wtn_failed(11),

  /// @hidden for internal use only
  ///
  max(12);

  final dynamic $value;
  const MixedStreamTaskErrorCode([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 字幕任务错误码。
        */
enum SubtitleErrorCode {
  /// @brief 客户端无法识别云端媒体处理发送的错误码。请联系技术支持。
  ///
  unknown(0),

  /// @brief 字幕已开启。
  ///
  success(1),

  /// @brief 云端媒体处理内部出现错误，请联系技术支持。
  ///
  postProcessError(2),

  /// @brief 第三方服务连接失败，请联系技术支持。
  ///
  asrConnectionError(3),

  /// @brief 第三方服务内部出现错误，请联系技术支持。
  ///
  asrServiceError(4),

  /// @brief 未进房导致调用`startSubtitle`失败。请加入房间后再调用此方法。
  ///
  beforeJoinRoom(5),

  /// @brief 字幕已开启，无需重复调用 `startSubtitle`。
  ///
  alreadyOn(6),

  /// @brief 你选择的目标语言目前暂不支持。
  ///
  unsupportedLanguage(7),

  /// @brief 云端媒体处理超时未响应，请联系技术支持。
  ///
  postProcessTimeout(8);

  final dynamic $value;
  const SubtitleErrorCode([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 单流转推直播事件
        */
enum SingleStreamTaskEvent {
  /// @hidden for internal use only
  ///
  base(0),

  /// @brief 任务发起成功。
  ///
  start_success(1),

  /// @brief 任务发起失败。
  ///
  start_failed(2),

  /// @brief 任务停止。
  ///
  stop_success(3),

  /// @brief 结束任务失败。
  ///
  stop_failed(4),

  /// @brief Warning 事件。
  ///
  warning(5);

  final dynamic $value;
  const SingleStreamTaskEvent([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief KTV 错误码。
        */
enum KTVErrorCode {
  /// @brief 成功。
  ///
  ok(0),

  /// @brief AppID 异常。
  ///
  appid_invalid(1),

  /// @brief 非法参数，传入的参数不正确。
  ///
  paras_invalid(2),

  /// @brief 获取歌曲资源失败。
  ///
  get_music_failed(3),

  /// @brief 获取歌词失败。
  ///
  get_lyric_failed(4),

  /// @brief 歌曲下架。
  ///
  music_takedown(5),

  /// @brief 歌曲文件下载失败。
  ///
  music_download(6),

  /// @brief MIDI 文件下载失败。
  ///
  midi_download_failed(7),

  /// @brief 系统繁忙。
  ///
  system_busy(8),

  /// @brief 网络异常。
  ///
  network(9),

  /// @brief KTV 功能未加入房间。
  ///
  not_join_room(10),

  /// @brief 解析数据失败。
  ///
  parse_data(11),

  /// @brief 已在下载中。
  ///
  downloading(12),

  /// @brief 下载失败，磁盘空间不足。清除缓存后重试。
  ///
  insufficient_disk_space(13),

  /// @brief 下载失败，音乐文件解密失败，联系技术支持人员。
  ///
  music_decryption_failed(14),

  /// @brief 下载失败，音乐文件重命名失败，请重试。
  ///
  file_rename_failed(15),

  /// @brief 下载失败，下载超时，请重试。
  ///
  download_timeout(16),

  /// @brief 清除缓存失败，可能原因是文件被占用或者系统异常，请重试。
  ///
  clear_cache_failed(17),

  /// @brief 取消下载。
  ///
  download_canceled(18),

  /// @hidden
  /// @deprecated 从 353 开始。
  /// @brief 下载失败。
  ///
  download(19),

  /// @platform android
  /// @brief 内部错误，联系技术支持人员。
  ///
  INTERNAL_DOMAIN(20),

  /// @platform ios
  /// @brief 内部错误，联系技术支持人员。
  ///
  ByteRTCKTVErrorCodeInternal(21);

  final dynamic $value;
  const KTVErrorCode([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 单流转推任务错误码
        */
enum SingleStreamTaskErrorCode {
  /// @brief 推流成功。
  ///
  ok(0),

  /// @hidden currently not available
  /// @brief 预留错误码，未启用
  ///
  base(1),

  /// @brief 服务端合流错误
  ///
  unknown_by_server(2),

  /// @brief 任务处理超时，请检查网络状态并重试。
  ///
  signal_request_timeout(3),

  /// @brief 服务端检测任务参数不合法
  ///
  invalid_param_by_server(4),

  /// @brief 转推任务在目标房间的用户ID被踢出目标房间
  ///
  remote_kicked(5),

  /// @brief 转推任务加入目标房间失败
  ///
  join_dest_room_failed(6),

  /// @brief 转推任务在源房间拉流超时
  ///
  receive_src_stream_timeout(7),

  /// @brief 音视频编码转推任务不支持
  ///
  not_surport_codec(8);

  final dynamic $value;
  const SingleStreamTaskErrorCode([this.$value]);
}

/** {zh}
        * @deprecated since 3.60, see onMixedStreamEvent{@link #IRTCEngineEventHandler#onMixedStreamEvent}.
* @detail errorcode
* @brief WTN 流状态码
        */
enum PublicStreamErrorCode {
  /// @brief 发布或订阅成功。
  ///
  success(0),

  /// @brief WTN 流的参数异常，请修改参数后重试。
  ///
  pushParamError(1),

  /// @brief 服务端状态异常，将自动重试。
  ///
  pushStateError(2),

  /// @brief 内部错误，不可恢复，请重试。
  ///
  pushInternalError(3),

  /// @brief 发布失败，将自动重试，请关注重试结果。
  ///
  pushError(4),

  /// @brief 发布失败，10 s 后会重试，重试 3 次后自动停止。
  ///
  pushTimeOut(5),

  /// @brief 订阅失败，发布端未开始发布流。
  ///
  pullNoPushStream(6);

  final dynamic $value;
  const PublicStreamErrorCode([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 回调警告码。 <br>
*        警告码说明 SDK 内部遇到问题正在尝试恢复。警告码仅作通知。
        */
enum WarningCode {
  /// @brief 发布音视频流失败。 <br>
  ///        当你在所在房间中发布音视频流时，由于服务器错误导致发布失败。SDK 会自动重试发布。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  publish_stream_failed(0),

  /// @brief 订阅音视频流失败。 <br>
  ///        当前房间中找不到订阅的音视频流导致订阅失败。SDK 会自动重试订阅，若仍订阅失败则建议你退出重试。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  subscribe_stream_failed_404(1),

  /// @brief 订阅音视频流失败。 <br>
  ///        当你订阅所在房间中的音视频流时，由于服务器错误导致订阅失败。SDK 会自动重试订阅。通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知。
  ///
  subscribe_stream_failed_5xx(2),

  /// @platform android
  /// @brief 当调用 `setUserVisibility` 将自身可见性设置为 false 后，再尝试发布流会触发此警告。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  publish_stream_forbidden(3),

  /// @brief 发送自定义广播消息失败, 当前你未在房间中。
  ///
  send_custom_message(4),

  /// @brief 当房间内人数超过 500 人时，停止向房间内已有用户发送 `onUserJoined` 和 `onUserLeave` 回调，并通过广播提示房间内所有用户。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  receive_user_notify_stop(5),

  /// @brief 用户已经在其他房间发布过流，或者用户正在发布 WTN 流。通过 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///
  user_in_publish(6),

  /// @platform ios
  /// @brief 指定的内部渲染画布句柄无效。 <br>
  ///        当你调用 setLocalVideoCanvas:{@link #ByteRTCEngine#setLocalVideoCanvas} 或 setRemoteVideoCanvas:withCanvas:{@link #ByteRTCEngine#setRemoteVideoCanvas:withCanvas} 时指定了无效的画布句柄，触发此回调。
  ///
  invalid_canvas_handle(7),

  /// @brief 摄像头权限异常，当前应用没有获取摄像头权限。
  ///
  no_camera_permission(8),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) SDK 鉴权失效。联系技术支持人员。
  ///
  invalid_sami_app_key_or_token(9),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 资源加载失败。传入正确的 DAT 路径，或联系技术支持人员。
  ///
  invalid_sami_resource_path(10),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 库加载失败。使用正确的库，或联系技术支持人员。
  ///
  load_sami_library_failed(11),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 不支持此音效。联系技术支持人员。
  ///
  invalid_sami_effect_type(12),

  /// @brief 当前正在进行回路测试，该接口调用无效
  ///
  in_echo_test_mode(13),

  /// @platform ios
  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 同样 roomid 的房间已经存在了
  ///
  ios_room_already_exist(14),

  /// @brief 麦克风权限异常，当前应用没有获取麦克风权限。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_NOPERMISSION instead.
  ///
  no_microphone_permission(15),

  /// @brief 音频采集设备启动失败。 <br>
  ///        启动音频采集设备失败，当前设备可能被其他应用占用。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICEFAILURE instead.
  ///
  audio_device_manager_recording_start_fail(16),

  /// @brief 音频播放设备启动失败警告。 <br>
  ///        可能由于系统资源不足，或参数错误。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICEFAILURE instead.
  ///
  audio_device_manager_playout_start_fail(17),

  /// @brief 无可用音频采集设备。 <br>
  ///        启动音频采集设备失败，请插入可用的音频采集设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICENOTFOUND instead.
  ///
  no_recording_device(18),

  /// @brief 无可用音频播放设备。 <br>
  ///        启动音频播放设备失败，请插入可用的音频播放设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceError{@link #MediaDeviceError}.MEDIA_DEVICE_ERROR_DEVICENOTFOUND instead.
  ///
  no_playout_device(19),

  /// @brief 当前音频设备没有采集到有效的声音数据，请检查更换音频采集设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceWarning{@link #MediaDeviceWarning}.MEDIA_DEVICE_WARNING_CAPTURE_SILENCE instead.
  ///
  recording_silence(20),

  /// @brief 不支持在 publishScreenAudio 之后，调用 setScreenAudioSourceType{@link #RTCEngine#setScreenAudioSourceType} 设置屏幕音频采集类型
  ///
  set_screen_audio_source_type_failed(21),

  /// @brief 不支持在 publishScreenAudio 之后，调用 setScreenAudioStreamIndex 设置屏幕音频共享发布类型
  ///
  set_screen_audio_stream_index_failed(22),

  /// @brief 设置语音音高不合法
  ///
  invalid_voice_pitch(23),

  /// @brief 外部音频源新旧接口混用
  ///
  invalid_call_for_ext_audio(24),

  /// @brief 媒体设备误操作警告。 <br>
  ///        使用自定义采集时，不可调用内部采集开关，调用时将触发此警告。
  /// @deprecated since 3.33 and will be deleted in 3.51, use MediaDeviceWarning{@link #MediaDeviceWarning}.MEDIA_DEVICE_WARNING_OPERATION_DENIED instead.
  ///
  media_device_operation_denied(25),

  /// @platform android
  /// @hidden currently not available
  /// @brief 函数调用顺序错误。
  ///
  WARNING_CODE_INVOKE_ERROR(26),

  /// @platform android
  /// @hidden for internal use only
  /// @brief 调度异常，服务器返回的媒体服务器地址不可用。
  ///
  WARNING_CODE_INVALID_EXPECT_MEDIA_SERVER_ADDRESS(27),

  /// @platform android
  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 获取房间信息失败警告
  /// @note SDK 获取房间信息失败（包含超时，返回非 200 的错误码），每隔两秒重试一次。 <br>
  ///        连续失败 5 次后，报该 warning，并继续重试。 <br>
  ///        建议提示用户：进入房间失败，请稍后再试
  ///
  WARNING_CODE_GET_ROOM_FAILED(28),

  /// @platform android
  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 自动订阅模式未关闭时，尝试开启手动订阅模式会触发此警告。 <br>
  ///        你需在进房前关闭自动订阅模式，再手动订阅音视频流。
  ///
  WARNING_CODE_SUBSCRIBE_STREAM_FORBIDEN(29),

  /// @platform android
  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 已存在同样 roomId 的房间。
  ///
  WARNING_CODE_ROOM_ID_ALREADY_EXIST(30),

  /// @platform ios
  /// @hidden currently not available
  /// @brief 函数调用顺序错误，当前代码中未使用。
  ///
  ByteRTCWarningCodeInvokeError(31),

  /// @platform ios
  /// @hidden for internal use only
  /// @brief 调度异常，服务器返回的媒体服务器地址不可用。
  ///
  ByteRTCWarningCodeInvalidExpectMediaServerAddress(32),

  /// @platform ios
  /// @brief 当调用 `setUserVisibility:` 将自身可见性设置为 false 后，再尝试发布流会触发此警告。通过 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason} 回调。
  ///
  ByteRTCWarningCodePublishStreamForbidden(33),

  /// @platform ios
  /// @hidden
  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 自动订阅模式未关闭时，尝试开启手动订阅模式会触发此警告。 <br>
  ///        你需在进房前关闭自动订阅模式，再调用 subscribeStreamVideo:subscribe:{@link #ByteRTCRoom#subscribeStreamVideo:subscribe}/subscribeStreamAudio:subscribe:{@link #ByteRTCRoom#subscribeStreamAudio:subscribe} 方法手动订阅音视频流。
  ///
  ByteRTCWarningCodeSubscribeStreamForbiden(34),

  unknown(35);

  final dynamic $value;
  const WarningCode([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 登录结果 <br>
*        调用 `login` 登录的结果，会通过 `onLoginResult` 回调通知用户。
        */
enum LoginErrorCode {
  /// @brief 调用 `login` 方法登录成功。
  ///
  success(0),

  /// @brief 调用 `login` 方法时使用的 Token 无效或过期失效。需要用户重新获取 Token。
  ///
  invalid_token(1),

  /// @brief 登录错误。 <br>
  ///        调用 `login` 方法时发生未知错误导致登录失败，需要重新登录。
  ///
  login_failed(2),

  /// @brief 调用 `login` 方法时传入的用户 ID 有问题。
  ///
  invalid_user_id(3),

  /// @brief 调用 `login` 登录时服务器错误。
  ///
  code_server_error(4);

  final dynamic $value;
  const LoginErrorCode([this.$value]);
}

/** {zh}
        * @detail errorcode
* @brief 音频文件录制的错误码
        */
enum AudioRecordingErrorCode {
  /// @brief 录制正常
  ///
  ok(0),

  /// @brief 没有文件写权限
  ///
  no_permission(1),

  /// @brief 没有进入房间
  ///
  not_in_room(2),

  /// @brief 录制已经开始
  ///
  already_started(3),

  /// @brief 录制还未开始
  ///
  not_started(4),

  /// @brief 录制失败。文件格式不支持。
  ///
  not_support(5),

  /// @brief 其他异常
  ///
  other(6);

  final dynamic $value;
  const AudioRecordingErrorCode([this.$value]);
}

class t_UserMessageSendResult {
  static $p_a.UserMessageSendResult code_to_android(
      UserMessageSendResult value) {
    var $m = {
      UserMessageSendResult.success:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_SUCCESS,
      UserMessageSendResult.timeout:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_TIMEOUT,
      UserMessageSendResult.broken:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_BROKEN,
      UserMessageSendResult.noReceiver:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NO_RECEIVER,
      UserMessageSendResult.noRelayPath:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NO_RELAY_PATH,
      UserMessageSendResult.exceedQPS:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_EXCEED_QPS,
      UserMessageSendResult.e2BSSendFailed:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_E2BS_SEND_FAILED,
      UserMessageSendResult.e2BSReturnFailed: $p_a
          .UserMessageSendResult.USER_MESSAGE_SEND_RESULT_E2BS_RETURN_FAILED,
      UserMessageSendResult.notJoin:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NOT_JOIN,
      UserMessageSendResult.init:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_INIT,
      UserMessageSendResult.noConnection:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NO_CONNECTION,
      UserMessageSendResult.exceedMaxLength:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_EXCEED_MAX_LENGTH,
      UserMessageSendResult.emptyUser:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_EMPTY_USER,
      UserMessageSendResult.notLogin:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NOT_LOGIN,
      UserMessageSendResult.serverParamsNotSet: $p_a
          .UserMessageSendResult.USER_MESSAGE_SEND_RESULT_SERVER_PARAMS_NOT_SET,
      UserMessageSendResult.unknown:
          $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_UNKNOWN,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.UserMessageSendResult;
  }

  static UserMessageSendResult android_to_code(
      $p_a.UserMessageSendResult value) {
    var $m = {
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_SUCCESS:
          UserMessageSendResult.success,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_TIMEOUT:
          UserMessageSendResult.timeout,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_BROKEN:
          UserMessageSendResult.broken,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NO_RECEIVER:
          UserMessageSendResult.noReceiver,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NO_RELAY_PATH:
          UserMessageSendResult.noRelayPath,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_EXCEED_QPS:
          UserMessageSendResult.exceedQPS,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_E2BS_SEND_FAILED:
          UserMessageSendResult.e2BSSendFailed,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_E2BS_RETURN_FAILED:
          UserMessageSendResult.e2BSReturnFailed,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NOT_JOIN:
          UserMessageSendResult.notJoin,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_INIT:
          UserMessageSendResult.init,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NO_CONNECTION:
          UserMessageSendResult.noConnection,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_EXCEED_MAX_LENGTH:
          UserMessageSendResult.exceedMaxLength,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_EMPTY_USER:
          UserMessageSendResult.emptyUser,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_NOT_LOGIN:
          UserMessageSendResult.notLogin,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_SERVER_PARAMS_NOT_SET:
          UserMessageSendResult.serverParamsNotSet,
      $p_a.UserMessageSendResult.USER_MESSAGE_SEND_RESULT_UNKNOWN:
          UserMessageSendResult.unknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as UserMessageSendResult;
  }

  static $p_i.ByteRTCUserMessageSendResult code_to_ios(
      UserMessageSendResult value) {
    var $m = {
      UserMessageSendResult.success:
          $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultSuccess,
      UserMessageSendResult.timeout:
          $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultTimeout,
      UserMessageSendResult.broken: $p_i.ByteRTCUserMessageSendResult
          .ByteRTCUserMessageSendResultNetworkDisconnected,
      UserMessageSendResult.noReceiver: $p_i
          .ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNoReceiver,
      UserMessageSendResult.noRelayPath: $p_i
          .ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNoRelayPath,
      UserMessageSendResult.exceedQPS: $p_i
          .ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultExceedQPS,
      UserMessageSendResult.e2BSSendFailed: $p_i.ByteRTCUserMessageSendResult
          .ByteRTCUserMessageSendResultE2BSSendFailed,
      UserMessageSendResult.e2BSReturnFailed: $p_i.ByteRTCUserMessageSendResult
          .ByteRTCUserMessageSendResultE2BSReturnFailed,
      UserMessageSendResult.notJoin:
          $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNotJoin,
      UserMessageSendResult.init:
          $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultInit,
      UserMessageSendResult.noConnection: $p_i.ByteRTCUserMessageSendResult
          .ByteRTCUserMessageSendResultNoConnection,
      UserMessageSendResult.exceedMaxLength: $p_i.ByteRTCUserMessageSendResult
          .ByteRTCUserMessageSendResultExceedMaxLength,
      UserMessageSendResult.emptyUser: $p_i
          .ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultEmptyUser,
      UserMessageSendResult.notLogin: $p_i
          .ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNotLogin,
      UserMessageSendResult.serverParamsNotSet: $p_i
          .ByteRTCUserMessageSendResult
          .ByteRTCUserMessageSendResultServerParamsNotSet,
      UserMessageSendResult.unknown:
          $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultUnknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCUserMessageSendResult;
  }

  static UserMessageSendResult ios_to_code(
      $p_i.ByteRTCUserMessageSendResult value) {
    var $m = {
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultSuccess:
          UserMessageSendResult.success,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultTimeout:
          UserMessageSendResult.timeout,
      $p_i.ByteRTCUserMessageSendResult
              .ByteRTCUserMessageSendResultNetworkDisconnected:
          UserMessageSendResult.broken,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNoReceiver:
          UserMessageSendResult.noReceiver,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNoRelayPath:
          UserMessageSendResult.noRelayPath,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultExceedQPS:
          UserMessageSendResult.exceedQPS,
      $p_i.ByteRTCUserMessageSendResult
              .ByteRTCUserMessageSendResultE2BSSendFailed:
          UserMessageSendResult.e2BSSendFailed,
      $p_i.ByteRTCUserMessageSendResult
              .ByteRTCUserMessageSendResultE2BSReturnFailed:
          UserMessageSendResult.e2BSReturnFailed,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNotJoin:
          UserMessageSendResult.notJoin,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultInit:
          UserMessageSendResult.init,
      $p_i.ByteRTCUserMessageSendResult
              .ByteRTCUserMessageSendResultNoConnection:
          UserMessageSendResult.noConnection,
      $p_i.ByteRTCUserMessageSendResult
              .ByteRTCUserMessageSendResultExceedMaxLength:
          UserMessageSendResult.exceedMaxLength,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultEmptyUser:
          UserMessageSendResult.emptyUser,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultNotLogin:
          UserMessageSendResult.notLogin,
      $p_i.ByteRTCUserMessageSendResult
              .ByteRTCUserMessageSendResultServerParamsNotSet:
          UserMessageSendResult.serverParamsNotSet,
      $p_i.ByteRTCUserMessageSendResult.ByteRTCUserMessageSendResultUnknown:
          UserMessageSendResult.unknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as UserMessageSendResult;
  }
}

class t_RoomMessageSendResult {
  static $p_a.RoomMessageSendResult code_to_android(
      RoomMessageSendResult value) {
    var $m = {
      RoomMessageSendResult.success:
          $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_SUCCESS,
      RoomMessageSendResult.exceedQPS:
          $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_EXCEED_QPS,
      RoomMessageSendResult.notJoin:
          $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_NOT_JOIN,
      RoomMessageSendResult.init:
          $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_INIT,
      RoomMessageSendResult.noConnection:
          $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_NO_CONNECTION,
      RoomMessageSendResult.exceedMaxLength:
          $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_EXCEED_MAX_LENGTH,
      RoomMessageSendResult.unknown:
          $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_UNKNOWN,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.RoomMessageSendResult;
  }

  static RoomMessageSendResult android_to_code(
      $p_a.RoomMessageSendResult value) {
    var $m = {
      $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_SUCCESS:
          RoomMessageSendResult.success,
      $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_EXCEED_QPS:
          RoomMessageSendResult.exceedQPS,
      $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_NOT_JOIN:
          RoomMessageSendResult.notJoin,
      $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_INIT:
          RoomMessageSendResult.init,
      $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_NO_CONNECTION:
          RoomMessageSendResult.noConnection,
      $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_EXCEED_MAX_LENGTH:
          RoomMessageSendResult.exceedMaxLength,
      $p_a.RoomMessageSendResult.ROOM_MESSAGE_SEND_RESULT_UNKNOWN:
          RoomMessageSendResult.unknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as RoomMessageSendResult;
  }

  static $p_i.ByteRTCRoomMessageSendResult code_to_ios(
      RoomMessageSendResult value) {
    var $m = {
      RoomMessageSendResult.success:
          $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultSuccess,
      RoomMessageSendResult.exceedQPS: $p_i
          .ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultExceedQPS,
      RoomMessageSendResult.notJoin:
          $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultNotJoin,
      RoomMessageSendResult.init:
          $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultInit,
      RoomMessageSendResult.timeout:
          $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultTimeout,
      RoomMessageSendResult.networkDisconnected: $p_i
          .ByteRTCRoomMessageSendResult
          .ByteRTCRoomMessageSendResultNetworkDisconnected,
      RoomMessageSendResult.noConnection: $p_i.ByteRTCRoomMessageSendResult
          .ByteRTCRoomMessageSendResultNoConnection,
      RoomMessageSendResult.exceedMaxLength: $p_i.ByteRTCRoomMessageSendResult
          .ByteRTCRoomMessageSendResultExceedMaxLength,
      RoomMessageSendResult.unknown:
          $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultUnknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCRoomMessageSendResult;
  }

  static RoomMessageSendResult ios_to_code(
      $p_i.ByteRTCRoomMessageSendResult value) {
    var $m = {
      $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultSuccess:
          RoomMessageSendResult.success,
      $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultExceedQPS:
          RoomMessageSendResult.exceedQPS,
      $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultNotJoin:
          RoomMessageSendResult.notJoin,
      $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultInit:
          RoomMessageSendResult.init,
      $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultTimeout:
          RoomMessageSendResult.timeout,
      $p_i.ByteRTCRoomMessageSendResult
              .ByteRTCRoomMessageSendResultNetworkDisconnected:
          RoomMessageSendResult.networkDisconnected,
      $p_i.ByteRTCRoomMessageSendResult
              .ByteRTCRoomMessageSendResultNoConnection:
          RoomMessageSendResult.noConnection,
      $p_i.ByteRTCRoomMessageSendResult
              .ByteRTCRoomMessageSendResultExceedMaxLength:
          RoomMessageSendResult.exceedMaxLength,
      $p_i.ByteRTCRoomMessageSendResult.ByteRTCRoomMessageSendResultUnknown:
          RoomMessageSendResult.unknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as RoomMessageSendResult;
  }
}

class t_SubtitleErrorCode {
  static $p_a.SubtitleErrorCode code_to_android(SubtitleErrorCode value) {
    var $m = {
      SubtitleErrorCode.unknown:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_UNKNOW,
      SubtitleErrorCode.success:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_SUCCESS,
      SubtitleErrorCode.postProcessError:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_POST_PROCESS_ERROR,
      SubtitleErrorCode.asrConnectionError:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_ASR_CONNECTION_ERROR,
      SubtitleErrorCode.asrServiceError:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_ASR_SERVICE_ERROR,
      SubtitleErrorCode.beforeJoinRoom:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_BEFORE_JOIN_ROOM,
      SubtitleErrorCode.alreadyOn:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_ALREADY_ON,
      SubtitleErrorCode.unsupportedLanguage:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_UNSUPPORTED_LANGUAGE,
      SubtitleErrorCode.postProcessTimeout:
          $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_POST_PROCESS_TIMEOUT,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.SubtitleErrorCode;
  }

  static SubtitleErrorCode android_to_code($p_a.SubtitleErrorCode value) {
    var $m = {
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_UNKNOW:
          SubtitleErrorCode.unknown,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_SUCCESS:
          SubtitleErrorCode.success,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_POST_PROCESS_ERROR:
          SubtitleErrorCode.postProcessError,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_ASR_CONNECTION_ERROR:
          SubtitleErrorCode.asrConnectionError,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_ASR_SERVICE_ERROR:
          SubtitleErrorCode.asrServiceError,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_BEFORE_JOIN_ROOM:
          SubtitleErrorCode.beforeJoinRoom,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_ALREADY_ON:
          SubtitleErrorCode.alreadyOn,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_UNSUPPORTED_LANGUAGE:
          SubtitleErrorCode.unsupportedLanguage,
      $p_a.SubtitleErrorCode.SUBTITLE_ERROR_CODE_POST_PROCESS_TIMEOUT:
          SubtitleErrorCode.postProcessTimeout,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as SubtitleErrorCode;
  }

  static $p_i.ByteRTCSubtitleErrorCode code_to_ios(SubtitleErrorCode value) {
    var $m = {
      SubtitleErrorCode.unknown:
          $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeUnknow,
      SubtitleErrorCode.success:
          $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeSuccess,
      SubtitleErrorCode.postProcessError: $p_i
          .ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodePostProcessError,
      SubtitleErrorCode.asrConnectionError: $p_i
          .ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeASRConnectionError,
      SubtitleErrorCode.asrServiceError:
          $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeASRServiceError,
      SubtitleErrorCode.beforeJoinRoom:
          $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeBeforeJoinRoom,
      SubtitleErrorCode.alreadyOn:
          $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeAlreadyOn,
      SubtitleErrorCode.unsupportedLanguage: $p_i
          .ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeUnsupportedLanguage,
      SubtitleErrorCode.postProcessTimeout: $p_i
          .ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodePostProcessTimeout,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCSubtitleErrorCode;
  }

  static SubtitleErrorCode ios_to_code($p_i.ByteRTCSubtitleErrorCode value) {
    var $m = {
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeUnknow:
          SubtitleErrorCode.unknown,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeSuccess:
          SubtitleErrorCode.success,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodePostProcessError:
          SubtitleErrorCode.postProcessError,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeASRConnectionError:
          SubtitleErrorCode.asrConnectionError,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeASRServiceError:
          SubtitleErrorCode.asrServiceError,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeBeforeJoinRoom:
          SubtitleErrorCode.beforeJoinRoom,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeAlreadyOn:
          SubtitleErrorCode.alreadyOn,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodeUnsupportedLanguage:
          SubtitleErrorCode.unsupportedLanguage,
      $p_i.ByteRTCSubtitleErrorCode.ByteRTCSubtitleErrorCodePostProcessTimeout:
          SubtitleErrorCode.postProcessTimeout,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as SubtitleErrorCode;
  }
}

class t_WarningCode {
  static $p_a.WarningCode code_to_android(WarningCode value) {
    var $m = {
      WarningCode.publish_stream_failed:
          $p_a.WarningCode.WARNING_CODE_PUBLISH_STREAM_FAILED,
      WarningCode.subscribe_stream_failed_404:
          $p_a.WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FAILED404,
      WarningCode.subscribe_stream_failed_5xx:
          $p_a.WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FAILED5XX,
      WarningCode.publish_stream_forbidden:
          $p_a.WarningCode.WARNING_CODE_PUBLISH_STREAM_FORBIDEN,
      WarningCode.send_custom_message:
          $p_a.WarningCode.WARNING_CODE_SEND_CUSTOM_MESSAGE,
      WarningCode.receive_user_notify_stop:
          $p_a.WarningCode.WARNING_CODE_RECEIVE_USER_NOTIFY_STOP,
      WarningCode.user_in_publish:
          $p_a.WarningCode.WARNING_CODE_USER_IN_PUBLISH,
      WarningCode.no_camera_permission:
          $p_a.WarningCode.WARNING_CODE_NO_CAMERA_PERMISSION,
      WarningCode.invalid_sami_app_key_or_token:
          $p_a.WarningCode.WARNING_CODE_INVALID_SAMI_APPKEY_OR_TOKEN,
      WarningCode.invalid_sami_resource_path:
          $p_a.WarningCode.WARNING_CODE_INVALID_RESOURCE_PATH,
      WarningCode.load_sami_library_failed:
          $p_a.WarningCode.WARNING_CODE_LOAD_SAMI_LIBRARY_FAILED,
      WarningCode.invalid_sami_effect_type:
          $p_a.WarningCode.WARNING_CODE_INVALID_SAMI_EFFECT_TYPE,
      WarningCode.in_echo_test_mode:
          $p_a.WarningCode.WARNING_CODE_IN_ECHO_TEST_MODE,
      WarningCode.no_microphone_permission:
          $p_a.WarningCode.WARNING_CODE_NO_MICROPHONE_PERMISSION,
      WarningCode.audio_device_manager_recording_start_fail:
          $p_a.WarningCode.WARNING_CODE_RECODING_DEVICE_START_FAILED,
      WarningCode.audio_device_manager_playout_start_fail:
          $p_a.WarningCode.WARNING_CODE_PLAYOUT_DEVICE_START_FAILED,
      WarningCode.no_recording_device:
          $p_a.WarningCode.WARNING_CODE_NO_RECORDING_DEVICE,
      WarningCode.no_playout_device:
          $p_a.WarningCode.WARNING_CODE_NO_PLAYOUT_DEVICE,
      WarningCode.recording_silence:
          $p_a.WarningCode.WARNING_CODE_RECORDING_SILENCE,
      WarningCode.set_screen_audio_source_type_failed:
          $p_a.WarningCode.WARNING_CODE_SET_SCREEN_AUDIO_SOURCE_TYPE_FAILED,
      WarningCode.set_screen_audio_stream_index_failed:
          $p_a.WarningCode.WARNING_CODE_SET_SCREEN_STREAM_INDEX_FAILED,
      WarningCode.invalid_voice_pitch:
          $p_a.WarningCode.WARNING_CODE_SET_SCREEN_STREAM_INVALID_VOICE_PITCH,
      WarningCode.invalid_call_for_ext_audio:
          $p_a.WarningCode.WARNING_CODE_INVALID_CALL_FOR_EXT_AUDIO,
      WarningCode.media_device_operation_denied:
          $p_a.WarningCode.WARNING_CODE_MEDIA_DEVICE_OPERATION_DENIED,
      WarningCode.WARNING_CODE_INVOKE_ERROR:
          $p_a.WarningCode.WARNING_CODE_INVOKE_ERROR,
      WarningCode.WARNING_CODE_INVALID_EXPECT_MEDIA_SERVER_ADDRESS:
          $p_a.WarningCode.WARNING_CODE_INVALID_EXPECT_MEDIA_SERVER_ADDRESS,
      WarningCode.WARNING_CODE_GET_ROOM_FAILED:
          $p_a.WarningCode.WARNING_CODE_GET_ROOM_FAILED,
      WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FORBIDEN:
          $p_a.WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FORBIDEN,
      WarningCode.WARNING_CODE_ROOM_ID_ALREADY_EXIST:
          $p_a.WarningCode.WARNING_CODE_ROOM_ID_ALREADY_EXIST,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.WarningCode;
  }

  static WarningCode android_to_code($p_a.WarningCode value) {
    var $m = {
      $p_a.WarningCode.WARNING_CODE_PUBLISH_STREAM_FAILED:
          WarningCode.publish_stream_failed,
      $p_a.WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FAILED404:
          WarningCode.subscribe_stream_failed_404,
      $p_a.WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FAILED5XX:
          WarningCode.subscribe_stream_failed_5xx,
      $p_a.WarningCode.WARNING_CODE_PUBLISH_STREAM_FORBIDEN:
          WarningCode.publish_stream_forbidden,
      $p_a.WarningCode.WARNING_CODE_SEND_CUSTOM_MESSAGE:
          WarningCode.send_custom_message,
      $p_a.WarningCode.WARNING_CODE_RECEIVE_USER_NOTIFY_STOP:
          WarningCode.receive_user_notify_stop,
      $p_a.WarningCode.WARNING_CODE_USER_IN_PUBLISH:
          WarningCode.user_in_publish,
      $p_a.WarningCode.WARNING_CODE_NO_CAMERA_PERMISSION:
          WarningCode.no_camera_permission,
      $p_a.WarningCode.WARNING_CODE_INVALID_SAMI_APPKEY_OR_TOKEN:
          WarningCode.invalid_sami_app_key_or_token,
      $p_a.WarningCode.WARNING_CODE_INVALID_RESOURCE_PATH:
          WarningCode.invalid_sami_resource_path,
      $p_a.WarningCode.WARNING_CODE_LOAD_SAMI_LIBRARY_FAILED:
          WarningCode.load_sami_library_failed,
      $p_a.WarningCode.WARNING_CODE_INVALID_SAMI_EFFECT_TYPE:
          WarningCode.invalid_sami_effect_type,
      $p_a.WarningCode.WARNING_CODE_IN_ECHO_TEST_MODE:
          WarningCode.in_echo_test_mode,
      $p_a.WarningCode.WARNING_CODE_NO_MICROPHONE_PERMISSION:
          WarningCode.no_microphone_permission,
      $p_a.WarningCode.WARNING_CODE_RECODING_DEVICE_START_FAILED:
          WarningCode.audio_device_manager_recording_start_fail,
      $p_a.WarningCode.WARNING_CODE_PLAYOUT_DEVICE_START_FAILED:
          WarningCode.audio_device_manager_playout_start_fail,
      $p_a.WarningCode.WARNING_CODE_NO_RECORDING_DEVICE:
          WarningCode.no_recording_device,
      $p_a.WarningCode.WARNING_CODE_NO_PLAYOUT_DEVICE:
          WarningCode.no_playout_device,
      $p_a.WarningCode.WARNING_CODE_RECORDING_SILENCE:
          WarningCode.recording_silence,
      $p_a.WarningCode.WARNING_CODE_SET_SCREEN_AUDIO_SOURCE_TYPE_FAILED:
          WarningCode.set_screen_audio_source_type_failed,
      $p_a.WarningCode.WARNING_CODE_SET_SCREEN_STREAM_INDEX_FAILED:
          WarningCode.set_screen_audio_stream_index_failed,
      $p_a.WarningCode.WARNING_CODE_SET_SCREEN_STREAM_INVALID_VOICE_PITCH:
          WarningCode.invalid_voice_pitch,
      $p_a.WarningCode.WARNING_CODE_INVALID_CALL_FOR_EXT_AUDIO:
          WarningCode.invalid_call_for_ext_audio,
      $p_a.WarningCode.WARNING_CODE_MEDIA_DEVICE_OPERATION_DENIED:
          WarningCode.media_device_operation_denied,
      $p_a.WarningCode.WARNING_CODE_INVOKE_ERROR:
          WarningCode.WARNING_CODE_INVOKE_ERROR,
      $p_a.WarningCode.WARNING_CODE_INVALID_EXPECT_MEDIA_SERVER_ADDRESS:
          WarningCode.WARNING_CODE_INVALID_EXPECT_MEDIA_SERVER_ADDRESS,
      $p_a.WarningCode.WARNING_CODE_GET_ROOM_FAILED:
          WarningCode.WARNING_CODE_GET_ROOM_FAILED,
      $p_a.WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FORBIDEN:
          WarningCode.WARNING_CODE_SUBSCRIBE_STREAM_FORBIDEN,
      $p_a.WarningCode.WARNING_CODE_ROOM_ID_ALREADY_EXIST:
          WarningCode.WARNING_CODE_ROOM_ID_ALREADY_EXIST,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as WarningCode;
  }

  static $p_i.ByteRTCWarningCode code_to_ios(WarningCode value) {
    var $m = {
      WarningCode.publish_stream_failed:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodePublishStreamFailed,
      WarningCode.subscribe_stream_failed_404:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSubscribeStreamFailed404,
      WarningCode.subscribe_stream_failed_5xx:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSubscribeStreamFailed5xx,
      WarningCode.send_custom_message:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSendCustomMessage,
      WarningCode.receive_user_notify_stop:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeCodeUserNotifyStop,
      WarningCode.user_in_publish:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeUserInPublish,
      WarningCode.invalid_canvas_handle:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvalidCanvasHandle,
      WarningCode.no_camera_permission:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoCameraPermission,
      WarningCode.invalid_sami_app_key_or_token:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvaildSamiAppkeyORToken,
      WarningCode.invalid_sami_resource_path:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvaildSamiResourcePath,
      WarningCode.load_sami_library_failed:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeLoadSamiLibraryFailed,
      WarningCode.invalid_sami_effect_type:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvaildSamiEffectType,
      WarningCode.in_echo_test_mode:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInEchoTestMode,
      WarningCode.ios_room_already_exist:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeRoomAlreadyExist,
      WarningCode.no_microphone_permission:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoMicrophonePermission,
      WarningCode.audio_device_manager_recording_start_fail: $p_i
          .ByteRTCWarningCode
          .ByteRTCWarningCodeAudioDeviceManagerRecordingStartFail,
      WarningCode.audio_device_manager_playout_start_fail: $p_i
          .ByteRTCWarningCode
          .ByteRTCWarningCodeAudioDeviceManagerPlayoutStartFail,
      WarningCode.no_recording_device:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoRecordingDevice,
      WarningCode.no_playout_device:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoPlayoutDevice,
      WarningCode.recording_silence:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeRecordingSilence,
      WarningCode.set_screen_audio_source_type_failed:
          $p_i.ByteRTCWarningCode.ByteRTCWarningSetScreenAudioSourceTypeFailed,
      WarningCode.set_screen_audio_stream_index_failed:
          $p_i.ByteRTCWarningCode.ByteRTCWarningSetScreenAudioStreamIndexFailed,
      WarningCode.invalid_voice_pitch:
          $p_i.ByteRTCWarningCode.ByteRTCWarningInvalidVoicePitch,
      WarningCode.invalid_call_for_ext_audio:
          $p_i.ByteRTCWarningCode.ByteRTCWarningInvalidCallForExtAudio,
      WarningCode.media_device_operation_denied:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeMediaDeviceOperationDennied,
      WarningCode.ByteRTCWarningCodeInvokeError:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvokeError,
      WarningCode.ByteRTCWarningCodeInvalidExpectMediaServerAddress: $p_i
          .ByteRTCWarningCode.ByteRTCWarningCodeInvalidExpectMediaServerAddress,
      WarningCode.ByteRTCWarningCodePublishStreamForbidden:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodePublishStreamForbidden,
      WarningCode.ByteRTCWarningCodeSubscribeStreamForbiden:
          $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSubscribeStreamForbiden,
      WarningCode.unknown: $p_i.ByteRTCWarningCode.unknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCWarningCode;
  }

  static WarningCode ios_to_code($p_i.ByteRTCWarningCode value) {
    var $m = {
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodePublishStreamFailed:
          WarningCode.publish_stream_failed,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSubscribeStreamFailed404:
          WarningCode.subscribe_stream_failed_404,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSubscribeStreamFailed5xx:
          WarningCode.subscribe_stream_failed_5xx,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSendCustomMessage:
          WarningCode.send_custom_message,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeCodeUserNotifyStop:
          WarningCode.receive_user_notify_stop,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeUserInPublish:
          WarningCode.user_in_publish,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvalidCanvasHandle:
          WarningCode.invalid_canvas_handle,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoCameraPermission:
          WarningCode.no_camera_permission,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvaildSamiAppkeyORToken:
          WarningCode.invalid_sami_app_key_or_token,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvaildSamiResourcePath:
          WarningCode.invalid_sami_resource_path,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeLoadSamiLibraryFailed:
          WarningCode.load_sami_library_failed,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvaildSamiEffectType:
          WarningCode.invalid_sami_effect_type,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInEchoTestMode:
          WarningCode.in_echo_test_mode,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeRoomAlreadyExist:
          WarningCode.ios_room_already_exist,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoMicrophonePermission:
          WarningCode.no_microphone_permission,
      $p_i.ByteRTCWarningCode
              .ByteRTCWarningCodeAudioDeviceManagerRecordingStartFail:
          WarningCode.audio_device_manager_recording_start_fail,
      $p_i.ByteRTCWarningCode
              .ByteRTCWarningCodeAudioDeviceManagerPlayoutStartFail:
          WarningCode.audio_device_manager_playout_start_fail,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoRecordingDevice:
          WarningCode.no_recording_device,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeNoPlayoutDevice:
          WarningCode.no_playout_device,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeRecordingSilence:
          WarningCode.recording_silence,
      $p_i.ByteRTCWarningCode.ByteRTCWarningSetScreenAudioSourceTypeFailed:
          WarningCode.set_screen_audio_source_type_failed,
      $p_i.ByteRTCWarningCode.ByteRTCWarningSetScreenAudioStreamIndexFailed:
          WarningCode.set_screen_audio_stream_index_failed,
      $p_i.ByteRTCWarningCode.ByteRTCWarningInvalidVoicePitch:
          WarningCode.invalid_voice_pitch,
      $p_i.ByteRTCWarningCode.ByteRTCWarningInvalidCallForExtAudio:
          WarningCode.invalid_call_for_ext_audio,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeMediaDeviceOperationDennied:
          WarningCode.media_device_operation_denied,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvokeError:
          WarningCode.ByteRTCWarningCodeInvokeError,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeInvalidExpectMediaServerAddress:
          WarningCode.ByteRTCWarningCodeInvalidExpectMediaServerAddress,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodePublishStreamForbidden:
          WarningCode.ByteRTCWarningCodePublishStreamForbidden,
      $p_i.ByteRTCWarningCode.ByteRTCWarningCodeSubscribeStreamForbiden:
          WarningCode.ByteRTCWarningCodeSubscribeStreamForbiden,
      $p_i.ByteRTCWarningCode.unknown: WarningCode.unknown,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as WarningCode;
  }
}

class t_ErrorCode {
  static $p_a.ErrorCode code_to_android(ErrorCode value) {
    var $m = {
      ErrorCode.invalidToken: $p_a.ErrorCode.ERROR_CODE_INVALID_TOKEN,
      ErrorCode.joinRoom: $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM,
      ErrorCode.noPublishPermission:
          $p_a.ErrorCode.ERROR_CODE_NO_PUBLISH_PERMISSION,
      ErrorCode.noSubscribePermission:
          $p_a.ErrorCode.ERROR_CODE_NO_SUBSCRIBE_PERMISSION,
      ErrorCode.duplicateLogin: $p_a.ErrorCode.ERROR_CODE_DUPLICATE_LOGIN,
      ErrorCode.appIdNull: $p_a.ErrorCode.ERROR_CODE_APP_ID_NULL,
      ErrorCode.kickedOut: $p_a.ErrorCode.ERROR_CODE_KICKED_OUT,
      ErrorCode.roomIdIllegal: $p_a.ErrorCode.ERROR_CODE_ROOM_ID_ILLEGAL,
      ErrorCode.tokenExpired: $p_a.ErrorCode.ERROR_CODE_TOKEN_EXPIRED,
      ErrorCode.updateTokenWithInvalidToken:
          $p_a.ErrorCode.ERROR_CODE_UPDATE_TOKEN_WITH_INVALID_TOKEN,
      ErrorCode.roomDismiss: $p_a.ErrorCode.ERROR_CODE_ROOM_DISMISS,
      ErrorCode.joinRoomWithoutLicenseAuthenticateSDK:
          $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_WITHOUT_LICENSE_AUTHENTICATE_SDK,
      ErrorCode.roomAlreadyExist: $p_a.ErrorCode.ERROR_CODE_ROOM_ALREADY_EXIST,
      ErrorCode.userIdDifferent: $p_a.ErrorCode.ERROR_CODE_USER_ID_DIFFERENT,
      ErrorCode.joinRoomServerLicenseExpired:
          $p_a.ErrorCode.ERROR_CODE_SERVER_LICENSE_EXPIRE,
      ErrorCode.joinRoomExceedsTheUpperLimit:
          $p_a.ErrorCode.ERROR_CODE_EXCEEDS_THE_UPPER_LIMIT,
      ErrorCode.joinRoomLicenseParameterError:
          $p_a.ErrorCode.ERROR_CODE_LICENSE_PARAMETER_ERROR,
      ErrorCode.joinRoomLicenseFilePathError:
          $p_a.ErrorCode.ERROR_CODE_LICENSE_FILE_PATH_ERROR,
      ErrorCode.joinRoomLicenseIllegal:
          $p_a.ErrorCode.ERROR_CODE_LICENSE_ILLEGAL,
      ErrorCode.joinRoomLicenseExpired:
          $p_a.ErrorCode.ERROR_CODE_LICENSE_EXPIRED,
      ErrorCode.joinRoomLicenseInformationNotMatch:
          $p_a.ErrorCode.ERROR_CODE_LICENSE_INFORMATION_NOT_MATCH,
      ErrorCode.joinRoomLicenseNotMatchWithCache:
          $p_a.ErrorCode.ERROR_CODE_LICENSE_NOT_MATCH_WITH_CACHE,
      ErrorCode.joinRoomRoomForbidden:
          $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_ROOM_FORBIDDEN,
      ErrorCode.joinRoomUserForbidden:
          $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_USER_FORBIDDEN,
      ErrorCode.joinRoomLicenseFunctionNotFound:
          $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_LICENSE_FUNCTION_NOT_FOUND,
      ErrorCode.loadSOLib: $p_a.ErrorCode.ERROR_CODE_LOAD_SO_LIB,
      ErrorCode.overStreamPublishLimit:
          $p_a.ErrorCode.ERROR_CODE_OVER_STREAM_PUBLISH_LIMIT,
      ErrorCode.abnormalServerStatus:
          $p_a.ErrorCode.ERROR_CODE_ABNORMAL_SERVER_STATUS,
      ErrorCode.ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED:
          $p_a.ErrorCode.ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED,
      ErrorCode.ERROR_CODE_WRONG_AREA_CODE:
          $p_a.ErrorCode.ERROR_CODE_WRONG_AREA_CODE,
      ErrorCode.overStreamSubscribeLimit:
          $p_a.ErrorCode.ERROR_CODE_OVER_SUBSCRIBE_LIMIT,
      ErrorCode.overScreenPublishLimit:
          $p_a.ErrorCode.ERROR_CODE_OVER_SCREEN_PUBLISH_LIMIT,
      ErrorCode.invalidAudioSyncUidRepeated:
          $p_a.ErrorCode.ERROR_CODE_INVALID_AUDIO_SYNC_USERID_REPEATED,
      ErrorCode.overVideoPublishLimit:
          $p_a.ErrorCode.ERROR_CODE_OVER_VIDEO_PUBLISH_LIMIT,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.ErrorCode;
  }

  static ErrorCode android_to_code($p_a.ErrorCode value) {
    var $m = {
      $p_a.ErrorCode.ERROR_CODE_INVALID_TOKEN: ErrorCode.invalidToken,
      $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM: ErrorCode.joinRoom,
      $p_a.ErrorCode.ERROR_CODE_NO_PUBLISH_PERMISSION:
          ErrorCode.noPublishPermission,
      $p_a.ErrorCode.ERROR_CODE_NO_SUBSCRIBE_PERMISSION:
          ErrorCode.noSubscribePermission,
      $p_a.ErrorCode.ERROR_CODE_DUPLICATE_LOGIN: ErrorCode.duplicateLogin,
      $p_a.ErrorCode.ERROR_CODE_APP_ID_NULL: ErrorCode.appIdNull,
      $p_a.ErrorCode.ERROR_CODE_KICKED_OUT: ErrorCode.kickedOut,
      $p_a.ErrorCode.ERROR_CODE_ROOM_ID_ILLEGAL: ErrorCode.roomIdIllegal,
      $p_a.ErrorCode.ERROR_CODE_TOKEN_EXPIRED: ErrorCode.tokenExpired,
      $p_a.ErrorCode.ERROR_CODE_UPDATE_TOKEN_WITH_INVALID_TOKEN:
          ErrorCode.updateTokenWithInvalidToken,
      $p_a.ErrorCode.ERROR_CODE_ROOM_DISMISS: ErrorCode.roomDismiss,
      $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_WITHOUT_LICENSE_AUTHENTICATE_SDK:
          ErrorCode.joinRoomWithoutLicenseAuthenticateSDK,
      $p_a.ErrorCode.ERROR_CODE_ROOM_ALREADY_EXIST: ErrorCode.roomAlreadyExist,
      $p_a.ErrorCode.ERROR_CODE_USER_ID_DIFFERENT: ErrorCode.userIdDifferent,
      $p_a.ErrorCode.ERROR_CODE_SERVER_LICENSE_EXPIRE:
          ErrorCode.joinRoomServerLicenseExpired,
      $p_a.ErrorCode.ERROR_CODE_EXCEEDS_THE_UPPER_LIMIT:
          ErrorCode.joinRoomExceedsTheUpperLimit,
      $p_a.ErrorCode.ERROR_CODE_LICENSE_PARAMETER_ERROR:
          ErrorCode.joinRoomLicenseParameterError,
      $p_a.ErrorCode.ERROR_CODE_LICENSE_FILE_PATH_ERROR:
          ErrorCode.joinRoomLicenseFilePathError,
      $p_a.ErrorCode.ERROR_CODE_LICENSE_ILLEGAL:
          ErrorCode.joinRoomLicenseIllegal,
      $p_a.ErrorCode.ERROR_CODE_LICENSE_EXPIRED:
          ErrorCode.joinRoomLicenseExpired,
      $p_a.ErrorCode.ERROR_CODE_LICENSE_INFORMATION_NOT_MATCH:
          ErrorCode.joinRoomLicenseInformationNotMatch,
      $p_a.ErrorCode.ERROR_CODE_LICENSE_NOT_MATCH_WITH_CACHE:
          ErrorCode.joinRoomLicenseNotMatchWithCache,
      $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_ROOM_FORBIDDEN:
          ErrorCode.joinRoomRoomForbidden,
      $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_USER_FORBIDDEN:
          ErrorCode.joinRoomUserForbidden,
      $p_a.ErrorCode.ERROR_CODE_JOIN_ROOM_LICENSE_FUNCTION_NOT_FOUND:
          ErrorCode.joinRoomLicenseFunctionNotFound,
      $p_a.ErrorCode.ERROR_CODE_LOAD_SO_LIB: ErrorCode.loadSOLib,
      $p_a.ErrorCode.ERROR_CODE_OVER_STREAM_PUBLISH_LIMIT:
          ErrorCode.overStreamPublishLimit,
      $p_a.ErrorCode.ERROR_CODE_ABNORMAL_SERVER_STATUS:
          ErrorCode.abnormalServerStatus,
      $p_a.ErrorCode.ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED:
          ErrorCode.ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED,
      $p_a.ErrorCode.ERROR_CODE_WRONG_AREA_CODE:
          ErrorCode.ERROR_CODE_WRONG_AREA_CODE,
      $p_a.ErrorCode.ERROR_CODE_OVER_SUBSCRIBE_LIMIT:
          ErrorCode.overStreamSubscribeLimit,
      $p_a.ErrorCode.ERROR_CODE_OVER_SCREEN_PUBLISH_LIMIT:
          ErrorCode.overScreenPublishLimit,
      $p_a.ErrorCode.ERROR_CODE_INVALID_AUDIO_SYNC_USERID_REPEATED:
          ErrorCode.invalidAudioSyncUidRepeated,
      $p_a.ErrorCode.ERROR_CODE_OVER_VIDEO_PUBLISH_LIMIT:
          ErrorCode.overVideoPublishLimit,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as ErrorCode;
  }

  static $p_i.ByteRTCErrorCode code_to_ios(ErrorCode value) {
    var $m = {
      ErrorCode.invalidToken:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeInvalidToken,
      ErrorCode.joinRoom: $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoom,
      ErrorCode.noPublishPermission:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeNoPublishPermission,
      ErrorCode.noSubscribePermission:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeNoSubscribePermission,
      ErrorCode.duplicateLogin:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeDuplicateLogin,
      ErrorCode.kickedOut: $p_i.ByteRTCErrorCode.ByteRTCErrorCodeKickedOut,
      ErrorCode.roomIdIllegal:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeRoomIdIllegal,
      ErrorCode.tokenExpired:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeTokenExpired,
      ErrorCode.updateTokenWithInvalidToken:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeUpdateTokenWithInvalidToken,
      ErrorCode.roomDismiss: $p_i.ByteRTCErrorCode.ByteRTCErrorCodeRoomDismiss,
      ErrorCode.joinRoomWithoutLicenseAuthenticateSDK:
          $p_i.ByteRTCErrorCode.ByteRTCJoinRoomWithoutLicenseAuthenticateSDK,
      ErrorCode.roomAlreadyExist: $p_i.ByteRTCErrorCode.ByteRTCRoomAlreadyExist,
      ErrorCode.userIdDifferent: $p_i.ByteRTCErrorCode.ByteRTCUserIDDifferent,
      ErrorCode.joinRoomServerLicenseExpired:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomServerLicenseExpired,
      ErrorCode.joinRoomExceedsTheUpperLimit:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomExceedsTheUpperLimit,
      ErrorCode.joinRoomLicenseParameterError:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseParameterError,
      ErrorCode.joinRoomLicenseFilePathError:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseFilePathError,
      ErrorCode.joinRoomLicenseIllegal:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseIllegal,
      ErrorCode.joinRoomLicenseExpired:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseExpired,
      ErrorCode.joinRoomLicenseInformationNotMatch: $p_i
          .ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseInformationNotMatch,
      ErrorCode.joinRoomLicenseNotMatchWithCache: $p_i
          .ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseNotMatchWithCache,
      ErrorCode.joinRoomRoomForbidden:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomRoomForbidden,
      ErrorCode.joinRoomUserForbidden:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomUserForbidden,
      ErrorCode.overStreamPublishLimit:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeOverStreamPublishLimit,
      ErrorCode.abnormalServerStatus:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeAbnormalServerStatus,
      ErrorCode.ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeMultiRoomUnpublishFailed,
      ErrorCode.ERROR_CODE_WRONG_AREA_CODE:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeWrongAreaCode,
      ErrorCode.overStreamSubscribeLimit:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeOverStreamSubscribeLimit,
      ErrorCode.invalidAudioSyncUidRepeated:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodInvalidAudioSyncUidRepeated,
      ErrorCode.overVideoPublishLimit:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeOverVideoPublishLimit,
      ErrorCode.ByteRTCErrorCodeDeadLockNotify:
          $p_i.ByteRTCErrorCode.ByteRTCErrorCodeDeadLockNotify,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCErrorCode;
  }

  static ErrorCode ios_to_code($p_i.ByteRTCErrorCode value) {
    var $m = {
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeInvalidToken:
          ErrorCode.invalidToken,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoom: ErrorCode.joinRoom,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeNoPublishPermission:
          ErrorCode.noPublishPermission,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeNoSubscribePermission:
          ErrorCode.noSubscribePermission,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeDuplicateLogin:
          ErrorCode.duplicateLogin,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeKickedOut: ErrorCode.kickedOut,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeRoomIdIllegal:
          ErrorCode.roomIdIllegal,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeTokenExpired:
          ErrorCode.tokenExpired,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeUpdateTokenWithInvalidToken:
          ErrorCode.updateTokenWithInvalidToken,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeRoomDismiss: ErrorCode.roomDismiss,
      $p_i.ByteRTCErrorCode.ByteRTCJoinRoomWithoutLicenseAuthenticateSDK:
          ErrorCode.joinRoomWithoutLicenseAuthenticateSDK,
      $p_i.ByteRTCErrorCode.ByteRTCRoomAlreadyExist: ErrorCode.roomAlreadyExist,
      $p_i.ByteRTCErrorCode.ByteRTCUserIDDifferent: ErrorCode.userIdDifferent,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomServerLicenseExpired:
          ErrorCode.joinRoomServerLicenseExpired,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomExceedsTheUpperLimit:
          ErrorCode.joinRoomExceedsTheUpperLimit,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseParameterError:
          ErrorCode.joinRoomLicenseParameterError,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseFilePathError:
          ErrorCode.joinRoomLicenseFilePathError,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseIllegal:
          ErrorCode.joinRoomLicenseIllegal,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseExpired:
          ErrorCode.joinRoomLicenseExpired,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseInformationNotMatch:
          ErrorCode.joinRoomLicenseInformationNotMatch,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomLicenseNotMatchWithCache:
          ErrorCode.joinRoomLicenseNotMatchWithCache,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomRoomForbidden:
          ErrorCode.joinRoomRoomForbidden,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeJoinRoomUserForbidden:
          ErrorCode.joinRoomUserForbidden,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeOverStreamPublishLimit:
          ErrorCode.overStreamPublishLimit,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeAbnormalServerStatus:
          ErrorCode.abnormalServerStatus,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeMultiRoomUnpublishFailed:
          ErrorCode.ERROR_CODE_MULTI_ROOM_UNPUBLISH_FAILED,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeWrongAreaCode:
          ErrorCode.ERROR_CODE_WRONG_AREA_CODE,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeOverStreamSubscribeLimit:
          ErrorCode.overStreamSubscribeLimit,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodInvalidAudioSyncUidRepeated:
          ErrorCode.invalidAudioSyncUidRepeated,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeOverVideoPublishLimit:
          ErrorCode.overVideoPublishLimit,
      $p_i.ByteRTCErrorCode.ByteRTCErrorCodeDeadLockNotify:
          ErrorCode.ByteRTCErrorCodeDeadLockNotify,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as ErrorCode;
  }
}

class t_LoginErrorCode {
  static $p_a.LoginErrorCode code_to_android(LoginErrorCode value) {
    var $m = {
      LoginErrorCode.success: $p_a.LoginErrorCode.LOGIN_ERROR_CODE_SUCCESS,
      LoginErrorCode.invalid_token:
          $p_a.LoginErrorCode.LOGIN_ERROR_CODE_INVALID_TOKEN,
      LoginErrorCode.login_failed:
          $p_a.LoginErrorCode.LOGIN_ERROR_CODE_LOGIN_FAILED,
      LoginErrorCode.invalid_user_id:
          $p_a.LoginErrorCode.LOGIN_ERROR_CODE_INVALID_USER_ID,
      LoginErrorCode.code_server_error:
          $p_a.LoginErrorCode.LOGIN_ERROR_CODE_SERVER_ERROR,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.LoginErrorCode;
  }

  static LoginErrorCode android_to_code($p_a.LoginErrorCode value) {
    var $m = {
      $p_a.LoginErrorCode.LOGIN_ERROR_CODE_SUCCESS: LoginErrorCode.success,
      $p_a.LoginErrorCode.LOGIN_ERROR_CODE_INVALID_TOKEN:
          LoginErrorCode.invalid_token,
      $p_a.LoginErrorCode.LOGIN_ERROR_CODE_LOGIN_FAILED:
          LoginErrorCode.login_failed,
      $p_a.LoginErrorCode.LOGIN_ERROR_CODE_INVALID_USER_ID:
          LoginErrorCode.invalid_user_id,
      $p_a.LoginErrorCode.LOGIN_ERROR_CODE_SERVER_ERROR:
          LoginErrorCode.code_server_error,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as LoginErrorCode;
  }

  static $p_i.ByteRTCLoginErrorCode code_to_ios(LoginErrorCode value) {
    var $m = {
      LoginErrorCode.success:
          $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeSuccess,
      LoginErrorCode.invalid_token:
          $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeInvalidToken,
      LoginErrorCode.login_failed:
          $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeLoginFailed,
      LoginErrorCode.invalid_user_id:
          $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeInvalidUserId,
      LoginErrorCode.code_server_error:
          $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeServerError,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCLoginErrorCode;
  }

  static LoginErrorCode ios_to_code($p_i.ByteRTCLoginErrorCode value) {
    var $m = {
      $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeSuccess:
          LoginErrorCode.success,
      $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeInvalidToken:
          LoginErrorCode.invalid_token,
      $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeLoginFailed:
          LoginErrorCode.login_failed,
      $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeInvalidUserId:
          LoginErrorCode.invalid_user_id,
      $p_i.ByteRTCLoginErrorCode.ByteRTCLoginErrorCodeServerError:
          LoginErrorCode.code_server_error,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as LoginErrorCode;
  }
}

class t_AudioRecordingErrorCode {
  static $p_a.AudioRecordingErrorCode code_to_android(
      AudioRecordingErrorCode value) {
    var $m = {
      AudioRecordingErrorCode.ok:
          $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_OK,
      AudioRecordingErrorCode.no_permission:
          $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NO_PERMISSION,
      AudioRecordingErrorCode.not_in_room:
          $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NOT_IN_ROOM,
      AudioRecordingErrorCode.already_started: $p_a
          .AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_ALREADY_STARTED,
      AudioRecordingErrorCode.not_started:
          $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NOT_STARTED,
      AudioRecordingErrorCode.not_support:
          $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NOT_SUPPORT,
      AudioRecordingErrorCode.other:
          $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_OTHER,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.AudioRecordingErrorCode;
  }

  static AudioRecordingErrorCode android_to_code(
      $p_a.AudioRecordingErrorCode value) {
    var $m = {
      $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_OK:
          AudioRecordingErrorCode.ok,
      $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NO_PERMISSION:
          AudioRecordingErrorCode.no_permission,
      $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NOT_IN_ROOM:
          AudioRecordingErrorCode.not_in_room,
      $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_ALREADY_STARTED:
          AudioRecordingErrorCode.already_started,
      $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NOT_STARTED:
          AudioRecordingErrorCode.not_started,
      $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_NOT_SUPPORT:
          AudioRecordingErrorCode.not_support,
      $p_a.AudioRecordingErrorCode.AUDIO_RECORDING_ERROR_CODE_OTHER:
          AudioRecordingErrorCode.other,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as AudioRecordingErrorCode;
  }

  static $p_i.ByteRTCAudioRecordingErrorCode code_to_ios(
      AudioRecordingErrorCode value) {
    var $m = {
      AudioRecordingErrorCode.ok:
          $p_i.ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingErrorCodeOk,
      AudioRecordingErrorCode.no_permission: $p_i.ByteRTCAudioRecordingErrorCode
          .ByteRTCAudioRecordingErrorCodeNoPermission,
      AudioRecordingErrorCode.not_in_room: $p_i
          .ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingErrorNotInRoom,
      AudioRecordingErrorCode.already_started: $p_i
          .ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingAlreadyStarted,
      AudioRecordingErrorCode.not_started:
          $p_i.ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingNotStarted,
      AudioRecordingErrorCode.not_support: $p_i.ByteRTCAudioRecordingErrorCode
          .ByteRTCAudioRecordingErrorCodeNotSupport,
      AudioRecordingErrorCode.other: $p_i
          .ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingErrorCodeOther,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCAudioRecordingErrorCode;
  }

  static AudioRecordingErrorCode ios_to_code(
      $p_i.ByteRTCAudioRecordingErrorCode value) {
    var $m = {
      $p_i.ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingErrorCodeOk:
          AudioRecordingErrorCode.ok,
      $p_i.ByteRTCAudioRecordingErrorCode
              .ByteRTCAudioRecordingErrorCodeNoPermission:
          AudioRecordingErrorCode.no_permission,
      $p_i.ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingErrorNotInRoom:
          AudioRecordingErrorCode.not_in_room,
      $p_i.ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingAlreadyStarted:
          AudioRecordingErrorCode.already_started,
      $p_i.ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingNotStarted:
          AudioRecordingErrorCode.not_started,
      $p_i.ByteRTCAudioRecordingErrorCode
              .ByteRTCAudioRecordingErrorCodeNotSupport:
          AudioRecordingErrorCode.not_support,
      $p_i.ByteRTCAudioRecordingErrorCode.ByteRTCAudioRecordingErrorCodeOther:
          AudioRecordingErrorCode.other,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as AudioRecordingErrorCode;
  }
}

class t_MixedStreamTaskErrorCode {
  static $p_a.MixedStreamTaskErrorCode code_to_android(
      MixedStreamTaskErrorCode value) {
    var $m = {
      MixedStreamTaskErrorCode.ok: $p_a.MixedStreamTaskErrorCode.OK,
      MixedStreamTaskErrorCode.base: $p_a.MixedStreamTaskErrorCode.BASE,
      MixedStreamTaskErrorCode.timeout: $p_a.MixedStreamTaskErrorCode.TIMEOUT,
      MixedStreamTaskErrorCode.invalid_param_by_server:
          $p_a.MixedStreamTaskErrorCode.INVALID_PARAM_BY_SERVER,
      MixedStreamTaskErrorCode.sub_timeout_by_server:
          $p_a.MixedStreamTaskErrorCode.SUB_TIMEOUT_BY_SERVER,
      MixedStreamTaskErrorCode.invalid_state_by_server:
          $p_a.MixedStreamTaskErrorCode.INVALID_STATE_BY_SERVER,
      MixedStreamTaskErrorCode.authentication_by_cdn:
          $p_a.MixedStreamTaskErrorCode.AUTHENTICATION_BY_CDN,
      MixedStreamTaskErrorCode.unknown_by_server:
          $p_a.MixedStreamTaskErrorCode.UNKNOWN_BY_SERVER,
      MixedStreamTaskErrorCode.signal_request_timeout:
          $p_a.MixedStreamTaskErrorCode.SIGNAL_REQUEST_TIMEOUT,
      MixedStreamTaskErrorCode.mix_image_fail:
          $p_a.MixedStreamTaskErrorCode.MIX_IMAGE_FAIL,
      MixedStreamTaskErrorCode.stream_sync_worse:
          $p_a.MixedStreamTaskErrorCode.STREAM_SYNC_WORSE,
      MixedStreamTaskErrorCode.push_wtn_failed:
          $p_a.MixedStreamTaskErrorCode.PUSH_WTN_FAILED,
      MixedStreamTaskErrorCode.max: $p_a.MixedStreamTaskErrorCode.MAX,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.MixedStreamTaskErrorCode;
  }

  static MixedStreamTaskErrorCode android_to_code(
      $p_a.MixedStreamTaskErrorCode value) {
    var $m = {
      $p_a.MixedStreamTaskErrorCode.OK: MixedStreamTaskErrorCode.ok,
      $p_a.MixedStreamTaskErrorCode.BASE: MixedStreamTaskErrorCode.base,
      $p_a.MixedStreamTaskErrorCode.TIMEOUT: MixedStreamTaskErrorCode.timeout,
      $p_a.MixedStreamTaskErrorCode.INVALID_PARAM_BY_SERVER:
          MixedStreamTaskErrorCode.invalid_param_by_server,
      $p_a.MixedStreamTaskErrorCode.SUB_TIMEOUT_BY_SERVER:
          MixedStreamTaskErrorCode.sub_timeout_by_server,
      $p_a.MixedStreamTaskErrorCode.INVALID_STATE_BY_SERVER:
          MixedStreamTaskErrorCode.invalid_state_by_server,
      $p_a.MixedStreamTaskErrorCode.AUTHENTICATION_BY_CDN:
          MixedStreamTaskErrorCode.authentication_by_cdn,
      $p_a.MixedStreamTaskErrorCode.UNKNOWN_BY_SERVER:
          MixedStreamTaskErrorCode.unknown_by_server,
      $p_a.MixedStreamTaskErrorCode.SIGNAL_REQUEST_TIMEOUT:
          MixedStreamTaskErrorCode.signal_request_timeout,
      $p_a.MixedStreamTaskErrorCode.MIX_IMAGE_FAIL:
          MixedStreamTaskErrorCode.mix_image_fail,
      $p_a.MixedStreamTaskErrorCode.STREAM_SYNC_WORSE:
          MixedStreamTaskErrorCode.stream_sync_worse,
      $p_a.MixedStreamTaskErrorCode.PUSH_WTN_FAILED:
          MixedStreamTaskErrorCode.push_wtn_failed,
      $p_a.MixedStreamTaskErrorCode.MAX: MixedStreamTaskErrorCode.max,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as MixedStreamTaskErrorCode;
  }

  static $p_i.ByteRTCMixedStreamTaskErrorCode code_to_ios(
      MixedStreamTaskErrorCode value) {
    var $m = {
      MixedStreamTaskErrorCode.ok: $p_i
          .ByteRTCMixedStreamTaskErrorCode.ByteRTCMixedStreamTaskErrorCodeOK,
      MixedStreamTaskErrorCode.base: $p_i
          .ByteRTCMixedStreamTaskErrorCode.ByteRTCMixedStreamTaskErrorCodeBase,
      MixedStreamTaskErrorCode.timeout: $p_i.ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeTimeOut,
      MixedStreamTaskErrorCode.invalid_param_by_server: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeInvalidParamByServer,
      MixedStreamTaskErrorCode.sub_timeout_by_server: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeSubTimeoutByServer,
      MixedStreamTaskErrorCode.invalid_state_by_server: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeInvalidStateByServer,
      MixedStreamTaskErrorCode.authentication_by_cdn: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeAuthenticationByCDN,
      MixedStreamTaskErrorCode.unknown_by_server: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeUnKnownErrorByServer,
      MixedStreamTaskErrorCode.signal_request_timeout: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeSignalRequestTimeout,
      MixedStreamTaskErrorCode.mix_image_fail: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeMixImageFailed,
      MixedStreamTaskErrorCode.stream_sync_worse: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodeStreamSyncWorse,
      MixedStreamTaskErrorCode.push_wtn_failed: $p_i
          .ByteRTCMixedStreamTaskErrorCode
          .ByteRTCMixedStreamTaskErrorCodePushWTNFailed,
      MixedStreamTaskErrorCode.max: $p_i
          .ByteRTCMixedStreamTaskErrorCode.ByteRTCMixedStreamTaskErrorCodeMax,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCMixedStreamTaskErrorCode;
  }

  static MixedStreamTaskErrorCode ios_to_code(
      $p_i.ByteRTCMixedStreamTaskErrorCode value) {
    var $m = {
      $p_i.ByteRTCMixedStreamTaskErrorCode.ByteRTCMixedStreamTaskErrorCodeOK:
          MixedStreamTaskErrorCode.ok,
      $p_i.ByteRTCMixedStreamTaskErrorCode.ByteRTCMixedStreamTaskErrorCodeBase:
          MixedStreamTaskErrorCode.base,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeTimeOut:
          MixedStreamTaskErrorCode.timeout,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeInvalidParamByServer:
          MixedStreamTaskErrorCode.invalid_param_by_server,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeSubTimeoutByServer:
          MixedStreamTaskErrorCode.sub_timeout_by_server,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeInvalidStateByServer:
          MixedStreamTaskErrorCode.invalid_state_by_server,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeAuthenticationByCDN:
          MixedStreamTaskErrorCode.authentication_by_cdn,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeUnKnownErrorByServer:
          MixedStreamTaskErrorCode.unknown_by_server,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeSignalRequestTimeout:
          MixedStreamTaskErrorCode.signal_request_timeout,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeMixImageFailed:
          MixedStreamTaskErrorCode.mix_image_fail,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodeStreamSyncWorse:
          MixedStreamTaskErrorCode.stream_sync_worse,
      $p_i.ByteRTCMixedStreamTaskErrorCode
              .ByteRTCMixedStreamTaskErrorCodePushWTNFailed:
          MixedStreamTaskErrorCode.push_wtn_failed,
      $p_i.ByteRTCMixedStreamTaskErrorCode.ByteRTCMixedStreamTaskErrorCodeMax:
          MixedStreamTaskErrorCode.max,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as MixedStreamTaskErrorCode;
  }
}

class t_SingleStreamTaskEvent {
  static $p_a.SingleStreamTaskEvent code_to_android(
      SingleStreamTaskEvent value) {
    var $m = {
      SingleStreamTaskEvent.base: $p_a.SingleStreamTaskEvent.BASE,
      SingleStreamTaskEvent.start_success:
          $p_a.SingleStreamTaskEvent.START_SUCCESS,
      SingleStreamTaskEvent.start_failed:
          $p_a.SingleStreamTaskEvent.START_FAILED,
      SingleStreamTaskEvent.stop_success:
          $p_a.SingleStreamTaskEvent.STOP_SUCCESS,
      SingleStreamTaskEvent.stop_failed: $p_a.SingleStreamTaskEvent.STOP_FAILED,
      SingleStreamTaskEvent.warning: $p_a.SingleStreamTaskEvent.WARNING,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.SingleStreamTaskEvent;
  }

  static SingleStreamTaskEvent android_to_code(
      $p_a.SingleStreamTaskEvent value) {
    var $m = {
      $p_a.SingleStreamTaskEvent.BASE: SingleStreamTaskEvent.base,
      $p_a.SingleStreamTaskEvent.START_SUCCESS:
          SingleStreamTaskEvent.start_success,
      $p_a.SingleStreamTaskEvent.START_FAILED:
          SingleStreamTaskEvent.start_failed,
      $p_a.SingleStreamTaskEvent.STOP_SUCCESS:
          SingleStreamTaskEvent.stop_success,
      $p_a.SingleStreamTaskEvent.STOP_FAILED: SingleStreamTaskEvent.stop_failed,
      $p_a.SingleStreamTaskEvent.WARNING: SingleStreamTaskEvent.warning,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as SingleStreamTaskEvent;
  }

  static $p_i.ByteRTCSingleStreamTaskEvent code_to_ios(
      SingleStreamTaskEvent value) {
    var $m = {
      SingleStreamTaskEvent.base:
          $p_i.ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventBase,
      SingleStreamTaskEvent.start_success: $p_i.ByteRTCSingleStreamTaskEvent
          .ByteRTCSingleStreamTaskEventStartSuccess,
      SingleStreamTaskEvent.start_failed: $p_i
          .ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventStartFailed,
      SingleStreamTaskEvent.stop_success: $p_i
          .ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventStopSuccess,
      SingleStreamTaskEvent.stop_failed: $p_i
          .ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventStopFailed,
      SingleStreamTaskEvent.warning:
          $p_i.ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventWarning,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCSingleStreamTaskEvent;
  }

  static SingleStreamTaskEvent ios_to_code(
      $p_i.ByteRTCSingleStreamTaskEvent value) {
    var $m = {
      $p_i.ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventBase:
          SingleStreamTaskEvent.base,
      $p_i.ByteRTCSingleStreamTaskEvent
              .ByteRTCSingleStreamTaskEventStartSuccess:
          SingleStreamTaskEvent.start_success,
      $p_i.ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventStartFailed:
          SingleStreamTaskEvent.start_failed,
      $p_i.ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventStopSuccess:
          SingleStreamTaskEvent.stop_success,
      $p_i.ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventStopFailed:
          SingleStreamTaskEvent.stop_failed,
      $p_i.ByteRTCSingleStreamTaskEvent.ByteRTCSingleStreamTaskEventWarning:
          SingleStreamTaskEvent.warning,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as SingleStreamTaskEvent;
  }
}

class t_SingleStreamTaskErrorCode {
  static $p_a.SingleStreamTaskErrorCode code_to_android(
      SingleStreamTaskErrorCode value) {
    var $m = {
      SingleStreamTaskErrorCode.ok: $p_a.SingleStreamTaskErrorCode.OK,
      SingleStreamTaskErrorCode.base: $p_a.SingleStreamTaskErrorCode.BASE,
      SingleStreamTaskErrorCode.unknown_by_server:
          $p_a.SingleStreamTaskErrorCode.UNKNOWN_BY_SERVER,
      SingleStreamTaskErrorCode.signal_request_timeout:
          $p_a.SingleStreamTaskErrorCode.SIGNAL_REQUEST_TIMEOUT,
      SingleStreamTaskErrorCode.invalid_param_by_server:
          $p_a.SingleStreamTaskErrorCode.INVALID_PARAM_BY_SERVER,
      SingleStreamTaskErrorCode.remote_kicked:
          $p_a.SingleStreamTaskErrorCode.REMOTE_KICKED,
      SingleStreamTaskErrorCode.join_dest_room_failed:
          $p_a.SingleStreamTaskErrorCode.JOIN_DEST_ROOM_FAIED,
      SingleStreamTaskErrorCode.receive_src_stream_timeout:
          $p_a.SingleStreamTaskErrorCode.RECEIVE_SRC_STREAM_TIMEOUT,
      SingleStreamTaskErrorCode.not_surport_codec:
          $p_a.SingleStreamTaskErrorCode.NOT_SURPORT_CODEC,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.SingleStreamTaskErrorCode;
  }

  static SingleStreamTaskErrorCode android_to_code(
      $p_a.SingleStreamTaskErrorCode value) {
    var $m = {
      $p_a.SingleStreamTaskErrorCode.OK: SingleStreamTaskErrorCode.ok,
      $p_a.SingleStreamTaskErrorCode.BASE: SingleStreamTaskErrorCode.base,
      $p_a.SingleStreamTaskErrorCode.UNKNOWN_BY_SERVER:
          SingleStreamTaskErrorCode.unknown_by_server,
      $p_a.SingleStreamTaskErrorCode.SIGNAL_REQUEST_TIMEOUT:
          SingleStreamTaskErrorCode.signal_request_timeout,
      $p_a.SingleStreamTaskErrorCode.INVALID_PARAM_BY_SERVER:
          SingleStreamTaskErrorCode.invalid_param_by_server,
      $p_a.SingleStreamTaskErrorCode.REMOTE_KICKED:
          SingleStreamTaskErrorCode.remote_kicked,
      $p_a.SingleStreamTaskErrorCode.JOIN_DEST_ROOM_FAIED:
          SingleStreamTaskErrorCode.join_dest_room_failed,
      $p_a.SingleStreamTaskErrorCode.RECEIVE_SRC_STREAM_TIMEOUT:
          SingleStreamTaskErrorCode.receive_src_stream_timeout,
      $p_a.SingleStreamTaskErrorCode.NOT_SURPORT_CODEC:
          SingleStreamTaskErrorCode.not_surport_codec,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as SingleStreamTaskErrorCode;
  }

  static $p_i.ByteRTCSingleStreamTaskErrorCode code_to_ios(
      SingleStreamTaskErrorCode value) {
    var $m = {
      SingleStreamTaskErrorCode.ok: $p_i
          .ByteRTCSingleStreamTaskErrorCode.ByteRTCSingleStreamTaskErrorCodeOK,
      SingleStreamTaskErrorCode.base: $p_i.ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeBase,
      SingleStreamTaskErrorCode.unknown_by_server: $p_i
          .ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeUnknownByServer,
      SingleStreamTaskErrorCode.signal_request_timeout: $p_i
          .ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeSignalRequestTimeout,
      SingleStreamTaskErrorCode.invalid_param_by_server: $p_i
          .ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeInvalidParamByServer,
      SingleStreamTaskErrorCode.remote_kicked: $p_i
          .ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeRemoteKicked,
      SingleStreamTaskErrorCode.join_dest_room_failed: $p_i
          .ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeJoinDestRoomFailed,
      SingleStreamTaskErrorCode.receive_src_stream_timeout: $p_i
          .ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeReceiveSrcStreamTimeout,
      SingleStreamTaskErrorCode.not_surport_codec: $p_i
          .ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeNotSurportCodec,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCSingleStreamTaskErrorCode;
  }

  static SingleStreamTaskErrorCode ios_to_code(
      $p_i.ByteRTCSingleStreamTaskErrorCode value) {
    var $m = {
      $p_i.ByteRTCSingleStreamTaskErrorCode.ByteRTCSingleStreamTaskErrorCodeOK:
          SingleStreamTaskErrorCode.ok,
      $p_i.ByteRTCSingleStreamTaskErrorCode
          .ByteRTCSingleStreamTaskErrorCodeBase: SingleStreamTaskErrorCode.base,
      $p_i.ByteRTCSingleStreamTaskErrorCode
              .ByteRTCSingleStreamTaskErrorCodeUnknownByServer:
          SingleStreamTaskErrorCode.unknown_by_server,
      $p_i.ByteRTCSingleStreamTaskErrorCode
              .ByteRTCSingleStreamTaskErrorCodeSignalRequestTimeout:
          SingleStreamTaskErrorCode.signal_request_timeout,
      $p_i.ByteRTCSingleStreamTaskErrorCode
              .ByteRTCSingleStreamTaskErrorCodeInvalidParamByServer:
          SingleStreamTaskErrorCode.invalid_param_by_server,
      $p_i.ByteRTCSingleStreamTaskErrorCode
              .ByteRTCSingleStreamTaskErrorCodeRemoteKicked:
          SingleStreamTaskErrorCode.remote_kicked,
      $p_i.ByteRTCSingleStreamTaskErrorCode
              .ByteRTCSingleStreamTaskErrorCodeJoinDestRoomFailed:
          SingleStreamTaskErrorCode.join_dest_room_failed,
      $p_i.ByteRTCSingleStreamTaskErrorCode
              .ByteRTCSingleStreamTaskErrorCodeReceiveSrcStreamTimeout:
          SingleStreamTaskErrorCode.receive_src_stream_timeout,
      $p_i.ByteRTCSingleStreamTaskErrorCode
              .ByteRTCSingleStreamTaskErrorCodeNotSurportCodec:
          SingleStreamTaskErrorCode.not_surport_codec,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as SingleStreamTaskErrorCode;
  }
}

class t_PublicStreamErrorCode {
  static $p_a.PublicStreamErrorCode code_to_android(
      PublicStreamErrorCode value) {
    var $m = {
      PublicStreamErrorCode.success:
          $p_a.PublicStreamErrorCode.ERROR_CODE_SUCCESS,
      PublicStreamErrorCode.pushParamError:
          $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_PARAM_ERROR,
      PublicStreamErrorCode.pushStateError:
          $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_STATE_ERROR,
      PublicStreamErrorCode.pushInternalError:
          $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_INTERNAL_ERROR,
      PublicStreamErrorCode.pushError:
          $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_ERROR,
      PublicStreamErrorCode.pushTimeOut:
          $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_TIMEOUT,
      PublicStreamErrorCode.pullNoPushStream:
          $p_a.PublicStreamErrorCode.ERROR_CODE_PULL_NO_PUSH_STREAM,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.PublicStreamErrorCode;
  }

  static PublicStreamErrorCode android_to_code(
      $p_a.PublicStreamErrorCode value) {
    var $m = {
      $p_a.PublicStreamErrorCode.ERROR_CODE_SUCCESS:
          PublicStreamErrorCode.success,
      $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_PARAM_ERROR:
          PublicStreamErrorCode.pushParamError,
      $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_STATE_ERROR:
          PublicStreamErrorCode.pushStateError,
      $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_INTERNAL_ERROR:
          PublicStreamErrorCode.pushInternalError,
      $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_ERROR:
          PublicStreamErrorCode.pushError,
      $p_a.PublicStreamErrorCode.ERROR_CODE_PUSH_TIMEOUT:
          PublicStreamErrorCode.pushTimeOut,
      $p_a.PublicStreamErrorCode.ERROR_CODE_PULL_NO_PUSH_STREAM:
          PublicStreamErrorCode.pullNoPushStream,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as PublicStreamErrorCode;
  }

  static $p_i.ByteRTCPublicStreamErrorCode code_to_ios(
      PublicStreamErrorCode value) {
    var $m = {
      PublicStreamErrorCode.success:
          $p_i.ByteRTCPublicStreamErrorCode.ByteRTCPublicStreamErrorCodeSuccess,
      PublicStreamErrorCode.pushParamError: $p_i.ByteRTCPublicStreamErrorCode
          .ByteRTCPublicStreamErrorCodePushParamError,
      PublicStreamErrorCode.pushStateError: $p_i.ByteRTCPublicStreamErrorCode
          .ByteRTCPublicStreamErrorCodePushStatusError,
      PublicStreamErrorCode.pushInternalError: $p_i.ByteRTCPublicStreamErrorCode
          .ByteRTCPublicStreamErrorCodePushInternalError,
      PublicStreamErrorCode.pushError: $p_i
          .ByteRTCPublicStreamErrorCode.ByteRTCPublicStreamErrorCodePushError,
      PublicStreamErrorCode.pushTimeOut: $p_i
          .ByteRTCPublicStreamErrorCode.ByteRTCPublicStreamErrorCodePushTimeOut,
      PublicStreamErrorCode.pullNoPushStream: $p_i.ByteRTCPublicStreamErrorCode
          .ByteRTCPublicStreamErrorCodePullNoPushStream,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCPublicStreamErrorCode;
  }

  static PublicStreamErrorCode ios_to_code(
      $p_i.ByteRTCPublicStreamErrorCode value) {
    var $m = {
      $p_i.ByteRTCPublicStreamErrorCode.ByteRTCPublicStreamErrorCodeSuccess:
          PublicStreamErrorCode.success,
      $p_i.ByteRTCPublicStreamErrorCode
              .ByteRTCPublicStreamErrorCodePushParamError:
          PublicStreamErrorCode.pushParamError,
      $p_i.ByteRTCPublicStreamErrorCode
              .ByteRTCPublicStreamErrorCodePushStatusError:
          PublicStreamErrorCode.pushStateError,
      $p_i.ByteRTCPublicStreamErrorCode
              .ByteRTCPublicStreamErrorCodePushInternalError:
          PublicStreamErrorCode.pushInternalError,
      $p_i.ByteRTCPublicStreamErrorCode.ByteRTCPublicStreamErrorCodePushError:
          PublicStreamErrorCode.pushError,
      $p_i.ByteRTCPublicStreamErrorCode.ByteRTCPublicStreamErrorCodePushTimeOut:
          PublicStreamErrorCode.pushTimeOut,
      $p_i.ByteRTCPublicStreamErrorCode
              .ByteRTCPublicStreamErrorCodePullNoPushStream:
          PublicStreamErrorCode.pullNoPushStream,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as PublicStreamErrorCode;
  }
}

class t_KTVErrorCode {
  static $p_a.KTVErrorCode code_to_android(KTVErrorCode value) {
    var $m = {
      KTVErrorCode.ok: $p_a.KTVErrorCode.OK,
      KTVErrorCode.appid_invalid: $p_a.KTVErrorCode.APPID_INVALID,
      KTVErrorCode.paras_invalid: $p_a.KTVErrorCode.PARAS_INVALID,
      KTVErrorCode.get_music_failed: $p_a.KTVErrorCode.GET_MUSIC_FAILED,
      KTVErrorCode.get_lyric_failed: $p_a.KTVErrorCode.GET_LYRIC_FAILED,
      KTVErrorCode.music_takedown: $p_a.KTVErrorCode.MUSIC_TAKEDOWN,
      KTVErrorCode.music_download: $p_a.KTVErrorCode.MUSIC_DOWNLOAD,
      KTVErrorCode.midi_download_failed: $p_a.KTVErrorCode.MIDI_DOWNLOAD_FAILED,
      KTVErrorCode.system_busy: $p_a.KTVErrorCode.SYSTEM_BUSY,
      KTVErrorCode.network: $p_a.KTVErrorCode.NETWORK,
      KTVErrorCode.not_join_room: $p_a.KTVErrorCode.NOT_JOIN_ROOM,
      KTVErrorCode.parse_data: $p_a.KTVErrorCode.PARSE_DATA,
      KTVErrorCode.downloading: $p_a.KTVErrorCode.DOWNLOADING,
      KTVErrorCode.insufficient_disk_space:
          $p_a.KTVErrorCode.INSUFFICIENT_DISK_SPACE,
      KTVErrorCode.music_decryption_failed:
          $p_a.KTVErrorCode.MUSIC_DECRYPTION_FAILED,
      KTVErrorCode.file_rename_failed: $p_a.KTVErrorCode.FILE_RENAME_FAILED,
      KTVErrorCode.download_timeout: $p_a.KTVErrorCode.DOWNLOAD_TIMEOUT,
      KTVErrorCode.clear_cache_failed: $p_a.KTVErrorCode.CLEAR_CACHE_FAILED,
      KTVErrorCode.download_canceled: $p_a.KTVErrorCode.DOWNLOAD_CANCELED,
      KTVErrorCode.download: $p_a.KTVErrorCode.DOWNLOAD,
      KTVErrorCode.INTERNAL_DOMAIN: $p_a.KTVErrorCode.INTERNAL_DOMAIN,
    };
    if (!($m.containsKey(value))) {
      throw Exception("android not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_a.KTVErrorCode;
  }

  static KTVErrorCode android_to_code($p_a.KTVErrorCode value) {
    var $m = {
      $p_a.KTVErrorCode.OK: KTVErrorCode.ok,
      $p_a.KTVErrorCode.APPID_INVALID: KTVErrorCode.appid_invalid,
      $p_a.KTVErrorCode.PARAS_INVALID: KTVErrorCode.paras_invalid,
      $p_a.KTVErrorCode.GET_MUSIC_FAILED: KTVErrorCode.get_music_failed,
      $p_a.KTVErrorCode.GET_LYRIC_FAILED: KTVErrorCode.get_lyric_failed,
      $p_a.KTVErrorCode.MUSIC_TAKEDOWN: KTVErrorCode.music_takedown,
      $p_a.KTVErrorCode.MUSIC_DOWNLOAD: KTVErrorCode.music_download,
      $p_a.KTVErrorCode.MIDI_DOWNLOAD_FAILED: KTVErrorCode.midi_download_failed,
      $p_a.KTVErrorCode.SYSTEM_BUSY: KTVErrorCode.system_busy,
      $p_a.KTVErrorCode.NETWORK: KTVErrorCode.network,
      $p_a.KTVErrorCode.NOT_JOIN_ROOM: KTVErrorCode.not_join_room,
      $p_a.KTVErrorCode.PARSE_DATA: KTVErrorCode.parse_data,
      $p_a.KTVErrorCode.DOWNLOADING: KTVErrorCode.downloading,
      $p_a.KTVErrorCode.INSUFFICIENT_DISK_SPACE:
          KTVErrorCode.insufficient_disk_space,
      $p_a.KTVErrorCode.MUSIC_DECRYPTION_FAILED:
          KTVErrorCode.music_decryption_failed,
      $p_a.KTVErrorCode.FILE_RENAME_FAILED: KTVErrorCode.file_rename_failed,
      $p_a.KTVErrorCode.DOWNLOAD_TIMEOUT: KTVErrorCode.download_timeout,
      $p_a.KTVErrorCode.CLEAR_CACHE_FAILED: KTVErrorCode.clear_cache_failed,
      $p_a.KTVErrorCode.DOWNLOAD_CANCELED: KTVErrorCode.download_canceled,
      $p_a.KTVErrorCode.DOWNLOAD: KTVErrorCode.download,
      $p_a.KTVErrorCode.INTERNAL_DOMAIN: KTVErrorCode.INTERNAL_DOMAIN,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as KTVErrorCode;
  }

  static $p_i.ByteRTCKTVErrorCode code_to_ios(KTVErrorCode value) {
    var $m = {
      KTVErrorCode.ok: $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeOK,
      KTVErrorCode.appid_invalid:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeAppidInValid,
      KTVErrorCode.paras_invalid:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeParasInValid,
      KTVErrorCode.get_music_failed:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeGetMusicFailed,
      KTVErrorCode.get_lyric_failed:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeGetLyricFailed,
      KTVErrorCode.music_takedown:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMusicTakedown,
      KTVErrorCode.music_download:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMusicDownload,
      KTVErrorCode.midi_download_failed:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMidiDownloadFailed,
      KTVErrorCode.system_busy:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeSystemBusy,
      KTVErrorCode.network: $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeNetwork,
      KTVErrorCode.not_join_room:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeNotJoinRoom,
      KTVErrorCode.parse_data:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeParseData,
      KTVErrorCode.downloading:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownloading,
      KTVErrorCode.insufficient_disk_space:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeInsufficientDiskSpace,
      KTVErrorCode.music_decryption_failed:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMusicDecryptionFailed,
      KTVErrorCode.file_rename_failed:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeFileRenameFailed,
      KTVErrorCode.download_timeout:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownloadTimeOut,
      KTVErrorCode.clear_cache_failed:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeClearCacheFailed,
      KTVErrorCode.download_canceled:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownloadCanceled,
      KTVErrorCode.download:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownload,
      KTVErrorCode.ByteRTCKTVErrorCodeInternal:
          $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeInternal,
    };
    if (!($m.containsKey(value))) {
      throw Exception("ios not support:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as $p_i.ByteRTCKTVErrorCode;
  }

  static KTVErrorCode ios_to_code($p_i.ByteRTCKTVErrorCode value) {
    var $m = {
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeOK: KTVErrorCode.ok,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeAppidInValid:
          KTVErrorCode.appid_invalid,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeParasInValid:
          KTVErrorCode.paras_invalid,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeGetMusicFailed:
          KTVErrorCode.get_music_failed,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeGetLyricFailed:
          KTVErrorCode.get_lyric_failed,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMusicTakedown:
          KTVErrorCode.music_takedown,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMusicDownload:
          KTVErrorCode.music_download,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMidiDownloadFailed:
          KTVErrorCode.midi_download_failed,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeSystemBusy:
          KTVErrorCode.system_busy,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeNetwork: KTVErrorCode.network,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeNotJoinRoom:
          KTVErrorCode.not_join_room,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeParseData:
          KTVErrorCode.parse_data,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownloading:
          KTVErrorCode.downloading,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeInsufficientDiskSpace:
          KTVErrorCode.insufficient_disk_space,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeMusicDecryptionFailed:
          KTVErrorCode.music_decryption_failed,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeFileRenameFailed:
          KTVErrorCode.file_rename_failed,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownloadTimeOut:
          KTVErrorCode.download_timeout,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeClearCacheFailed:
          KTVErrorCode.clear_cache_failed,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownloadCanceled:
          KTVErrorCode.download_canceled,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeDownload:
          KTVErrorCode.download,
      $p_i.ByteRTCKTVErrorCode.ByteRTCKTVErrorCodeInternal:
          KTVErrorCode.ByteRTCKTVErrorCodeInternal,
    };
    if (!($m.containsKey(value))) {
      throw Exception("invalid value:" + value.toString());
    }
    // @ts-ignore
    return $m[value] as KTVErrorCode;
  }
}
