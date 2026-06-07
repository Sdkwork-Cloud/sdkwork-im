/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';

enum ByteRTCRoomMessageSendResult {
  /// @brief 消息发送成功
  ///
  ByteRTCRoomMessageSendResultSuccess(200),

  /// @brief 发送超时，没有发送
  ///
  ByteRTCRoomMessageSendResultTimeout(1),

  /// @brief 通道断开，没有发送
  ///
  ByteRTCRoomMessageSendResultNetworkDisconnected(2),

  /// @brief 超过 QPS 限制
  ///
  ByteRTCRoomMessageSendResultExceedQPS(5),

  /// @brief 失败，发送方未加入房间
  ///
  ByteRTCRoomMessageSendResultNotJoin(100),

  /// @brief 失败，连接未完成初始化。
  ///
  ByteRTCRoomMessageSendResultInit(101),

  /// @brief 失败，没有可用的连接。
  ///
  ByteRTCRoomMessageSendResultNoConnection(102),

  /// @brief 消息超过最大长度，当前为 64KB
  ///
  ByteRTCRoomMessageSendResultExceedMaxLength(103),

  /// @brief 失败，未知错误。
  ///
  ByteRTCRoomMessageSendResultUnknown(1000);

  final dynamic $value;
  const ByteRTCRoomMessageSendResult([this.$value]);
}

enum ByteRTCNetworkDetectionStartReturn {
  /// @brief 成功开始探测。
  ///
  ByteRTCNetworkDetectionStartReturnSuccess(0),

  /// @brief 开始探测失败。参数错误，上下行探测均为 `false`，或期望带宽超过了范围 [100,10000]
  ///
  ByteRTCNetworkDetectionStartReturnParamErr(1),

  /// @brief 开始探测失败。失败原因为，本地已经开始推拉流
  ///
  ByteRTCNetworkDetectionStartReturnStreaming(2),

  /// @brief 已经开始探测，无需重复开启
  ///
  ByteRTCNetworkDetectionStartReturnStarted(3),

  /// @brief 不支持该功能
  ///
  ByteRTCNNetworkDetectionStartReturnNotSupport(4);

  final dynamic $value;
  const ByteRTCNetworkDetectionStartReturn([this.$value]);
}

enum ByteRTCKTVErrorCode {
  /// @brief 成功。
  ///
  ByteRTCKTVErrorCodeOK(0),

  /// @brief AppID 异常。
  ///
  ByteRTCKTVErrorCodeAppidInValid(-3000),

  /// @brief 非法参数，传入的参数不正确。
  ///
  ByteRTCKTVErrorCodeParasInValid(-3001),

  /// @brief 获取歌曲资源失败。
  ///
  ByteRTCKTVErrorCodeGetMusicFailed(-3002),

  /// @brief 获取歌词失败。
  ///
  ByteRTCKTVErrorCodeGetLyricFailed(-3003),

  /// @brief 歌曲下架。
  ///
  ByteRTCKTVErrorCodeMusicTakedown(-3004),

  /// @brief 歌曲文件下载失败。
  ///
  ByteRTCKTVErrorCodeMusicDownload(-3005),

  /// @brief MIDI 文件下载失败。
  ///
  ByteRTCKTVErrorCodeMidiDownloadFailed(-3006),

  /// @brief 系统繁忙。
  ///
  ByteRTCKTVErrorCodeSystemBusy(-3007),

  /// @brief 网络异常。
  ///
  ByteRTCKTVErrorCodeNetwork(-3008),

  /// @brief KTV 功能未加入房间。
  ///
  ByteRTCKTVErrorCodeNotJoinRoom(-3009),

  /// @brief 解析数据失败。
  ///
  ByteRTCKTVErrorCodeParseData(-3010),

  /// @brief 已在下载中。
  ///
  ByteRTCKTVErrorCodeDownloading(-3012),

  /// @brief 内部错误，联系技术支持人员。
  ///
  ByteRTCKTVErrorCodeInternal(-3013),

  /// @brief 下载失败，磁盘空间不足。清除缓存后重试。
  ///
  ByteRTCKTVErrorCodeInsufficientDiskSpace(-3014),

  /// @brief 下载失败，音乐文件解密失败，联系技术支持人员。
  ///
  ByteRTCKTVErrorCodeMusicDecryptionFailed(-3015),

  /// @brief 下载失败，音乐文件重命名失败，请重试。
  ///
  ByteRTCKTVErrorCodeFileRenameFailed(-3016),

  /// @brief 下载失败，下载超时，请重试。
  ///
  ByteRTCKTVErrorCodeDownloadTimeOut(-3017),

