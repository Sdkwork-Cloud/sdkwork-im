/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

export '../codegen/pack/callback.dart' show IAudioFrameObserver;
export '../codegen/pack/errorcode.dart'
    show PublicStreamErrorCode, SubtitleErrorCode, SingleStreamTaskErrorCode;
export '../codegen/pack/keytype.dart'
    show
        ForwardStreamInfo,
        AVSyncState,
        AudioSelectionPriority,
        DataMessageSourceType,
        EchoTestResult,
        FallbackOrRecoverReason,
        FirstFramePlayState,
        FirstFrameSendState,
        ForwardStreamError,
        ForwardStreamEvent,
        ForwardStreamEventInfo,
        ForwardStreamState,
        ForwardStreamStateInfo,
        HardwareEchoDetectionResult,
        LocalAudioStats,
        LocalAudioStreamError,
        LocalAudioStreamState,
        LocalStreamStats,
        LocalVideoStats,
        LocalVideoStreamError,
        LocalVideoStreamState,
        MediaDeviceError,
        MediaDeviceState,
        MediaDeviceWarning,
        NetworkDetectionLinkType,
        NetworkDetectionStopReason,
        NetworkQualityStats,
        NetworkTimeInfo,
        PerformanceAlarmMode,
        PerformanceAlarmReason,
        ProblemFeedbackInfo,
        ProblemFeedbackRoomInfo,
        PublishFallbackOption,
        RTCRoomStats,
        RecordingConfig,
        RecordingErrorCode,
        RecordingFileType,
        RecordingInfo,
        RecordingProgress,
        RecordingState,
        RemoteAudioState,
        RemoteAudioStateChangeReason,
        RemoteAudioStats,
        RemoteStreamKey,
        RemoteStreamStats,
        RemoteUserPriority,
        RemoteVideoState,
        RemoteVideoStateChangeReason,
        RemoteVideoStats,
        ReturnStatus,
        RoomProfile,
        SEICountPerFrame,
        SEIStreamUpdateEvent,
        SetRoomExtraInfoResult,
        StreamIndex,
        StreamRemoveReason,
        SubscribeState,
        SubtitleConfig,
        SubtitleMessage,
        SubtitleMode,
        SubtitleState,
        SyncInfoStreamType,
        UserInfo,
        AudioMixingDualMonoMode,
        UserVisibilityChangeError,
        SingleStreamPushType,
        DestInfo,
        AudioFormat,
        AudioFrame,
        PlayerEvent,
        SimulcastStreamType,
        AudioFrameCallbackMethod,
        VideoCodecType;

/// @brief 音视频质量反馈问题类型
enum ProblemFeedbackOption {
  /// @brief 没有问题
  none(0),

  /// @brief 其他问题
  other_message(1),

  /// @brief 连接失败
  disconnected(2),

  /// @brief 耳返延迟大
  ear_back_delay(3),

  /// @brief 本端有杂音
  local_noise(4),

  /// @brief 本端声音卡顿
  local_audio_lagging(5),

  /// @brief 本端无声音
  local_no_audio(6),

  /// @brief 本端声音大/小
  local_audio_strength(7),

  /// @brief 本端有回声
  local_echo(8),

  /// @brief 本端视频模糊
  local_video_fuzzy(9),

  /// @brief 本端音视频不同步
  local_not_sync(10),

  /// @brief 本端视频卡顿
  local_video_lagging(11),

  /// @brief 本端无画面
  local_no_video(12),

  /// @brief 远端有杂音
  remote_noise(13),

  /// @brief 远端声音卡顿
  remote_audio_lagging(14),

  /// @brief 远端无声音
  remote_no_audio(15),

  /// @brief 远端声音大/小
  remote_audio_strength(16),

  /// @brief 远端有回声
  remote_echo(17),

  /// @brief 远端视频模糊
  remote_video_fuzzy(18),

  /// @brief 远端音视频不同步
  remote_not_sync(19),

  /// @brief 远端视频卡顿
  remote_video_lagging(20),

  /// @brief 远端无画面
  remote_no_video(21);

  final dynamic $value;
  const ProblemFeedbackOption([this.$value]);
}

/// @brief 自定义加密类型
enum EncryptType {
  /// @brief 不使用内置加密（默认）
  customize(0),

  /// @brief AES-128-CBC 加密算法
  aes128CBC(1),

  /// @brief AES-256-CBC 加密算法
  aes256CBC(2),

  /// @brief AES-128-ECB 加密算法
  aes128ECB(3),

  /// @brief AES-256-ECB 加密算法
  aes256ECB(4);

  final dynamic $value;
  const EncryptType([this.$value]);
}

/// @brief 蓝牙传输协议
/// @note 仅 iOS 适用。
enum BluetoothMode {
  /// 默认采用 auto 模式，具体如下：
  /// <table border>
  /// <tr>
  ///   <th>场景</th>
  ///   <th> HFP </th>
  ///   <th> A2DP </th>
  /// </tr>
  /// <tr>
  ///   <th>纯通话场景</th>
  ///   <th> 蓝牙设备支持 HFP </th>
  ///   <th> 蓝牙设备不支持 HFP </th>
  /// </tr>
  /// <tr>
  ///   <th>纯媒体场景</th>
  ///   <th> 使用蓝牙设备采集播放音频 </th>
  ///   <th> 使用 iOS 设备采集音频，蓝牙设备播放音频 </th>
  /// </tr>
  /// </table>
  auto(0),

  /// @brief 高级音频分配配置文件（A2DP）。立体声、高音质。采用 iOS 设备进行音频采集，蓝牙设备进行播放。
  a2dp(1),

  /// @brief 免提配置文件（HFP）。单声道、普通音质。音频采集和播放设备都使用蓝牙设备。
  hfp(2);

  final dynamic $value;
  const BluetoothMode([this.$value]);
}

class EchoTestConfig {
  String token;
  String userId;
  String roomId;
  bool enableAudio;
  bool enableVideo;
  int audioReportInterval;

  EchoTestConfig({
    required this.userId,
    required this.roomId,
    required this.token,
    required this.enableAudio,
    required this.enableVideo,
    required this.audioReportInterval,
  });

  Map<String, dynamic> toMap() {
    return <String, dynamic>{
      'userId': userId,
      'roomId': roomId,
      'token': token,
      'enableAudio': enableAudio,
      'enableVideo': enableVideo,
      'audioReportInterval': audioReportInterval,
    };
  }
}