  /// @brief 清除缓存失败，可能原因是文件被占用或者系统异常，请重试。
  ///
  ByteRTCKTVErrorCodeClearCacheFailed(-3018),

  /// @brief 取消下载。
  ///
  ByteRTCKTVErrorCodeDownloadCanceled(-3019),

  /// @hidden
  /// @deprecated 从 353 开始
  /// @brief 下载失败。
  ///
  ByteRTCKTVErrorCodeDownload(-3011);

  final dynamic $value;
  const ByteRTCKTVErrorCode([this.$value]);
}

enum ByteRTCWarningCode {
  /// @brief 发布音视频流失败。通过 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason} 回调。 <br>
  ///        当你在所在房间中发布音视频流时，由于服务器错误导致发布失败。SDK 会自动重试发布。
  ///
  ByteRTCWarningCodePublishStreamFailed(-2002),

  /// @brief 订阅音视频流失败。通过以下回调通知： rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason}、rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason}。 <br>
  ///        当前房间中找不到订阅的音视频流导致订阅失败。SDK 会自动重试订阅，若仍订阅失败则建议你退出重试。
  ///
  ByteRTCWarningCodeSubscribeStreamFailed404(-2003),

  /// @brief 订阅音视频流失败。通过以下回调通知： rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason}、rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason}。 <br>
  ///        当你订阅所在房间中的音视频流时，由于服务器错误导致订阅失败。SDK 会自动重试订阅。
  ///
  ByteRTCWarningCodeSubscribeStreamFailed5xx(-2004),

  /// @hidden currently not available
  /// @brief 函数调用顺序错误，当前代码中未使用。
  ///
  ByteRTCWarningCodeInvokeError(-2005),

  /// @hidden for internal use only
  /// @brief 调度异常，服务器返回的媒体服务器地址不可用。
  ///
  ByteRTCWarningCodeInvalidExpectMediaServerAddress(-2007),

  /// @brief 当调用 `setUserVisibility:` 将自身可见性设置为 false 后，再尝试发布流会触发此警告。通过 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason} 回调。
  ///
  ByteRTCWarningCodePublishStreamForbidden(-2009),

  /// @brief 发送自定义广播消息失败，当前你未在房间中。
  ///
  ByteRTCWarningCodeSendCustomMessage(-2011),

  /// @brief 当房间内人数超过 500 人时，停止向房间内已有用户发送 `rtcEngine:onUserJoined:elapsed:` 和 `rtcEngine:onUserLeave:reason:` 回调，并通过广播提示房间内所有用户。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCWarningCodeCodeUserNotifyStop(-2013),

  /// @brief 用户已经在其他房间发布过流，或者用户正在发布 WTN 流。通过 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason} 回调，提示`kPublishStateChangeReasonNoPublishPermission` 回调。
  ///
  ByteRTCWarningCodeUserInPublish(-2014),

  /// @brief 当前正在进行回路测试，该接口调用无效
  ///
  ByteRTCWarningCodeInEchoTestMode(-2017),

  /// @brief 摄像头权限异常，当前应用没有获取摄像头权限
  ///
  ByteRTCWarningCodeNoCameraPermission(-5001),

  /// @brief 不支持在 publishScreenAudio: 之后，通过 `setScreenAudioSourceType` 设置屏幕音频采集类型
  ///
  ByteRTCWarningSetScreenAudioSourceTypeFailed(-5009),

  /// @brief 不支持在 publishScreenAudio: 之后，通过 `setScreenAudioStreamIndex` 设置屏幕音频混流类型
  ///
  ByteRTCWarningSetScreenAudioStreamIndexFailed(-5010),

  /// @brief 设置语音音高不合法
  ///
  ByteRTCWarningInvalidVoicePitch(-5011),

  /// @brief 外部音频源新旧接口混用
  ///
  ByteRTCWarningInvalidCallForExtAudio(-5013),

  /// @brief 指定的内部渲染画布句柄无效。 <br>
  ///        当你调用 setLocalVideoCanvas:{@link #ByteRTCEngine#setLocalVideoCanvas} 或 setRemoteVideoCanvas:withCanvas:{@link #ByteRTCEngine#setRemoteVideoCanvas:withCanvas} 时指定了无效的画布句柄，触发此回调。
  ///
  ByteRTCWarningCodeInvalidCanvasHandle(-6001),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) SDK 鉴权失效。联系技术支持人员。
  ///
  ByteRTCWarningCodeInvaildSamiAppkeyORToken(-7002),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 资源加载失败。传入正确的 DAT 路径，或联系技术支持人员。
  ///
  ByteRTCWarningCodeInvaildSamiResourcePath(-7003),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 库加载失败。使用正确的库，或联系技术支持人员。
  ///
  ByteRTCWarningCodeLoadSamiLibraryFailed(-7004),

  /// @brief [音频技术](https://www.volcengine.com/docs/6489/71986) 不支持此音效。联系技术支持人员。
  ///
  ByteRTCWarningCodeInvaildSamiEffectType(-7005),

  /// @hidden
  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 自动订阅模式未关闭时，尝试开启手动订阅模式会触发此警告。 <br>
  ///        你需在进房前关闭自动订阅模式，再调用 subscribeStreamVideo:subscribe:{@link #ByteRTCRoom#subscribeStreamVideo:subscribe}/subscribeStreamAudio:subscribe:{@link #ByteRTCRoom#subscribeStreamAudio:subscribe} 方法手动订阅音视频流。
  ///
  ByteRTCWarningCodeSubscribeStreamForbiden(-2010),

  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @brief 同样 roomid 的房间已经存在了
  ///
  ByteRTCWarningCodeRoomAlreadyExist(-2015),

  /// @brief 已在 3.33 版本中废弃，使用 ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceNoPermission 代替。 <br>
  ///        麦克风权限异常，当前应用没有获取麦克风权限。
  /// @deprecated since 3.33 and will be deleted in 3.51, use ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceNoPermission instead.
  ///
  ByteRTCWarningCodeNoMicrophonePermission(-5002),

  /// @brief 已在 3.33 版本中废弃，使用 ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceFailure 代替。 <br>
  ///        音频采集设备启动失败。 <br>
  ///        启动音频采集设备失败，当前设备可能被其他应用占用。
  /// @deprecated since 3.33 and will be deleted in 3.51, use ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceFailure instead.
  ///
  ByteRTCWarningCodeAudioDeviceManagerRecordingStartFail(-5003),

  /// @brief 已在 3.33 版本中废弃，使用 ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceFailure 代替. <br>
  ///        音频播放设备启动失败警告。 <br>
  ///        可能由于系统资源不足，或参数错误。
  /// @deprecated since 3.33 and will be deleted in 3.51, use ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceFailure instead.
  ///
  ByteRTCWarningCodeAudioDeviceManagerPlayoutStartFail(-5004),

  /// @brief 已在 3.33 版本中废弃，使用 ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceNotFound 代替。 <br>
  ///        无可用音频采集设备。 <br>
  ///        启动音频采集设备失败，请插入可用的音频采集设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceNotFound instead.
  ///
  ByteRTCWarningCodeNoRecordingDevice(-5005),

  /// @brief 已在 3.33 版本中废弃，使用 ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceNotFound 代替。 <br>
  ///        无可用音频播放设备。 <br>
  ///        启动音频播放设备失败，请插入可用的音频播放设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}.ByteRTCMediaDeviceErrorDeviceNotFound instead.
  ///
  ByteRTCWarningCodeNoPlayoutDevice(-5006),

  /// @brief 已在 3.33 版本中废弃，使用 ByteRTCMediaDeviceWarning{@link #ByteRTCMediaDeviceWarning}.ByteRTCMediaDeviceWarningCaptureSilence 代替。 <br>
  ///        当前音频设备没有采集到有效的声音数据，请检查更换音频采集设备。
  /// @deprecated since 3.33 and will be deleted in 3.51, use ByteRTCMediaDeviceWarning{@link #ByteRTCMediaDeviceWarning}.ByteRTCMediaDeviceWarningCaptureSilence instead.
  ///
  ByteRTCWarningCodeRecordingSilence(-5007),

  /// @brief 已在 3.33 版本中废弃，使用 ByteRTCMediaDeviceWarning{@link #ByteRTCMediaDeviceWarning}.ByteRTCMediaDeviceWarningOperationDenied 代替。 <br>
  ///        媒体设备误操作警告。 <br>
  ///        使用自定义采集时，不可调用内部采集开关，调用时触发此警告。
  /// @deprecated since 3.33 and will be deleted in 3.51, use ByteRTCMediaDeviceWarning{@link #ByteRTCMediaDeviceWarning}.ByteRTCMediaDeviceWarningOperationDenied instead.
  ///
  ByteRTCWarningCodeMediaDeviceOperationDennied(-5008),

  unknown(-1);

  final dynamic $value;
  const ByteRTCWarningCode([this.$value]);
}

enum ByteRTSErrorCode {
  /// @brief Token 无效。 <br>
  ///        进房时使用的 Token 无效或过期失效。需要用户重新获取 Token，并调用 <br>
  ///        `updateToken:` 方法更新 Token。
  ///
  ByteRTSErrorCodeInvalidToken(-1000),

  /// @brief 加入房间错误。 <br>
  ///        进房时发生未知错误导致加入房间失败。需要用户重新加入房间。
  ///
  ByteRTSErrorCodeJoinRoom(-1001),

  /// @brief 相同用户 ID 的用户加入本房间，当前用户被踢出房间
  ///
  ByteRTSErrorCodeDuplicateLogin(-1004),

  /// @brief 服务端调用 OpenAPI 将当前用户踢出房间
  ///
  ByteRTSErrorCodeKickedOut(-1006),

  /// @brief 当调用 `createRtcRoom:` ，如果 roomId 非法，会返回 null，并抛出该错误
  ///
  ByteRTSErrorCodeRoomIdIllegal(-1007),

  /// @brief Token 过期。调用 `joinRoomByKey:roomId:userInfo:rtcRoomConfig:` 使用新的 Token 重新加入房间。
  ///
  ByteRTSErrorCodeTokenExpired(-1009),

  /// @brief 调用 `updateToken:` 传入的 Token 无效
  ///
  ByteRTSErrorCodeUpdateTokenWithInvalidToken(-1010),

  /// @brief 服务端调用 OpenAPI 解散房间，所有用户被移出房间。
  ///
  ByteRTSErrorCodeRoomDismiss(-1011),

  /// @brief 通话回路检测已经存在同样 roomId 的房间了
  ///
  ByteRTSErrorRoomAlreadyExist(-1013),

  /// @brief 加入多个房间时使用了不同的 uid。 <br>
  ///        同一个引擎实例中，用户需使用同一个 uid 加入不同的房间。
  ///
  ByteRTSErrorUserIDDifferent(-1014),

  /// @brief 服务端异常状态导致退出房间。 <br>
  ///        SDK 与信令服务器断开，并不再自动重连，可联系技术支持。
  ///
  ByteRTSErrorCodeAbnormalServerStatus(-1084);

  final dynamic $value;
  const ByteRTSErrorCode([this.$value]);
}

enum ByteRTCPublicStreamErrorCode {
  /// @brief 发布或订阅成功。
  ///
  ByteRTCPublicStreamErrorCodeSuccess(0),

  /// @brief WTN 流的参数异常，请修改参数后重试。
  ///
  ByteRTCPublicStreamErrorCodePushParamError(1191),

  /// @brief 服务端状态异常，将自动重试。
  ///
  ByteRTCPublicStreamErrorCodePushStatusError(1192),

  /// @brief 内部错误，不可恢复，请重试。
  ///
  ByteRTCPublicStreamErrorCodePushInternalError(1193),

  /// @brief 发布失败，将自动重试，请关注重试结果。
  ///
  ByteRTCPublicStreamErrorCodePushError(1195),

  /// @brief 发布失败，10 s 后会重试，重试 3 次后自动停止。
  ///
  ByteRTCPublicStreamErrorCodePushTimeOut(1196),

  /// @brief 订阅失败，发布端未开始发布流。
  ///
  ByteRTCPublicStreamErrorCodePullNoPushStream(1300);

  final dynamic $value;
  const ByteRTCPublicStreamErrorCode([this.$value]);
}

enum ByteRTCUserMessageSendResult {
  /// @brief 发送消息成功。
  ///
  ByteRTCUserMessageSendResultSuccess(0),

  /// @brief 消息发送失败。发送超时。
  ///
  ByteRTCUserMessageSendResultTimeout(1),

  /// @brief 消息发送失败。连接断开，消息未发出。
  ///
  ByteRTCUserMessageSendResultNetworkDisconnected(2),

  /// @brief 消息发送失败。找不到接收方。
  ///
  ByteRTCUserMessageSendResultNoReceiver(3),

  /// @brief 消息发送失败。远端用户没有登录或进房。
  ///
  ByteRTCUserMessageSendResultNoRelayPath(4),

  /// @brief 消息发送失败。超过 QPS 限制。
  ///
  ByteRTCUserMessageSendResultExceedQPS(5),

  /// @brief 消息发送失败。应用服务器未收到客户端发送的消息。 <br>
  ///        由 `sendServerMessage`/`sendServerBinaryMessage` 触发，通过 `onServerMessageSendResult` 回调。
  ///
  ByteRTCUserMessageSendResultE2BSSendFailed(17),

  /// @brief 消息发送失败。应用服务器接收到了客户端发送的消息，但响应失败。 <br>
  ///        由 `sendServerMessage`/`sendServerBinaryMessage` 触发，通过 `onServerMessageSendResult` 回调。
  ///
  ByteRTCUserMessageSendResultE2BSReturnFailed(18),

  /// @brief 消息发送失败。消息发送方没有加入房间。
  ///
  ByteRTCUserMessageSendResultNotJoin(100),

  /// @brief 消息发送失败。连接未完成初始化。
  ///
  ByteRTCUserMessageSendResultInit(101),

  /// @brief 消息发送失败。没有可用的数据传输通道连接。
  ///
  ByteRTCUserMessageSendResultNoConnection(102),

  /// @brief 消息发送失败。消息超过最大长度 (64 KB)。
  ///
  ByteRTCUserMessageSendResultExceedMaxLength(103),

  /// @brief 消息发送失败。接收方用户 ID 为空。
  ///
  ByteRTCUserMessageSendResultEmptyUser(104),

  /// @brief 消息发送失败。房间外或应用服务器消息发送方没有登录。
  ///
  ByteRTCUserMessageSendResultNotLogin(105),

  /// @brief 消息发送失败。发送消息给业务方服务器之前没有设置参数。
  ///
  ByteRTCUserMessageSendResultServerParamsNotSet(106),

  /// @brief 失败，未知错误。
  ///
  ByteRTCUserMessageSendResultUnknown(1000);

  final dynamic $value;
  const ByteRTCUserMessageSendResult([this.$value]);
}

enum ByteRTCErrorCode {
  /// @brief Token 无效。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 进房时使用的 Token 参数有误或过期失效。需要重新获取 Token，并调用 updateToken:{@link #ByteRTCRTSRoom#updateToken} 方法更新 Token。
  ///
  ByteRTCErrorCodeInvalidToken(-1000),

  /// @brief 加入房间错误。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        进房时发生未知错误导致加入房间失败。需要用户重新加入房间。
  ///
  ByteRTCErrorCodeJoinRoom(-1001),

  /// @brief 没有发布音视频流权限。通过以下回调通知： rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason}。 <br>
  ///        用户在所在房间中发布音视频流失败，失败原因为用户没有发布流的权限。
  ///
  ByteRTCErrorCodeNoPublishPermission(-1002),

  /// @brief 没有订阅音视频流权限。通过以下回调通知： rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason}、rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason}。 <br>
  ///        用户订阅所在房间中的音视频流失败，失败原因为用户没有订阅流的权限。
  ///
  ByteRTCErrorCodeNoSubscribePermission(-1003),

  /// @brief 相同用户 ID 的用户加入本房间，当前用户被踢出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeDuplicateLogin(-1004),

  /// @brief 服务端调用 OpenAPI 将当前用户踢出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeKickedOut(-1006),

  /// @brief 当调用 `createRtcRoom:` ，如果 roomId 非法，会返回 null，并抛出该错误。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeRoomIdIllegal(-1007),

  /// @brief Token 过期。加入房间后 Token 过期时，返回此错误码。需使用新的 Token 重新加入房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeTokenExpired(-1009),

  /// @brief 调用 `updateToken:` 传入的 Token 无效。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeUpdateTokenWithInvalidToken(-1010),

  /// @brief 服务端调用 OpenAPI 解散房间，所有用户被移出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeRoomDismiss(-1011),

  /// @hidden internal use only
  /// @brief 加入房间错误。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        调用 `joinRoom:userInfo:roomConfig:` 方法时, LICENSE 计费账号未使用 LICENSE_AUTHENTICATE SDK，加入房间错误。
  ///
  ByteRTCJoinRoomWithoutLicenseAuthenticateSDK(-1012),

  /// @brief 通话回路检测已经存在同样 roomId 的房间了。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomAlreadyExist(-1013),

  /// @brief 加入多个房间时使用了不同的 uid。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        同一个引擎实例中，用户需使用同一个 uid 加入不同的房间。
  ///
  ByteRTCUserIDDifferent(-1014),

  /// @hidden internal use only
  /// @brief 服务端 license 过期，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomServerLicenseExpired(-1017),

  /// @hidden internal use only
  /// @brief 超过服务端 license 许可的并发量上限，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomExceedsTheUpperLimit(-1018),

  /// @hidden internal use only
  /// @brief license 参数错误，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomLicenseParameterError(-1019),

  /// @hidden internal use only
  /// @brief license 证书路径错误。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomLicenseFilePathError(-1020),

  /// @hidden internal use only
  /// @brief license 证书不合法。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomLicenseIllegal(-1021),

  /// @hidden internal use only
  /// @brief license 证书已经过期，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomLicenseExpired(-1022),

  /// @hidden internal use only
  /// @brief license 证书内容不匹配。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomLicenseInformationNotMatch(-1023),

  /// @hidden internal use only
  /// @brief license 当前证书与缓存证书不匹配。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomLicenseNotMatchWithCache(-1024),

  /// @brief 房间被封禁。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomRoomForbidden(-1025),

  /// @brief 用户被封禁。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCErrorCodeJoinRoomUserForbidden(-1026),

  /// @brief 订阅音视频流失败，订阅音视频流总数超过上限。通过以下回调通知： rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason}、rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason}。 <br>
  ///        游戏场景下，为了保证音视频通话的性能和质量，服务器会限制用户订阅的音视频流总数。当用户订阅的音视频流总数已达上限时，继续订阅更多流时会失败，同时用户会收到此错误通知。
  ///
  ByteRTCErrorCodeOverStreamSubscribeLimit(-1070),

  /// @brief 发布流失败，发布流总数超过上限。通过 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason} 回调。 <br>
  ///        RTC 系统会限制单个房间内发布的总流数，总流数包括视频流、音频流和屏幕流。如果房间内发布流数已达上限时，本地用户再向房间中发布流时会失败，同时会收到此错误通知。
  ///
  ByteRTCErrorCodeOverStreamPublishLimit(-1080),

  /// @brief 服务端异常状态导致退出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        SDK 与信令服务器断开，并不再自动重连，可联系技术支持。
  ///
  ByteRTCErrorCodeAbnormalServerStatus(-1084),

  /// @hidden for internal use only
  /// @brief 在一路流推多房间的场景下，在至少有两个房间在发布同一路流时，其中一个房间取消发布失败，此时需要业务方重试或者由业务方通知用户重试取消发布。
  ///
  ByteRTCErrorCodeMultiRoomUnpublishFailed(-1085),

  /// @hidden for internal use only
  /// @brief 指定服务区域时传入错误参数。
  ///
  ByteRTCErrorCodeWrongAreaCode(-1086),

  /// @hidden for internal use only
  /// @brief notify deadlock
  ///
  ByteRTCErrorCodeDeadLockNotify(-1111),

  /// @deprecated since 3.52, use ByteRTCErrorCodeOverStreamPublishLimit-1080）instead
  /// @brief 发布视频流总数超过上限。 <br>
  ///        RTC 系统会限制单个房间内发布的视频流数。如果房间内发布视频流数已达上限时，本地用户再向房间中发布视频流时会失败，同时会收到此错误通知。
  ///
  ByteRTCErrorCodeOverVideoPublishLimit(-1082),

  /// @deprecated since 3.60, use ByteRTCAVSyncEventInvalidUidRepeated = 0 carried by rtcRoom:onAVSyncEvent:userId:eventCode:{@link #ByteRTCRoomDelegate#rtcRoom:onAVSyncEvent:userId:eventCode} instead.
  /// @brief 音视频同步失败。 <br>
  ///        当前音频源已与其他视频源关联同步关系。 <br>
  ///        单个音频源不支持与多个视频源同时同步。 <br>
  ///        通过 rtcRoom:onStreamStateChanged:withUid:state:extraInfo: 回调。
  ///
  ByteRTCErrorCodInvalidAudioSyncUidRepeated(-1083);

  final dynamic $value;
  const ByteRTCErrorCode([this.$value]);
}

enum ByteRTSWarningCode {
  /// @brief 进房失败。 <br>
  ///        初次进房或者由于网络状况不佳断网重连时，由于服务器错误导致进房失败。SDK 会自动重试进房。
  ///
  ByteRTSWarningCodeJoinRoomFailed(-2001),

  /// @brief 发布音视频流失败。 <br>
  ///        当你在所在房间中发布音视频流时，由于服务器错误导致发布失败。SDK 会自动重试发布。
  ///
  ByteRTSWarningCodePublishStreamFailed(-2002),

  /// @hidden currently not available
  /// @brief 函数调用顺序错误，当前代码中未使用。
  ///
  ByteRTSWarningCodeInvokeError(-2005),

  /// @hidden for internal use only
  /// @brief 调度异常，服务器返回的媒体服务器地址不可用。
  ///
  ByteRTSWarningCodeInvalidExpectMediaServerAddress(-2007),

  /// @brief 发送自定义广播消息失败，当前你未在房间中。
  ///
  ByteRTSWarningCodeSendCustomMessage(-2011),

  /// @brief 新生成的房间已经替换了同样 roomId 的旧房间
  ///
  ByteRTSWarningCodeOldRoomBeenReplaced(-2016),

  /// @hidden
  /// @deprecated since 3.46 and will be deleted in 3.52.
  /// @brief 同样 roomid 的房间已经存在了
  ///
  ByteRTSWarningCodeRoomAlreadyExist(-2015);

  final dynamic $value;
  const ByteRTSWarningCode([this.$value]);
}

enum ByteRTCSingleStreamTaskEvent {
  /// @hidden for internal use only
  ///
  ByteRTCSingleStreamTaskEventBase(0),

  /// @brief 任务发起成功。
  ///
  ByteRTCSingleStreamTaskEventStartSuccess(1),

  /// @brief 任务发起失败。
  ///
  ByteRTCSingleStreamTaskEventStartFailed(2),

  /// @brief 任务停止。
  ///
  ByteRTCSingleStreamTaskEventStopSuccess(3),

  /// @brief 结束任务失败。
  ///
  ByteRTCSingleStreamTaskEventStopFailed(4),

  /// @brief Warning 事件
  ///
  ByteRTCSingleStreamTaskEventWarning(5);

  final dynamic $value;
  const ByteRTCSingleStreamTaskEvent([this.$value]);
}

enum ByteRTCChorusCacheSyncError {
  /// @brief 成功。
  ///
  ByteRTCChorusCacheSyncErrorOK(0),

  /// @brief 失败。推送至 CDN 时，应进行以下设置： <br>
  ///        - `IMixedStreamConfig.MixedStreamSyncControlConfig.enable_sync = true`；
  ///        - `IMixedStreamConfig.MixedStreamSyncControlConfig.base_user_id = {uid of producer}`。
  ///
  ByteRTCChorusCacheSyncErrorWrongState(1),

  /// @brief 缓存同步功能已启动，不需要重复开启。
  ///
  ByteRTCChorusCacheSyncErrorAlreadyRunning(2);

  final dynamic $value;
  const ByteRTCChorusCacheSyncError([this.$value]);
}

enum ByteRTCSingleStreamTaskErrorCode {
  /// @brief 推流成功。
  ///
  ByteRTCSingleStreamTaskErrorCodeOK(0),

  /// @hidden currently not available
  /// @brief 预留错误码，未启用
  ///
  ByteRTCSingleStreamTaskErrorCodeBase(1090),

  /// @brief 服务端合流错误
  ///
  ByteRTCSingleStreamTaskErrorCodeUnknownByServer(1091),

  /// @brief 任务处理超时，请检查网络状态并重试。
  ///
  ByteRTCSingleStreamTaskErrorCodeSignalRequestTimeout(1092),

  /// @brief 服务端检测任务参数不合法
  ///
  ByteRTCSingleStreamTaskErrorCodeInvalidParamByServer(1093),

  /// @brief 转推任务在目标房间的用户ID被踢出目标房间
  ///
  ByteRTCSingleStreamTaskErrorCodeRemoteKicked(1094),

  /// @brief 转推任务加入目标房间失败
  ///
  ByteRTCSingleStreamTaskErrorCodeJoinDestRoomFailed(1095),

  /// @brief 转推任务在源房间拉流超时
  ///
  ByteRTCSingleStreamTaskErrorCodeReceiveSrcStreamTimeout(1096),

  /// @brief 音视频编码转推任务不支持
  ///
  ByteRTCSingleStreamTaskErrorCodeNotSurportCodec(1097);

  final dynamic $value;
  const ByteRTCSingleStreamTaskErrorCode([this.$value]);
}

enum ByteRTCLoginErrorCode {
  /// @brief 调用 login:uid:{@link #ByteRTCEngine#login:uid} 方法登录成功。
  ///
  ByteRTCLoginErrorCodeSuccess(0),

  /// @brief 调用 login:uid:{@link #ByteRTCEngine#login:uid} 方法时使用的 Token 无效或过期失效。需要用户重新获取 Token。
  ///
  ByteRTCLoginErrorCodeInvalidToken(-1000),

  /// @brief 登录错误 <br>
  ///        调用 login:uid:{@link #ByteRTCEngine#login:uid} 方法时发生未知错误导致登录失败。需要用户重新登录。
  ///
  ByteRTCLoginErrorCodeLoginFailed(-1001),

  /// @brief 调用 login:uid:{@link #ByteRTCEngine#login:uid} 方法时传入的用户 ID 有问题。
  ///
  ByteRTCLoginErrorCodeInvalidUserId(-1002),

  /// @brief 调用 login:uid:{@link #ByteRTCEngine#login:uid} 登录时服务器错误。
  ///
  ByteRTCLoginErrorCodeServerError(-1003);

  final dynamic $value;
  const ByteRTCLoginErrorCode([this.$value]);
}

enum ByteRTCMixedStreamTaskErrorCode {
  /// @brief 推流成功。
  ///
  ByteRTCMixedStreamTaskErrorCodeOK(0),

  /// @hidden currently not available
  /// @brief 预留错误码，未启用
  ///
  ByteRTCMixedStreamTaskErrorCodeBase(1090),

  /// @brief 任务处理超时，请检查网络状态并重试
  ///
  ByteRTCMixedStreamTaskErrorCodeTimeOut(1091),

  /// @brief 服务端检测到错误的推流参数
  ///
  ByteRTCMixedStreamTaskErrorCodeInvalidParamByServer(1092),

  /// @brief 对流的订阅超时
  ///
  ByteRTCMixedStreamTaskErrorCodeSubTimeoutByServer(1093),

  /// @brief 合流服务端内部错误。
  ///
  ByteRTCMixedStreamTaskErrorCodeInvalidStateByServer(1094),

  /// @brief 合流服务端推 CDN 失败。
  ///
  ByteRTCMixedStreamTaskErrorCodeAuthenticationByCDN(1095),

  /// @brief 服务端未知错误。
  ///
  ByteRTCMixedStreamTaskErrorCodeUnKnownErrorByServer(1096),

  /// @brief 服务端接收信令超时，请检查网络状态并重试。
  ///
  ByteRTCMixedStreamTaskErrorCodeSignalRequestTimeout(1097),

  /// @brief 图片合流失败。
  ///
  ByteRTCMixedStreamTaskErrorCodeMixImageFailed(1098),

  /// @hidden currently not available
  /// @brief 缓存未同步。
  ///
  ByteRTCMixedStreamTaskErrorCodeStreamSyncWorse(1099),

  /// @brief 发布 WTN 流失败
  ///
  ByteRTCMixedStreamTaskErrorCodePushWTNFailed(1195),

  /// @hidden for internal use only
  ///
  ByteRTCMixedStreamTaskErrorCodeMax(1199);

  final dynamic $value;
  const ByteRTCMixedStreamTaskErrorCode([this.$value]);
}

enum ByteRTCSubtitleErrorCode {
  /// @brief 客户端无法识别云端媒体处理发送的错误码。
  ///
  ByteRTCSubtitleErrorCodeUnknow(-1),

  /// @brief 字幕已开启。
  ///
  ByteRTCSubtitleErrorCodeSuccess(0),

  /// @brief 云端媒体处理内部出现错误，请联系技术支持。
  ///
  ByteRTCSubtitleErrorCodePostProcessError(1),

  /// @brief 第三方服务连接失败，请联系技术支持。
  ///
  ByteRTCSubtitleErrorCodeASRConnectionError(2),

  /// @brief 第三方服务内部出现错误，请联系技术支持。
  ///
  ByteRTCSubtitleErrorCodeASRServiceError(3),

  /// @brief 未进房导致调用`startSubtitle`失败。请加入房间后再调用此方法。
  ///
  ByteRTCSubtitleErrorCodeBeforeJoinRoom(4),

  /// @brief 字幕已开启，无需重复调用 `startSubtitle`。
  ///
  ByteRTCSubtitleErrorCodeAlreadyOn(5),

  /// @brief 用户选择的目标语言目前暂不支持。
  ///
  ByteRTCSubtitleErrorCodeUnsupportedLanguage(6),

  /// @brief 云端媒体处理超时未响应，请联系技术支持。
  ///
  ByteRTCSubtitleErrorCodePostProcessTimeout(7);

  final dynamic $value;
  const ByteRTCSubtitleErrorCode([this.$value]);
}

enum ByteRTCAudioRecordingErrorCode {
  /// @brief 录制正常
  ///
  ByteRTCAudioRecordingErrorCodeOk(0),

  /// @brief 没有文件写权限
  ///
  ByteRTCAudioRecordingErrorCodeNoPermission(-1),

  /// @brief 没有进入房间
  ///
  ByteRTCAudioRecordingErrorNotInRoom(-2),

  /// @brief 录制已经开始
  ///
  ByteRTCAudioRecordingAlreadyStarted(-3),

  /// @brief 录制还未开始
  ///
  ByteRTCAudioRecordingNotStarted(-4),

  /// @brief 录制失败。文件格式不支持。
  ///
  ByteRTCAudioRecordingErrorCodeNotSupport(-5),

  /// @brief 其他异常
  ///
  ByteRTCAudioRecordingErrorCodeOther(-6);

  final dynamic $value;
  const ByteRTCAudioRecordingErrorCode([this.$value]);
}
