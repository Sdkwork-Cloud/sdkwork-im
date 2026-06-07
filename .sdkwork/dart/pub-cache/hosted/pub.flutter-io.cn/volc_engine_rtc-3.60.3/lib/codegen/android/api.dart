/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';
import 'keytype.dart';
import 'callback.dart';
import 'types.dart';

class IGameRoom extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.IGameRoom';
  static get codegen_$namespace => _$namespace;

  IGameRoom([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @author luomingkang
  /// @brief 退出并销毁调用 createGameRoom{@link #RTCEngine#createGameRoom} 所创建的游戏房间实例。
  ///

  FutureOr<void> destroy() async {
    return await nativeCall('destroy', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间。 <br>
  ///        调用 createGameRoom{@link #RTCEngine#createGameRoom} 创建游戏房间实例后，调用此方法加入游戏房间，同房间内其他用户进行音频通话。
  /// @param token 动态密钥。用于对进房用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///        使用不同 AppID 的 App 是不能互通的。 <br>
  ///        请务必保证生成 Token 使用的 AppID 和创建引擎时使用的 AppID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  /// @return
  ///        - 0：方法调用成功。触发以下回调：
  ///          - 本端收到房间状态通知 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///          - 本端收到房间内已发布流的通知 onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调。
  ///        - -1：roomID / userInfo.uid 包含了无效的参数。
  ///        - -2：已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom{@link #IGameRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调告知。
  /// @note
  ///       - 同一个 App ID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后进房的用户会将先进房的用户踢出房间，并且先进房的用户会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调通知，错误类型详见 ERROR_CODE_DUPLICATE_LOGIN{@link #ErrorCode#ERROR_CODE_DUPLICATE_LOGIN}。
  ///       - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 onConnectionStateChanged{@link #IRTCEngineEventHandler#onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调通知；如果加入房间的用户是可见用户，远端用户会收到 onUserJoined{@link #IRTCRoomEventHandler#onUserJoined} 回调通知。
  ///

  FutureOr<int> joinRoom(String token, UserInfo userInfo) async {
    return await nativeCall('joinRoom', [token, userInfo]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 通过设置 IGameRoom{@link #IGameRoom} 对象的事件句柄，监听此对象对应的回调事件。
  /// @param rtcRoomEventHandler 参看 IRTCRoomEventHandler{@link #IRTCRoomEventHandler}
  /// @return
  ///        - 0：调用成功。
  ///        - <0：调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> setRTCRoomEventHandler(
      IRTCRoomEventHandler rtcRoomEventHandler) async {
    return await nativeCall('setRTCRoomEventHandler', [rtcRoomEventHandler]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 离开游戏房间。 <br>
  ///        调用此方法结束通话过程，并释放所有通话相关的资源。
  /// @return
  ///        - 0：调用成功。如果用户是房间内可见用户，触发以下回调：
  ///            - 远端用户收到 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 回调通知。
  ///            - 正在发布的流会被取消发布。远端用户收到 onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///        - < 0：调用失败，参看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 加入游戏房间后，必须调用此方法结束通话，否则无法开始下一次通话。
  ///       - 此方法是异步操作，调用返回时并没有真正退出房间。真正退出房间后，本地会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调通知。你必须在收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调后，再销毁房间或引擎，或调用 joinRoom{@link #RTCRoom#joinRoom} 再次加入房间。
  ///       - 调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 将自身设为可见的用户离开房间后，房间内其他用户会收到 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 回调通知。
  ///

  FutureOr<int> leaveRoom() async {
    return await nativeCall('leaveRoom', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 更新游戏房间的 Token。 <br>
  ///        收到 onTokenWillExpire{@link #IRTCRoomEventHandler#onTokenWillExpire}，onPublishPrivilegeTokenWillExpire{@link #IRTCRoomEventHandler#onPublishPrivilegeTokenWillExpire}， 或 onSubscribePrivilegeTokenWillExpire{@link #IRTCRoomEventHandler#onSubscribePrivilegeTokenWillExpire} 时，你必须重新获取 Token，并调用此方法更新 Token，以保证通话的正常进行。
  /// @param token 重新获取的有效 Token。 <br>
  ///        如果 Token 无效，你会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged}，错误码是 `-1010`。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note 请勿同时调用 updateToken{@link #IGameRoom#updateToken} 和 joinRoom{@link #IGameRoom#joinRoom} 方法更新 Token。若因 Token 过期或无效导致加入房间失败或已被移出房间，你应该在获取新的有效 Token 后调用 joinRoom{@link #IGameRoom#joinRoom} 重新加入房间。
  ///

  FutureOr<int> updateToken(String token) async {
    return await nativeCall('updateToken', [token]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 获取游戏房间的范围语音接口实例。
  /// @return 方法调用结果： <br>
  ///        - IRangeAudio：成功，返回一个 IRangeAudio{@link #IRangeAudio} 实例。
  ///        - null：失败，当前 SDK 不支持范围语音功能。
  /// @note 首次调用该方法须在创建房间后、加入房间前。范围语音相关 API 和调用时序详见[范围语音](https://www.volcengine.com/docs/6348/114727)。
  ///

  FutureOr<IRangeAudio> getRangeAudio() async {
    final result = await nativeCall('getRangeAudio', []);
    return packObject(result,
        () => IRangeAudio(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，调用该接口开启或关闭麦克风。同房间其他用户会收到回调 OnAudioDeviceStateChanged{@link #IRTCEngineEventHandler#OnAudioDeviceStateChanged}。
  /// @param enable 是否开启麦克风：<br>
  ///             - true：开启麦克风，采集并发布音频流。
  ///             - false：默认设置。关闭麦克风并停止发布音频流。
  /// @return
  ///        - 0：接口调用成功。
  ///        - -3：接口调用失败。没有加入房间。
  /// @note 不可与 enableAudioSend{@link #IGameRoom#enableAudioSend} 同时调用。
  ///

  FutureOr<int> enableMicrophone(boolean enable) async {
    return await nativeCall('enableMicrophone', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，开启或关闭扬声器。
  /// @param enable 是否开启扬声器：<br>
  ///               - true：开启扬声器，接收所有远端用户的音频流。
  ///               - false：默认设置。关闭扬声器，停止接收所有远端用户的音频流。
  /// @return
  ///        - 0：接口调用成功。
  ///        - -3：接口调用失败。没有加入房间。
  ///

  FutureOr<int> enableSpeakerphone(boolean enable) async {
    return await nativeCall('enableSpeakerphone', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，开始或停止发送音频流。调用此接口不影响音频采集。同房间其他用户会收到相应的回调。
  /// @param enable 是否发送音频流：<br>
  ///               - true：发送音频流。
  ///               - false：默认设置。停止发送音频流（不会关闭麦克风），即静音。
  /// @return
  ///        - 0：表示参数检查通过，不代表打开麦克风会成功，比如房间不存在
  ///        - -3：接口调用失败。没有加入房间。
  /// @note 不可与 EnableMicrophone{@link #IGameRoom#EnableMicrophone} 同时调用。
  ///

  FutureOr<int> enableAudioSend(boolean enable) async {
    return await nativeCall('enableAudioSend', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 是否接收某个特定用户的音频流。关闭声音接收不会影响扬声器或其他音频输出设备的状态。
  /// @param userId 用户 ID，最大长度为128字节的非空字符串。支持的字符集范围为: <br>
  ///            1. 26个大写字母 A ~ Z<br>
  ///            2. 26个小写字母 a ~ z<br>
  ///            3. 10个数字 0 ~ 9<br>
  ///            4. 下划线"_", at符"\@", 减号"-"
  /// @param enable 是否接收指定用户的音频流：<br>
  ///               - true：接收该用户的音频流。即允许该用户的音频数据被传递到本地设备并播放。
  ///               - false：默认设置，不接收该用户的音频流，即不播放该用户的声音。但不会关闭扬声器，扬声器仍可用于其他音频输出。
  /// @return
  ///        - 0：接口调用成功
  ///        - -2：传入的用户 ID 为空字符串。
  ///

  FutureOr<int> enableAudioReceive(String userId, boolean enable) async {
    return await nativeCall('enableAudioReceive', [userId, enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 调节某个游戏房间内所有远端用户的音频播放音量（非系统硬件音量）。
  /// @param volume 音频播放音量值和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///              - 0: 静音
  ///              - 100: 原始音量，默认值
  ///              - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内， <br>
  ///        - 该方法与 setRemoteAudioPlaybackVolume{@link #RTCEngine#setRemoteAudioPlaybackVolume} 互斥，最新调用的任一方法设置的音量将覆盖此前已设置的音量，效果不叠加；
  ///        - 当该方法与 setPlaybackVolume{@link #RTCEngine#setPlaybackVolume} 方法共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。
  ///

  FutureOr<int> setRemoteRoomAudioPlaybackVolume(int volume) async {
    return await nativeCall('setRemoteRoomAudioPlaybackVolume', [volume]);
  }
}

class IRTCAudioDeviceManager extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.IRTCAudioDeviceManager';
  static get codegen_$namespace => _$namespace;

  IRTCAudioDeviceManager([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @brief 启动音频播放设备检测。测试启动后，循环播放指定的音频文件，同时将通过 `onAudioPlaybackDeviceTestVolume` 回调播放时的音量信息。
  /// @param testAudioFilePath 指定播放设备检测的音频文件网络地址。支持的格式包括 mp3，aac，m4a，3gp 和 wav。
  /// @param interval 设置 `onAudioPlaybackDeviceTestVolume` 音量回调的时间间隔，推荐设置为 200 毫秒或以上。单位为毫秒。最小值为 10 毫秒。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///       - 该方法可在进房前和进房后调用，不可与其它音频设备测试功能同时应用。
  ///       - 调用 stopAudioPlaybackDeviceTest{@link #IRTCAudioDeviceManager#stopAudioPlaybackDeviceTest} 可以停止测试。
  ///

  FutureOr<int> startAudioPlaybackDeviceTest(
      String testAudioFilePath, int interval) async {
    return await nativeCall(
        'startAudioPlaybackDeviceTest', [testAudioFilePath, interval]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 停止音频播放测试。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note 调用 startAudioPlaybackDeviceTest{@link #IRTCAudioDeviceManager#startAudioPlaybackDeviceTest} 后，调用本方法停止测试。
  ///

  FutureOr<int> stopAudioPlaybackDeviceTest() async {
    return await nativeCall('stopAudioPlaybackDeviceTest', []);
  }

  /// @detail api
  /// @brief 开始音频采集设备和音频播放设备测试。
  /// @param interval 测试中会收到 `enableAudioPropertiesReport` 回调，本参数指定了该周期回调的时间间隔，单位为毫秒。建议设置到大于 200 毫秒。最小不得少于 10 毫秒。
  /// @return 方法调用结果 <br>
  ///       - 0：方法调用成功
  ///       - < 0：方法调用失败
  /// @note
  ///       - 该方法在进房前后均可调用。且不可与其它音频设备测试功能同时应用。
  ///       - 调用本接口 30 s 后，采集自动停止，并开始播放采集到的声音。录音播放完毕后，设备测试流程自动结束。你也可以在 30 s 内调用 stopAudioDeviceRecordAndPlayTest{@link #IRTCAudioDeviceManager#stopAudioDeviceRecordAndPlayTest} 来停止采集并开始播放此前采集到的声音。
  ///       - 调用 stopAudioDevicePlayTest{@link #IRTCAudioDeviceManager#stopAudioDevicePlayTest} 可以停止音频设备采集和播放测试。
  ///       - 你不应在测试过程中，调用 `enableAudioPropertiesReport` 注册音量提示回调。
  ///       - 该方法仅在本地进行音频设备测试，不涉及网络连接。
  ///

  FutureOr<int> startAudioDeviceRecordTest(int interval) async {
    return await nativeCall('startAudioDeviceRecordTest', [interval]);
  }

  /// @detail api
  /// @brief 停止采集本地音频，并开始播放采集到的声音。录音播放完毕后，设备测试流程结束。 <br>
  /// 调用 startAudioDeviceRecordTest{@link #IRTCAudioDeviceManager#startAudioDeviceRecordTest} 30 s 内调用本接口来停止采集并开始播放此前采集到的声音。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note 调用本接口开始播放录音后，可以在播放过程中调用 stopAudioDevicePlayTest{@link #IRTCAudioDeviceManager#stopAudioDevicePlayTest} 停止播放。
  ///

  FutureOr<int> stopAudioDeviceRecordAndPlayTest() async {
    return await nativeCall('stopAudioDeviceRecordAndPlayTest', []);
  }

  /// @detail api
  /// @brief 停止由调用 startAudioDeviceRecordTest{@link #IRTCAudioDeviceManager#startAudioDeviceRecordTest} 开始的音频播放设备测试。 <br>
  ///        在音频播放设备测试自动结束前，可调用本接口停止音频采集与播放测试。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  ///

  FutureOr<int> stopAudioDevicePlayTest() async {
    return await nativeCall('stopAudioDevicePlayTest', []);
  }
}

class RTCEngine extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.RTCEngine';
  static get codegen_$namespace => _$namespace;

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 创建引擎对象 <br>
  ///        如果当前进程中未创建引擎实例，那么你必须先使用此方法，以使用 RTC 提供的各种音视频能力。 <br>
  ///        如果当前进程中已创建了引擎实例，再次调用此方法时，会返回已创建的引擎实例。
  /// @param config SDK 引擎配置参数，详见 EngineConfig{@link #EngineConfig}
  /// @param handler SDK 回调给应用层的 Handler，详见 IRTCEngineEventHandler{@link #IRTCEngineEventHandler}
  /// @return
  ///        - RTCEngine：创建成功。返回一个可用的 RTCEngine{@link #RTCEngine} 实例
  ///        - Null：EngineConfig 无效 详见 EngineConfig{@link #EngineConfig} ，so 文件加载失败。
  /// @note 你应注意保持 handler 的生命周期必须大于 RTCEngine{@link #RTCEngine} 的生命周期，即 handler 必须在调用 destroyRTCEngine{@link #RTCEngine#destroyRTCEngine} 之后销毁。
  ///

  static FutureOr<RTCEngine> createRTCEngine(
      EngineConfig config, IRTCEngineEventHandler handler) async {
    final result = await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'createRTCEngine',
      [config, handler],
      'com.volcengine.rtc.hybrid_runtime',
    );
    return packObject(result,
        () => RTCEngine(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 销毁由 createRTCEngine{@link #RTCEngine#createRTCEngine} 所创建的引擎实例，并释放所有相关资源。
  /// @note
  ///      - 请确保和需要销毁的 RTCEngine{@link #RTCEngine} 实例相关的业务场景全部结束后，才调用此方法
  ///      - 该方法在调用之后，会销毁所有和此 RTCEngine{@link #RTCEngine} 实例相关的内存，并且停止与媒体服务器的任何交互
  ///      - 调用本方法会启动 SDK 退出逻辑。引擎线程会保留，直到退出逻辑完成。因此，不要在回调线程中直接调用此 API，会导致死锁。同时此方法是耗时操作，不建议在主线程调用本方法，避免主线程阻塞。
  ///

  static FutureOr<void> destroyRTCEngine() async {
    return await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'destroyRTCEngine',
      [],
      'com.volcengine.rtc.hybrid_runtime',
    );
  }

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 获取 SDK 当前的版本号。
  /// @return SDK 当前的版本号。
  ///

  static FutureOr<String> getSDKVersion() async {
    return await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'getSDKVersion',
      [],
      'com.volcengine.rtc.hybrid_runtime',
    );
  }

  /// @detail api
  /// @author caofanglu
  /// @brief 配置 SDK 本地日志参数，包括日志级别、存储路径、日志文件最大占用的总空间、日志文件名前缀。
  /// @param logConfig 本地日志参数，参看 RTCLogConfig{@link #RTCLogConfig}。
  /// @return
  ///        - 0：成功。
  ///        - –1：失败，本方法必须在创建引擎前调用。
  ///        - –2：失败，参数填写错误。
  /// @note 本方法必须在调用 createRTCEngine{@link #RTCEngine#createRTCEngine} 之前调用。
  ///

  static FutureOr<int> setLogConfig(RTCLogConfig logConfig) async {
    return await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'setLogConfig',
      [logConfig],
      'com.volcengine.rtc.hybrid_runtime',
    );
  }

  RTCEngine([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @hidden for internal use only
  /// @author wangzhanqiang
  /// @brief 设置引擎事件回调的接收类，必须继承自 IRTCEngineEventHandler{@link #IRTCEngineEventHandler} 。
  /// @param engineEventHandler <br>
  ///        事件处理器接口类，详见 IRTCEngineEventHandler{@link #IRTCEngineEventHandler} 。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用方需要自行实现一个继承自 IRTCEngineEventHandler{@link #IRTCEngineEventHandler} 的类，并重载其中需要关注的事件。
  ///        - 该回调为异步回调
  ///        - 所有的事件回调均会在独立的回调线程内触发，请接收回调事件时注意所有与线程运行环境有关的操作，如需要在 UI 线程内执行的操作等，
  ///          请勿直接在回调函数的实现中直接进行操作。
  ///

  FutureOr<int> setRtcVideoEventHandler(
      IRTCEngineEventHandler engineEventHandler) async {
    return await nativeCall('setRtcVideoEventHandler', [engineEventHandler]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取音频设备管理接口
  /// @return 音频设备管理接口 IRTCAudioDeviceManager{@link #IRTCAudioDeviceManager}
  ///

  FutureOr<IRTCAudioDeviceManager> getAudioDeviceManager() async {
    final result = await nativeCall('getAudioDeviceManager', []);
    return packObject(
        result,
        () => IRTCAudioDeviceManager(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhangzhenyu.samuel
  /// @brief 立即开启内部视频采集。默认为关闭状态。 <br>
  ///        内部视频采集指：使用 RTC SDK 内置视频采集模块，进行采集。 <br>
  ///        调用该方法后，本地用户会收到 onVideoDeviceStateChanged{@link #IRTCEngineEventHandler#onVideoDeviceStateChanged} 的回调。 <br>
  ///        本地用户在非隐身状态下调用该方法后，房间中的其他用户会收到 onUserStartVideoCapture{@link #IRTCEngineEventHandler#onUserStartVideoCapture} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 自 v3.37.0 版本，使用本接口需要在 Gradle 里引入 Kotlin。
  ///       - 调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 可以停止内部视频采集。否则，只有当销毁引擎实例时，内部视频采集才会停止。
  ///       - 创建引擎后，无论是否发布视频数据，你都可以调用该方法开启内部视频采集。只有当（内部或外部）视频采集开始以后视频流才会发布。
  ///       - 如果需要从自定义视频采集切换为内部视频采集，你必须先停止发布流，关闭自定义采集，再调用此方法手动开启内部采集。
  ///       - 内部视频采集使用的摄像头由 switchCamera{@link #RTCEngine#switchCamera} 接口指定。
  ///       - 你还可以联系技术支持人员，帮助你在服务端配置采集格式并下发到 Android 端。
  ///

  FutureOr<int> startVideoCapture() async {
    return await nativeCall('startVideoCapture', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhangzhenyu.samuel
  /// @brief 立即关闭内部视频采集。默认为关闭状态。 <br>
  ///        内部视频采集指：使用 RTC SDK 内置视频采集模块，进行采集。 <br>
  ///        调用该方法，本地用户会收到 onVideoDeviceStateChanged{@link #IRTCEngineEventHandler#onVideoDeviceStateChanged} 的回调。 <br>
  ///        非隐身用户进房后调用该方法，房间中的其他用户会收到 onUserStopVideoCapture{@link #IRTCEngineEventHandler#onUserStopVideoCapture} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 可以开启内部视频采集。
  ///       - 如果不调用本方法停止内部视频采集，则只有当销毁引擎实例时，内部视频采集才会停止。
  ///

  FutureOr<int> stopVideoCapture() async {
    return await nativeCall('stopVideoCapture', []);
  }

  /// @detail api
  /// @author dixing
  /// @brief 开启内部音频采集。默认为关闭状态。 <br>
  ///        内部采集是指：使用 RTC SDK 内置的音频采集机制进行音频采集。 <br>
  ///        调用该方法开启后，本地用户会收到 onAudioDeviceStateChanged{@link #IRTCEngineEventHandler#onAudioDeviceStateChanged} 的回调。 <br>
  ///        非隐身用户进房后调用该方法，房间中的其他用户会收到 onUserStartAudioCapture{@link #IRTCEngineEventHandler#onUserStartAudioCapture} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 若未取得当前设备的麦克风权限，调用该方法后会触发 onAudioDeviceStateChanged{@link #IRTCEngineEventHandler#onAudioDeviceStateChanged} 回调，对应的错误码为 `MediaDeviceError.MEDIA_DEVICE_ERROR_NOPERMISSION = 1`。
  ///       - 调用 stopAudioCapture{@link #RTCEngine#stopAudioCapture} 可以关闭音频采集设备，否则，SDK 只会在销毁引擎的时候自动关闭设备。
  ///       - 由于不同硬件设备初始化响应时间不同，频繁调用 stopAudioCapture{@link #RTCEngine#stopAudioCapture} 和本接口闭麦/开麦可能出现短暂无声问题，建议使用 publishStreamAudio{@link #RTCRoom#publishStreamAudio} 实现临时闭麦和重新开麦。
  ///       - 创建引擎后，无论是否发布音频数据，你都可以调用该方法开启音频采集，并且调用后方可发布音频。
  ///       - 如果需要从自定义音频采集切换为内部音频采集，你必须先停止发布流，调用 setAudioSourceType{@link #RTCEngine#setAudioSourceType} 关闭自定义采集，再调用此方法手动开启内部采集。
  ///

  FutureOr<int> startAudioCapture() async {
    return await nativeCall('startAudioCapture', []);
  }

  /// @detail api
  /// @author dixing
  /// @brief 立即关闭内部音频采集。默认为关闭状态。 <br>
  ///        内部采集是指：使用 RTC SDK 内置的音频采集机制进行音频采集。 <br>
  ///        调用该方法，本地用户会收到 onAudioDeviceStateChanged{@link #IRTCEngineEventHandler#onAudioDeviceStateChanged} 的回调。 <br>
  ///        非隐身用户进房后调用该方法，房间中的其他用户会收到 onUserStopAudioCapture{@link #IRTCEngineEventHandler#onUserStopAudioCapture} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 startAudioCapture{@link #RTCEngine#startAudioCapture} 可以开启内部音频采集设备。
  ///       - 如果不调用本方法停止内部音频采集，则只有当销毁引擎实例时，内部音频采集才会停止。
  ///

  FutureOr<int> stopAudioCapture() async {
    return await nativeCall('stopAudioCapture', []);
  }

  /// @hidden(macOS,Windows,Linux)
  /// @valid since 3.60.
  /// @detail api
  /// @author gongzhengduo
  /// @brief 设置音频场景类型。 <br>
  ///        选择音频场景后，SDK 会自动根据场景切换对应的音量模式（通话音量/媒体音量）和改场景下的最佳音频配置。 <br>
  /// @param audioScenario 音频场景类型，参看 AudioScenarioType{@link #AudioScenarioType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 此接口在进房前后调用都有效。
  ///        - 通话音量更适合通话、会议等对信息准确度更高的场景。通话音量会激活系统硬件信号处理，使通话声音更清晰。同时，音量无法降低到 0。
  ///        - 媒体音量更适合娱乐场景，因其声音的表现力会更强。媒体音量下，最低音量可以为 0。
  ///

  FutureOr<int> setAudioScenario(AudioScenarioType audioScenario) async {
    return await nativeCall('setAudioScenario', [audioScenario.$value]);
  }

  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 设置音质档位。 <br>
  ///        当所选的 ChannelProfile{@link #ChannelProfile} 中的音频参数无法满足你的场景需求时，调用本接口切换的音质档位。
  /// @param audioProfile 音质档位，参看 AudioProfileType{@link #AudioProfileType}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法在进房前后均可调用；
  ///        - 支持通话过程中动态切换音质档位。
  ///

  FutureOr<int> setAudioProfile(AudioProfileType audioProfile) async {
    return await nativeCall('setAudioProfile', [audioProfile.$value]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author liuchuang
  /// @brief 支持根据业务场景，设置通话中的音频降噪模式。
  /// @param ansMode 降噪模式。具体参见 AnsMode{@link #AnsMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该接口进房前后均可调用，可重复调用，仅最后一次调用生效。
  ///        - 降噪算法包含传统降噪和 AI 降噪。传统降噪主要是抑制平稳噪声，比如空调声、风扇声等。而 AI 降噪主要是抑制非平稳噪声，比如键盘敲击声、桌椅碰撞声等。
  ///        - 只有以下 ChannelProfile{@link #ChannelProfile} 场景时，调用本接口可以开启 AI 降噪。其余场景的 AI 降噪不会生效。
  ///                 -  游戏语音模式： `CHANNEL_PROFILE_GAME(2)`
  ///                 -  高音质游戏模式： `CHANNEL_PROFILE_GAME_HD(8)`
  ///                 -  云游戏模式： `CHANNEL_PROFILE_CLOUD_GAME(3)`
  ///                 -  1 vs 1 音视频通话： `CHANNEL_PROFILE_CHAT(5)`
  ///                 -  多端同步播放音视频： `CHANNEL_PROFILE_LW_TOGETHER(7)`
  ///                 -  云端会议中的个人设备： `CHANNEL_PROFIEL_MEETING`
  ///                 -  课堂互动模式： `CHANNEL_PROFILE_MEETING_ROOM(17)`
  ///                 -  云端会议中的会议室终端： `CHANNEL_PROFILE_CLASSROOM(18)`
  ///

  FutureOr<int> setAnsMode(AnsMode ansMode) async {
    return await nativeCall('setAnsMode', [ansMode.$value]);
  }

  /// @valid since 3.32
  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置变声特效类型
  /// @param voiceChanger 变声特效类型，参看 VoiceChangerType{@link #VoiceChangerType}
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 如需使用该功能，需集成 SAMI 动态库，详情参看[按需集成插件](#1108726)文档。
  ///        - 在进房前后都可设置。
  ///        - 对 RTC SDK 内部采集的音频和自定义采集的音频都生效。
  ///        - 只对单声道音频生效。
  ///        - 与 setVoiceReverbType{@link #RTCEngine#setVoiceReverbType} 互斥，后设置的特效会覆盖先设置的特效。
  ///

  FutureOr<int> setVoiceChangerType(VoiceChangerType voiceChanger) async {
    return await nativeCall('setVoiceChangerType', [voiceChanger.$value]);
  }

  /// @valid since 3.32
  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置混响特效类型
  /// @param voiceReverb 混响特效类型，参看 VoiceReverbType{@link #VoiceReverbType}
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 在进房前后都可设置。
  ///        - 对 RTC SDK 内部采集的音频和自定义采集的音频都生效。
  ///        - 只对单声道音频生效。
  ///        - 与 setVoiceChangerType{@link #RTCEngine#setVoiceChangerType} 互斥，后设置的特效会覆盖先设置的特效。
  ///

  FutureOr<int> setVoiceReverbType(VoiceReverbType voiceReverb) async {
    return await nativeCall('setVoiceReverbType', [voiceReverb.$value]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置本地采集语音的均衡效果。包含内部采集和外部采集，但不包含混音音频文件。
  /// @param voiceEqualizationConfig 语音均衡效果，参看 VoiceEqualizationConfig{@link #VoiceEqualizationConfig}
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 根据奈奎斯特采样率，音频采样率必须大于等于设置的中心频率的两倍，否则，设置不生效。
  ///

  FutureOr<int> setLocalVoiceEqualization(
      VoiceEqualizationConfig voiceEqualizationConfig) async {
    return await nativeCall(
        'setLocalVoiceEqualization', [voiceEqualizationConfig]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置本地采集音频的混响效果。包含内部采集和外部采集，但不包含混音音频文件。
  /// @param config 混响效果，参看 VoiceReverbConfig{@link #VoiceReverbConfig}
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 调用 enableLocalVoiceReverb{@link #RTCEngine#enableLocalVoiceReverb} 开启混响效果。
  ///

  FutureOr<int> setLocalVoiceReverbParam(VoiceReverbConfig config) async {
    return await nativeCall('setLocalVoiceReverbParam', [config]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 开启本地音效混响效果
  /// @param enable 是否开启
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 调用 setLocalVoiceReverbParam{@link #RTCEngine#setLocalVoiceReverbParam} 设置混响效果。
  ///

  FutureOr<int> enableLocalVoiceReverb(boolean enable) async {
    return await nativeCall('enableLocalVoiceReverb', [enable]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author sunhang.io
  /// @brief 设置本地视频渲染时使用的视图，并设置渲染模式。
  /// @param videoCanvas 视图信息和渲染模式, 参看 VideoCanvas{@link #VideoCanvas}
  /// @return
  ///        - 0: 成功
  ///        - -2: 参数错误。
  ///        - -12: 本方法不支持在 Audio SDK 中使用。
  /// @note
  ///       - 你应在加入房间前，绑定本地视图。退出房间后，此设置仍然有效。
  ///       - 如果需要解除绑定，你可以调用本方法传入空视图。
  ///

  FutureOr<int> setLocalVideoCanvas(VideoCanvas videoCanvas) async {
    return await nativeCall('setLocalVideoCanvas', [videoCanvas]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfujun.911
  /// @brief 修改本地视频渲染模式和背景色。
  /// @param renderMode 渲染模式。参看 VideoCanvas{@link #VideoCanvas}.renderMode
  /// @param backgroundColor 背景颜色。参看 VideoCanvas{@link #VideoCanvas}.backgroundColor
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 你可以在本地视频渲染过程中，调用此接口。调用结果会实时生效。
  ///

  FutureOr<int> updateLocalVideoCanvas(
      int renderMode, int backgroundColor) async {
    return await nativeCall(
        'updateLocalVideoCanvas', [renderMode, backgroundColor]);
  }

  /// @deprecated since 3.60, use setLocalVideoSink(IVideoSink videoSink, LocalVideoSinkConfig config) instead.
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author sunhang.io
  /// @brief 将本地视频流与自定义渲染器绑定。
  /// @param videoSink 自定义视频渲染器，参看 IVideoSink{@link #IVideoSink}
  /// @param requiredFormat videoSink 适用的视频帧编码格式，参看 PixelFormat{@link #PixelFormat}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - RTC SDK 默认使用自带的渲染器（内部渲染器）进行视频渲染。
  ///        - 退房时将清除绑定状态。
  ///        - 如果需要解除绑定，你必须将 videoSink 设置为 null。
  ///        - 一般在收到 onFirstLocalVideoFrameCaptured{@link #IRTCEngineEventHandler#onFirstLocalVideoFrameCaptured} 回调通知完成本地视频首帧采集后，调用此方法为视频流绑定自定义渲染器；然后加入房间。
  ///        - 本方法获取的是前处理后的视频帧。
  ///

  FutureOr<int> setLocalVideoSink(
      IVideoSink videoSink, int requiredFormat) async {
    return await nativeCall('setLocalVideoSink', [videoSink, requiredFormat]);
  }

  /// @hidden for internal use only
  /// @valid since 3.54
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author Yujianli
  /// @brief 设置视频降噪模式。
  /// @param mode 视频降噪模式。参看 VideoDenoiseMode{@link #VideoDenoiseMode}。
  /// @return
  ///        - 0: API 调用成功。 用户可以根据回调函数 onVideoDenoiseModeChanged{@link #IRTCEngineEventHandler#onVideoDenoiseModeChanged} 判断视频降噪是否开启。
  ///        - < 0: API 调用失败。
  /// @note 该功能仅 arm 架构支持。
  ///

  FutureOr<int> setVideoDenoiser(VideoDenoiseMode mode) async {
    return await nativeCall('setVideoDenoiser', [mode.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 为采集到的视频流开启镜像
  /// @param mirrorType 镜像类型，参看 MirrorType{@link #MirrorType}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 切换视频源不影响镜像设置。
  ///        - 屏幕视频流始终不受镜像设置影响。
  ///        - 使用外部渲染器时，`mirrorType` 支持设置为 `0`（无镜像）和 `3`（本地预览和编码传输镜像），不支持设置为 `1`（本地预览镜像）。
  ///        - 该接口调用前，各视频源的初始状态如下：
  ///        <table>
  ///           <tr><th></th><th>前置摄像头</th><th>后置摄像头</th><th>自定义采集视频源</th> <th>桌面端摄像头</th> </tr>
  ///           <tr><td>移动端</td><td>本地预览镜像，编码传输不镜像</td><td> 本地预览不镜像，编码传输不镜像 </td><td> 本地预览不镜像，编码传输不镜像 </td><td>/</td></tr>
  ///           <tr><td>桌面端</td><td>/</td><td>/</td><td> 本地预览不镜像，编码传输不镜像 </td><td> 本地预览镜像，编码传输不镜像 </td></tr>
  ///        </table>
  ///

  FutureOr<int> setLocalVideoMirrorType(MirrorType mirrorType) async {
    return await nativeCall('setLocalVideoMirrorType', [mirrorType.$value]);
  }

  /// @valid since 3.57
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @brief 使用内部渲染时，为远端流开启镜像。
  /// @param streamId 流 ID，用于指定需要镜像的视频流。
  /// @param mirrorType 远端流的镜像类型，参看 RemoteMirrorType{@link #RemoteMirrorType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0: 调用失败，参看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  ///

  FutureOr<int> setRemoteVideoMirrorType(
      String streamId, RemoteMirrorType mirrorType) async {
    return await nativeCall(
        'setRemoteVideoMirrorType', [streamId, mirrorType.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 设置采集视频的旋转模式。默认以 App 方向为旋转参考系。 <br>
  ///        接收端渲染视频时，将按照和发送端相同的方式进行旋转。
  /// @param rotationMode 视频旋转参考系为 App 方向或重力方向，参看 VideoRotationMode{@link #VideoRotationMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 旋转仅对内部视频采集生效，不适用于外部视频源和屏幕源。
  ///        - 调用该接口时已开启视频采集，将立即生效；调用该接口时未开启视频采集，则将在采集开启后生效。
  ///        - 更多信息请参考[视频采集方向](https://www.volcengine.com/docs/6348/106458)。
  ///

  FutureOr<int> setVideoRotationMode(VideoRotationMode rotationMode) async {
    return await nativeCall('setVideoRotationMode', [rotationMode.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhangzhenyu.samuel
  /// @brief 切换视频内部采集时使用的前置/后置摄像头 <br>
  ///        调用此接口后，在本地会触发 onVideoDeviceStateChanged{@link #IRTCEngineEventHandler#onVideoDeviceStateChanged} 回调。
  /// @param cameraId 摄像头 ID，参看 CameraId{@link #CameraId}
  /// @return
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///        - 默认使用前置摄像头。
  ///        - 如果你正在使用相机进行视频采集，切换操作当即生效；如果相机未启动，后续开启内部采集时，会打开设定的摄像头。
  ///

  FutureOr<int> switchCamera(CameraId cameraId) async {
    return await nativeCall('switchCamera', [cameraId.$value]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 强制切换当前的音频播放路由。默认使用 setDefaultAudioRoute{@link #RTCEngine#setDefaultAudioRoute} 中设置的音频路由。 <br>
  ///        音频播放路由发生变化时，会收到 onAudioRouteChanged{@link #IRTCEngineEventHandler#onAudioRouteChanged} 回调。
  /// @param audioRoute 音频播放路由，参见 AudioRoute{@link #AudioRoute}。 <br>
  ///        对 Android 设备，不同的音频设备连接状态下，可切换的音频设备情况不同。参见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836)。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///      - 对于绝大多数音频场景，推荐使用 setDefaultAudioRoute{@link #RTCEngine#setDefaultAudioRoute} 设置默认音频路由，并借助 RTC SDK 的音频路由自动切换逻辑即可完成。切换逻辑参见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836)。你应仅在例外的场景下，使用此接口，比如在接入外接音频设备时，手动切换音频路由。
  ///      - 本接口仅支持在通话模式下使用。
  ///      - 不同音频场景中，音频路由和发布订阅状态到音量类型的映射关系详见 AudioScenarioType{@link #AudioScenarioType} 。
  ///

  FutureOr<int> setAudioRoute(AudioRoute audioRoute) async {
    return await nativeCall('setAudioRoute', [audioRoute.$value]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取当前使用的音频播放路由。
  /// @return 详见 AudioRoute{@link #AudioRoute}
  /// @note 要设置音频路由，详见 setAudioRoute{@link #RTCEngine#setAudioRoute}。
  ///

  FutureOr<AudioRoute> getAudioRoute() async {
    return await nativeCall('getAudioRoute', []);
  }

  /// @detail api
  /// @author dixing
  /// @brief 将默认的音频播放设备设置为听筒或扬声器。
  /// @param route 音频播放设备。参看 AudioRoute{@link #AudioRoute}。仅支持听筒或扬声器。
  /// @return
  ///        - 0: 方法调用成功。
  ///        - < 0: 方法调用失败。
  /// @note 对于音频路由切换逻辑，参见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836)。
  ///

  FutureOr<int> setDefaultAudioRoute(AudioRoute route) async {
    return await nativeCall('setDefaultAudioRoute', [route.$value]);
  }

  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 启用匹配外置声卡的音频处理模式
  /// @param enable <br>
  ///        - true: 开启
  ///        - false: 不开启(默认)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 当采用外接声卡进行音频采集时，建议开启此模式，以获得更好的音质。
  ///        - 开启此模式时，仅支持耳机播放。如果需要使用扬声器或者外置音箱播放，关闭此模式。
  ///

  FutureOr<int> enableExternalSoundCard(boolean enable) async {
    return await nativeCall('enableExternalSoundCard', [enable]);
  }

  /// @valid since 3.58.1
  /// @detail api
  /// @author shiyayun
  /// @brief 设置是否将采集到的音频信号静音，而不影响改变本端硬件采集状态。
  /// @param mute 是否静音音频采集。 <br>
  ///        - True：静音（关闭麦克风）
  ///        - False：（默认）开启麦克风
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 该方法用于设置是否使用静音数据替换设备采集到的音频数据进行推流，不影响 SDK 音频流的采集发布状态。
  ///        - 静音后通过 setCaptureVolume{@link #RTCEngine#setCaptureVolume} 调整音量不会取消静音状态，音量状态会保存至取消静音。
  ///        - 调用 startAudioCapture{@link #RTCEngine#startAudioCapture} 开启音频采集前后，都可以使用此接口设置采集音量。
  ///

  FutureOr<int> muteAudioCapture(boolean mute) async {
    return await nativeCall('muteAudioCapture', [mute]);
  }

  /// @valid since 3.60.
  /// @detail api
  /// @author shiyayun
  /// @brief 静音/取消静音屏幕共享时采集的音频。<br>
  ///        调用此方法后，SDK 将发送静音数据来代替真实的屏幕音频数据，不影响本端音频设备的采集状态和 SDK 音频流的采集发布状态。
  /// @param mute 是否静音屏幕音频。 <br>
  ///        - True：静音。远端用户听不到来自你屏幕共享的声音。
  ///        - False：（默认）取消静音。恢复发送屏幕共享的音频。
  /// @return
  ///        - 0：调用成功。
  ///        - < 0：调用失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 静音后通过 setCaptureVolume{@link #RTCEngine#setCaptureVolume} 调整音量不会取消静音状态，音量状态会保存至取消静音。
  ///        - 调用 startAudioCapture{@link #RTCEngine#startAudioCapture} 开启音频采集前后，都可以使用此接口设置采集音量。
  ///

  FutureOr<int> muteScreenAudioCapture(boolean mute) async {
    return await nativeCall('muteScreenAudioCapture', [mute]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 调节音频采集音量
  /// @param volume 采集的音量值和原始音量的百分比，范围是 [0, 400]，单位为 \%，自带溢出保护。 <br>
  ///               只改变音频数据的音量信息，不涉及本端硬件的音量调节。 <br>
  ///        为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///       - 0：静音
  ///       - 100：原始音量
  ///       - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 在开启音频采集前后，你都可以使用此接口设定采集音量。
  ///

  FutureOr<int> setCaptureVolume(int volume) async {
    return await nativeCall('setCaptureVolume', [volume]);
  }

  /// @detail api
  /// @valid Available since 3.60.
  /// @author wangjunzheng
  /// @brief 调节屏幕共享时采集的音频音量。<br>
  ///        只改变音频数据的音量信息，不影响麦克风采集的音量，也不会改变本端音频设备本身的音量。
  /// @param volume 采集的音量值和原始音量的百分比，范围是 [0, 400]，单位为 \%，自带溢出保护。<br>
  ///        为保证更好的通话质量，建议将 volume 值设为 [0, 100]。<br>
  ///         - 0：静音
  ///         - 100：原始音量
  ///         - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        + 0: 调用成功。<br>
  ///        + < 0: 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 在开启屏幕音频采集前后，你都可以使用此接口设定采集音量。
  ///

  FutureOr<int> setScreenCaptureVolume(int volume) async {
    return await nativeCall('setScreenCaptureVolume', [volume]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 调节本地播放的所有远端用户音频混音后的音量，混音内容包括远端人声、音乐、音效等。 <br>
  ///        播放音频前或播放音频时，你都可以使用此接口设定播放音量。
  /// @param volume 音频播放音量值和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。 <br>
  ///        为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///       - 0：静音
  ///       - 100：原始音量
  ///       - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内，当该方法与 setRemoteAudioPlaybackVolume{@link #RTCEngine#setRemoteAudioPlaybackVolume} 或 setRemoteRoomAudioPlaybackVolume{@link #RTCRoom#setRemoteRoomAudioPlaybackVolume} 共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。
  ///

  FutureOr<int> setPlaybackVolume(int volume) async {
    return await nativeCall('setPlaybackVolume', [volume]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 开启本地语音变调功能，多用于 K 歌场景。 <br>
  ///        使用该方法，你可以对本地语音的音调进行升调或降调等调整。
  /// @param pitch 相对于语音原始音调的升高/降低值，取值范围[-12，12]，默认值为 0，即不做调整。 <br>
  ///        取值范围内每相邻两个值的音高距离相差半音，正值表示升调，负值表示降调，设置的绝对值越大表示音调升高或降低越多。 <br>
  ///        超出取值范围则设置失败，并且会触发 onWarning{@link #IRTCEngineEventHandler#onWarning} 回调，提示 WarningCode{@link #WarningCode} 错误码为 `WARNING_CODE_SET_SCREEN_STREAM_INVALID_VOICE_PITCH` 设置语音音调不合法
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> setLocalVoicePitch(int pitch) async {
    return await nativeCall('setLocalVoicePitch', [pitch]);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 开启/关闭音量均衡功能。 <br>
  ///        开启音量均衡功能后，人声的响度会调整为 -16lufs。如果已调用 setAudioMixingLoudness 传入了混音音乐的原始响度，此音乐播放时，响度会调整为 -20lufs。
  /// @param enable 是否开启音量均衡功能： <br>
  ///       - true: 是
  ///       - false: 否
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 该接口须在调用 start{@link #IAudioEffectPlayer#start} 开始播放音频文件之前调用。
  ///

  FutureOr<int> enableVocalInstrumentBalance(boolean enable) async {
    return await nativeCall('enableVocalInstrumentBalance', [enable]);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 打开/关闭音量闪避功能，适用于在 RTC 通话过程中会同时播放短视频或音乐的场景，如“一起看”等。 <br>
  ///        开启该功能后，当检测到远端人声时，RTC 的本地的媒体播放音量会自动减弱，从而保证远端人声的清晰可辨；当远端人声消失时，RTC 的本地媒体音量会恢复到闪避前的音量水平。
  /// @param enable 是否开启音量闪避： <br>
  ///        - true: 是
  ///        - false: 否
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> enablePlaybackDucking(boolean enable) async {
    return await nativeCall('enablePlaybackDucking', [enable]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 登陆 RTS 服务器。 <br>
  ///        必须先登录，才能调用 sendUserMessageOutsideRoom{@link #RTCEngine#sendUserMessageOutsideRoom} 和 sendServerMessage{@link #RTCEngine#sendServerMessage} 发送房间外点对点消息和向应用服务器发送消息 <br>
  ///        在调用本接口登录后，如果想要登出，需要调用 logout{@link #RTCEngine#logout}。
  /// @param token 用户登录必须携带的 Token，用于鉴权验证。 <br>
  ///                测试时可使用[控制台](https://console.volcengine.com/rtc/listRTC)生成临时 Token，`roomId` 填任意值。 <br>
  ///                正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token，`roomId` 置空，Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。
  /// @param uid <br>
  ///        用户 ID <br>
  ///        用户 ID 在 appid 的维度下是唯一的。
  /// @return
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note 本地用户调用此方法登录成功后，会收到 onLoginResult{@link #IRTCEngineEventHandler#onLoginResult} 回调通登录结果，远端用户不会收到通知。
  ///

  FutureOr<int> login(String token, String uid) async {
    return await nativeCall('login', [token, uid]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 登出 RTS 服务器。 <br>
  ///        调用本接口登出后，无法调用房间外消息以及端到服务器消息相关的方法或收到相关回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用本接口登出后，必须先调用 login{@link #RTCEngine#login} 登录。
  ///       - 本地用户调用此方法登出后，会收到 onLogout{@link #IRTCEngineEventHandler#onLogout} 回调通知结果，远端用户不会收到通知。
  ///

  FutureOr<int> logout() async {
    return await nativeCall('logout', []);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 更新用户用于登录的 Token <br>
  ///        Token 有一定的有效期，当 Token 过期时，需调用此方法更新登录的 Token 信息。 <br>
  ///        调用 login{@link #RTCEngine#login} 方法登录时，如果使用了过期的 Token 将导致登录失败，并会收到 onLoginResult{@link #IRTCEngineEventHandler#onLoginResult} 回调通知，错误码为 `LOGIN_ERROR_CODE_INVALID_TOKEN`。此时需要重新获取 Token，并调用此方法更新 Token。
  /// @param token <br>
  ///        更新的动态密钥
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 如果 Token 无效导致登录失败，则调用此方法更新 Token 后，SDK 会自动重新登录，而用户不需要自己调用 login{@link #RTCEngine#login} 方法。
  ///       - Token 过期时，如果已经成功登录，则不会受到影响。Token 过期的错误会在下一次使用过期 Token 登录时，或因本地网络状况不佳导致断网重新登录时通知给用户。
  ///

  FutureOr<int> updateLoginToken(String token) async {
    return await nativeCall('updateLoginToken', [token]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 设置应用服务器参数 <br>
  ///        客户端调用 sendServerMessage{@link #RTCEngine#sendServerMessage} 或 sendServerBinaryMessage{@link #RTCEngine#sendServerBinaryMessage} 发送消息给应用服务器之前，必须需要设置有效签名和应用服务器地址。
  /// @param signature 动态签名，应用服务器可使用该签名验证消息来源。 <br>
  ///                  签名需自行定义，可传入任意非空字符串，建议将 uid 等信息编码为签名。 <br>
  ///                  设置的签名会以 post 形式发送至通过本方法中 url 参数设置的应用服务器地址。
  /// @param url 应用服务器的地址
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 用户必须调用 login{@link #RTCEngine#login} 登录后，才能调用本接口。
  ///       - 调用本接口后，SDK 会使用 onServerParamsSetResult{@link #IRTCEngineEventHandler#onServerParamsSetResult} 返回相应结果。
  ///

  FutureOr<int> setServerParams(String signature, String url) async {
    return await nativeCall('setServerParams', [signature, url]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 查询对端用户或本端用户的登录状态
  /// @param peerUserID 需要查询的用户 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 必须调用 login{@link #RTCEngine#login} 登录后，才能调用本接口。
  ///       - 调用本接口后，SDK 会使用 onGetPeerOnlineStatus{@link #IRTCEngineEventHandler#onGetPeerOnlineStatus} 回调通知查询结果。
  ///       - 在发送房间外消息之前，用户可以通过本接口了解对端用户是否登录，从而决定是否发送消息。也可以通过本接口查询自己查看自己的登录状态。
  ///

  FutureOr<int> getPeerOnlineStatus(String peerUserID) async {
    return await nativeCall('getPeerOnlineStatus', [peerUserID]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 给房间外指定的用户发送文本消息（P2P）
  /// @param uid <br>
  ///        消息接收用户的 ID
  /// @param message <br>
  ///        发送的文本消息内容。 <br>
  ///        消息不超过 64 KB。
  /// @param config 消息类型，参看 MessageConfig{@link #MessageConfig}。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  /// @note
  ///       - 在发送房间外文本消息前，必须先调用 login{@link #RTCEngine#login} 完成登录。
  ///       - 用户调用本接口发送文本信息后，会收到一次 onUserMessageSendResultOutsideRoom{@link #IRTCEngineEventHandler#onUserMessageSendResultOutsideRoom} 回调，得知消息是否成功发送。
  ///       - 若文本消息发送成功，则 uid 所指定的用户会通过 onUserMessageReceivedOutsideRoom{@link #IRTCEngineEventHandler#onUserMessageReceivedOutsideRoom} 回调收到该消息。
  ///

  FutureOr<long> sendUserMessageOutsideRoom(
      String uid, String message, MessageConfig config) async {
    return await nativeCall(
        'sendUserMessageOutsideRoom', [uid, message, config.$value]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 给房间外指定的用户发送二进制消息（P2P）
  /// @param uid <br>
  ///        消息接收用户的 ID
  /// @param buffer <br>
  ///        发送的二进制消息内容 <br>
  ///        消息不超过 64KB。
  /// @param config 消息类型，参看 MessageConfig{@link #MessageConfig}。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  ///        - -1：发送失败。消息为空。
  /// @note
  ///       - 在发送房间外二进制消息前，必须先调用 login{@link #RTCEngine#login} 完成登录。
  ///       - 用户调用本接口发送二进制消息后，会收到一次 onUserMessageSendResultOutsideRoom{@link #IRTCEngineEventHandler#onUserMessageSendResultOutsideRoom} 回调，通知消息是否发送成功；
  ///       - 若二进制消息发送成功，则 uid 所指定的用户会通过 onUserBinaryMessageReceivedOutsideRoom{@link #IRTCEngineEventHandler#onUserBinaryMessageReceivedOutsideRoom} 回调收到该条消息。
  ///

  FutureOr<long> sendUserBinaryMessageOutsideRoom(
      String uid, ArrayBuffer message, MessageConfig config) async {
    return await nativeCall(
        'sendUserBinaryMessageOutsideRoom', [uid, message, config.$value]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 客户端给应用服务器发送文本消息（P2Server）
  /// @param message <br>
  ///        发送的文本消息内容 <br>
  ///        消息不超过 64 KB。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  /// @note
  ///       - 在向应用服务器发送文本消息前，必须先调用 login{@link #RTCEngine#login} 完成登录，随后调用 setServerParams{@link #RTCEngine#setServerParams} 设置应用服务器。
  ///       - 调用本接口后会收到一次 onServerMessageSendResult{@link #IRTCEngineEventHandler#onServerMessageSendResult} 回调，通知消息发送方是否发送成功。
  ///       - 若文本消息发送成功，则之前调用 setServerParams{@link #RTCEngine#setServerParams} 设置的应用服务器会收到该条消息。
  ///

  FutureOr<long> sendServerMessage(String message) async {
    return await nativeCall('sendServerMessage', [message]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 客户端给应用服务器发送二进制消息（P2Server）
  /// @param buffer <br>
  ///        发送的二进制消息内容 <br>
  ///        消息不超过 64KB。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  ///        - -1：发送失败。消息为空。
  /// @note
  ///       - 在向应用服务器发送二进制消息前，必须先调用 login{@link #RTCEngine#login} 完成登录，随后调用 setServerParams{@link #RTCEngine#setServerParams} 设置应用服务器。
  ///       - 调用本接口后，会收到一次 onServerMessageSendResult{@link #IRTCEngineEventHandler#onServerMessageSendResult} 回调，通知消息发送方发送成功或失败；
  ///       - 若二进制消息发送成功，则之前调用 setServerParams{@link #RTCEngine#setServerParams} 设置的应用服务器会收到该条消息。
  ///

  FutureOr<long> sendServerBinaryMessage(ArrayBuffer buffer) async {
    return await nativeCall('sendServerBinaryMessage', [buffer]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 开启通话前网络探测
  /// @param isTestUplink 是否探测上行带宽
  /// @param expectedUplinkBitrate 期望上行带宽，单位：kbps<br>范围为 `{0, [100-10000]}`，其中， `0` 表示由 SDK 指定最高码率。
  /// @param isTestDownlink 是否探测下行带宽
  /// @param expectedDownlinkBitrate 期望下行带宽，单位：kbps<br>范围为 `{0, [100-10000]}`，其中， `0` 表示由 SDK 指定最高码率。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 成功调用本接口后，会在 3s 内收到一次 onNetworkDetectionResult{@link #IRTCEngineEventHandler#onNetworkDetectionResult} 回调，此后每 2s 收到一次该回调，通知探测结果；
  ///       - 若探测停止，则会收到一次 onNetworkDetectionStopped{@link #IRTCEngineEventHandler#onNetworkDetectionStopped} 通知探测停止。
  ///

  FutureOr<int> startNetworkDetection(
      boolean isTestUplink,
      int expectedUplinkBitrate,
      boolean isTestDownlink,
      int expectedDownlinkBitrate) async {
    return await nativeCall('startNetworkDetection', [
      isTestUplink,
      expectedUplinkBitrate,
      isTestDownlink,
      expectedDownlinkBitrate
    ]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 停止通话前网络探测
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用本接口后，会收到一次 onNetworkDetectionStopped{@link #IRTCEngineEventHandler#onNetworkDetectionStopped} 回调通知探测停止。
  ///

  FutureOr<int> stopNetworkDetection() async {
    return await nativeCall('stopNetworkDetection', []);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 设置并开启指定的音频数据帧回调
  /// @param method 音频回调方法，参看 AudioFrameCallbackMethod{@link #AudioFrameCallbackMethod}。 <br>
  ///               当音频回调方法设置为 `AUDIO_FRAME_CALLBACK_RECORD(0)`、`AUDIO_FRAME_CALLBACK_PLAYBACK(1)`、`AUDIO_FRAME_CALLBACK_MIXED(2)`、 `AUDIO_FRAME_CALLBACK_CAPTURE_MIXED(5)` 时，你需要在参数 `format` 中指定准确的采样率和声道，暂不支持设置为自动。 <br>
  ///               当音频回调方法设置为 `AUDIO_FRAME_CALLBACK_REMOTE_USER(3)`时，将 `format` 中的各个字段设置为默认值。
  /// @param format 音频参数格式，参看 AudioFormat{@link #AudioFormat}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 开启音频回调并调用 registerAudioFrameObserver{@link #RTCEngine#registerAudioFrameObserver} 后，IAudioFrameObserver{@link #IAudioFrameObserver} 会收到对应的音频回调。两者调用顺序没有限制且相互独立。
  ///

  FutureOr<int> enableAudioFrameCallback(
      AudioFrameCallbackMethod method, AudioFormat format) async {
    return await nativeCall(
        'enableAudioFrameCallback', [method.$value, format]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 关闭音频回调
  /// @param method 音频回调方法，参看 AudioFrameCallbackMethod{@link #AudioFrameCallbackMethod}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 该方法需要在调用 enableAudioFrameCallback{@link #RTCEngine#enableAudioFrameCallback} 之后调用。
  ///

  FutureOr<int> disableAudioFrameCallback(
      AudioFrameCallbackMethod method) async {
    return await nativeCall('disableAudioFrameCallback', [method.$value]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 注册音频数据回调观察者。
  /// @param observer 音频数据观察者，参看 IAudioFrameObserver{@link #IAudioFrameObserver}。如果传入 null，则取消注册。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 注册音频数据回调观察者并调用 enableAudioFrameCallback{@link #RTCEngine#enableAudioFrameCallback} 后，IAudioFrameObserver{@link #IAudioFrameObserver} 会收到对应的音频回调。对回调中收到的音频数据进行处理，不会影响 RTC 的编码发送或渲染。
  ///

  FutureOr<int> registerAudioFrameObserver(IAudioFrameObserver observer) async {
    return await nativeCall('registerAudioFrameObserver', [observer]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 注册自定义音频处理器。 <br>
  ///        注册完成后，你可以调用 enableAudioProcessor{@link #RTCEngine#enableAudioProcessor}，对本地采集到的音频进行处理，RTC SDK 将对处理后的音频进行编码和发送。也可以对接收到的远端音频进行自定义处理，RTC SDK 将对处理后的音频进行渲染。
  /// @param processor 自定义音频处理器，详见 IAudioFrameProcessor{@link #IAudioFrameProcessor}。 <br>
  ///        SDK 只持有 processor 的弱引用，你应保证其生命周期。需要取消注册时，设置此参数为 nullptr。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  /// - 重复调用此接口时，仅最后一次调用生效。
  /// - 更多相关信息，详见[音频自定义处理](https://www.volcengine.com/docs/6348/80635)。
  ///

  FutureOr<int> registerAudioProcessor(IAudioFrameProcessor processor) async {
    return await nativeCall('registerAudioProcessor', [processor]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 设置并开启指定的音频帧回调，进行自定义处理。
  /// @param method 音频帧类型，参看 AudioProcessorMethod{@link #AudioProcessorMethod}。可多次调用此接口，处理不同类型的音频帧。 <br>
  ///        选择不同类型的音频帧将收到对应的回调： <br>
  ///        - 选择本地采集的音频时，会收到 onProcessRecordAudioFrame{@link #IAudioFrameProcessor#onProcessRecordAudioFrame}。
  ///        - 选择远端音频流的混音音频时，会收到 onProcessPlayBackAudioFrame{@link #IAudioFrameProcessor#onProcessPlayBackAudioFrame}。
  ///        - 选择远端音频流时，会收到 onProcessRemoteUserAudioFrame{@link #IAudioFrameProcessor#onProcessRemoteUserAudioFrame}。
  ///        - 选择软件耳返音频时，会收到 onProcessEarMonitorAudioFrame{@link #IAudioFrameProcessor#onProcessEarMonitorAudioFrame}。
  ///        - 选择屏幕共享音频流时，会收到 onProcessScreenAudioFrame{@link #IAudioFrameProcessor#onProcessScreenAudioFrame}。
  /// @param format 设定自定义处理时获取的音频帧格式，参看 AudioFormat{@link #AudioFormat}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 在调用此接口前，你需要调用 registerAudioProcessor{@link #RTCEngine#registerAudioProcessor} 注册自定义音频处理器。
  ///        - 要关闭音频自定义处理，调用 disableAudioProcessor{@link #RTCEngine#disableAudioProcessor}。
  ///

  FutureOr<int> enableAudioProcessor(
      AudioProcessorMethod method, AudioFormat format) async {
    return await nativeCall('enableAudioProcessor', [method.$value, format]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 关闭自定义音频处理。
  /// @param method 音频帧类型，参看 AudioProcessorMethod{@link #AudioProcessorMethod}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> disableAudioProcessor(AudioProcessorMethod method) async {
    return await nativeCall('disableAudioProcessor', [method.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhushufan.ref
  /// @brief 设置自定义视频前处理器。 <br>
  ///        使用这个视频前处理器，你能够调用 processVideoFrame{@link #IVideoProcessor#processVideoFrame} 对 RTC SDK 采集得到的视频帧进行前处理，并将处理后的视频帧用于 RTC 音视频通信。
  /// @param processor 自定义视频处理器，详见 IVideoProcessor{@link #IVideoProcessor}。如果传入 null，则不对 RTC SDK 采集得到的视频帧进行前处理。 <br>
  ///        SDK 只持有 processor 的弱引用，你应保证其生命周期。
  /// @param config 自定义视频前处理器适用的设置，详见 VideoPreprocessorConfig{@link #VideoPreprocessorConfig}。 <br>
  ///               当前，`config` 中的 `required_pixel_format` 仅支持：`I420`、`TEXTURE_2D` 和 `Unknown`： <br>
  ///               - 设置为 `Unknown` 时，RTC SDK 给出供 processor 处理的视频帧格式即采集的格式。
  ///                 你可以通过 pixelFormat{@link #IVideoFrame#pixelFormat} 获取实际采集的视频帧格式，支持的格式为：`kVideoPixelFormatI420`、 `kVideoPixelFormatTexture2D` 和 `kVideoPixelFormatTextureOES` <br>
  ///               - 设置为 `I420` 或 `TEXTURE_2D` 时，RTC SDK 会将采集得到的视频转变为对应的格式，供前处理使用。
  ///               - 设置为其他值时，此方法调用失败。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 经前处理后，返回给 RTC SDK 的视频帧格式仅支持 `I420` 和 `TEXTURE_2D`。
  ///

  FutureOr<int> registerLocalVideoProcessor(
      IVideoProcessor processor, VideoPreprocessorConfig config) async {
    return await nativeCall('registerLocalVideoProcessor', [processor, config]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 设置本地摄像头数码变焦参数，包括缩放倍数，移动步长。
  /// @param type 数码变焦参数类型，缩放系数或移动步长。参看 ZoomConfigType{@link #ZoomConfigType}。必填。
  /// @param size 缩放系数或移动步长，保留到小数点后三位。默认值为 0。必填。 <br>
  ///                  选择不同 `type` 时有不同的取值范围。当计算后的结果超过缩放和移动边界时，取临界值。 <br>
  ///                  - `ZOOM_FOCUS_OFFSET(0)`：缩放系数增量，范围为 [0, 7]。例如，设置为 0.5 时，如果调用 setVideoDigitalZoomControl{@link #RTCEngine#setVideoDigitalZoomControl} 选择 Zoom in，则缩放系数增加 0.5。缩放系数范围 [1，8]，默认为 `1`，原始大小。
  ///                  - `ZOOM_MOVE_OFFSET(1)`：移动百分比，范围为 [0, 0.5]，默认为 0，不移动。如果调用 setVideoDigitalZoomControl{@link #RTCEngine#setVideoDigitalZoomControl} 选择的是左右移动，则移动距离为 size x 原始视频宽度；如果选择的是上下移动，则移动距离为 size x 原始视频高度。例如，视频帧边长为 1080 px，设置为 0.5 时，实际移动距离为 0.5 x 1080 px = 540 px。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 每次调用本接口只能设置一种参数。如果缩放系数和移动步长都需要设置，分别调用本接口传入相应参数。
  ///        - 由于移动步长的默认值为 `0` ，在调用 setVideoDigitalZoomControl{@link #RTCEngine#setVideoDigitalZoomControl} 或 startVideoDigitalZoomControl{@link #RTCEngine#startVideoDigitalZoomControl} 进行数码变焦操作前，应先调用本接口。
  ///

  FutureOr<int> setVideoDigitalZoomConfig(
      ZoomConfigType type, float size) async {
    return await nativeCall('setVideoDigitalZoomConfig', [type.$value, size]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 控制本地摄像头数码变焦，缩放或移动一次。设置对本地预览画面和发布到远端的视频都生效。
  /// @param direction 数码变焦操作类型，参看 ZoomDirectionType{@link #ZoomDirectionType}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 由于默认步长为 `0`，调用该方法前需通过 setVideoDigitalZoomConfig{@link #RTCEngine#setVideoDigitalZoomConfig} 设置参数。
  ///        - 调用该方法进行移动前，应先使用本方法或 startVideoDigitalZoomControl{@link #RTCEngine#startVideoDigitalZoomControl} 进行放大，否则无法移动。
  ///        - 当数码变焦操作超出范围时，将置为临界值。例如，移动到了图片边界、放大到了 8 倍、缩小到原图大小。
  ///        - 如果你希望实现持续数码变焦操作，调用 startVideoDigitalZoomControl{@link #RTCEngine#startVideoDigitalZoomControl}。
  ///        - 如果你需要对摄像头进行光学变焦控制，参看 setCameraZoomRatio{@link #RTCEngine#setCameraZoomRatio}。
  ///

  FutureOr<int> setVideoDigitalZoomControl(ZoomDirectionType direction) async {
    return await nativeCall('setVideoDigitalZoomControl', [direction.$value]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 开启本地摄像头持续数码变焦，缩放或移动。设置对本地预览画面和发布到远端的视频都生效。
  /// @param direction 数码变焦操作类型，参看 ZoomDirectionType{@link #ZoomDirectionType}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 由于默认步长为 `0`，调用该方法前需通过 setVideoDigitalZoomConfig{@link #RTCEngine#setVideoDigitalZoomConfig} 设置参数。
  ///        - 调用该方法进行移动前，应先使用本方法或 setVideoDigitalZoomControl{@link #RTCEngine#setVideoDigitalZoomControl} 进行放大，否则无法移动。
  ///        - 当数码变焦操作超出范围时，将置为临界值并停止操作。例如，移动到了图片边界、放大到了 8 倍、缩小到原图大小。
  ///        - 你也可以调用 stopVideoDigitalZoomControl{@link #RTCEngine#stopVideoDigitalZoomControl} 手动停止控制。
  ///        - 如果你希望实现单次数码变焦操作，调用 setVideoDigitalZoomControl{@link #RTCEngine#setVideoDigitalZoomControl}。
  ///        - 如果你需要对摄像头进行光学变焦控制，参看 setCameraZoomRatio{@link #RTCEngine#setCameraZoomRatio}。
  ///

  FutureOr<int> startVideoDigitalZoomControl(
      ZoomDirectionType direction) async {
    return await nativeCall('startVideoDigitalZoomControl', [direction.$value]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 停止本地摄像头持续数码变焦。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 关于开始数码变焦，参看 startVideoDigitalZoomControl{@link #RTCEngine#startVideoDigitalZoomControl}。
  ///

  FutureOr<int> stopVideoDigitalZoomControl() async {
    return await nativeCall('stopVideoDigitalZoomControl', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 注册本地视频帧监测器。 <br>
  ///        无论使用内部采集还是自定义采集，调用该方法后，SDK 每监测到一帧本地视频帧时，都会将视频帧信息通过 onLocalEncodedVideoFrame{@link #ILocalEncodedVideoFrameObserver#onLocalEncodedVideoFrame} 回调给用户。
  /// @param observer 本地频帧监测器，参看 ILocalEncodedVideoFrameObserver{@link #ILocalEncodedVideoFrameObserver} 。将参数设置为 null 则取消注册。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 该方法可在进房前后的任意时间调用，在进房前调用可保证尽可能早地监测视频帧并触发回调
  ///

  FutureOr<int> registerLocalEncodedVideoFrameObserver(
      ILocalEncodedVideoFrameObserver observer) async {
    return await nativeCall(
        'registerLocalEncodedVideoFrameObserver', [observer]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 注册远端编码后视频数据回调。 <br>
  ///        完成注册后，当 SDK 监测到远端编码后视频帧时，会触发 onRemoteEncodedVideoFrame{@link #IRemoteEncodedVideoFrameObserver#onRemoteEncodedVideoFrame} 回调
  /// @param observer 远端编码后视频数据监测器，参看 IRemoteEncodedVideoFrameObserver{@link #IRemoteEncodedVideoFrameObserver}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 更多自定义解码功能说明参看 [自定义视频编解码](https://www.volcengine.com/docs/6348/82921#\%E8\%87\%AA\%E5\%AE\%9A\%E4\%B9\%89\%E8\%A7\%86\%E9\%A2\%91\%E8\%A7\%A3\%E7\%A0\%81)。
  ///       - 该方法适用于手动订阅，并且进房前后均可调用，建议在进房前调用。
  ///       - 引擎销毁前需取消注册，调用该方法将参数设置为 "null" 即可。
  ///

  FutureOr<int> registerRemoteEncodedVideoFrameObserver(
      IRemoteEncodedVideoFrameObserver observer) async {
    return await nativeCall(
        'registerRemoteEncodedVideoFrameObserver', [observer]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 设置向 SDK 输入的视频源，包括屏幕流 <br>
  ///        默认使用内部采集。内部采集指：使用 RTC SDK 内置的视频采集机制进行视频采集。
  /// @param type 视频输入源类型，参看 VideoSourceType{@link #VideoSourceType}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法进房前后均可调用。
  ///        - 当你已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 开启内部采集后，再调用此方法切换至自定义采集时，SDK 会自动关闭内部采集。
  ///        - 当你调用此方法开启自定义采集后，想要切换至内部采集，你必须先调用此方法关闭自定义采集，然后调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 手动开启内部采集。
  ///        - 当你需要向 SDK 推送自定义编码后的视频帧，你需调用该方法将视频源切换至自定义编码视频源。
  ///

  FutureOr<int> setVideoSourceType(VideoSourceType type) async {
    return await nativeCall('setVideoSourceType', [type.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 推送外部视频帧。
  /// @param frame 视频帧的数据信息
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///       - 该方法主动将视频帧数据用 VideoFrameData{@link #VideoFrameData} 类封装后传递给 SDK。
  ///       - 请确保在你调用本方法前已调用 setVideoSourceType{@link #RTCEngine#setVideoSourceType} 设置为自定义视频采集。
  ///       - 当使用纹理数据时， 确保 createRTCEngine{@link #RTCEngine#createRTCEngine} 中的 `eglContext`与 `frame` 中的 `eglContext` 为 `sharedContext` 或者相同，否则会无法编码。
  ///       - 支持的格式：Raw： I420, NV12, RGBA；纹理： Texture2D, TextureOES。
  ///

  FutureOr<int> pushExternalVideoFrame(VideoFrameData frame) async {
    return await nativeCall('pushExternalVideoFrame', [frame]);
  }

  /// @hidden internal use only
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao
  /// @brief 关闭缓存同步功能。
  /// @return 查看 ReturnStatus{@link #ReturnStatus}。
  ///

  FutureOr<int> stopChorusCacheSync() async {
    return await nativeCall('stopChorusCacheSync', []);
  }

  /// @valid since 3.60. 自 3.60 起，该接口替代了 `startPushMixedStreamToCDN` 和 `startPushPublicStream` 方法用于实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此接口。
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author lizheng
  /// @brief 指定房间中的媒体流，合成后一路流发布到 CDN 或发布一路 WTN 流。
  /// @param taskId 转推直播任务 ID，长度不超过 127 字节。 当 MixedStreamConfig{@link #MixedStreamConfig} 中的 `PushTargetType = 0` 时， 用于标识转推直播任务。你可以在同一房间内发起多个转推直播任务，并用不同的 ID 加以区分。当你需要发起多个转推直播任务时，应使用多个 ID；当你仅需发起一个转推直播任务时，建议使用空字符串。 <br>
  /// 当 `PushTargetType = 1` 时，为公共流，此参数设置无效，传空即可。
  /// @param pushTargetConfig 推流目标配置参数，比如设置推流地址、WTN 流 ID。参看 MixedStreamPushTargetConfig{@link #MixedStreamPushTargetConfig}。
  /// @param mixedConfig 合流转推配置参数，比如设置合流的图片、视频视图布局和音频属性。参看 MixedStreamConfig{@link #MixedStreamConfig}。
  /// @return
  ///        - 0: 成功。你可以通过 onMixedStreamEvent{@link #IRTCEngineEventHandler#onMixedStreamEvent} 回调获取启动结果和推流过程中的事件。
  ///        - !0: 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 在[控制台](https://console.volcengine.com/rtc/cloudRTC?tab=callback)配置了转推直播和 WTN 流的服务端回调后，调用本接口会收到相应回调。重复调用该接口时，第二次调用会同时触发 [TranscodeStarted](https://www.volcengine.com/docs/6348/75125#transcodestarted) 和 [TranscodeUpdated](https://www.volcengine.com/docs/6348/75125#transcodeupdated)。
  ///       - 调用 stopPushMixedStream{@link #RTCEngine#stopPushMixedStream} 停止转推直播。
  ///       - 调用 updatePushMixedStream{@link #RTCEngine#updatePushMixedStream} 可以更新部分任务参数。
  ///

  FutureOr<int> startPushMixedStream(
      String taskId,
      MixedStreamPushTargetConfig pushTargetConfig,
      MixedStreamConfig mixedConfig) async {
    return await nativeCall(
        'startPushMixedStream', [taskId, pushTargetConfig, mixedConfig]);
  }

  /// @valid since 3.60. 自 3.60 起，该接口替代了 `updatePushMixedStreamToCDN` 和 `updatePublicStreamParam` 方法用于实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此接口。
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author lizheng
  /// @brief 更新由 startPushMixedStream{@link #RTCEngine#startPushMixedStream} 启动的任务参数，会收到 onMixedStreamEvent{@link #IRTCEngineEventHandler#onMixedStreamEvent} 回调。
  /// @param taskId 转推直播任务 ID。指定想要更新参数设置的转推直播任务。仅当 MixedStreamConfig{@link #MixedStreamConfig} 中的 `PushTargetType = 0` 时有效。
  /// @param pushTargetConfig 推流目标配置参数，比如设置推流地址、WTN 流 ID。参看 MixedStreamPushTargetConfig{@link #MixedStreamPushTargetConfig}。
  /// @param mixedConfig 转推直播配置参数，参看 MixedStreamConfig{@link #MixedStreamConfig}。除特殊说明外，均支持过程中更新。 <br>
  ///        调用时，结构体中没有传入值的属性，会被更新为默认值。
  /// @return
  ///        - 0: 成功。
  ///        - !0: 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  ///

  FutureOr<int> updatePushMixedStream(
      String taskId,
      MixedStreamPushTargetConfig pushTargetConfig,
      MixedStreamConfig mixedConfig) async {
    return await nativeCall(
        'updatePushMixedStream', [taskId, pushTargetConfig, mixedConfig]);
  }

  /// @valid since 3.60. 自 3.60 起，该接口替代了 `stopPushStreamToCDN` 方法来停止合流转推任务。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此接口。
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @brief 停止由 startPushMixedStream{@link #RTCEngine#startPushMixedStream} 启动的任务。
  /// @param taskId 转推直播任务 ID。指定想要更新参数设置的转推直播任务。
  /// @param targetType 参看 MixedStreamPushTargetType{@link #MixedStreamPushTargetType}。
  /// @return
  ///        - 0: 成功
  ///        - !0: 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  ///

  FutureOr<int> stopPushMixedStream(
      String taskId, MixedStreamPushTargetType targetType) async {
    return await nativeCall('stopPushMixedStream', [taskId, targetType.$value]);
  }

  FutureOr<int> pushClientMixedStreamExternalVideoFrame(
      String uid, VideoFrameData frame) async {
    return await nativeCall(
        'pushClientMixedStreamExternalVideoFrame', [uid, frame]);
  }

  FutureOr<int> setClientMixedStreamObserver(
      IClientMixedStreamObserver observer) async {
    return await nativeCall('setClientMixedStreamObserver', [observer]);
  }

  FutureOr<int> startClientMixedStream(
      String taskId,
      MixedStreamConfig mixedConfig,
      ClientMixedStreamConfig extraConfig) async {
    return await nativeCall(
        'startClientMixedStream', [taskId, mixedConfig, extraConfig]);
  }

  /// @hidden for internal use only
  /// @hiddensdk(audiosdk)
  ///

  FutureOr<int> updateClientMixedStream(
      String taskId,
      MixedStreamConfig mixedConfig,
      ClientMixedStreamConfig extraConfig) async {
    return await nativeCall(
        'updateClientMixedStream', [taskId, mixedConfig, extraConfig]);
  }

  /// @hidden for internal use only
  /// @hiddensdk(audiosdk)
  ///

  FutureOr<int> stopClientMixedStream(String taskId) async {
    return await nativeCall('stopClientMixedStream', [taskId]);
  }

  /// @valid since 3.60.
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author lizheng
  /// @brief 将房间内某一路音视频流，推送到指定的 RTC 房间或 CDN 地址。此过程不涉及编解码。
  /// @param taskId 任务 ID。 <br>
  ///               你可以发起多个转推直播任务，并用不同的任务 ID 加以区分。当你需要发起多个转推直播任务时，应使用多个 ID；当你仅需发起一个转推直播任务时，建议使用空字符串。
  /// @param param 转推直播配置参数。详见 PushSingleStreamParam{@link #PushSingleStreamParam}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 在调用该接口前，你需要在[控制台](https://console.volcengine.com/rtc/cloudRTC?tab=live&from=doc)开启转推直播功能。
  ///       - 调用该方法后，关于启动结果和推流过程中的错误，会收到 onSingleStreamEvent{@link #IRTCEngineEventHandler#onSingleStreamEvent} 回调。
  ///       - 在[控制台](https://console.volcengine.com/rtc/cloudRTC?tab=callback)配置了转推直播服务端回调后，调用本接口会收到相应回调。重复调用该接口时，第二次调用会同时触发 [TranscodeStarted](https://www.volcengine.com/docs/6348/75125#transcodestarted) 和 [TranscodeUpdated](https://www.volcengine.com/docs/6348/75125#transcodeupdated)。
  ///       - 调用 stopPushSingleStream{@link #RTCEngine#stopPushSingleStream} 停止任务。
  ///       - 由于本功能不进行编解码，所以推到 RTMP 的视频流会根据推流端的分辨率、编码方式、关闭摄像头等变化而变化。
  ///

  FutureOr<int> startPushSingleStream(
      String taskId, PushSingleStreamParam param) async {
    return await nativeCall('startPushSingleStream', [taskId, param]);
  }

  /// @valid since 3.60. 自 3.60 起，该接口替代了 `stopPushStreamToCDN` 方法来停止单流转推直播任务。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此接口。
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao
  /// @brief 停止通过 startPushSingleStream{@link #RTCEngine#startPushSingleStream} 发起的单流转推任务。
  /// @param taskId 任务 ID。可以指定想要停止的单流转推直播任务。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> stopPushSingleStream(String taskId) async {
    return await nativeCall('stopPushSingleStream', [taskId]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 设置 RTC SDK 内部采集时的视频采集参数。 <br>
  ///        如果你的项目使用了 SDK 内部采集模块，可以通过本接口指定视频采集参数包括模式、分辨率、帧率。
  /// @param videoCaptureConfig 视频采集参数。参看: VideoCaptureConfig{@link #VideoCaptureConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  /// - 本接口在引擎创建后即可调用，建议在调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 前调用本接口。
  /// - 建议同一设备上的不同引擎使用相同的视频采集参数。
  /// - 如果调用本接口前使用内部模块开始视频采集，采集参数默认为 Auto 模式。
  ///

  FutureOr<int> setVideoCaptureConfig(
      VideoCaptureConfig videoCaptureConfig) async {
    return await nativeCall('setVideoCaptureConfig', [videoCaptureConfig]);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author wangyu.1705
  /// @brief 发布端设置全景视频，包括分辨率、高清视野和低清背景分辨率、Tile 大小，以及其他常规编码参数。
  /// @param encoderConfig 期望发布的最大分辨率视频流参数。参看 VideoEncoderConfig{@link #VideoEncoderConfig}。 <br>
  ///                      支持 8K 和 4K 两种分辨率的全景视频。
  /// @param parameters 全景视频的编码参数，JSONObject 格式 <br>
  ///                  - 8K: HD: 7680x3840, LD: 2560x1280, Tile: 640x640
  ///                  - 4K: HD: 3840x1920, LD: 1280x640, Tile: 320x320
  /// { <br>
  ///  "rtc.fov_config":{ <br>
  ///      "mode":0,   //模式，只支持 `0` 等距柱状投影（Equirectangular Projection，ERP）模式 <br>
  ///      "hd_width":3840,    //高清视野的宽 <br>
  ///      "hd_height":1920,   //高清视野的高 <br>
  ///      "ld_width":1280,    //低清背景的宽 <br>
  ///      "ld_height":640,    //低清背景的高 <br>
  ///      "tile_width":320,   //Tile 的宽，取值建议能被全景视频宽、高清视野宽、低清背景宽整除 <br>
  ///      "tile_height":320,  //Tile 的高，取值建议能被全景视频高、高清视野高、低清背景高整除 <br>
  ///      "framerate":30, //帧率 <br>
  ///      "max_kbps":40000} //期望编码码率 <br>
  /// @return 方法调用结果： <br>
  ///        - 0：成功
  ///        - !0：失败
  /// @note
  ///        - 发布全景视频前，绑定自定义采集器，必须调用该方法设置编码参数。支持的视频格式包括 YUV 或者 Texture 纹理。
  ///        - 通过 onFrame{@link #IVideoSink#onFrame} ，接收端获取到视频帧和解码需要的信息，传给自定义渲染器进行渲染。
  ///

  FutureOr<int> setVideoEncoderConfig(
      VideoEncoderConfig encoderConfig, JSONObject parameters) async {
    return await nativeCall(
        'setVideoEncoderConfig', [encoderConfig, parameters]);
  }

  /// @valid since 3.60.
  /// @detail api
  /// @author zhoubohui
  /// @brief 发布端进行大小流(simulcast)设置。
  /// @param mode 详见 VideoSimulcastMode{@link #VideoSimulcastMode}。默认为只发送单流。你应在进房前调用修改本参数。
  /// @param streamConfig 小流参数。最多可设置 3 路。分辨率按照从小到大顺序，且每路流参数分辨率需小于大流 setVideoEncoderConfig{@link #RTCEngine#setVideoEncoderConfig} 设置参数。否则可能会设置失败。参看 VideoEncoderConfig{@link #VideoEncoderConfig}。
  ///        其余模式下，默认小流参数为 160px × 90px, 码率为 50kpbs。
  /// @return 方法调用结果： <br>
  ///        - 0：成功
  ///        - !0：失败
  /// @note
  ///        - 调用本方法前，SDK 默认仅发布一条分辨率为 640px × 360px \@15fps 的视频流。
  ///        - 本方法适用于摄像头采集的视频流。
  ///        - 更多信息详见[推送多路流](https://www.volcengine.com/docs/6348/70139)文档。
  ///

  FutureOr<int> setLocalSimulcastMode(
      VideoSimulcastMode mode, Array<VideoEncoderConfig> streamConfig) async {
    return await nativeCall(
        'setLocalSimulcastMode', [mode.$value, streamConfig]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 切换音频采集方式
  /// @param type 音频数据源，详见 AudioSourceType{@link #AudioSourceType}。 <br>
  ///             默认使用内部音频采集。音频采集和渲染方式无需对应。
  /// @return 方法调用结果： <br>
  ///        - =0: 切换成功。
  ///        - <0：切换失败。
  /// @note
  ///      - 进房前后调用此方法均有效。
  ///      - 如果你调用此方法由内部采集切换至自定义采集，SDK 会自动关闭内部采集。然后，调用 pushExternalAudioFrame{@link #RTCEngine#pushExternalAudioFrame} 推送自定义采集的音频数据到 RTC SDK 用于传输。
  ///      - 如果你调用此方法由自定义采集切换至内部采集，你必须再调用 startAudioCapture{@link #RTCEngine#startAudioCapture} 手动开启内部采集。
  ///

  FutureOr<int> setAudioSourceType(AudioSourceType type) async {
    return await nativeCall('setAudioSourceType', [type.$value]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 切换音频渲染方式
  /// @param type 音频输出类型，详见 AudioRenderType{@link #AudioRenderType} <br>
  ///             默认使用内部音频渲染。音频采集和渲染方式无需对应。
  /// @return 方法调用结果： <br>
  ///        - =0: 切换成功。
  ///        - <0：切换失败。
  /// @note
  ///      - 进房前后调用此方法均有效。
  ///      - 如果你调用此方法切换至自定义渲染，调用 pullExternalAudioFrame{@link #RTCEngine#pullExternalAudioFrame} 获取音频数据。
  ///

  FutureOr<int> setAudioRenderType(AudioRenderType type) async {
    return await nativeCall('setAudioRenderType', [type.$value]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 推送自定义采集的音频数据到 RTC SDK。
  /// @param audioFrame 音频数据帧，详见 AudioFrame{@link #AudioFrame} <br>
  ///                   - 音频采样格式必须为 S16。音频缓冲区内的数据格式必须为 PCM，其容量大小应该为 audioFrame.samples × audioFrame.channel × 2。
  ///                   - 必须指定具体的采样率和声道数，不支持设置为自动。
  /// @return 方法调用结果 <br>
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 推送自定义采集的音频数据前，必须先调用 setAudioSourceType{@link #RTCEngine#setAudioSourceType} 开启自定义采集。
  ///       - 你必须每隔 10 毫秒推送一次外部采集的音频数据。单次推送的 samples (音频采样点个数）应该为 audioFrame.sampleRate / 100。比如设置采样率为 48000 时， 每次应该推送 480 个采样点。
  ///

  FutureOr<int> pushExternalAudioFrame(AudioFrame audioFrame) async {
    return await nativeCall('pushExternalAudioFrame', [audioFrame]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 拉取下行音频数据用于自定义音频渲染。 <br>
  ///        调用该方法后，SDK 会主动拉取待播放的音频数据，包括远端已解码和混音后的音频数据，用于外部播放。
  /// @param audioFrame 音频数据帧，详见 AudioFrame{@link #AudioFrame}
  /// @return 方法调用结果 <br>
  ///          - 0: 设置成功
  ///          - < 0: 设置失败
  /// @note
  ///       - 拉取外部音频数据前，必须先调用 setAudioRenderType{@link #RTCEngine#setAudioRenderType} 启用自定义音频渲染。
  ///       - 由于 RTC SDK 的帧长为 10 毫秒，你应当每隔 10 毫秒拉取一次音频数据。确保音频采样点数（sample）x 拉取频率等于 audioFrame 的采样率 （sampleRate）。如设置采样率为 48000 时，每 10 毫秒调用本接口拉取数据，每次应拉取 480 个采样点。
  ///       - 音频采样格式为 S16。音频缓冲区内的数据格式为 PCM 数据，其容量大小为 audioFrame.samples × audioFrame.channel × 2。
  ///

  FutureOr<int> pullExternalAudioFrame(AudioFrame audioFrame) async {
    return await nativeCall('pullExternalAudioFrame', [audioFrame]);
  }

  /// @detail api
  /// @region 自定义音频回声消除参考信号
  /// @author cuiyao
  /// @brief 向 SDK 发送由自定义处理后的 PCM 音频数据，作为回声消除的参考信号
  /// @param audioFrame 音频数据帧，详见 AudioFrame{@link #AudioFrame}
  /// @return 方法调用结果  <br>
  ///        + 0：方法调用成功  <br>
  ///        + <-1：方法调用失败  <br>
  /// @note  <br>
  ///       + 由于 RTC SDK 的帧长为 10 毫秒，你应当每隔 10 毫秒发送一次音频数据。确保音频采样点数（sample）x 发送频率等于 audioFrame 的采样率 （sampleRate）。如设置采样率为 48000 时，每 10 毫秒调用本接口发送数据，每次应发送 480 个采样点。  <br>
  ///       + 音频采样格式为 S16。音频缓冲区内的数据格式为 PCM 数据，其容量大小为 audioFrame.samples × audioFrame.channel × 2。
  ///

  FutureOr<int> pushReferenceAudioPCMData(AudioFrame audioFrame) async {
    return await nativeCall('pushReferenceAudioPCMData', [audioFrame]);
  }

  /// @hidden for internal use only
  /// @region 自定义音频采集渲染
  /// @brief 是否使用sdk音频编码功能。
  /// @param enable 是否使用sdk音频编码功能。
  ///      true: 打开音频编码（默认）
  ///      false: 关闭音频编码直接转推。
  /// @note
  ///       - 在pushExternalEncodedAudioFrame{@link #RTCEngine#pushExternalEncodedAudioFrame}之前调用。
  ///

  FutureOr<void> enableAudioEncoding(boolean enable) async {
    return await nativeCall('enableAudioEncoding', [enable]);
  }

  /// @hidden for internal use only
  /// @region 自定义音频采集渲染
  /// @brief 是否使用sdk音频解码功能。
  /// @param enable 是否使用sdk音频解码功能。
  ///      true: 打开音频解码功能（默认）
  ///      false: 关闭音频解码功能直接转推。
  /// @note
  ///       - 在registerRemoteEncodedAudioFrameObserver之前调用。
  ///

  FutureOr<void> enableAudioDecoding(boolean enable) async {
    return await nativeCall('enableAudioDecoding', [enable]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 创建 RTC 房间实例。 <br>
  ///        调用此方法仅返回一个房间实例，你仍需调用 joinRoom{@link #RTCRoom#joinRoom} 才能真正地创建/加入房间。 <br>
  ///        多次调用此方法以创建多个 RTCRoom{@link #RTCRoom} 实例。分别调用各 RTCRoom 实例中的 joinRoom{@link #RTCRoom#joinRoom} 方法，同时加入多个房间。 <br>
  ///        多房间模式下，用户可以同时订阅各房间的音视频流。
  /// @param roomId 标识通话房间的房间 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。
  /// @return 创建的 RTCRoom{@link #RTCRoom} 房间实例。
  ///         返回 NULL 时，请确认指定房间是否已经存在或 roomId 格式错误。
  /// @note
  ///        - 如果需要加入的房间已存在，你仍需先调用本方法来获取 RTCRoom 实例，再调用 joinRoom{@link #RTCRoom#joinRoom} 加入房间。
  ///        - 请勿使用同样的 roomId 创建多个房间，否则后创建的房间实例会替换先创建的房间实例。
  ///        - 如果你需要在多个房间发布音视频流，无须创建多房间，直接调用 startForwardStreamToRooms{@link #RTCRoom#startForwardStreamToRooms} 开始跨房间转发媒体流。
  ///

  FutureOr<RTCRoom> createRTCRoom(String roomId) async {
    final result = await nativeCall('createRTCRoom', [roomId]);
    return packObject(
        result, () => RTCRoom(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 创建游戏语音房间实例。 <br>
  ///        调用此方法仅返回一个房间实例，你仍需调用 joinRoom{@link #IGameRoom#joinRoom} 才能真正地创建/加入房间。 <br>
  ///        多次调用此方法以创建多个 IGameRoom{@link #IGameRoom} 实例。分别调用各 RTCRoom 实例中的 joinRoom{@link #IGameRoom#joinRoom} 方法，同时加入多个房间。 <br>
  ///        多房间模式下，用户可以同时订阅各房间的音视频流。
  /// @param roomId 标识通话房间的房间 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。
  /// @param config 游戏语音房间配置。参看 GameRoomConfig{@link #GameRoomConfig}。
  /// @return 创建的 IGameRoom{@link #IGameRoom} 房间实例。
  ///         返回 NULL 时，请确认指定房间是否已经存在或 roomId 格式错误。
  /// @note
  ///        - 如果需要加入的房间已存在，你仍需先调用本方法来获取 IGameRoom实例，再调用 joinRoom{@link #IGameRoom#joinRoom} 加入房间。
  ///        - 请勿使用同样的 roomId 创建多个房间，否则后创建的房间实例会替换先创建的房间实例。
  ///        - 如果你需要在多个房间发布音视频流，无须创建多房间，直接调用 startForwardStreamToRooms{@link #RTCRoom#startForwardStreamToRooms} 开始跨房间转发媒体流。
  ///

  FutureOr<IGameRoom> createGameRoom(
      String roomId, GameRoomConfig config) async {
    final result = await nativeCall('createGameRoom', [roomId, config]);
    return packObject(result,
        () => IGameRoom(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @brief 创建 RTSRoom{@link #RTSRoom} 实例。<br>
  ///        调用此方法仅返回一个房间实例，你仍需调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 才能真正地创建/加入房间。
  /// @param roomId 标识通话房间的房间 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。
  /// @return 创建的 RTSRoom{@link #RTSRoom} 房间实例。
  /// @note 请勿使用同样的 roomId 创建多个房间，否则后创建的房间实例会替换先创建的房间实例。
  ///

  FutureOr<RTSRoom> createRTSRoom(String roomId) async {
    final result = await nativeCall('createRTSRoom', [roomId]);
    return packObject(
        result, () => RTSRoom(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置发布的音视频流的回退选项。 <br>
  ///        你可以调用该接口设置网络不佳或设备性能不足时从大流起进行降级处理，以保证通话质量。
  /// @param option 本地发布的音视频流回退选项，参看 PublishFallbackOption{@link #PublishFallbackOption}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅在调用 setLocalSimulcastMode{@link #RTCEngine#setlocalsimulcastmode-2} 开启了发送多路视频流的情况下生效。
  ///        - 该方法必须在进房前设置，进房后设置或更改设置无效。
  ///        - 调用该方法后，如因性能或网络不佳产生发布性能回退或恢复，本端会提前收到 onPerformanceAlarms{@link #IRTCEngineEventHandler#onPerformanceAlarms} 回调发出的告警，以便采集设备配合调整。
  ///        - 设置回退后，本地发布的音视频流发生回退或从回退中恢复时，远端会收到 onSimulcastSubscribeFallback{@link #IRTCEngineEventHandler#onSimulcastSubscribeFallback} 回调，通知该情况。
  ///        - 你可以调用客户端 API 或者在服务端下发策略设置回退。当使用服务端下发配置实现时，下发配置优先级高于在客户端使用 API 设定的配置。
  ///

  FutureOr<int> setPublishFallbackOption(PublishFallbackOption option) async {
    return await nativeCall('setPublishFallbackOption', [option.$value]);
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置订阅的音视频流的回退选项。 <br>
  ///        你可调用该接口设置网络不佳或设备性能不足时允许订阅流进行降级或只订阅音频流，以保证通话流畅。
  /// @param option 订阅的音视频流回退选项，参看 SubscribeFallbackOptions{@link #SubscribeFallbackOptions}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 你必须在进房前设置，进房后设置或更改设置无效。
  ///        - 设置回退选项后，订阅的音视频流发生回退或从回退中恢复时，会收到 onSimulcastSubscribeFallback{@link #IRTCEngineEventHandler#onSimulcastSubscribeFallback} 和 onRemoteVideoSizeChanged{@link #IRTCEngineEventHandler#onRemoteVideoSizeChanged} 回调通知。
  ///        - 你可以调用 API 或者在服务端下发策略设置回退。当使用服务端下发配置实现时，下发配置优先级高于在客户端使用 API 设定的配置。
  ///

  FutureOr<int> setSubscribeFallbackOption(
      SubscribeFallbackOptions option) async {
    return await nativeCall('setSubscribeFallbackOption', [option.$value]);
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置用户优先级。
  /// @param roomid 房间 ID
  /// @param uid <br>
  ///        远端用户的 ID 。
  /// @param priority <br>
  ///        远端用户的优先级，详见枚举类型 RemoteUserPriority{@link #RemoteUserPriority} 。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该方法与 setSubscribeFallbackOption{@link #RTCEngine#setSubscribeFallbackOption} 搭配使用。
  ///        - 如果开启了订阅流回退选项，弱网或性能不足时会优先保证收到的高优先级用户的流的质量。
  ///        - 该方法在进房前后都可以使用，可以修改远端用户的优先级。
  ///

  FutureOr<int> setRemoteUserPriority(
      String roomid, String uid, RemoteUserPriority priority) async {
    return await nativeCall(
        'setRemoteUserPriority', [roomid, uid, priority.$value]);
  }

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 设置业务标识参数 <br>
  ///        可通过 businessId 区分不同的业务场景。businessId 由客户自定义，相当于一个“标签”，可以分担和细化现在 AppId 的逻辑划分的功能，但不需要鉴权。
  /// @param businessId <br>
  ///        用户设置的自己的 businessId 值 <br>
  ///        businessId 只是一个标签，颗粒度需要用户自定义。
  /// @return
  ///        - 0： 成功。
  ///        - -2： 输入非法，合法字符包括所有小写字母、大写字母和数字，除此外还包括四个独立字符，分别是：英文句号，短横线，下划线和 \@ 。
  /// @note
  ///        - 需要在进房前调用，进房后调用该方法无效。
  ///

  FutureOr<int> setBusinessId(String businessId) async {
    return await nativeCall('setBusinessId', [businessId]);
  }

  /// @detail api
  /// @author wangjunlin.3182
  /// @brief 设置传输时使用内置加密的方式。
  /// @param aesType 加密类型。可选参数为 0、1、2、3、4。代表的含义如下： <br>
  ///        0. 不加密 <br>
  ///        1. AES-128-CBC <br>
  ///        2. AES-256-CBC <br>
  ///        3. AES-128-ECB <br>
  ///        4. AES-256-ECB
  /// @param key 加密密钥。长度限制为 36 位，超出部分将会被截断。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法与 setCustomizeEncryptHandler{@link #RTCEngine#setCustomizeEncryptHandler} 为互斥关系，即按照调用顺序，最后一个调用的方法为最终生效的版本。
  ///        - 该方法必须在调用 joinRoom{@link #RTCRoom#joinRoom} 之前调用，可重复调用，以最后调用的参数作为生效参数
  ///

  FutureOr<int> setEncryptInfo(int aesType, String key) async {
    return await nativeCall('setEncryptInfo', [aesType, key]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 获取视频特效接口。
  /// @return 视频特效接口，参看 IVideoEffect{@link #IVideoEffect}。
  ///

  FutureOr<IVideoEffect> getVideoEffectInterface() async {
    final result = await nativeCall('getVideoEffectInterface', []);
    return packObject(result,
        () => IVideoEffect(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 开启/关闭基础美颜。
  /// @param enable 基础美颜开关 <br>
  ///        - true: 开启基础美颜
  ///        - false: 关闭基础美颜（默认）
  /// @return
  ///        - 0: 调用成功。
  ///        - –1001: RTC SDK 版本不支持此功能。
  ///        - -12: 本方法不支持在 Audio SDK 中使用。
  ///        - <0: 调用失败，特效 SDK 内部错误，具体错误码请参考[错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///        - 本方法不能与高级视频特效接口共用。如已购买高级视频特效，建议参看[集成指南](https://www.volcengine.com/docs/6348/114717)使用高级美颜、特效、贴纸功能等。
  ///        - 使用此功能需要集成特效 SDK，建议使用特效 SDK v4.4.2+ 版本。更多信息参看 [Native 端基础美颜](https://www.volcengine.com/docs/6348/372605)。
  ///        - 调用 setBeautyIntensity{@link #RTCEngine#setBeautyIntensity} 设置基础美颜强度。若在调用本方法前没有设置美颜强度，则使用默认强度。各基础美颜模式的强度默认值分别为：美白 0.7，磨皮 0.8，锐化 0.5，清晰 0.7。
  ///        - 本方法仅适用于视频源，不适用于屏幕源。
  ///

  FutureOr<int> enableEffectBeauty(boolean enable) async {
    return await nativeCall('enableEffectBeauty', [enable]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 调整基础美颜强度
  /// @param beautyMode 基础美颜模式，参看 EffectBeautyMode{@link #EffectBeautyMode}。
  /// @param intensity 美颜强度，取值范围为 [0,1]。强度为 0 表示关闭。 <br>
  ///                  各基础美颜模式的强度默认值分别为：美白 0.7，磨皮 0.8，锐化 0.5，清晰 0.7。
  /// @return
  ///        - 0: 调用成功。
  ///        - –2: `intensity` 范围超限。
  ///        - –1001: RTC SDK 版本不支持此功能。
  ///        - <0: 调用失败，特效 SDK 内部错误，具体错误码请参考[错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///        - 若在调用 enableEffectBeauty{@link #RTCEngine#enableEffectBeauty} 前设置美颜强度，则对应美颜功能的强度初始值会根据设置更新。
  ///        - 销毁引擎后，美颜功能强度恢复默认值。
  ///

  FutureOr<int> setBeautyIntensity(
      EffectBeautyMode beautyMode, float intensity) async {
    return await nativeCall(
        'setBeautyIntensity', [beautyMode.$value, intensity]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 在自定义视频前处理及编码前，设置 RTC 链路中的视频帧朝向，默认为 Adaptive 模式。 <br>
  ///        移动端开启视频特效贴纸，或使用自定义视频前处理时，建议固定视频帧朝向为 Portrait 模式。单流转推场景下，建议根据业务需要固定视频帧朝向为 Portrait 或 Landscape 模式。不同模式的具体显示效果参看[视频帧朝向](https://www.volcengine.com/docs/6348/128787)。
  /// @param orientation 视频帧朝向，参看 VideoOrientation{@link #VideoOrientation}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 视频帧朝向设置仅适用于内部采集视频源。对于自定义采集视频源，设置视频帧朝向可能会导致错误，例如宽高对调。屏幕源不支持设置视频帧朝向。
  ///        - 编码分辨率的更新与视频帧处理是异步操作，进房后切换视频帧朝向可能导致画面出现短暂的裁切异常，因此建议在进房前设置视频帧朝向，且不在进房后进行切换。
  ///

  FutureOr<int> setVideoOrientation(VideoOrientation orientation) async {
    return await nativeCall('setVideoOrientation', [orientation.$value]);
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置运行时的参数
  /// @param params 保留参数
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 该接口需在 joinRoom{@link #RTCRoom#joinRoom} 和 startAudioCapture{@link #RTCEngine#startAudioCapture} 之前调用。
  ///

  FutureOr<int> setRuntimeParameters(JSONObject params) async {
    return await nativeCall('setRuntimeParameters', [params]);
  }

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 将用户反馈的问题上报到 RTC。
  /// @param types 预设问题列表，参看 ProblemFeedbackOption{@link #ProblemFeedbackOption}
  /// @param info 预设问题以外的其他问题的具体描述、房间信息，参看 ProblemFeedbackInfo{@link #ProblemFeedbackInfo}
  /// @return
  ///         - 0: 成功。
  ///         - -3: 失败。
  /// @note
  ///         - 你可以在 [RTC 控制台](https://console.volcengine.com/rtc/callQualityRTC/feedback)上查看用户通过此接口提交的反馈详情和整体趋势。
  ///         - 如果用户上报时在房间内，那么问题会定位到用户当前所在的一个或多个房间；如果用户上报时不在房间内，那么问题会定位到引擎此前退出的房间。
  ///

  FutureOr<int> feedback(
      List<ProblemFeedbackOption> types, ProblemFeedbackInfo info) async {
    return await nativeCall('feedback', [types, info]);
  }

  /// @valid since 353
  /// @detail api
  /// @author likai.666
  /// @valid since 353
  /// @brief 获取 C++ 层 [IRTCEngine 句柄](https://www.volcengine.com/docs/6348/70095#irtcengine)。
  /// @return
  ///         - >0：方法调用成功, 返回 C++ 层 `IRTCEngine` 的地址。
  ///         - -1：方法调用失败
  /// @note 在一些场景下，获取 C++ 层 `IRTCEngine`，并通过其完成操作，相较于通过 Java 封装层完成有显著更高的执行效率。典型的场景有：视频/音频帧自定义处理，音视频通话加密等。
  ///

  FutureOr<long> getNativeHandle() async {
    return await nativeCall('getNativeHandle', []);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 开启录制语音通话，生成本地文件。 <br>
  ///        在进房前后开启录制，如果未打开麦克风采集，录制任务正常进行，只是不会将数据写入生成的本地文件；只有调用 startAudioCapture{@link #RTCEngine#startAudioCapture} 接口打开麦克风采集后，才会将录制数据写入本地文件。
  /// @param config 参看 AudioRecordingConfig{@link #AudioRecordingConfig}
  /// @return
  ///        - 0: 正常
  ///        - -2: 参数设置异常
  ///        - -3: 当前版本 SDK 不支持该特性，请联系技术支持人员
  /// @note
  ///        - 录制包含各种音频效果。但不包含混音的背景音乐。
  ///        - 调用 stopAudioRecording{@link #RTCEngine#stopAudioRecording} 关闭录制。
  ///        - 加入房间前后均可调用。在进房前调用该方法，退房之后，录制任务不会自动停止，需调用 stopAudioRecording{@link #RTCEngine#stopAudioRecording} 关闭录制。在进房后调用该方法，退房之后，录制任务会自动被停止。如果加入了多个房间，录制的文件中会包含各个房间的音频。
  ///        - 调用该方法后，你会收到 onAudioRecordingStateUpdate{@link #IRTCEngineEventHandler#onAudioRecordingStateUpdate} 回调。
  ///

  FutureOr<int> startAudioRecording(AudioRecordingConfig config) async {
    return await nativeCall('startAudioRecording', [config]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 停止音频文件录制
  /// @return
  ///         - 0: 正常
  ///         - -3: 当前版本 SDK 不支持该特性，请联系技术支持人员
  /// @note 调用 startAudioRecording{@link #RTCEngine#startAudioRecording} 开启本地录制后，你必须调用该方法停止录制。
  ///

  FutureOr<int> stopAudioRecording() async {
    return await nativeCall('stopAudioRecording', []);
  }

  /// @valid since 3.53
  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 创建音效播放器实例。
  /// @return 音效播放器。详见 IAudioEffectPlayer{@link #IAudioEffectPlayer}。
  ///

  FutureOr<IAudioEffectPlayer> getAudioEffectPlayer() async {
    final result = await nativeCall('getAudioEffectPlayer', []);
    return packObject(
        result,
        () => IAudioEffectPlayer(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @valid since 3.53
  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 创建音乐播放器实例。
  /// @param playerId 音乐播放器实例 id。取值范围为 `[0, 3]`。最多同时存在 4 个实例，超出取值范围时返回 nullptr。
  /// @return 音乐播放器实例，详见 IMediaPlayer{@link #IMediaPlayer}
  ///

  FutureOr<IMediaPlayer> getMediaPlayer(int playerId) async {
    final result = await nativeCall('getMediaPlayer', [playerId]);
    return packObject(result,
        () => IMediaPlayer(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author liyi.000
  /// @brief 在屏幕共享时，设置屏幕音频的采集方式（内部采集/自定义采集）
  /// @param sourceType 屏幕音频输入源类型, 参看 AudioSourceType{@link #AudioSourceType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///      - 默认采集方式是 RTC SDK 内部采集。
  ///      - 如果设定为内部采集，你必须再调用 startScreenCapture 开始采集。开启后，可以再次本接口切换为外部采集，此时内部采集将自动停止。
  ///      - 如果设定为自定义采集，你必须再调用 pushScreenAudioFrame{@link #RTCEngine#pushScreenAudioFrame} 将自定义采集到的屏幕音频帧推送到 RTC SDK。
  /// @order 5
  ///

  FutureOr<int> setScreenAudioSourceType(AudioSourceType sourceType) async {
    return await nativeCall('setScreenAudioSourceType', [sourceType.$value]);
  }

  /// @detail api
  /// @author liyi.000
  /// @brief 使用自定义采集方式，采集屏幕共享时的屏幕音频时，将音频帧推送至 RTC SDK 处进行编码等处理。
  /// @param audioFrame 音频数据帧，参见 AudioFrame{@link #AudioFrame} <br>
  ///                   - 音频采样格式为 S16。音频缓冲区内的数据格式必须为 PCM 数据，其容量大小应该为 samples × frame.channel × 2。
  ///                   - 必须指定具体的采样率和声道数，不支持设置为自动。
  /// @return 方法调用结果 <br>
  ///        - 0: 设置成功。
  ///        - < 0: 设置失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 调用此接口推送屏幕共享时的自定义采集的音频数据前，必须调用 setScreenAudioSourceType{@link #RTCEngine#setScreenAudioSourceType} 开启屏幕音频自定义采集。
  ///        - 你应每隔 10 毫秒，调用一次此方法推送一次自定义采集的音频帧。一次推送的音频帧中应包含 frame.sample_rate / 100 个音频采样点。比如，假如采样率为 48000Hz，则每次应该推送 480 个采样点。
  ///        - 调用此接口将自定义采集的音频帧推送到 RTC SDK 后，你必须调用 publishScreenAudio 将采集到的屏幕音频推送到远端。在调用 publishScreenAudio 前，推送到 RTC SDK 的音频帧信息会丢失。
  /// @order 8
  ///

  FutureOr<int> pushScreenAudioFrame(AudioFrame audioFrame) async {
    return await nativeCall('pushScreenAudioFrame', [audioFrame]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 注册自定义编码帧推送事件回调
  /// @param handler 自定义编码帧回调类，参看 IExternalVideoEncoderEventHandler{@link #IExternalVideoEncoderEventHandler}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 该方法需在进房前调用。
  ///       - 引擎销毁前需取消注册，调用该方法将参数设置为 "null" 即可。
  ///

  FutureOr<int> setExternalVideoEncoderEventHandler(
      IExternalVideoEncoderEventHandler handler) async {
    return await nativeCall('setExternalVideoEncoderEventHandler', [handler]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @brief 开启/关闭耳返功能，并控制是否对耳返音频应用本地音频处理。
  /// @param mode 是否开启耳返功能，参看 EarMonitorMode{@link #EarMonitorMode}。默认关闭。
  /// @param filter 是否对耳返音频应用本地音频处理，参看 EarMonitorAudioFilter{@link #EarMonitorAudioFilter}。默认不经过音频处理。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 耳返功能仅适用于由 RTC SDK 内部采集的音频。
  ///        - 使用耳返功能必须佩戴耳机。为保证低延时耳返最佳体验，建议佩戴有线耳机。蓝牙耳机不支持硬件耳返。
  ///        - RTC SDK 支持硬件耳返和软件耳返。一般来说，硬件耳返延时低且音质好。如果 App 在手机厂商的硬件耳返白名单内，且运行环境存在支持硬件耳返的 SDK，RTC SDK 默认启用硬件耳返。使用华为手机硬件耳返功能时，请添加[华为硬件耳返的依赖配置](https://www.volcengine.com/docs/6348/1155036)。
  ///

  FutureOr<int> setEarMonitorMode(
      EarMonitorMode mode, EarMonitorAudioFilter filter) async {
    return await nativeCall('setEarMonitorMode', [mode.$value, filter.$value]);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 设置耳返音量。
  /// @param volume 耳返音量，调节范围：[0,100]，单位：\%
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 设置耳返音量前，你必须先调用 setEarMonitorMode{@link #RTCEngine#setEarMonitorMode} 打开耳返功能。
  ///

  FutureOr<int> setEarMonitorVolume(int volume) async {
    return await nativeCall('setEarMonitorVolume', [volume]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 启用音频信息提示。开启提示后，你可以收到 onLocalAudioPropertiesReport{@link #IRTCEngineEventHandler#onLocalAudioPropertiesReport}，onRemoteAudioPropertiesReport{@link #IRTCEngineEventHandler#onRemoteAudioPropertiesReport} 和 onActiveSpeaker{@link #IRTCEngineEventHandler#onActiveSpeaker}。
  /// @param config 详见 AudioPropertiesConfig{@link #AudioPropertiesConfig}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> enableAudioPropertiesReport(
      AudioPropertiesConfig config) async {
    return await nativeCall('enableAudioPropertiesReport', [config]);
  }

  /// @hidden 3.60 for internal use only
  /// @detail api
  /// @region 音频管理
  /// @author gengjunjie
  /// @brief 启用人声检测。开启提示后，你可以收到 onAudioVADStateUpdate{@link #IRTCEngineEventHandler#onAudioVADStateUpdate}。
  /// @param interval 回调间隔，单位毫秒.
  ///       + `<= 0`: 关闭人声识别能力回调。
  ///       + `[100, 3000]`: 开启人声识别能力回调，并将信息提示间隔设置为此值。
  ///       + 不合法的 interval 值：小于 100 设置为 100，超出 3000 设置为 3000。
  /// @return
  ///        + 0: 调用成功。
  ///        + < 0: 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> enableAudioVADReport(int interval) async {
    return await nativeCall('enableAudioVADReport', [interval]);
  }

  /// @hidden 3.60 for internal use only
  /// @detail api
  /// @region 音频管理
  /// @author shiyayun
  /// @brief 启用音频信息提示。开启提示后，你可以收到 onAudioAEDStateUpdate{@link #IRTCEngineEventHandler#onAudioAEDStateUpdate。
  /// @param interval 回调间隔，单位毫秒。<br>
  ///       + `<= 0`: 关闭回调。
  ///       + `[100, 3000]`: 开启回调，并将信息提示间隔设置为此值。推荐设置为 2000。
  ///       + 不合法的 interval 值：小于 100 设置为 100，超出 3000 设置为 3000。
  /// @return
  ///        + 0: 调用成功。
  ///        + < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> enableAudioAEDReport(int interval) async {
    return await nativeCall('enableAudioAEDReport', [interval]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 发送音频流同步信息。将消息通过音频流发送到远端，并实现与音频流同步，该接口调用成功后，远端用户会收到 onStreamSyncInfoReceived{@link #IRTCEngineEventHandler#onStreamSyncInfoReceived} 回调。
  /// @param data 消息内容。
  /// @param config 音频流同步信息的相关配置。详见 StreamSyncInfoConfig{@link #StreamSyncInfoConfig} 。
  /// @return
  ///        - >=0: 消息发送成功。返回成功发送的次数。
  ///        - -1: 消息发送失败。消息长度大于 255 字节。
  ///        - -2: 消息发送失败。传入的消息内容为空。
  ///        - -3: 消息发送失败。通过屏幕流进行消息同步时，此屏幕流还未发布。
  ///        - -4: 消息发送失败。通过用麦克风或自定义设备采集到的音频流进行消息同步时，此音频流还未发布，详见错误码 ErrorCode{@link #ErrorCode}。
  /// @note
  /// - 调用本接口的频率建议不超过 50 次每秒。
  /// - 在 `CHANNEL_PROFILE_INTERACTIVE_PODCAST` 房间模式下，此消息一定会送达。在其他房间模式下，如果本地用户未说话，此消息不一定会送达。
  ///

  FutureOr<int> sendStreamSyncInfo(
      ArrayBuffer data, StreamSyncInfoConfig config) async {
    return await nativeCall('sendStreamSyncInfo', [data, config]);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检测当前使用的摄像头（前置/后置），是否支持闪光灯。
  /// @return
  ///        - true: 支持
  ///        - false: 不支持
  /// @note 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检测闪光能力。
  ///

  FutureOr<boolean> isCameraTorchSupported() async {
    return await nativeCall('isCameraTorchSupported', []);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检测当前使用的摄像头（前置/后置），是否支持变焦（数码/光学变焦）。
  /// @return
  ///        - true: 支持
  ///        - false: 不支持
  /// @note 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检测摄像头变焦能力。
  ///

  FutureOr<boolean> isCameraZoomSupported() async {
    return await nativeCall('isCameraZoomSupported', []);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头（前置/后置）的光学变焦倍数
  /// @param zoom 变焦倍数。取值范围是 [1, <最大变焦倍数>]。 <br>
  ///             最大变焦倍数可以通过调用 getCameraZoomMaxRatio{@link #RTCEngine#getCameraZoomMaxRatio} 获取。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能设置摄像头变焦倍数。
  ///        - 设置结果在调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 关闭内部采集后失效。
  ///        - 你可以调用 setVideoDigitalZoomConfig{@link #RTCEngine#setVideoDigitalZoomConfig} 设置数码变焦参数， 调用 setVideoDigitalZoomControl{@link #RTCEngine#setVideoDigitalZoomControl} 进行数码变焦。
  ///

  FutureOr<int> setCameraZoomRatio(float zoom) async {
    return await nativeCall('setCameraZoomRatio', [zoom]);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 获取当前使用的摄像头（前置/后置）的最大变焦倍数
  /// @return 最大变焦倍数
  /// @note 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检测摄像头最大变焦倍数。
  ///

  FutureOr<float> getCameraZoomMaxRatio() async {
    return await nativeCall('getCameraZoomMaxRatio', []);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 打开/关闭当前使用的摄像头（前置/后置）的闪光灯
  /// @param torchState 闪光灯状态。参考 TorchState{@link #TorchState}
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能设置闪光灯。
  ///        - 设置结果在调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 关闭内部采集后失效。
  ///

  FutureOr<int> setCameraTorch(TorchState torchState) async {
    return await nativeCall('setCameraTorch', [torchState.$value]);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检查当前使用的摄像头是否支持手动对焦。
  /// @return
  ///        - true: 支持。
  ///        - false: 不支持。
  /// @note 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集，才能检查摄像头是否支持手动对焦。
  ///

  FutureOr<boolean> isCameraFocusPositionSupported() async {
    return await nativeCall('isCameraFocusPositionSupported', []);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头的对焦点。
  /// @param x 对焦点水平方向归一化坐标。以本地预览画布的左上为原点，取值范围为 [0, 1]，0 表示最左边，1 表示最右边。
  /// @param y 对焦点垂直方向归一化坐标。以本地预览画布的左上为原点，取值范围为 [0, 1]，0 表示最上边，1 表示最下边。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集，并且使用 SDK 内部渲染时，才能设置对焦点。
  ///        - 移动设备时，自动取消对焦点设置。
  ///        - 调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 关闭内部采集后，设置的对焦点失效。
  ///

  FutureOr<int> setCameraFocusPosition(float x, float y) async {
    return await nativeCall('setCameraFocusPosition', [x, y]);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检查当前使用的摄像头是否支持手动设置曝光点。
  /// @return
  ///        - true: 支持。
  ///        - false: 不支持。
  /// @note 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检查曝光点设置能力。
  ///

  FutureOr<boolean> isCameraExposurePositionSupported() async {
    return await nativeCall('isCameraExposurePositionSupported', []);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头的曝光点
  /// @param x 曝光点水平方向归一化坐标。以本地预览画布的左上为原点，取值范围为 [0, 1]，0 表示最左边，1 表示最右边。
  /// @param y 曝光点垂直方向归一化坐标。以本地预览画布的左上为原点，取值范围为 [0, 1]，0 表示最上边，1 表示最下边。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集，并且使用 SDK 内部渲染时，才能设置曝光点。
  ///        - 移动设备时，自动取消曝光点设置。
  ///        - 调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 关闭内部采集后，设置的曝光点失效。
  ///

  FutureOr<int> setCameraExposurePosition(float x, float y) async {
    return await nativeCall('setCameraExposurePosition', [x, y]);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头的曝光补偿。
  /// @param val 曝光补偿值，取值范围 [-1, 1]，0 为系统默认值(没有曝光补偿)。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能设置曝光补偿。
  ///        - 调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 关闭内部采集后，设置的曝光补偿失效。
  ///

  FutureOr<int> setCameraExposureCompensation(float val) async {
    return await nativeCall('setCameraExposureCompensation', [val]);
  }

  /// @valid since 353
  /// @detail api
  /// @author yinkaisheng
  /// @brief 启用或禁用内部采集时人脸自动曝光模式。此模式会改善强逆光下，脸部过暗的问题；但也会导致 ROI 以外区域过亮/过暗的问题。
  /// @param enable 是否启用。iOS默认开启，Android默认关闭。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。
  /// @note 你必须在调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 开启内部采集前，调用此接口方可生效。
  ///

  FutureOr<int> enableCameraAutoExposureFaceMode(boolean enable) async {
    return await nativeCall('enableCameraAutoExposureFaceMode', [enable]);
  }

  /// @hidden(macOS)
  /// @valid since 353
  /// @detail api
  /// @author yinkaisheng
  /// @brief 设置内部采集适用动态帧率时，帧率的最小值。
  /// @param framerate 最小值。单位为 fps。默认值是 7。 <br>
  ///                  动态帧率的最大帧率是通过 setVideoCaptureConfig{@link #RTCEngine#setVideoCaptureConfig} 设置的帧率值。当传入参数大于最大帧率时，使用固定帧率模式，帧率为最大帧率；当传入参数小于最大帧率时，使用动态帧率。
  /// @return
  ///        - 0: 成功.
  ///        - !0: 失败.
  /// @note
  ///        - 你必须在调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 开启内部采集前，调用此接口方可生效。
  ///        - 如果由于性能降级、静态适配等原因导致采集最大帧率变化时，已设置的最小帧率值会与新的采集最大帧率值重新比较。比较结果变化可能导致固定/动态帧率模式切换。
  ///        - 对 Android，默认开启动态帧率模式
  ///        - 对 iOS，默认使用固定帧率模式
  ///

  FutureOr<int> setCameraAdaptiveMinimumFrameRate(int framerate) async {
    return await nativeCall('setCameraAdaptiveMinimumFrameRate', [framerate]);
  }

  /// @valid since 3.58
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhuhongshuyu
  /// @brief 开启自定义采集视频帧的 Alpha 通道编码功能。 <br>
  ///        适用于需要分离推流端视频主体与背景，且在拉流端可自定义渲染背景的场景。
  /// @param alphaLayout 分离后的 Alpha 通道相对于 RGB 通道信息的排列位置。当前仅支持 AlphaLayout.TOP，即置于 RGB 通道信息上方。
  /// @return 方法调用结果： <br>
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该接口仅作用于自定义采集的、并且使用 RGBA 色彩模型的视频帧，包括 VideoPixelFormat.TEXTURE_2D、VideoPixelFormat.TEXTURE_OES、VideoPixelFormat.RGBA。
  ///        - 该接口须在发布视频流之前调用。
  ///        - 调用本接口开启 Alpha 通道编码后，你需调用 pushExternalVideoFrame{@link #RTCEngine#pushExternalVideoFrame} 把自定义采集的视频帧推送至 RTC SDK。若推送了不支持的视频帧格式，则调用 pushExternalVideoFrame{@link #RTCEngine#pushExternalVideoFrame} 时会返回错误码 ReturnStatus.RETURN_STATUS_PARAMETER_ERR。
  ///

  FutureOr<int> enableAlphaChannelVideoEncode(AlphaLayout alphaLayout) async {
    return await nativeCall(
        'enableAlphaChannelVideoEncode', [alphaLayout.$value]);
  }

  /// @valid since 3.58
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhuhongshuyu
  /// @brief 关闭外部采集视频帧的 Alpha 通道编码功能。
  /// @return 方法调用结果： <br>
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 该接口须在停止发布视频流之后调用。
  ///

  FutureOr<int> disableAlphaChannelVideoEncode() async {
    return await nativeCall('disableAlphaChannelVideoEncode', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 通过视频帧发送 SEI 数据。 <br>
  ///        在视频通话场景下，SEI 数据会随视频帧发送；在语音通话场景下，SDK 会自动生成一路 16px × 16px 的黑帧视频流用来发送 SEI 数据。
  /// @param message SEI 消息，建议每帧 SEI 数据总长度不超过 4 KB。超过长度限制的消息会被丢弃。
  /// @param repeatCount 消息发送重复次数。取值范围是 [0, max{29, \%{视频帧率}-1}]。推荐范围 [2,4]。 <br>
  ///                    调用此接口后，这些 SEI 数据会添加到从当前视频帧开始的连续 `\%{repeatCount}+1` 个视频帧中。
  /// @param mode SEI 发送模式，参看 SEICountPerFrame{@link #SEICountPerFrame}。
  /// @return
  ///        - >= 0: 将被添加到视频帧中的 SEI 的数量。
  ///        - < 0: 发送失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 每秒发送的 SEI 消息数量建议不超过当前的视频帧率。在语音通话场景下，黑帧帧率为 15 fps。
  ///        - 语音通话场景中，仅支持在内部采集模式下调用该接口发送 SEI 数据。
  ///        - 视频通话场景中，使用自定义采集并通过 pushExternalVideoFrame{@link #RTCEngine#pushExternalVideoFrame} 推送至 SDK 的视频帧，若本身未携带 SEI 数据，也可通过本接口发送 SEI 数据；若原视频帧中已添加了 SEI 数据，则调用此方法不生效。
  ///        - 视频帧仅携带前后 2s 内收到的 SEI 数据；语音通话场景下，若调用此接口后 1min 内未有 SEI 数据发送，则 SDK 会自动取消发布视频黑帧。
  ///        - 消息发送成功后，远端会收到 onSEIMessageReceived{@link #IRTCEngineEventHandler#onSEIMessageReceived} 回调。
  ///        - 语音通话切换至视频通话时，会停止使用黑帧发送 SEI 数据，自动转为用采集到的正常视频帧发送 SEI 数据。
  ///

  FutureOr<int> sendSEIMessage(
      ArrayBuffer message, int repeatCount, SEICountPerFrame mode) async {
    return await nativeCall(
        'sendSEIMessage', [message, repeatCount, mode.$value]);
  }

  /// @hidden for internal use only
  /// @valid since 3.56
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief WTN 流视频帧发送 SEI 数据。
  /// @param channelId SEI 消息的传输通道，取值范围 [0 - 255]。通过此参数，你可以为不同接受方设置不同的 ChannelID，这样不同接收方可以根据回调中的 ChannelID 选择应关注的 SEI 信息。
  /// @param message SEI 消息。
  /// @param repeatCount 消息发送重复次数。取值范围是 [0, max{29, \%{video帧率}-1}]。推荐范围 [2,4]。 <br>
  ///                    调用此接口后，SEI 数据会添加到从当前视频帧开始的连续 `repeat_count+1` 个视频帧中。
  /// @param mode SEI 发送模式，参看 SEICountPerFrame{@link #SEICountPerFrame}。
  /// @return
  ///        - < 0：说明调用失败
  ///        - = 0：说明当前发送队列已满，无法发送
  ///        - > 0: 说明调用成功，该数值为已经发送 SEI 的数量
  /// @note
  ///        - 每秒发送的 SEI 消息数量建议不超过当前的视频帧率
  ///        - 视频通话场景中，使用自定义采集并通过 pushExternalVideoFrame{@link #RTCEngine#pushExternalVideoFrame} 推送至 SDK 的视频帧，若本身未携带 SEI 数据，也可通过本接口发送 SEI 数据；若原视频帧中已添加了 SEI 数据，则调用此方法不生效。
  ///        - 视频帧仅携带前后 2s 内收到的 SEI 数据
  ///        - 消息发送成功后，远端会收到 onWTNSEIMessageReceived{@link #IWTNStreamEventHandler#onWTNSEIMessageReceived} 回调。
  ///        - 调用失败时，本地及远端都不会收到回调。
  ///

  FutureOr<int> sendPublicStreamSEIMessage(int channelId, ArrayBuffer message,
      int repeatCount, SEICountPerFrame mode) async {
    return await nativeCall('sendPublicStreamSEIMessage',
        [channelId, message, repeatCount, mode.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 该方法将通话过程中的音视频数据录制到本地的文件中。
  /// @param config 本地录制参数配置，参看 RecordingConfig{@link #RecordingConfig}
  /// @param recordingType 本地录制的媒体类型，参看 RecordingType{@link #RecordingType}<br>
  ///                      注意：屏幕流仅支持录制视频（RECORD_VIDEO_ONLY）；主流支持录制所有类型。
  /// @return 0: 正常 <br>
  ///        -1: 参数设置异常 <br>
  ///        -2: 当前版本 SDK 不支持该特性，请联系技术支持人员
  /// @note
  ///        - 该方法需在进房后调用。
  ///        - 调用该方法后，你会收到 onRecordingStateUpdate{@link #IRTCEngineEventHandler#onRecordingStateUpdate} 回调。
  ///        - 如果录制正常，系统每秒钟会通过 onRecordingProgressUpdate{@link #IRTCEngineEventHandler#onRecordingProgressUpdate} 回调通知录制进度。
  ///

  FutureOr<int> startFileRecording(
      RecordingConfig config, RecordingType recordingType) async {
    return await nativeCall(
        'startFileRecording', [config, recordingType.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 停止本地录制
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用 startFileRecording{@link #RTCEngine#startFileRecording} 开启本地录制后，你必须调用该方法停止录制。
  ///        - 调用该方法后，你会收到 onRecordingStateUpdate{@link #IRTCEngineEventHandler#onRecordingStateUpdate} 回调提示录制结果。
  ///

  FutureOr<int> stopFileRecording() async {
    return await nativeCall('stopFileRecording', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 推送自定义编码后的视频流
  /// @param videoIndex 对应的编码流下标，从 0 开始，如果调用 setVideoEncoderConfig{@link #RTCEngine#setVideoEncoderConfig} 设置了多路流，此处数量须与之保持一致
  /// @param encodedVideoFrame 编码流视频帧信息，参看 RTCEncodedVideoFrame{@link #RTCEncodedVideoFrame}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 目前仅支持推送 H264 和 ByteVC1 格式的视频帧，且视频流协议格式须为 Annex B 格式。
  ///        - 该函数运行在用户调用线程内
  ///        - 推送自定义编码视频帧前，必须调用 setVideoSourceType{@link #RTCEngine#setVideoSourceType} 将视频输入源切换至自定义编码视频源。
  ///

  FutureOr<int> pushExternalEncodedVideoFrame(
      int videoIndex, RTCEncodedVideoFrame encodedVideoFrame) async {
    return await nativeCall(
        'pushExternalEncodedVideoFrame', [videoIndex, encodedVideoFrame]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangqianqian.1104
  /// @brief 使用 RTC SDK 内部采集模块开始采集屏幕音频流和（或）视频流。
  /// @param type 媒体类型，参看 ScreenMediaType{@link #ScreenMediaType}。
  /// @param mediaProjectionResultData 向 Android 设备申请屏幕共享权限后，拿到的 Intent 数据，参看 [getMediaProjection](https://developer.android.com/reference/android/media/projection/MediaProjectionManager#getMediaProjection(int,\%20android.content.Intent))。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用本接口时，采集模式应为内部模式。在外部采集模式下调用无效，并将触发 onVideoDeviceWarning{@link #IRTCEngineEventHandler#onVideoDeviceWarning} 或 onAudioDeviceWarning{@link #IRTCEngineEventHandler#onAudioDeviceWarning} 回调。
  ///        - 采集后，你还需要调用 publishStreamAudio{@link #RTCRoom#publishStreamAudio} 和 publishStreamVideo{@link #RTCRoom#publishStreamVideo} 发布采集到的屏幕音视频。
  ///        - 开启屏幕音频/视频采集成功后，本地用户会收到 onVideoDeviceStateChanged{@link #IRTCEngineEventHandler#onVideoDeviceStateChanged} 和 onAudioDeviceStateChanged{@link #IRTCEngineEventHandler#onAudioDeviceStateChanged} 的回调。
  ///        - 要关闭屏幕音视频内部采集，调用 stopScreenCapture{@link #RTCEngine#stopScreenCapture}。
  ///

  FutureOr<int> startScreenCapture(
      ScreenMediaType type, Intent mediaProjectionResultData) async {
    return await nativeCall(
        'startScreenCapture', [type.$value, mediaProjectionResultData]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangqianqian.1104
  /// @brief 使用 RTC SDK 内部屏幕采集后，更新采集的媒体类型。
  /// @param type 媒体类型，指定屏幕采集媒体类型，参看 ScreenMediaType{@link #ScreenMediaType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 在 startScreenCapture{@link #RTCEngine#startScreenCapture} 后调用该方法。
  ///

  FutureOr<int> updateScreenCapture(ScreenMediaType type) async {
    return await nativeCall('updateScreenCapture', [type.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangqianqian.1104
  /// @brief 在屏幕共享时，停止使用 RTC SDK 内部采集方式采集屏幕音视频。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用本接口时，采集模式应为内部模式。在外部采集模式下调用无效，并将触发 onVideoDeviceWarning{@link #IRTCEngineEventHandler#onVideoDeviceWarning} 或 onAudioDeviceWarning{@link #IRTCEngineEventHandler#onAudioDeviceWarning} 回调。
  ///      - 要开始屏幕音视频内部采集，调用 startScreenCapture{@link #RTCEngine#startScreenCapture}。
  ///

  FutureOr<int> stopScreenCapture() async {
    return await nativeCall('stopScreenCapture', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhushufan.ref
  /// @brief 在指定视频流上添加水印。
  /// @param imagePath 水印图片路径，支持本地文件绝对路径、Asset 资源路径（/assets/xx.png）、URI 地址（content://），长度限制为 512 字节。 <br>
  ///        水印图片为 PNG 或 JPG 格式。
  /// @param watermarkConfig 水印参数，参看 RTCWatermarkConfig{@link #RTCWatermarkConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用 clearVideoWatermark{@link #RTCEngine#clearVideoWatermark} 移除指定视频流的水印。
  ///        - 同一路流只能设置一个水印，新设置的水印会代替上一次的设置。你可以多次调用本方法来设置不同流的水印。
  ///        - 进入房间前后均可调用此方法。
  ///        - 若开启本地预览镜像，或开启本地预览和编码传输镜像，则远端水印均不镜像；在开启本地预览水印时，本端水印会镜像。
  ///        - 开启大小流后，水印对大小流均生效，且针对小流进行等比例缩小。
  ///

  FutureOr<int> setVideoWatermark(
      String imagePath, RTCWatermarkConfig watermarkConfig) async {
    return await nativeCall('setVideoWatermark', [imagePath, watermarkConfig]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhushufan.ref
  /// @brief 移除指定视频流的水印。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> clearVideoWatermark() async {
    return await nativeCall('clearVideoWatermark', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 摄像头处于关闭状态时，使用静态图片填充本地推送的视频流。 <br>
  ///        调用 `stopVideoCapture` 接口时，会开始推静态图片。若要停止发送图片，可传入空字符串或启用内部摄像头采集。 <br>
  ///        可重复调用该接口来更新图片。
  /// @param filePath 设置静态图片的路径。 <br>
  ///        支持本地文件绝对路径和 Asset 资源路径(/assets/xx.png)，不支持网络链接，长度限制为 512 字节。 <br>
  ///        静态图片支持类型为 JPEG/JPG、PNG、BMP。 <br>
  ///        若图片宽高比与设置的编码宽高比不一致，图片会被等比缩放，黑边填充空白区域。推流帧率与码率与设置的编码参数一致。
  /// @return
  ///        - 0: 成功。
  ///        - -2: 失败。确保传入的 filePath 为有效路径。
  ///        - -12: 本方法不支持在 Audio SDK 中使用。
  /// @note
  ///        - 该接口只适用于 SDK 内部摄像头采集，不适用于自定义视频采集。
  ///        - 本地预览无法看到静态图片。
  ///        - 进入房间前后均可调用此方法。在多房间场景中，静态图片仅在发布的房间中生效。
  ///        - 针对该静态图片，滤镜和镜像效果不生效，水印效果生效。
  ///        - 只有主流能设置静态图片，屏幕流不支持设置。
  ///        - 开启大小流后，静态图片对大小流均生效，且针对小流进行等比例缩小。
  ///

  FutureOr<int> setDummyCaptureImagePath(String filePath) async {
    return await nativeCall('setDummyCaptureImagePath', [filePath]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfujun.911
  /// @brief 截取本地视频画面
  /// @param callback 本地截图的回调。参看 ISnapshotResultCallback{@link #ISnapshotResultCallback}。
  /// @return 本地截图任务的编号，从 `1` 开始递增。
  /// @note
  ///        - 对截取的画面，包含本地视频处理的全部效果，包含旋转，镜像，美颜等。
  ///        - 不管采用 SDK 内部采集，还是自定义采集，都可以进行截图。
  ///

  FutureOr<long> takeLocalSnapshot(ISnapshotResultCallback callback) async {
    return await nativeCall('takeLocalSnapshot', [callback]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfujun.911
  /// @brief 截取本地视频画面
  /// @param streamId 远端用户的 streamId。
  /// @param callback 本地截图的回调。参看 ISnapshotResultCallback{@link #ISnapshotResultCallback}。
  /// @return 本地截图任务的编号，从 `1` 开始递增。
  /// @note
  ///        - 对截取的画面，包含本地视频处理的全部效果，包含旋转，镜像，美颜等。
  ///        - 不管采用 SDK 内部采集，还是自定义采集，都可以进行截图。
  ///

  FutureOr<long> takeRemoteSnapshot(
      String streamId, ISnapshotResultCallback callback) async {
    return await nativeCall('takeRemoteSnapshot', [streamId, callback]);
  }

  /// @detail api
  /// @author wangfujun.911
  /// @brief 截取远端的视频流画面，生成 JPG 文件，并保存到本地指定路径。 <br>
  ///        调用该方法后，SDK 会触发回调 onRemoteSnapshotTakenToFile{@link #IRTCEngineEventHandler#onRemoteSnapshotTakenToFile} 报告截图是否成功，以及截取的图片信息。
  /// @param streamId 待截取的远端视频流 ID。
  /// @param filePath 截图的本地保存路径（绝对路径），需精确到文件名及格式，文件扩展名必须为 `.jpg`，并请确保路径存在且可写。示例：`/sdcard/Pictures/snapshot.jpg`。
  /// @return 远端截图任务的编号，从 `1` 开始递增。此编号可用于追踪任务状态或进行其他管理操作。
  ///

  FutureOr<long> takeRemoteSnapshotToFile(
      String streamId, String filePath) async {
    return await nativeCall('takeRemoteSnapshotToFile', [streamId, filePath]);
  }

  /// @detail api
  /// @author wangfujun.911
  /// @brief 截取本地的视频流画面，生成 JPG 文件，并保存到本地指定路径。 <br>
  ///        调用该方法后，SDK 会触发回调 onLocalSnapshotTakenToFile{@link #IRTCEngineEventHandler#onLocalSnapshotTakenToFile} 报告截图是否成功，以及截取的图片信息。
  /// @param filePath 截图的本地保存路径（绝对路径），需精确到文件名及格式，文件扩展名必须为 `.jpg`，并请确保路径存在且可写。示例：`/sdcard/Pictures/snapshot.jpg`。
  /// @return 远端截图任务的编号，从 `1` 开始递增。此编号可用于追踪任务状态或进行其他管理操作。
  ///

  FutureOr<long> takeLocalSnapshotToFile(String filePath) async {
    return await nativeCall('takeLocalSnapshotToFile', [filePath]);
  }

  /// @detail api
  /// @author qipengxiang
  /// @brief 开启音视频回路测试。 <br>
  ///        在进房前，用户可调用该接口对音视频通话全链路进行检测，包括对音视频设备以及用户上下行网络的检测，从而帮助用户判断是否可以正常发布和接收音视频流。 <br>
  ///        开始检测后，SDK 会录制你声音或视频。如果你在设置的延时范围内收到了回放，则视为音视频回路测试正常。
  /// @param config 回路测试参数设置，参看 EchoTestConfig{@link #EchoTestConfig}。
  /// @param delayTime 音视频延迟播放的时间间隔，用于指定在开始检测多长时间后期望收到回放。取值范围为 [2,10]，单位为秒，默认为 2 秒。
  /// @return 方法调用结果： <br>
  ///        - 0：成功
  ///        - -2：失败，参数异常
  ///        - -4：失败，用户已进房
  ///        - -6：失败，当前用户已经在检测中
  ///        - -7：失败，音视频均不检查
  ///        - -8：失败，已经存在相同 roomId 的房间
  /// @note
  ///        - 调用该方法开始音视频回路检测后，你可以调用 stopEchoTest{@link #RTCEngine#stopEchoTest} 立即结束测试，也可等待测试 60s 后自动结束，以更换设备进行下一次测试，或进房。
  ///        - 在该方法之前调用的所有跟设备控制、流控制相关的方法均在开始检测时失效，在结束检测后恢复生效。
  ///        - 在调用 startEchoTest{@link #RTCEngine#startEchoTest} 和 stopEchoTest{@link #RTCEngine#stopEchoTest} 之间调用的所有跟设备采集、流控制、进房相关的方法均不生效，并会收到 onWarning{@link #IRTCEngineEventHandler#onWarning} 回调，提示警告码为 `WARNING_CODE_IN_ECHO_TEST_MODE`。
  ///        - 音视频回路检测的结果会通过 onEchoTestResult{@link #IRTCEngineEventHandler#onEchoTestResult} 回调通知。
  ///

  FutureOr<int> startEchoTest(EchoTestConfig config, int delayTime) async {
    return await nativeCall('startEchoTest', [config, delayTime]);
  }

  /// @detail api
  /// @author qipengxiang
  /// @brief 停止音视频回路测试。 <br>
  ///        调用 startEchoTest{@link #RTCEngine#startEchoTest} 开启音视频回路检测后，你必须调用该方法停止检测。
  /// @return 方法调用结果： <br>
  ///        - 0：成功。
  ///        - -3：失败，未开启回路检测。
  /// @note 音视频回路检测结束后，所有对系统设备及音视频流的控制均会恢复到开始检测前的状态。
  ///

  FutureOr<int> stopEchoTest() async {
    return await nativeCall('stopEchoTest', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @deprecated since 3.57, use setRemoteVideoSink{@link #RTCEngine#setRemoteVideoSink} instead.
  /// @region 自定义视频采集渲染
  /// @author sunhang.io
  /// @brief 将远端视频流与自定义渲染器绑定。
  /// @param streamId 流 ID，用于指定需要渲染的视频流。
  /// @param videoSink 自定义视频渲染器，参看 IVideoSink{@link #IVideoSink}
  /// @param requiredFormat videoSink 适用的视频帧编码格式，参看 PixelFormat{@link #PixelFormat}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - RTC SDK 默认使用 RTC SDK 自带的渲染器（内部渲染器）进行视频渲染。
  ///        - 该方法进房前后均可以调用。若想在进房前调用，你需要在加入房间前获取远端流信息；若无法预先获取远端流信息，你可以在加入房间并通过 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo} 回调获取到远端流信息之后，再调用该方法。
  ///        - 如果需要解除绑定，你必须将 videoSink 设置为 null。退房时将清除绑定状态。
  ///        - 本方法获取的是后处理后的视频帧。
  ///

  FutureOr<int> setRemoteVideoSink(
      String streamId, IVideoSink videoSink, int requiredFormat) async {
    return await nativeCall(
        'setRemoteVideoSink', [streamId, videoSink, requiredFormat]);
  }

  /// @deprecated since 3.56 and will be deleted in 3.62. Use [updateRemoteStreamVideoCanvas](70080#updateremotestreamvideocanvas-2) instead.
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfujun.911
  /// @brief 修改远端视频帧的渲染设置，包括渲染模式和背景颜色。
  /// @param streamId 流 ID，用于指定需要修改渲染设置的视频流。
  /// @param renderMode 渲染模式，参看 VideoCanvas{@link #VideoCanvas}.renderMode
  /// @param backgroundColor 背景颜色，参看 VideoCanvas{@link #VideoCanvas}.backgroundColor
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 你可以在远端视频渲染过程中，调用此接口。调用结果会实时生效。
  ///

  FutureOr<int> updateRemoteStreamVideoCanvas(
      String streamId, int renderMode, int backgroundColor) async {
    return await nativeCall('updateRemoteStreamVideoCanvas',
        [streamId, renderMode, backgroundColor]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author sunhang.io
  /// @brief 渲染来自指定远端用户的视频流时，设置使用的视图和渲染模式。 <br>
  ///        要解除绑定，将 `videoCanvas` 设置为空。
  /// @param streamId 流 ID，用于指定需要设置视图和渲染模式的视频流。
  /// @param videoCanvas 视图信息和渲染模式，参看 VideoCanvas{@link #VideoCanvas}。3.56 版本起支持通过 `renderRotation` 设置远端视频渲染的旋转角度。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 本地用户离开房间时，会解除调用此 API 建立的绑定关系；远端用户离开房间则不会影响。
  ///

  FutureOr<int> setRemoteVideoCanvas(
      String streamId, VideoCanvas videoCanvas) async {
    return await nativeCall('setRemoteVideoCanvas', [streamId, videoCanvas]);
  }

  /// @hidden for internal use only
  /// @valid since 3.54
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author yinkaisheng
  /// @brief 设置远端视频超分模式。
  /// @param streamId 流 ID，用于指定需要设置超分模式的视频流。
  /// @param mode 超分模式，参看 VideoSuperResolutionMode{@link #VideoSuperResolutionMode}。
  /// @return
  ///        - 0: RETURN_STATUS_SUCCESS，SDK 调用成功，并不代表超分模式实际状态，需要根据回调 onRemoteVideoSuperResolutionModeChanged{@link #IRTCEngineEventHandler#onRemoteVideoSuperResolutionModeChanged} 判断实际状态。
  ///        - -1: RETURN_STATUS_NATIVE_IN_VALID，native library 未加载。
  ///        - -2: RETURN_STATUS_PARAMETER_ERR，参数非法，指针为空或字符串为空。
  ///        - -9: RETURN_STATUS_SCREEN_NOT_SUPPORT，不支持对屏幕流开启超分。
  /// @note
  ///        - 该功能仅 arm 架构支持。
  ///        - 该方法须进房后调用。
  ///        - 远端用户视频流的原始分辨率不能超过 640 × 360 px。
  ///        - 支持对一路远端流开启超分，不支持对多路流开启超分。
  ///

  FutureOr<int> setRemoteVideoSuperResolution(
      String streamId, VideoSuperResolutionMode mode) async {
    return await nativeCall(
        'setRemoteVideoSuperResolution', [streamId, mode.$value]);
  }

  /// @detail api
  /// @author huanghao
  /// @brief 调节本端播放收到的远端流时的音量。你必须在进房后进行设置。流的发布状态改变不影响设置生效。
  /// @param streamId 流 ID，用于指定要调节音量的远端流。
  /// @param volume 音量值和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。 <br>
  ///               为保证更好的通话质量，建议将 volume 值设为 [0,100]。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内： <br>
  ///        - 当该方法与 setRemoteRoomAudioPlaybackVolume{@link #RTCRoom#setRemoteRoomAudioPlaybackVolume} 共同使用时，本地收听用户 A 的音量为后调用的方法设置的音量；
  ///        - 当该方法与 setPlaybackVolume{@link #RTCEngine#setPlaybackVolume} 方法共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。
  ///        - 当你调用该方法设置远端流音量后，如果远端退房，接口设置失效。
  ///

  FutureOr<int> setRemoteAudioPlaybackVolume(
      String streamId, int volume) async {
    return await nativeCall('setRemoteAudioPlaybackVolume', [streamId, volume]);
  }

  /// @detail api
  /// @hidden internal use only
  /// @author majun.lvhiei
  /// @brief 在听众端，设置订阅的所有远端音频流精准对齐后播放。
  /// @param streamId 流 ID，作为对齐基准的远端音频流。一般选择主唱的音频流。<br>
  ///                  你必须在收到 onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio}, 确认此音频流已发布后，调用此 API。
  /// @param mode 是否对齐，默认不对齐。参看 AudioAlignmentMode{@link #AudioAlignmentMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 你必须在实时合唱场景下使用此功能。在加入房间时，所有人应设置 ChannelProfile{@link #ChannelProfile} 为 `CHANNEL_PROFILE_CHORUS`。
  ///        - 订阅的所有远端流必须通过 startAudioMixing 开启了背景音乐混音，并将 AudioMixingConfig中的 `syncProgressToRecordFrame` 设置为 `true`。
  ///        - 如果订阅的某个音频流延迟过大，可能无法实现精准对齐。
  ///        - 合唱的参与者不应调用此 API，因为调用此 API 会增加延迟。如果希望从听众变为合唱参与者，应关闭对齐功能。
  ///

  FutureOr<int> setAudioAlignmentProperty(
      String streamId, AudioAlignmentMode mode) async {
    return await nativeCall(
        'setAudioAlignmentProperty', [streamId, mode.$value]);
  }

  /// @detail api
  /// @brief 在订阅远端视频流之前，设置远端视频数据解码方式
  /// @param streamId 远端流 ID，指定对哪一路视频流进行解码方式设置
  /// @param config 视频解码方式，参看 VideoDecoderConfig{@link #VideoDecoderConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 当你想要对远端流进行自定义解码时，你需要先调用 registerRemoteEncodedVideoFrameObserver{@link #RTCEngine#registerRemoteEncodedVideoFrameObserver} 注册远端视频流监测器，然后再调用该接口将解码方式设置为自定义解码。监测到的视频数据会通过 onRemoteEncodedVideoFrame{@link #IRemoteEncodedVideoFrameObserver#onRemoteEncodedVideoFrame} 回调出来。
  ///        - 自 3.56 起，要用于自动订阅场景下，你可以设置 `key` 中的 `RoomId` 和 `UserId` 为 `nullptr`，此时，通过此接口设置的解码方式根据 `key` 中的 `StreamIndex` 值，适用于所有的远端主流或屏幕流的解码方式。
  ///

  FutureOr<int> setVideoDecoderConfig(
      String streamId, VideoDecoderConfig config) async {
    return await nativeCall('setVideoDecoderConfig', [streamId, config.$value]);
  }

  /// @detail api
  /// @brief 在订阅远端视频流之后，向远端请求关键帧
  /// @param streamId 远端流 ID。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅适用于手动订阅模式，并且在成功订阅远端流之后使用。
  ///        - 该方法适用于调用 setVideoDecoderConfig{@link #RTCEngine#setVideoDecoderConfig} 开启自定义解码功能后，并且自定义解码失败的情况下使用
  ///

  FutureOr<int> requestRemoteVideoKeyFrame(String streamId) async {
    return await nativeCall('requestRemoteVideoKeyFrame', [streamId]);
  }

  /// @detail api
  /// @author daining.nemo
  /// @brief 开启云代理
  /// @param cloudProxiesInfo 云代理服务器信息列表。参看 CloudProxyInfo{@link #CloudProxyInfo}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 在加入房间前调用此接口
  ///        - 在开启云代理后，进行通话前网络探测
  ///        - 开启云代理后，并成功链接云代理服务器后，会收到 onCloudProxyConnected{@link #IRTCEngineEventHandler#onCloudProxyConnected}。
  ///        - 要关闭云代理，调用 stopCloudProxy{@link #RTCEngine#stopCloudProxy}。
  ///

  FutureOr<int> startCloudProxy(List<CloudProxyInfo> cloudProxiesInfo) async {
    return await nativeCall('startCloudProxy', [cloudProxiesInfo]);
  }

  /// @detail api
  /// @author daining.nemo
  /// @brief 关闭云代理
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 要开启云代理，调用 startCloudProxy{@link #RTCEngine#startCloudProxy}。
  ///

  FutureOr<int> stopCloudProxy() async {
    return await nativeCall('stopCloudProxy', []);
  }

  /// @detail api
  /// @author songxiaomeng.19
  /// @brief 通过 NTP 协议，获取网络时间。
  /// @return 网络时间。参看 NetworkTimeInfo{@link #NetworkTimeInfo}。
  /// @note
  ///        - 第一次调用此接口会启动网络时间同步功能，并返回 `0`。同步完成后，会收到 onNetworkTimeSynchronized{@link #IRTCEngineEventHandler#onNetworkTimeSynchronized}，此后，再次调用此 API，即可获取准确的网络时间。
  ///        - 在合唱场景下，合唱参与者应在相同的网络时间播放背景音乐。
  ///

  FutureOr<NetworkTimeInfo> getNetworkTimeInfo() async {
    final result = await nativeCall('getNetworkTimeInfo', []);
    return packObject(result,
        () => NetworkTimeInfo(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 获取 WTN 管理接口。
  /// @return WTN 管理接口，参看 IWTNStream{@link #IWTNStream}。
  ///

  FutureOr<IWTNStream> getWTNStream() async {
    final result = await nativeCall('getWTNStream', []);
    return packObject(result,
        () => IWTNStream(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author zhangcaining
  /// @brief 开启通话前回声检测
  /// @param testAudioFilePath 用于回声检测的音频文件的绝对路径，路径字符串使用 UTF-8 编码格式，支持以下音频格式: mp3，aac，m4a，3gp，wav。 <br>
  ///         音频文件不为静音文件，推荐时长为 10 ～ 20 秒。
  /// @return 方法调用结果： <br>
  ///        - 0: 成功。
  ///        - -1：失败。上一次检测未结束，请先调用 stopHardwareEchoDetection{@link #RTCEngine#stopHardwareEchoDetection} 停止检测 后重新调用本接口。
  ///        - -2：失败。路径不合法或音频文件格式不支持。
  /// @note
  ///        - 只有当 ChannelProfile{@link #ChannelProfile} 为 `CHANNEL_PROFIEL_MEETING` 和 `CHANNEL_PROFILE_MEETING_ROOM` 时支持开启本功能。
  ///        - 开启检测前，你需要向用户获取音频设备的使用权限。
  ///        - 开启检测前，请确保音频设备没有被静音，采集和播放音量正常。
  ///        - 调用本接口后监听 onHardwareEchoDetectionResult 获取检测结果。
  ///        - 检测期间，进程将独占音频设备，无法使用其他音频设备测试接口： startEchoTest{@link #RTCEngine#startEchoTest}、startAudioDeviceRecordTest{@link #IRTCAudioDeviceManager#startAudioDeviceRecordTest} 或 startAudioPlaybackDeviceTest{@link #IRTCAudioDeviceManager#startAudioPlaybackDeviceTest}。
  ///        - 调用 stopHardwareEchoDetection{@link #RTCEngine#stopHardwareEchoDetection} 停止检测，释放对音频设备的占用。
  ///

  FutureOr<int> startHardwareEchoDetection(String testAudioFilePath) async {
    return await nativeCall('startHardwareEchoDetection', [testAudioFilePath]);
  }

  /// @detail api
  /// @author zhangcaining
  /// @brief 停止通话前回声检测
  /// @return 方法调用结果： <br>
  ///        - 0: 成功。
  ///        - -1：失败。
  /// @note
  ///       - 关于开启通话前回声检测，参看 startHardwareEchoDetection{@link #RTCEngine#startHardwareEchoDetection} 。
  ///       - 建议在收到 onHardwareEchoDetectionResult{@link #IRTCEngineEventHandler#onHardwareEchoDetectionResult} 通知的检测结果后，调用本接口停止检测。
  ///       - 在用户进入房间前结束回声检测，释放对音频设备的占用，以免影响正常通话。
  ///

  FutureOr<int> stopHardwareEchoDetection() async {
    return await nativeCall('stopHardwareEchoDetection', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfeng.1004
  /// @brief 启用蜂窝网络辅助增强，改善通话质量。
  /// @param config 参看 MediaTypeEnhancementConfig{@link #MediaTypeEnhancementConfig}。
  /// @return 方法调用结果： <br>
  ///        - 0: 成功。
  ///        - -1：失败，内部错误。
  ///        - -2: 失败，输入参数错误。
  /// @note 此功能默认不开启。
  ///

  FutureOr<int> setCellularEnhancement(
      MediaTypeEnhancementConfig config) async {
    return await nativeCall('setCellularEnhancement', [config]);
  }

  /// @detail api
  /// @author keshixing.rtc
  /// @brief 设置本地代理。
  /// @param configurations 本地代理配置参数。参看 LocalProxyConfiguration{@link #LocalProxyConfiguration}。 <br>
  ///        你可以根据自己的需要选择同时设置 Http 隧道 和 Socks5 两类代理，或者单独设置其中一类代理。如果你同时设置了 Http 隧道 和 Socks5 两类代理，此时，媒体和信令采用 Socks5 代理， Http 请求采用 Http 隧道代理；如果只设置 Http 隧道 或 Socks5 一类代理，媒体、信令和 Http 请求均采用已设置的代理。 <br>
  ///        调用此接口设置本地代理后，若想清空当前已有的代理设置，可再次调用此接口，选择不设置任何代理即可清空。
  /// @note
  ///       - 该方法需要在进房前调用。
  ///       - 调用该方法设置本地代理后，SDK 会触发 onLocalProxyStateChanged{@link #IRTCEngineEventHandler#onLocalProxyStateChanged} ，返回代理连接的状态。
  ///

  FutureOr<int> setLocalProxy(
      List<LocalProxyConfiguration> configurations) async {
    return await nativeCall('setLocalProxy', [configurations]);
  }

  /// @valid since 3.56
  /// @detail api
  /// @author likai.666
  /// @brief 创建视频设备管理实例
  /// @return 视频设备管理实例，详见 IVideoDeviceManager{@link #IVideoDeviceManager}
  ///

  FutureOr<IVideoDeviceManager> getVideoDeviceManager() async {
    final result = await nativeCall('getVideoDeviceManager', []);
    return packObject(
        result,
        () => IVideoDeviceManager(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangxiaosen
  /// @brief 设置本端采集的视频帧的旋转角度。 <br>
  ///        当摄像头倒置或者倾斜安装时，可调用本接口进行调整。对于手机等普通设备，可调用 setVideoRotationMode{@link #RTCEngine#setVideoRotationMode} 实现旋转。
  /// @param rotation 相机朝向角度，默认为 `VIDEO_ROTATION_0(0)`，无旋转角度。详见 VideoRotation{@link #VideoRotation}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 对于内部采集的视频画面，如果已调用 setVideoRotationMode{@link #RTCEngine#setVideoRotationMode} 设置了旋转方向，会在此基础上叠加旋转角度。
  ///        - 调用本接口也将对自定义采集视频画面生效，在原有的旋转角度基础上叠加本次设置。
  ///        - 视频贴纸特效或通过 enableVirtualBackground{@link #IVideoEffect#enableVirtualBackground} 增加的虚拟背景，也会跟随本接口的设置进行旋转。
  ///        - 本地渲染视频和发送到远端的视频都会相应旋转，但不会应用到单流转推中。如果希望在单流转推的视频中应用旋转，调用 setVideoOrientation{@link #RTCEngine#setVideoOrientation}。
  ///

  FutureOr<int> setVideoCaptureRotation(VideoRotation rotation) async {
    return await nativeCall('setVideoCaptureRotation', [rotation.$value]);
  }
}

class IAudioEffectPlayer extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.audio.IAudioEffectPlayer';
  static get codegen_$namespace => _$namespace;

  IAudioEffectPlayer([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @brief 开始播放音效文件。 <br>
  ///        可以通过传入不同的 ID 和 filepath 多次调用本方法，以实现同时播放多个音效文件，实现音效叠加。
  /// @param effectId 音效 ID。用于标识音效，请保证音效 ID 唯一性。 <br>
  ///        如果使用相同的 ID 重复调用本方法后，上一个音效会停止，下一个音效开始，并收到 onAudioEffectPlayerStateChanged{@link #IAudioEffectPlayerEventHandler#onAudioEffectPlayerStateChanged}。
  /// @param filePath 音效文件路径。 <br>
  ///        支持在线文件的 URL、本地文件的 URI、本地文件的绝对路径或以 `/assets/` 开头的本地文件路径。对于在线文件的 URL，仅支持 https 协议。 <br>
  ///        推荐的音效文件采样率：8KHz、16KHz、22.05KHz、44.1KHz、48KHz。 <br>
  ///        不同平台支持的本地音效文件格式: <br>
  ///        <table>
  ///           <tr><th></th><th>mp3</th><th>mp4</th><th>aac</th><th>m4a</th><th>3gp</th><th>wav</th><th>ogg</th><th>ts</th><th>wma</th></tr>
  ///           <tr><td>Android</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td></td></tr>
  ///           <tr><td>iOS/macOS</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td></td><td></td></tr>
  ///           <tr><td>Windows</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td>Y</td><td>Y</td></tr>
  ///        </table>
  ///        不同平台支持的在线音效文件格式: <br>
  ///        <table>
  ///           <tr><th></th><th>mp3</th><th>mp4</th><th>aac</th><th>m4a</th><th>3gp</th><th>wav</th><th>ogg</th><th>ts</th><th>wma</th></tr>
  ///           <tr><td>Android</td><td>Y</td><td></td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td></td><td></td></tr>
  ///           <tr><td>iOS/macOS</td><td>Y</td><td></td><td>Y</td><td>Y</td><td></td><td>Y</td><td></td><td></td><td></td></tr>
  ///           <tr><td>Windows</td><td>Y</td><td></td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td>Y</td><td>Y</td></tr>
  ///        </table>
  /// @param config 音效配置，详见 AudioEffectPlayerConfig{@link #AudioEffectPlayerConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 如果已经通过 preload{@link #IAudioEffectPlayer#preload} 将文件加载至内存，确保此处的 ID 与 preload{@link #IAudioEffectPlayer#preload} 设置的 ID 相同。
  ///       - 开始播放音效文件后，可以调用 stop{@link #IAudioEffectPlayer#stop} 方法停止播放音效文件。
  ///

  FutureOr<int> start(
      int effectId, String filePath, AudioEffectPlayerConfig config) async {
    return await nativeCall('start', [effectId, filePath, config]);
  }

  /// @detail api
  /// @brief 停止播放音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start{@link #IAudioEffectPlayer#start} 方法开始播放音效文件后，可以调用本方法停止播放音效文件。
  ///       - 调用本方法停止播放音效文件后，该音效文件会被自动卸载。
  ///

  FutureOr<int> stop(int effectId) async {
    return await nativeCall('stop', [effectId]);
  }

  /// @detail api
  /// @brief 停止播放所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start{@link #IAudioEffectPlayer#start} 方法开始播放音效文件后，可以调用本方法停止播放所有音效文件。
  ///       - 调用本方法停止播放所有音效文件后，该音效文件会被自动卸载。
  ///

  FutureOr<int> stopAll() async {
    return await nativeCall('stopAll', []);
  }

  /// @detail api
  /// @brief 预加载指定音乐文件到内存中，以避免频繁播放同一文件时的重复加载，减少 CPU 占用。
  /// @param effectId 音效 ID。用于标识音效，请保证音效 ID 唯一性。 <br>
  ///        如果使用相同的 ID 重复调用本方法，后一次会覆盖前一次。 <br>
  ///        如果先调用 start{@link #IAudioEffectPlayer#start}，再使用相同的 ID 调用本方法 ，会收到回调 onAudioEffectPlayerStateChanged{@link #IAudioEffectPlayerEventHandler#onAudioEffectPlayerStateChanged} ，通知前一个音效停止，然后加载下一个音效。 <br>
  ///        调用本方法预加载 A.mp3 后，如果需要使用相同的 ID 调用 start{@link #IAudioEffectPlayer#start} 播放 B.mp3，请先调用 unload{@link #IAudioEffectPlayer#unload} 卸载 A.mp3 ，否则会报错 AUDIO_MIXING_ERROR_LOAD_CONFLICT。
  /// @param filePath 音效文件路径。支持本地文件的 URI、本地文件的绝对路径或以 `/assets/` 开头的本地文件路径。 <br>
  ///                 预加载的文件长度不得超过 20s。 <br>
  ///                 不同平台支持的音效文件格式和 start{@link #IAudioEffectPlayer#start} 一致。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 本方法只是预加载指定音效文件，只有调用 start{@link #IAudioEffectPlayer#start} 方法才开始播放指定音效文件。
  ///       - 调用本方法预加载的指定音效文件可以通过 unload{@link #IAudioEffectPlayer#unload} 卸载。
  ///

  FutureOr<int> preload(int effectId, String filePath) async {
    return await nativeCall('preload', [effectId, filePath]);
  }

  /// @detail api
  /// @brief 卸载指定音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 仅在调用 start{@link #IAudioEffectPlayer#start} 或 preload{@link #IAudioEffectPlayer#preload} 后调用此接口。
  ///

  FutureOr<int> unload(int effectId) async {
    return await nativeCall('unload', [effectId]);
  }

  /// @detail api
  /// @brief 卸载所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> unloadAll() async {
    return await nativeCall('unloadAll', []);
  }

  /// @detail api
  /// @brief 暂停播放音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start{@link #IAudioEffectPlayer#start} 方法开始播放音效文件后，可以通过调用本方法暂停播放音效文件。
  ///       - 调用本方法暂停播放音效文件后，可调用 resume{@link #IAudioEffectPlayer#resume} 方法恢复播放。
  ///

  FutureOr<int> pause(int effectId) async {
    return await nativeCall('pause', [effectId]);
  }

  /// @detail api
  /// @brief 暂停播放所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start{@link #IAudioEffectPlayer#start} 方法开始播放音效文件后，可以通过调用本方法暂停播放所有音效文件。
  ///       - 调用本方法暂停播放所有音效文件后，可调用 resumeAll{@link #IAudioEffectPlayer#resumeAll} 方法恢复所有播放。
  ///

  FutureOr<int> pauseAll() async {
    return await nativeCall('pauseAll', []);
  }

  /// @detail api
  /// @brief 恢复播放音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 调用 pause{@link #IAudioEffectPlayer#pause} 方法暂停播放音效文件后，可以通过调用本方法恢复播放。
  ///

  FutureOr<int> resume(int effectId) async {
    return await nativeCall('resume', [effectId]);
  }

  /// @detail api
  /// @brief 恢复播放所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 调用 pauseAll{@link #IAudioEffectPlayer#pauseAll} 方法暂停所有正在播放音效文件后，可以通过调用本方法恢复播放。
  ///

  FutureOr<int> resumeAll() async {
    return await nativeCall('resumeAll', []);
  }

  /// @detail api
  /// @brief 设置音效文件的起始播放位置。
  /// @param effectId 音效 ID
  /// @param position 音效文件起始播放位置，单位为毫秒。 <br>
  ///        你可以通过 getDuration{@link #IAudioEffectPlayer#getDuration} 获取音效文件总时长，position 的值应小于音效文件总时长。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 在播放在线文件时，调用此接口可能造成播放延迟的现象。
  ///        - 仅在调用 start{@link #IAudioEffectPlayer#start} 后调用此接口。
  ///

  FutureOr<int> setPosition(int effectId, int position) async {
    return await nativeCall('setPosition', [effectId, position]);
  }

  /// @detail api
  /// @brief 获取音效文件播放进度。
  /// @param effectId 音效 ID
  /// @return
  ///        - >0: 成功, 音效文件播放进度，单位为毫秒。
  ///        - < 0: 失败
  /// @note
  ///        - 在播放在线文件时，调用此接口可能造成播放延迟的现象。
  ///        - 仅在调用 start{@link #IAudioEffectPlayer#start} 后调用此接口。
  ///

  FutureOr<int> getPosition(int effectId) async {
    return await nativeCall('getPosition', [effectId]);
  }

  /// @detail api
  /// @brief 调节指定音效的音量大小，包括音效文件和 PCM 音频。
  /// @param effectId 音效 ID
  /// @param volume 播放音量相对原音量的比值。单位为 \%。范围为 `[0, 400]`，建议范围是 `[0, 100]`。带溢出保护。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 仅在调用 start{@link #IAudioEffectPlayer#start} 后调用此接口。
  ///

  FutureOr<int> setVolume(int effectId, int volume) async {
    return await nativeCall('setVolume', [effectId, volume]);
  }

  /// @detail api
  /// @brief 设置所有音效的音量大小，包括音效文件和 PCM 音效。
  /// @param volume 播放音量相对原音量的比值。单位为 \%。范围为 `[0, 400]`，建议范围是 `[0, 100]`。带溢出保护。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 该接口的优先级低于 setVolume{@link #IAudioEffectPlayer#setVolume}，即通过 setVolume{@link #IAudioEffectPlayer#setVolume} 单独设置了音量的音效 ID，不受该接口设置的影响。
  ///

  FutureOr<int> setVolumeAll(int volume) async {
    return await nativeCall('setVolumeAll', [volume]);
  }

  /// @detail api
  /// @brief 获取当前音量。
  /// @param effectId 音效 ID
  /// @return
  ///        - >0: 成功, 当前音量值。
  ///        - < 0: 失败
  /// @note 仅在调用 start{@link #IAudioEffectPlayer#start} 后调用此接口。
  ///

  FutureOr<int> getVolume(int effectId) async {
    return await nativeCall('getVolume', [effectId]);
  }

  /// @detail api
  /// @brief 获取音效文件时长。
  /// @param effectId 音效 ID
  /// @return
  ///        - >0: 成功, 音效文件时长，单位为毫秒。
  ///        - < 0: 失败
  /// @note 仅在调用 start{@link #IAudioEffectPlayer#start} 后调用此接口。
  ///

  FutureOr<int> getDuration(int effectId) async {
    return await nativeCall('getDuration', [effectId]);
  }

  /// @detail api
  /// @brief 设置回调句柄。
  /// @param handler 参看 IAudioEffectPlayerEventHandler{@link #IAudioEffectPlayerEventHandler}。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。
  ///

  FutureOr<int> setEventHandler(IAudioEffectPlayerEventHandler handler) async {
    return await nativeCall('setEventHandler', [handler]);
  }
}

class IVideoDeviceManager extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.IVideoDeviceManager';
  static get codegen_$namespace => _$namespace;

  IVideoDeviceManager([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @valid since 3.56
  /// @detail api
  /// @author likai.666
  /// @brief 获取当前系统内视频采集设备列表。
  /// @return 包含系统中所有视频采集设备的列表，参看 VideoDeviceInfo{@link #VideoDeviceInfo}。
  ///

  FutureOr<List<VideoDeviceInfo>> enumerateVideoCaptureDevices() async {
    return await nativeCall('enumerateVideoCaptureDevices', []);
  }

  /// @valid since 3.56
  /// @detail api
  /// @author likai.666
  /// @brief 设置当前视频采集设备
  /// @param deviceId 视频设备 ID，可以通过 enumerateVideoCaptureDevices{@link #IVideoDeviceManager#enumerateVideoCaptureDevices} 获取
  /// @return
  ///        - 0：方法调用成功
  ///        - !0：方法调用失败
  ///

  FutureOr<int> setVideoCaptureDevice(String deviceId) async {
    return await nativeCall('setVideoCaptureDevice', [deviceId]);
  }
}

class IWTNStream extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.IWTNStream';
  static get codegen_$namespace => _$namespace;

  IWTNStream([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，此接口替代了 `startPlayPublicStream` 和 `stopPlayPublicStream` 方法来订阅/取消订阅指定 WTN 视频流，如果你使用了这两个方法，请迁移至此接口。
  /// @author hanchenchen
  /// @brief 订阅/取消订阅指定 WTN 视频流 <br>
  ///        无论用户是否在房间内，都可以调用本接口订阅/取消订阅指定的 WTN 视频流。
  /// @param streamId WTN 流 ID，如果指定流暂未发布，则本地客户端将在其开始发布后接收到流数据。
  /// @param subscribe 是否订阅 WTN 流 <br>
  ///       - true：订阅
  ///       - false：取消订阅
  /// @return
  ///        - 0: 成功。同时将收到 onWTNVideoSubscribeStateChanged{@link #IWTNStreamEventHandler#onWTNVideoSubscribeStateChanged} 回调。
  ///        - !0: 失败。当参数不合法或参数为空，调用失败。
  /// @note
  ///        - 一个客户端最多同时播放 5 路 WTN 流，请及时调用 subscribeWTNVideoStream{@link #IWTNStream#subscribeWTNVideoStream}/subscribeWTNAudioStream{@link #IWTNStream#subscribeWTNAudioStream} 取消订阅 WTN 流，避免订阅的 WTN 流数量超限。
  ///        - 在调用本接口之前，建议先绑定渲染视图。
  ///              - 调用 setWTNRemoteVideoCanvas{@link #IWTNStream#setWTNRemoteVideoCanvas} 绑定内部渲染视图
  ///              - 调用 setWTNRemoteVideoSink{@link #IWTNStream#setWTNRemoteVideoSink} 绑定自定义渲染视图
  ///        - 调用本接口后，可以通过 onWTNFirstRemoteVideoFrameDecoded{@link #IWTNStreamEventHandler#onWTNFirstRemoteVideoFrameDecoded} 回调 WTN 视频流的首帧解码情况。
  ///        - 调用本接口后，可以通过 onWTNSEIMessageReceived{@link #IWTNStreamEventHandler#onWTNSEIMessageReceived} 回调 WTN 流中包含的 SEI 信息。
  /// @order 0
  ///

  FutureOr<int> subscribeWTNVideoStream(
      String streamId, boolean subscribe) async {
    return await nativeCall('subscribeWTNVideoStream', [streamId, subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，此接口替代了 `startPlayPublicStream` 和 `stopPlayPublicStream` 方法来订阅/取消订阅指定 WTN 音频流，如果你使用了这两个方法，请迁移至此接口。
  /// @author hanchenchen
  /// @brief 订阅/取消订阅指定 WTN 音频流 <br>
  ///        无论用户是否在房间内，都可以调用本接口订阅/取消订阅指定的 WTN 音频流。
  /// @param streamId WTN 流 ID，如果指定流暂未发布，则本地客户端将在其开始发布后接收到流数据。
  /// @param subscribe 是否订阅 WTN 流 <br>
  ///       - true：订阅
  ///       - false：取消订阅
  /// @return
  ///        - 0: 成功。同时将收到 onWTNAudioSubscribeStateChanged{@link #IWTNStreamEventHandler#onWTNAudioSubscribeStateChanged} 回调。
  ///        - !0: 失败。当参数不合法或参数为空，调用失败。
  /// @note
  ///        - 一个客户端最多同时播放 5 路 WTN 流，请及时调用 subscribeWTNVideoStream{@link #IWTNStream#subscribeWTNVideoStream}/subscribeWTNAudioStream{@link #IWTNStream#subscribeWTNAudioStream} 取消订阅WTN 流，避免订阅的 WTN 流数量超限。
  ///        - 在调用本接口之前，建议先绑定渲染视图。
  ///              - 调用 setWTNRemoteVideoCanvas{@link #IWTNStream#setWTNRemoteVideoCanvas} 绑定内部渲染视图
  ///              - 调用 setWTNRemoteVideoSink{@link #IWTNStream#setWTNRemoteVideoSink} 绑定自定义渲染视图
  ///        - 调用本接口后，可以通过 onWTNFirstRemoteAudioFrame{@link #IWTNStreamEventHandler#onWTNFirstRemoteAudioFrame} 回调 WTN 音频流的音频首帧解码情况。
  ///        - 调用本接口后，可以通过 onWTNSEIMessageReceived{@link #IWTNStreamEventHandler#onWTNSEIMessageReceived} 回调 WTN 流中包含的 SEI 信息。
  /// @order 1
  ///

  FutureOr<int> subscribeWTNAudioStream(
      String streamId, boolean subscribe) async {
    return await nativeCall('subscribeWTNAudioStream', [streamId, subscribe]);
  }

  /// @author hanchenchen
  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `setPublicStreamVideoCanvas` 方法来实现下述功能。你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此接口。
  /// @brief 为指定 WTN 流绑定内部渲染视图
  /// @param streamId WTN 流 ID
  /// @param canvas 内部渲染视图，如果需要解除视频的绑定视图，把 VideoCanvas{@link #VideoCanvas} 设置为空。
  /// @return
  ///        - 0：成功
  ///        - !0：失败
  /// @order 2
  ///

  FutureOr<int> setWTNRemoteVideoCanvas(
      String streamId, VideoCanvas canvas) async {
    return await nativeCall('setWTNRemoteVideoCanvas', [streamId, canvas]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `setPublicStreamVideoSink` 方法来实现下述功能。你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此接口。
  /// @author hanchenchen
  /// @brief 为指定 WTN 流绑定自定义渲染器。详见[自定义视频渲染](https://www.volcengine.com/docs/6348/81201)。
  /// @param streamId WTN 流 ID
  /// @param videoSink 自定义视频渲染器，自定义视频渲染器，需要释放渲染器资源时，将  `videoSink` 设置为 `null`。参看 IVideoSink{@link #IVideoSink}
  /// @param config 远端视频帧回调配置，参看 RemoteVideoSinkConfig{@link #RemoteVideoSinkConfig}
  /// @return
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @order 3
  ///

  FutureOr<int> setWTNRemoteVideoSink(String streamId, IVideoSink videoSink,
      RemoteVideoSinkConfig config) async {
    return await nativeCall(
        'setWTNRemoteVideoSink', [streamId, videoSink, config]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `setPublicStreamAudioPlaybackVolume` 方法来实现下述功能。你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此接口。
  /// @author hanchenchen
  /// @brief 调节 WTN 流的音频播放音量。
  /// @param streamId WTN 流 ID
  /// @param volume 音频播放音量值和原始音量值的比值，该比值的范围是 `[0, 400]`，单位为 \%，且自带溢出保护。为保证更好的音频质量，建议设定在 `[0, 100]` 之间，其中 100 为系统默认值。
  /// @return
  ///         - 0: 成功调用。
  ///         - -2: 参数错误。
  /// @order 4
  ///

  FutureOr<int> setWTNRemoteAudioPlaybackVolume(
      String streamId, int volume) async {
    return await nativeCall(
        'setWTNRemoteAudioPlaybackVolume', [streamId, volume]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 设置 WTN 流回调接口
  /// @param handler WTN 回调类，参看 IWTNStreamEventHandler{@link #IWTNStreamEventHandler}。
  /// @order 5
  ///

  FutureOr<int> setWTNStreamEventHandler(IWTNStreamEventHandler handler) async {
    return await nativeCall('setWTNStreamEventHandler', [handler]);
  }
}

class IMediaPlayer extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.audio.IMediaPlayer';
  static get codegen_$namespace => _$namespace;

  IMediaPlayer([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @brief 打开音乐文件。 <br>
  ///        一个播放器实例仅能够同时打开一个音乐文件。如果需要同时打开多个音乐文件，请创建多个音乐播放器实例。 <br>
  ///        要播放 PCM 格式的音频数据，参看 openWithCustomSource{@link #IMediaPlayer#openWithCustomSource}。`openWithCustomSource` 和此 API 互斥。
  /// @param filePath 音乐文件路径。 <br>
  ///        支持在线文件的 URL、本地文件的 URI、本地文件的绝对路径或以 `/assets/` 开头的本地文件路径。对于在线文件的 URL，仅支持 https 协议。 <br>
  ///        推荐的采样率：8KHz、16KHz、22.05KHz、44.1KHz、48KHz。 <br>
  ///        不同平台支持的本地文件格式: <br>
  ///        <table>
  ///           <tr><th></th><th>mp3</th><th>mp4</th><th>aac</th><th>m4a</th><th>3gp</th><th>wav</th><th>ogg</th><th>ts</th><th>wma</th></tr>
  ///           <tr><td>Android</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td></td></tr>
  ///           <tr><td>iOS/macOS</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td></td><td></td></tr>
  ///           <tr><td>Windows</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td>Y</td><td>Y</td></tr>
  ///        </table>
  ///        不同平台支持的在线文件格式: <br>
  ///        <table>
  ///           <tr><th></th><th>mp3</th><th>mp4</th><th>aac</th><th>m4a</th><th>3gp</th><th>wav</th><th>ogg</th><th>ts</th><th>wma</th></tr>
  ///           <tr><td>Android</td><td>Y</td><td></td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td></td><td></td></tr>
  ///           <tr><td>iOS/macOS</td><td>Y</td><td></td><td>Y</td><td>Y</td><td></td><td>Y</td><td></td><td></td><td></td></tr>
  ///           <tr><td>Windows</td><td>Y</td><td></td><td>Y</td><td>Y</td><td>Y</td><td>Y</td><td></td><td>Y</td><td>Y</td></tr>
  ///        </table>
  /// @param config 详见 MediaPlayerConfig{@link #MediaPlayerConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> open(String filePath, MediaPlayerConfig config) async {
    return await nativeCall('open', [filePath, config]);
  }

  /// @detail api
  /// @brief 播放音乐。你仅需要在调用 open{@link #IMediaPlayer#open}，且未开启自动播放时，调用此方法。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  /// - 要播放 PCM 格式的音频数据，参看 openWithCustomSource{@link #IMediaPlayer#openWithCustomSource}。`openWithCustomSource` 和此 API 互斥。
  /// - 调用本方法播放音频文件后，可调用 stop{@link #IMediaPlayer#stop} 方法暂停播放。
  ///

  FutureOr<int> start() async {
    return await nativeCall('start', []);
  }

  /// @detail api
  /// @brief 启动音频裸数据混音。 <br>
  ///        要播放音乐文件，参看 open{@link #IMediaPlayer#open}。`open` 与此 API 互斥。
  /// @param source 数据源，详见 MediaPlayerCustomSource{@link #MediaPlayerCustomSource}
  /// @param config 详见 MediaPlayerConfig{@link #MediaPlayerConfig}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用本方法启动后，再调用 pushExternalAudioFrame{@link #IMediaPlayer#pushExternalAudioFrame} 推送音频数据，才会开始混音。
  ///       - 如要结束 PCM 音频数据混音，调用 stop{@link #IMediaPlayer#stop}。
  ///

  FutureOr<int> openWithCustomSource(
      MediaPlayerCustomSource source, MediaPlayerConfig config) async {
    return await nativeCall('openWithCustomSource', [source, config]);
  }

  /// @detail api
  /// @brief 调用 open{@link #IMediaPlayer#open}, start{@link #IMediaPlayer#start}, 或 openWithCustomSource{@link #IMediaPlayer#openWithCustomSource} 开始播放后，可以调用本方法停止。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> stop() async {
    return await nativeCall('stop', []);
  }

  /// @detail api
  /// @brief 调用 open{@link #IMediaPlayer#open}，或 start{@link #IMediaPlayer#start} 开始播放音频文件后，调用本方法暂停播放。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用本方法暂停播放后，可调用 resume{@link #IMediaPlayer#resume} 恢复播放。
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> pause() async {
    return await nativeCall('pause', []);
  }

  /// @detail api
  /// @brief 调用 pause{@link #IMediaPlayer#pause} 暂停音频播放后，调用本方法恢复播放。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        此接口仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> resume() async {
    return await nativeCall('resume', []);
  }

  /// @detail api
  /// @brief 调节指定混音的音量大小，包括音乐文件混音和 PCM 混音。
  /// @param volume 播放音量相对原音量的比值。单位为 \%。范围为 `[0, 400]`，建议范围是 `[0, 100]`。带溢出保护。
  /// @param type 详见 AudioMixingType{@link #AudioMixingType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///

  FutureOr<int> setVolume(int volume, AudioMixingType type) async {
    return await nativeCall('setVolume', [volume, type.$value]);
  }

  /// @detail api
  /// @brief 获取当前音量
  /// @param type 详见 AudioMixingType{@link #AudioMixingType}。
  /// @return
  ///        - >0: 成功, 当前音量值。
  ///        - < 0: 失败
  /// @note 仅在音频播放进行状态时，调用此方法。包括音乐文件混音和 PCM 混音。
  ///

  FutureOr<int> getVolume(AudioMixingType type) async {
    return await nativeCall('getVolume', [type.$value]);
  }

  /// @detail api
  /// @brief 获取音乐文件时长。
  /// @return
  ///        - >0: 成功, 音乐文件时长，单位为毫秒。
  ///        - < 0: 失败
  /// @note
  ///        - 仅在音频播放进行状态时，调用此方法。
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> getTotalDuration() async {
    return await nativeCall('getTotalDuration', []);
  }

  /// @detail api
  /// @brief 获取混音音乐文件的实际播放时长，单位为毫秒。
  /// @return
  ///        - >0: 实际播放时长。
  ///        - < 0: 失败。
  /// @note
  ///        - 实际播放时长指的是歌曲不受停止、跳转、倍速、卡顿影响的播放时长。例如，若歌曲正常播放到 1:30 时停止播放 30s 或跳转进度到 2:00, 随后继续正常播放 2 分钟，则实际播放时长为 3 分 30 秒。
  ///        - 仅在音频播放进行状态，且 setProgressInterval{@link #IMediaPlayer#setProgressInterval} 设置间隔大于 `0` 时，调用此方法。
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> getPlaybackDuration() async {
    return await nativeCall('getPlaybackDuration', []);
  }

  /// @detail api
  /// @brief 获取音乐文件播放进度。
  /// @return
  ///        - >0: 成功, 音乐文件播放进度，单位为毫秒。
  ///        - < 0: 失败
  /// @note
  ///        - 仅在音频播放进行状态时，调用此方法。
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> getPosition() async {
    return await nativeCall('getPosition', []);
  }

  /// @valid since 3.59
  /// @detail api
  /// @author wangfeng.1004
  /// @brief 获取播放器状态
  /// @return 播放器当前状态，参看 PlayerState{@link #PlayerState}。
  /// @note 仅在音频实例创建后，调用此方法。
  ///

  FutureOr<PlayerState> getState() async {
    return await nativeCall('getState', []);
  }

  /// @detail api
  /// @brief 开启变调功能，多用于 K 歌场景。
  /// @param pitch 与音乐文件原始音调相比的升高/降低值，取值范围为 `[-12，12]`，默认值为 0。每相邻两个值的音高距离相差半音，正值表示升调，负值表示降调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 仅支持音乐文件混音，不支持 PCM 数据。
  ///

  FutureOr<int> setAudioPitch(int pitch) async {
    return await nativeCall('setAudioPitch', [pitch]);
  }

  /// @detail api
  /// @brief 设置音乐文件的起始播放位置。
  /// @param position 音乐文件起始播放位置，单位为毫秒。 <br>
  ///        你可以通过 getTotalDuration{@link #IMediaPlayer#getTotalDuration} 获取音乐文件总时长，position 的值应小于音乐文件总时长。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。
  ///        - 在播放在线文件时，调用此接口可能造成播放延迟的现象。
  ///        - 调用本接口后，会收到 onMediaPlayerEvent{@link #IMediaPlayerEventHandler#onMediaPlayerEvent} 回调。
  ///

  FutureOr<int> setPosition(int position) async {
    return await nativeCall('setPosition', [position]);
  }

  /// @detail api
  /// @brief 设置当前音乐文件的声道模式
  /// @param mode 声道模式。默认的声道模式和源文件一致，详见 AudioMixingDualMonoMode{@link #AudioMixingDualMonoMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> setAudioDualMonoMode(AudioMixingDualMonoMode mode) async {
    return await nativeCall('setAudioDualMonoMode', [mode.$value]);
  }

  /// @detail api
  /// @brief 获取当前音乐文件的音轨数
  /// @return + >= 0：成功，返回当前音乐文件的音轨数
  ///         - < 0：方法调用失败
  /// @note
  ///        - 仅在音频播放进行状态时，调用此方法。
  ///        - 此方法仅支持音乐文件，不支持 PCM 数据。
  ///

  FutureOr<int> getAudioTrackCount() async {
    return await nativeCall('getAudioTrackCount', []);
  }

  /// @detail api
  /// @brief 指定当前音乐文件的播放音轨
  /// @param index 指定的播放音轨，从 0 开始，取值范围为 `[0, getAudioTrackCount()-1]`。 <br>
  ///        设置的参数值需要小于 getAudioTrackCount{@link #IMediaPlayer#getAudioTrackCount} 的返回值
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 仅在音频播放进行状态时，调用此方法。
  ///        - 此方法仅支持音乐文件，不支持 PCM 数据。
  ///        - 调用本接口后，会收到 onMediaPlayerEvent{@link #IMediaPlayerEventHandler#onMediaPlayerEvent} 回调。
  ///

  FutureOr<int> selectAudioTrack(int index) async {
    return await nativeCall('selectAudioTrack', [index]);
  }

  /// @detail api
  /// @brief 设置播放速度
  /// @param speed 播放速度与原始文件速度的比例，单位：\%，取值范围为 `[50,200]`，默认值为 100。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 此方法对音频文件可用，不支持 PCM 数据。
  ///

  FutureOr<int> setPlaybackSpeed(int speed) async {
    return await nativeCall('setPlaybackSpeed', [speed]);
  }

  /// @detail api
  /// @brief 设置音频文件混音时，收到 onMediaPlayerPlayingProgress{@link #IMediaPlayerEventHandler#onMediaPlayerPlayingProgress} 的间隔。
  /// @param interval 时间间隔，单位毫秒。 <br>
  ///       - interval > 0 时，触发回调。实际间隔为 10 的倍数。如果输入数值不能被 10 整除，将自动向上取整。例如传入 `52`，实际间隔为 60 ms。
  ///       - interval <= 0 时，不会触发回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 此方法仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> setProgressInterval(long interval) async {
    return await nativeCall('setProgressInterval', [interval]);
  }

  /// @detail api
  /// @brief 如果你需要使用 enableVocalInstrumentBalance{@link #RTCEngine#enableVocalInstrumentBalance} 对音频文件/PCM 音频数据设置音量均衡，你必须通过此接口传入其原始响度。
  /// @param loudness 原始响度，单位：lufs，取值范围为 `[-70.0, 0.0]`。 <br>
  ///        当设置的值小于 -70.0lufs 时，则默认调整为 -70.0lufs，大于 0.0lufs 时，则不对该响度做音量均衡处理。默认值为 1.0lufs，即不做处理。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 此方法对音频文件和音频裸数据播放都可用。
  ///

  FutureOr<int> setLoudness(float loudness) async {
    return await nativeCall('setLoudness', [loudness]);
  }

  /// @detail api
  /// @brief 注册回调句柄以在本地音乐文件混音时，收到相关回调。
  /// @param observer 参看 IMediaPlayerAudioFrameObserver{@link #IMediaPlayerAudioFrameObserver}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        此接口仅支持音频文件，不支持 PCM 数据。
  ///

  FutureOr<int> registerAudioFrameObserver(
      IMediaPlayerAudioFrameObserver observer) async {
    return await nativeCall('registerAudioFrameObserver', [observer]);
  }

  /// @detail api
  /// @brief 推送用于混音的 PCM 音频帧数据
  /// @param audioFrame 音频帧，详见 AudioFrame{@link #AudioFrame}。 <br>
  ///                   - 音频采样格式必须为 S16。音频缓冲区内的数据格式必须为 PCM，其容量大小应该为 `audioFrame.samples × audioFrame.channel × 2`。
  ///                   - 必须指定具体的采样率和声道数，不支持设置为自动。
  /// @return
  ///       - 0: 成功
  ///       - < 0: 失败
  /// @note
  ///      - 调用该方法前，须通过 openWithCustomSource{@link #IMediaPlayer#openWithCustomSource} 启动外部音频流混音。
  ///      - 使用参考建议：首次推送数据，请在应用侧先缓存一定数据（如 200 毫秒），然后一次性推送过去；此后的推送操作定时 10 毫秒一次，并且每次的音频数据量为 10 毫秒数据量。
  ///      - 如果要暂停播放，暂停推送即可。
  ///

  FutureOr<int> pushExternalAudioFrame(AudioFrame audioFrame) async {
    return await nativeCall('pushExternalAudioFrame', [audioFrame]);
  }

  /// @detail api
  /// @brief 设置回调句柄。
  /// @param handler 参看 IMediaPlayerEventHandler{@link #IMediaPlayerEventHandler}。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。
  ///

  FutureOr<int> setEventHandler(IMediaPlayerEventHandler handler) async {
    return await nativeCall('setEventHandler', [handler]);
  }
}

class IVideoProcessor extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.IVideoProcessor';
  static get codegen_$namespace => _$namespace;

  IVideoProcessor([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 获取 RTC SDK 采集得到的视频帧，根据 registerLocalVideoProcessor{@link #RTCEngine#registerLocalVideoProcessor} 设置的视频前处理器，进行视频前处理，最终将处理后的视频帧给到 RTC SDK 用于编码传输。
  /// @param frame RTC SDK 采集得到的视频帧，参看 IVideoFrame{@link #IVideoFrame}。
  /// @return 经过视频前处理后的视频帧，返回给 RTC SDK 供编码和传输，参看 IVideoFrame{@link #IVideoFrame}。
  /// @note
  ///       - 在进行视频前处理前，你需要调用 registerLocalVideoProcessor{@link #RTCEngine#registerLocalVideoProcessor} 设置视频前处理器。
  ///       - 如果需要取消视频前处理，可以将视频前处理器设置为 nullptr。
  ///       - 对于纹理类型的视频数据，应用层需要在自己的工作线程准备OpenGL上下文进行视频处理。
  ///       - 应用层实现IVideoProcessor.processVideoFrame 接口时，可以直接修改参数 frame 的缓冲区，将frame引用计数加一(调用addRef方法)，然后将修改后的frame返回。应用层也可以创建一个新的视频帧并返回。
  ///       - 对于 IVideoProcessor.processVideoFrame返回的视频帧，在SDK层消费完成后，SDK内部总是调用这个视频帧的releaseRef方法。
  ///

  FutureOr<IVideoFrame> processVideoFrame(IVideoFrame frame) async {
    final result = await nativeCall('processVideoFrame', [frame]);
    return packObject(result,
        () => IVideoFrame(const NativeClassOptions([], disableInit: true)));
  }
}

class RTCRoom extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.RTCRoom';
  static get codegen_$namespace => _$namespace;

  RTCRoom([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @author shenpengliang
  /// @brief 退出并销毁调用 createRTCRoom{@link #RTCEngine#createRTCRoom} 所创建的房间实例。
  ///

  FutureOr<void> destroy() async {
    return await nativeCall('destroy', []);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 加入 RTC 房间。 <br>
  ///        调用 createRTCRoom{@link #RTCEngine#createRTCRoom} 创建房间后，调用此方法加入房间，同房间内其他用户进行音视频通话。
  /// @param token 动态密钥。用于对进房用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///        使用不同 AppID 的 App 是不能互通的。 <br>
  ///        请务必保证生成 Token 使用的 AppID 和创建引擎时使用的 AppID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  /// @param userVisibility 用户可见性。建议在进房时将用户可见性都设置为 `false`，并在用户需要发送音视频流时再通过 setUserVisibility{@link #RTCRoom#setUserVisibility} 设置为 `true`。从而避免因房间内用户达到数量上限所导致的进房失败。默认情况下，一个 RTC 房间最多同时容纳 50 名可见用户，其中最多 30 人可同时上麦，更多信息参看[用户和媒体流上限](https://www.volcengine.com/docs/6348/257549)。
  /// @param roomConfig 房间参数配置，设置房间模式以及是否自动发布或订阅流。具体配置模式参看 RTCRoomConfig{@link #RTCRoomConfig}。
  /// @return
  ///        - 0：方法调用成功。
  ///        -  0: 成功。触发以下回调：
  ///          - 本端收到房间状态通知 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///          - 本端收到本地流发布状态通知 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调
  ///          - 本端收到流订阅状态通知 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调。
  ///          - 本端收到房间内已发布流的通知 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调。
  ///          - 如果本端用户为可见用户，房间内其他用户收到 onUserJoined{@link #IRTCRoomEventHandler#onUserJoined} 回调通知。
  ///        - -1：roomID / userInfo.uid 包含了无效的参数。
  ///        - -2：已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom{@link #RTSRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调告知。
  /// @note
  ///       - 同一个 App ID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后进房的用户会将先进房的用户踢出房间，并且先进房的用户会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调通知，错误类型详见 ERROR_CODE_DUPLICATE_LOGIN{@link #ErrorCode#ERROR_CODE_DUPLICATE_LOGIN}。
  ///       - 房间内不可见用户的容量远远大于可见用户，而且用户默认可见，因此对于不参与互动的用户，你需要调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 更改为不可见用户。从而避免因房间内用户达到数量上限所导致的进房失败。默认情况下，一个 RTC 房间最多同时容纳 50 名可见用户，其中最多 30 人可同时上麦，更多信息参看[用户和媒体流上限](https://www.volcengine.com/docs/6348/257549)。
  ///       - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 onConnectionStateChanged{@link #IRTCEngineEventHandler#onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调通知；如果加入房间的用户是可见用户，远端用户会收到 onUserJoined{@link #IRTCRoomEventHandler#onUserJoined} 回调通知。
  ///

  FutureOr<int> joinRoom(String token, UserInfo userInfo,
      boolean userVisibility, RTCRoomConfig roomConfig) async {
    return await nativeCall(
        'joinRoom', [token, userInfo, userVisibility, roomConfig]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 通过设置 RTCRoom{@link #RTCRoom} 对象的事件句柄，监听此对象对应的回调事件。
  /// @param rtcRoomEventHandler 参看 IRTCRoomEventHandler{@link #IRTCRoomEventHandler}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> setRTCRoomEventHandler(
      IRTCRoomEventHandler rtcRoomEventHandler) async {
    return await nativeCall('setRTCRoomEventHandler', [rtcRoomEventHandler]);
  }

  /// @detail api
  /// @brief 设置用户可见性。未调用该接口前，本地用户默认对他人可见。 <br>
  ///        默认情况下，一个 RTC 房间最多同时容纳 50 名可见用户，最多 30 人可同时上麦。更多信息参看[用户和媒体流上限](https://www.volcengine.com/docs/6348/257549)。
  /// @param enable 设置用户是否对房间内其他用户可见： <br>
  ///        - true: 可见，用户可以在房间内发布音视频流，房间中的其他用户将收到用户的行为通知，例如进房、开启视频采集和退房。
  ///        - false: 不可见，用户不可以在房间内发布音视频流，房间中的其他用户不会收到用户的行为通知，例如进房、开启视频采集和退房。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0: 调用失败。参看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  ///        设置用户可见性，会收到设置成功/失败回调 onUserVisibilityChanged{@link #IRTCRoomEventHandler#onUserVisibilityChanged}。（v3.54 新增）
  ///        - 在加入房间前设置用户可见性，若设置的可见性与默认值不同，将在加入房间时触发本回调。
  ///        - 在加入房间后设置用户可见性，若可见性前后不同，会触发本回调。
  ///        - 在断网重连后，若可见性发生改变，会触发本回调。
  /// @note
  ///       - 在加入房间前后，用户均可调用此方法设置用户可见性。
  ///       - 在房间内，调用此方法成功切换用户可见性后，房间内其他用户会收到相应的回调。
  ///       - 从可见换至不可见时，房间内其他用户会收到 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave}。
  ///       - 从不可见切换至可见时，房间内其他用户会收到 onUserJoined{@link #IRTCRoomEventHandler#onUserJoined}。
  ///       - 若调用该方法将可见性设为 `false`，此时尝试发布流会收到 `WARNING_CODE_PUBLISH_STREAM_FORBIDEN` 警告。
  ///

  FutureOr<int> setUserVisibility(boolean enable) async {
    return await nativeCall('setUserVisibility', [enable]);
  }

  /// @detail api
  /// @brief 设置发流端音画同步。 <br>
  ///        当同一用户同时使用两个通话设备分别采集发送音频和视频时，有可能会因两个设备所处的网络环境不一致而导致发布的流不同步，此时你可以在视频发送端调用该接口，SDK 会根据音频流的时间戳自动校准视频流，以保证接收端听到音频和看到视频在时间上的同步性。
  /// @param audioUserId 音频发送端的用户 ID，将该参数设为空则可解除当前音视频的同步关系。
  /// @return
  ///        - 0: 调用成功。调用该接口后音画同步状态发生改变时，你会收到 onAVSyncStateChange{@link #IRTCRoomEventHandler#onAVSyncStateChange} 回调。
  ///        - < 0 : 调用失败。你也可以通过监听 onAVSyncEvent{@link #IRTCRoomEventHandler#onAVSyncEvent} 获取错误详情。同一 RTC 房间内允许存在多个音视频同步关系，但需注意单个音频源不支持与多个视频源同时同步。
  /// @note
  ///        - 该方法在进房前后均可调用。
  ///        - 进行音画同步的音频发布用户 ID 和视频发布用户 ID 须在同一个 RTC 房间内。
  ///        - 如需更换同步音频源，再次调用该接口传入新的 `audioUserId` 即可；如需更换同步视频源，需先解除当前的同步关系，后在新视频源端开启同步。
  ///

  FutureOr<int> setMultiDeviceAVSync(String audioUserId) async {
    return await nativeCall('setMultiDeviceAVSync', [audioUserId]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @author zhoubohui
  /// @brief 设置期望订阅的远端视频流类型。比如大流、中流、小流等。
  /// @param streamId 目标要订阅的远端视频流 ID。
  /// @param streamType 远端视频流类型，参看 SimulcastStreamType{@link #SimulcastStreamType}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 若使用 342 及以前版本的 SDK，调用该方法前请联系技术支持人员开启按需订阅功能。
  ///        - 该方法仅在发布端调用 setLocalSimulcastMode{@link #RTCEngine#setLocalSimulcastMode} 开启了发送多路视频流的情况下生效。
  ///        - 若发布端开启了推送多路流功能，但订阅端不对流参数进行设置，则默认接受发送端设置的分辨率最大的一路视频流。该方法可在进房后调用。
  /// @order 1
  ///

  FutureOr<int> setRemoteSimulcastStreamType(
      String streamId, SimulcastStreamType streamType) async {
    return await nativeCall(
        'setRemoteSimulcastStreamType', [streamId, streamType.$value]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `publishStream`、`unpublishStream` 、`publishScreen` 和 `unpublishScreen` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此接口。
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 发布/取消发布视频流。
  /// @param publish 是否发布视频流。<br>
  ///                - `true`: 发布。
  ///                - `false`: 取消发布。
  ///
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果你已经在用户进房时通过调用 joinRoom{@link #RTCRoom#joinRoom} 成功选择了自动发布，则无需再调用本接口。
  ///        - 调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法将自身设置为不可见后无法调用该方法，需将自身切换至可见后方可调用该方法发布摄像头视频流。
  ///        - 如果你需要发布麦克风采集到的音频流，调用 publishStreamAudio{@link #RTCRoom#publishStreamAudio}。
  ///        - 如果你需要向多个房间发布流，调用 startForwardStreamToRooms{@link #RTCRoom#startForwardStreamToRooms}。
  ///        - 调用此方法后，房间中的所有远端用户会收到 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo} 回调通知，订阅了视频流的远端用户会收到 onFirstRemoteVideoFrameDecoded{@link #IRTCEngineEventHandler#onFirstRemoteVideoFrameDecoded} 回调。
  ///

  FutureOr<int> publishStreamVideo(boolean publish) async {
    return await nativeCall('publishStreamVideo', [publish]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `publishStream`、`unpublishStream` 、`publishScreen` 和 `unpublishScreen` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此接口。
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 发布/取消发布音频流。
  /// @param publish 是否发布音频流。<br>
  ///                - `true`: 发布。
  ///                - `false`: 取消发布。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果你已经在用户进房时通过调用 joinRoom{@link #RTCRoom#joinRoom} 成功选择了自动发布，则无需再调用本接口。
  ///        - 调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法将自身设置为不可见后无法调用该方法，需将自身切换至可见后方可调用该方法发布音频流。
  ///        - 如果你需要发布摄像头采集到的视频流，调用 publishStreamVideo{@link #RTCRoom#publishStreamVideo}。
  ///        - 如果你需要向多个房间发布流，调用 startForwardStreamToRooms{@link #RTCRoom#startForwardStreamToRooms}。
  ///        - 调用此方法后，房间中的所有远端用户会收到 onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调通知，其中成功收到了音频流的远端用户会收到 onFirstRemoteAudioFrame{@link #IRTCEngineEventHandler#onFirstRemoteAudioFrame} 回调。
  /// @order 0
  ///

  FutureOr<int> publishStreamAudio(boolean publish) async {
    return await nativeCall('publishStreamAudio', [publish]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeStream`, `unsubscribeStream`, `subscribeScreen` 和 `unsubscribeScreen` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用这两个方法，请迁移至该接口。
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 订阅/取消订阅房间内指定的远端视频流。
  /// @param streamId 目标远端视频流 ID。
  /// @param subscribe 是否订阅该视频流。<br>
  ///                  - `true`: 订阅。
  ///                  - `false`: 取消订阅。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 在调用本接口时已经订阅该远端流（手动订阅或自动订阅），则将根据本次传入的参数，更新订阅配置。
  ///        - 你必须先通过 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo} 回调获取当前房间里的远端摄像头流信息，然后调用本方法按需订阅。
  ///        - 调用该方法后，你会收到 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged} 通知方法调用结果。
  ///        - 成功订阅远端用户的媒体流后，订阅关系将持续到调用 subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo} 取消订阅或本端用户退房。
  ///        - 关于其他调用异常，你会收到 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged} 回调通知，具体异常原因参看 SubscribeStateChangeReason{@link #SubscribeStateChangeReason}。
  ///

  FutureOr<int> subscribeStreamVideo(String streamId, boolean subscribe) async {
    return await nativeCall('subscribeStreamVideo', [streamId, subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeStream`, `unsubscribeStream`, `subscribeScreen` 和 `unsubscribeScreen` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用这两个方法，请迁移至该接口。
  /// @author xuyiling.x10
  /// @brief 订阅/取消订阅房间内指定的远端音频流。
  /// @param streamId 目标远端音频流 ID。
  /// @param subscribe 是否要订阅指定的远端音频流。<br>
  ///                  - `true`: 订阅。
  ///                  - `false`: 取消订阅。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///        - 若当前用户在调用本接口时已经订阅该远端音频流（手动订阅或自动订阅），则将根据本次传入的参数，更新订阅配置。
  ///        - 你必须先通过 onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调获取当前房间里的远端麦克风流信息，然后调用本方法按需订阅。
  ///        - 调用该方法后，你会收到 onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 通知方法调用结果。
  ///        - 成功订阅远端用户的媒体流后，订阅关系将持续到调用 subscribeStreamAudio{@link #RTCRoom#subscribeStreamAudio} 取消订阅或本端用户退房。
  ///        - 关于其他调用异常，你会收到 onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调通知，具体异常原因参看 SubscribeStateChangeReason{@link #SubscribeStateChangeReason}。
  ///

  FutureOr<int> subscribeStreamAudio(String streamId, boolean subscribe) async {
    return await nativeCall('subscribeStreamAudio', [streamId, subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeAllStreams` 和 `unsubscribeAllStreams` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用这两个方法，请迁移至该接口。
  /// @author yejing.luna
  /// @brief 订阅/取消订阅房间内所有远端视频流（通过摄像头采集的）。
  /// @param subscribe 是否订阅所有远端视频流。<br>
  ///                - `true`: 订阅。
  ///                - `false`: 取消订阅。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 多次调用订阅接口时，将根据末次调用接口和传入的参数，更新订阅配置：subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo}、subscribeStreamAudio{@link #RTCRoom#subscribeStreamAudio}。
  ///        - 开启音频选路后，如果房间内的媒体流超过上限，建议通过调用单流订阅接口逐一指定需要订阅的媒体流。
  ///        - 调用该方法后，你会收到 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 通知方法调用结果，包含异常原因。
  ///        - 成功订阅远端用户的媒体流后，订阅关系将持续到调用 subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo}、subscribeStreamAudio{@link #RTCRoom#subscribeStreamAudio} 取消订阅或本端用户退房。
  /// @order 5
  ///

  FutureOr<int> subscribeAllStreamsVideo(boolean subscribe) async {
    return await nativeCall('subscribeAllStreamsVideo', [subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeAllStreams` 和 `unsubscribeAllStreams` 方法，用于下面的功能。如果您已升级到 3.60 或更高版本，并且仍在使用这些方法，请尽快迁移到该接口。
  /// @author yejing.luna
  /// @brief 订阅或取消订阅所有远端音频流（通过麦克风采集的）。
  /// @param subscribe 是否订阅所有远端音频流：<br>
  ///                  - `true`: 订阅。
  ///                  - `false`: 取消订阅。
  /// @return 方法调用结果： <br>
  ///        - 0：成功。
  ///        - < 0：失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 多次调用订阅接口时，将根据末次调用接口和传入的参数，更新订阅配置：subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo}、subscribeStreamAudio{@link #RTCRoom#subscribeStreamAudio}。
  ///        - 开启音频选路后，如果房间内的媒体流超过上限，建议通过调用单流订阅接口逐一指定需要订阅的媒体流。
  ///        - 调用该方法后，你会收到 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 通知方法调用结果，包含异常原因。
  ///        - 成功订阅远端用户的媒体流后，订阅关系将持续到调用 subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo}、subscribeStreamAudio{@link #RTCRoom#subscribeStreamAudio} 取消订阅或本端用户退房。
  /// @order 5
  ///

  FutureOr<int> subscribeAllStreamsAudio(boolean subscribe) async {
    return await nativeCall('subscribeAllStreamsAudio', [subscribe]);
  }

  /// @hidden for internal use only.
  /// @detail api
  /// @author shenpengliang
  /// @brief 设置是否订阅自己的流
  /// @param enable 订阅自己/不订阅自己
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  ///

  FutureOr<int> enableSubscribeLocalStream(boolean enable) async {
    return await nativeCall('enableSubscribeLocalStream', [enable]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `pauseAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @author shenpengliang
  /// @brief 暂停接收所有远端视频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该方法仅暂停远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。
  ///        - 若想恢复接收远端流，需调用 resumeAllSubscribedStreamVideo{@link #RTCRoom#resumeAllSubscribedStreamVideo}。
  ///        - 多房间场景下，仅暂停接收发布在当前所在房间的流。
  ///

  FutureOr<int> pauseAllSubscribedStreamVideo() async {
    return await nativeCall('pauseAllSubscribedStreamVideo', []);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `pauseAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @author shenpengliang
  /// @brief 暂停接收所有远端音频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅暂停远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。
  ///        - 若想恢复接收远端流，需调用 resumeAllSubscribedStreamVideo{@link #RTCRoom#resumeAllSubscribedStreamVideo}。
  ///        - 多房间场景下，仅暂停接收发布在当前所在房间的流。
  ///

  FutureOr<int> pauseAllSubscribedStreamAudio() async {
    return await nativeCall('pauseAllSubscribedStreamAudio', []);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `resumeAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @author shenpengliang
  /// @brief 恢复接收所有远端视频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该方法仅恢复远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。
  ///

  FutureOr<int> resumeAllSubscribedStreamVideo() async {
    return await nativeCall('resumeAllSubscribedStreamVideo', []);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `resumeAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @author shenpengliang
  /// @brief 恢复接收所有远端音频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅恢复远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。
  ///

  FutureOr<int> resumeAllSubscribedStreamAudio() async {
    return await nativeCall('resumeAllSubscribedStreamAudio', []);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 开始跨房间转发媒体流。 <br>
  ///        在调用 joinRoom{@link #RTCRoom#joinRoom} 后调用本接口，实现向多个房间转发媒体流，适用于跨房间连麦等场景。
  /// @param forwardStreamInfos 跨房间媒体流转发指定房间的信息。参看 ForwardStreamInfo{@link #ForwardStreamInfo}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 调用本方法后，将在本端触发 onForwardStreamStateChanged{@link #IRTCRoomEventHandler#onForwardStreamStateChanged} 回调。
  ///        - 调用本方法后，你可以通过监听 onForwardStreamEvent{@link #IRTCRoomEventHandler#onForwardStreamEvent} 回调来获取各个目标房间在转发媒体流过程中的相关事件。
  ///        - 开始转发后，目标房间中的用户将接收到本地用户进房 onUserJoined{@link #IRTCRoomEventHandler#onUserJoined} 和发流 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调。
  ///        - 调用本方法后，可以调用 updateForwardStreamToRooms{@link #RTCRoom#updateForwardStreamToRooms} 更新目标房间信息，例如，增加或减少目标房间等。
  ///        - 调用本方法后，可以调用 stopForwardStreamToRooms{@link #RTCRoom#stopForwardStreamToRooms} 停止向所有房间转发媒体流。
  ///        - 调用本方法后，可以调用 pauseForwardStreamToAllRooms{@link #RTCRoom#pauseForwardStreamToAllRooms} 暂停向所有房间转发媒体流。
  ///

  FutureOr<int> startForwardStreamToRooms(
      List<ForwardStreamInfo> forwardStreamInfos) async {
    return await nativeCall('startForwardStreamToRooms', [forwardStreamInfos]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 更新跨房间媒体流转发信息。 <br>
  ///        通过 startForwardStreamToRooms{@link #RTCRoom#startForwardStreamToRooms} 发起媒体流转发后，可调用本方法增加或者减少目标房间，或更新房间密钥。 <br>
  ///        调用本方法增加或删减房间后，将在本端触发 onForwardStreamStateChanged{@link #IRTCRoomEventHandler#onForwardStreamStateChanged} 回调，包含发生了变动的目标房间中媒体流转发状态。
  /// @param forwardStreamInfos 跨房间媒体流转发目标房间信息。参看 ForwardStreamInfo{@link #ForwardStreamInfo}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        增加或删减目标房间后，新增目标房间中的用户将接收到本地用户进房 onUserJoined{@link #IRTCRoomEventHandler#onUserJoined} 和发布 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调。
  ///

  FutureOr<int> updateForwardStreamToRooms(
      List<ForwardStreamInfo> forwardStreamInfos) async {
    return await nativeCall('updateForwardStreamToRooms', [forwardStreamInfos]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 停止跨房间媒体流转发。 <br>
  ///        通过 startForwardStreamToRooms{@link #RTCRoom#startForwardStreamToRooms} 发起媒体流转发后，可调用本方法停止向所有目标房间转发媒体流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 调用本方法后，将在本端触发 onForwardStreamStateChanged{@link #IRTCRoomEventHandler#onForwardStreamStateChanged} 回调。
  ///        - 调用本方法后，原目标房间中的用户将接收到本地用户停止发布 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调和退房 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 的回调。
  ///        - 如果需要停止向指定的房间转发媒体流，请调用 updateForwardStreamToRooms{@link #RTCRoom#updateForwardStreamToRooms} 更新房间信息。
  ///        - 如果需要暂停转发，请调用 pauseForwardStreamToAllRooms{@link #RTCRoom#pauseForwardStreamToAllRooms}，并在之后随时调用 resumeForwardStreamToAllRooms{@link #RTCRoom#resumeForwardStreamToAllRooms} 快速恢复转发。
  ///

  FutureOr<int> stopForwardStreamToRooms() async {
    return await nativeCall('stopForwardStreamToRooms', []);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 暂停跨房间媒体流转发。 <br>
  ///        通过 startForwardStreamToRooms{@link #RTCRoom#startForwardStreamToRooms} 发起媒体流转发后，可调用本方法暂停向所有目标房间转发媒体流。 <br>
  ///        调用本方法暂停向所有目标房间转发后，你可以随时调用 resumeForwardStreamToAllRooms{@link #RTCRoom#resumeForwardStreamToAllRooms} 快速恢复转发。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 调用本方法后，目标房间中的用户将接收到本地用户停止发布 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调和退房 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 的回调。
  ///

  FutureOr<int> pauseForwardStreamToAllRooms() async {
    return await nativeCall('pauseForwardStreamToAllRooms', []);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 恢复跨房间媒体流转发。 <br>
  ///        调用 pauseForwardStreamToAllRooms{@link #RTCRoom#pauseForwardStreamToAllRooms} 暂停转发之后，调用本方法恢复向所有目标房间转发媒体流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note 目标房间中的用户将接收到本地用户进房 onUserJoined{@link #IRTCRoomEventHandler#onUserJoined} 和发布 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调 的回调。
  ///

  FutureOr<int> resumeForwardStreamToAllRooms() async {
    return await nativeCall('resumeForwardStreamToAllRooms', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 获取范围语音接口实例。
  /// @return 方法调用结果： <br>
  ///        - IRangeAudio：成功，返回一个 IRangeAudio{@link #IRangeAudio} 实例。
  ///        - null：失败，当前 SDK 不支持范围语音功能。
  /// @note 首次调用该方法须在创建房间后、加入房间前。范围语音相关 API 和调用时序详见[范围语音](https://www.volcengine.com/docs/6348/114727)。
  ///

  FutureOr<IRangeAudio> getRangeAudio() async {
    final result = await nativeCall('getRangeAudio', []);
    return packObject(result,
        () => IRangeAudio(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 获取空间音频接口实例。
  /// @return 空间音频管理接口实例。如果返回 NULL，则表示不支持空间音频，详见 ISpatialAudio{@link #ISpatialAudio} 。
  /// @note
  ///       - 首次调用该方法须在创建房间后、加入房间前。空间音频相关 API 和调用时序详见[空间音频](https://www.volcengine.com/docs/6348/93903)。
  ///       - 只有在使用支持真双声道播放的设备时，才能开启空间音频效果；
  ///       - 在网络状况不佳的情况下，即使开启了这一功能，也不会产生空间音频效果；
  ///       - 机型性能不足可能会导致音频卡顿，使用低端机时，不建议开启空间音频效果；
  ///       - 空间音频效果在启用服务端选路功能时，不生效。
  ///

  FutureOr<ISpatialAudio> getSpatialAudio() async {
    final result = await nativeCall('getSpatialAudio', []);
    return packObject(result,
        () => ISpatialAudio(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author zhangcaining
  /// @brief 调节某个房间内所有远端用户的音频播放音量。
  /// @param volume 音频播放音量和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。 <br>
  ///        为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///              - 0: 静音
  ///              - 100: 原始音量，默认值
  ///              - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内， <br>
  ///        - 该方法与 setRemoteAudioPlaybackVolume{@link #RTCEngine#setRemoteAudioPlaybackVolume} 互斥，最新调用的任一方法设置的音量将覆盖此前已设置的音量，效果不叠加；
  ///        - 当该方法与 setPlaybackVolume{@link #RTCEngine#setPlaybackVolume} 方法共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。
  ///

  FutureOr<int> setRemoteRoomAudioPlaybackVolume(int volume) async {
    return await nativeCall('setRemoteRoomAudioPlaybackVolume', [volume]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author yejing.luna
  /// @brief 设置本端发布流在音频选路中的优先级。
  /// @param audioSelectionPriority 本端发布流在音频选路中的优先级，默认正常参与音频选路。参见 AudioSelectionPriority{@link #AudioSelectionPriority}。
  /// @note
  /// 在控制台上为本 appId 开启音频选路后，调用本接口才会生效。进房前后调用均可生效。更多信息参见[音频选路](https://www.volcengine.com/docs/6348/113547)。 <br>
  /// 如果本端用户同时加入不同房间，使用本接口进行的设置相互独立。
  ///

  FutureOr<int> setAudioSelectionConfig(
      AudioSelectionPriority audioSelectionPriority) async {
    return await nativeCall(
        'setAudioSelectionConfig', [audioSelectionPriority.$value]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author lichangfeng.rtc
  /// @brief 设置/更新 RTC 房间附加信息，可用于标识房间状态或属性，或灵活实现各种业务逻辑。
  /// @param key 房间附加信息键值，长度小于 10 字节。 <br>
  ///        同一房间内最多可存在 5 个 key，超出则会从第一个 key 起进行替换。
  /// @param value 房间附加信息内容，长度小于 128 字节。
  /// @return
  ///        - 0: 方法调用成功，返回本次调用的任务编号；
  ///        - <0: 方法调用失败，具体原因详见 SetRoomExtraInfoResult{@link #SetRoomExtraInfoResult}。
  /// @note
  ///       - 在设置房间附加信息前，必须先调用 joinRoom{@link #RTCRoom#joinRoom} 加入房间。
  ///       - 调用该方法后，会收到一次 onSetRoomExtraInfoResult{@link #IRTCRoomEventHandler#onSetRoomExtraInfoResult} 回调，提示设置结果。
  ///       - 调用该方法成功设置附加信息后，同一房间内的其他用户会收到关于该信息的回调 onRoomExtraInfoUpdate{@link #IRTCRoomEventHandler#onRoomExtraInfoUpdate}。
  ///       - 新进房的用户会收到进房前房间内已有的全部附加信息通知。
  ///

  FutureOr<long> setRoomExtraInfo(String key, String value) async {
    return await nativeCall('setRoomExtraInfo', [key, value]);
  }

  /// @detail api
  /// @region 流管理
  /// @brief 设置当前推流的流附加信息。
  /// @param extra_info 流附加信息。长度不超过1024的字符串。
  /// @return 方法调用结果： <br>
  ///        + 0：成功；<br>
  ///        + !0：失败。
  /// @note
  ///        + 可通过此函数设置当前推流的流附加信息。流附加信息是流 ID 的附加信息标识，不同于流 ID 在推流过程中不可修改，流附加信息可以在对应流 ID 的推流中途修改。开发者可根据流附加信息来实现流 ID 相关的可变内容的同步。
  ///        + 该方法在进房前后均可调用
  ///        + 相同房间内的其他用户会通过 [onRoomStreamExtraInfoUpdate] 回调函数获得通知。
  ///

  FutureOr<int> setStreamExtraInfo(String extra_info) async {
    return await nativeCall('setStreamExtraInfo', [extra_info]);
  }

  /// @detail api
  /// @brief 设置期望订阅的远端视频流的参数。
  /// @param streamId 期望配置订阅参数的远端视频流 ID。
  /// @param remoteVideoConfig 期望配置的远端视频流参数，参看 RemoteVideoConfig{@link #RemoteVideoConfig}。
  /// @return 方法调用结果： <br>
  ///        + 0：成功。<br>
  ///        + !0：失败。
  /// @note
  ///        + 若使用 342 及以前版本的 SDK，调用该方法前请联系技术支持人员开启按需订阅功能。  <br>
  ///        + 该方法仅在发布端调用 setLocalSimulcastMode{@link #RTCEngine#setLocalSimulcastMode}  开启了发送多路视频流的情况下生效，此时订阅端将收到来自发布端与期望设置的参数最相近的一路流；否则订阅端只会收到一路参数为分辨率 640px × 360px、帧率 15fps 的视频流。  <br>
  ///        + 若发布端开启了推送多路流功能，但订阅端不对流参数进行设置，则默认接受发送端设置的分辨率最大的一路视频流。  <br>
  ///        + 该方法需在进房后调用。  <br>
  ///        + SDK 会根据发布端和所有订阅端的设置灵活调整视频流的参数，具体调整策略详见[推送多路流](https://www.volcengine.com/docs/6348/70139)文档。
  ///

  FutureOr<int> setRemoteVideoConfig(
      String streamId, RemoteVideoConfig videoConfig) async {
    return await nativeCall('setRemoteVideoConfig', [streamId, videoConfig]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author qiaoxingwang
  /// @brief 识别或翻译房间内所有用户的语音，形成字幕。 <br>
  ///        调用该方法时，可以在 SubtitleMode{@link #SubtitleMode} 中选择语音识别或翻译模式。如果选择识别模式，语音识别文本会通过 onSubtitleMessageReceived{@link #IRTCRoomEventHandler#onSubtitleMessageReceived} 事件回调给你； <br>
  ///        如果选择翻译模式，你会同时收到两个 onSubtitleMessageReceived{@link #IRTCRoomEventHandler#onSubtitleMessageReceived} 回调，分别包含字幕原文及字幕译文。 <br>
  ///        调用该方法后，你会收到 onSubtitleStateChanged{@link #IRTCRoomEventHandler#onSubtitleStateChanged} 回调，通知字幕是否开启。
  /// @param subtitleConfig 字幕配置信息。参看 SubtitleConfig{@link #SubtitleConfig}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 使用字幕功能前，你需要在 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle) 开启实时字幕功能。
  ///        - 如果你需要使用流式语音识别模式，你应在 [语音技术控制台](https://console.volcengine.com/speech/service/16) 创建流式语音识别应用。创建时，服务类型应选择 `流式语音识别`，而非 `音视频字幕生成`。创建后，在 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle) 上启动流式语音识别，并填写创建语音技术应用时获取的相关信息，包括：APP ID，Access Token，和 Cluster ID。
  ///        - 如果你需要使用实时语音翻译模式，你应开通机器翻译服务，参考 [开通服务](https://www.volcengine.com/docs/4640/130262)。完成开通后，在 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle) 上启用实时语音翻译模式。
  ///        - 此方法需要在进房后调用。
  ///        - 如需指定源语言，你需要在调用 `joinRoom` 接口进房时，通过 extraInfo 参数传入格式为`"语种英文名": "语种代号"` JSON 字符串，例如设置源语言为英文时，传入 `"source_language": "en"`。如未指定源语言，SDK 会将系统语种设定为源语言。如果你的系统语种不是中文、英文和日文，此时 SDK 会自动将中文设为源语言。
  ///          - 识别模式下，你可以传入 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle)上预设或自定义的语种英文名和语种代号。识别模式下支持的语言参看[识别模式语种支持](https://www.volcengine.com/docs/6561/109880#场景-语种支持)。
  ///          - 翻译模式下，你需要传入机器翻译规定的语种英文名和语种代号。翻译模式下支持的语言及对应的代号参看[翻译模式语言支持](https://www.volcengine.com/docs/4640/35107)。
  ///

  FutureOr<int> startSubtitle(SubtitleConfig subtitleConfig) async {
    return await nativeCall('startSubtitle', [subtitleConfig]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author qiaoxingwang
  /// @brief 关闭字幕。 <br>
  ///        调用该方法后，用户会收到 onSubtitleStateChanged{@link #IRTCRoomEventHandler#onSubtitleStateChanged} 回调，通知字幕是否关闭。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  ///

  FutureOr<int> stopSubtitle() async {
    return await nativeCall('stopSubtitle', []);
  }

  /// @valid since 3.53
  /// @detail api
  /// @author gechangwu
  /// @brief 获取 RTC 房间 ID。
  /// @return 房间 ID。
  ///

  FutureOr<String> getRoomId() async {
    return await nativeCall('getRoomId', []);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @author xuyiling
  /// @brief 获取通话 ID。<br>
  ///        该方法需要在加入 RTC 房间后调用。当创建一个房间开启音视频通话后，系统会为该房间生成一个对应的通话 ID，标识此房间的通话。
  /// @return 通话 ID。
  ///

  FutureOr<String> getCallId() async {
    return await nativeCall('getCallId', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 离开游戏房间。 <br>
  ///        调用此方法结束通话过程，并释放所有通话相关的资源。
  /// @return
  ///        - 0：调用成功。如果用户是房间内可见用户，触发以下回调：
  ///            - 远端用户收到 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 回调通知。
  ///            - 正在发布的流会被取消发布。远端用户收到 onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调通知。
  ///        - < 0：调用失败，参看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 加入游戏房间后，必须调用此方法结束通话，否则无法开始下一次通话。
  ///       - 此方法是异步操作，调用返回时并没有真正退出房间。真正退出房间后，本地会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调通知。你必须在收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调后，再销毁房间或引擎，或调用 joinRoom{@link #RTCRoom#joinRoom} 再次加入房间。
  ///       - 调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 将自身设为可见的用户离开房间后，房间内其他用户会收到 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 回调通知。
  ///

  FutureOr<int> leaveRoom() async {
    return await nativeCall('leaveRoom', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 更新游戏房间的 Token。 <br>
  ///        收到 onTokenWillExpire{@link #IRTCRoomEventHandler#onTokenWillExpire}，onPublishPrivilegeTokenWillExpire{@link #IRTCRoomEventHandler#onPublishPrivilegeTokenWillExpire}， 或 onSubscribePrivilegeTokenWillExpire{@link #IRTCRoomEventHandler#onSubscribePrivilegeTokenWillExpire} 时，你必须重新获取 Token，并调用此方法更新 Token，以保证通话的正常进行。
  /// @param token 重新获取的有效 Token。 <br>
  ///        如果 Token 无效，你会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged}，错误码是 `-1010`。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note 请勿同时调用 updateToken{@link #IGameRoom#updateToken} 和 joinRoom{@link #IGameRoom#joinRoom} 方法更新 Token。若因 Token 过期或无效导致加入房间失败或已被移出房间，你应该在获取新的有效 Token 后调用 joinRoom{@link #IGameRoom#joinRoom} 重新加入房间。
  ///

  FutureOr<int> updateToken(String token) async {
    return await nativeCall('updateToken', [token]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，调用该接口开启或关闭麦克风。同房间其他用户会收到回调 OnAudioDeviceStateChanged{@link #IRTCEngineEventHandler#OnAudioDeviceStateChanged}。
  /// @param enable 是否开启麦克风：<br>
  ///             - true：开启麦克风，采集并发布音频流。
  ///             - false：默认设置。关闭麦克风并停止发布音频流。
  /// @return
  ///        - 0：接口调用成功。
  ///        - -3：接口调用失败。没有加入房间。
  /// @note 不可与 enableAudioSend{@link #IGameRoom#enableAudioSend} 同时调用。
  ///

  FutureOr<int> enableMicrophone(boolean enable) async {
    return await nativeCall('enableMicrophone', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，开启或关闭扬声器。
  /// @param enable 是否开启扬声器：<br>
  ///               - true：开启扬声器，接收所有远端用户的音频流。
  ///               - false：默认设置。关闭扬声器，停止接收所有远端用户的音频流。
  /// @return
  ///        - 0：接口调用成功。
  ///        - -3：接口调用失败。没有加入房间。
  ///

  FutureOr<int> enableSpeakerphone(boolean enable) async {
    return await nativeCall('enableSpeakerphone', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，开始或停止发送音频流。调用此接口不影响音频采集。同房间其他用户会收到相应的回调。
  /// @param enable 是否发送音频流：<br>
  ///               - true：发送音频流。
  ///               - false：默认设置。停止发送音频流（不会关闭麦克风），即静音。
  /// @return
  ///        - 0：表示参数检查通过，不代表打开麦克风会成功，比如房间不存在
  ///        - -3：接口调用失败。没有加入房间。
  /// @note 不可与 EnableMicrophone{@link #IGameRoom#EnableMicrophone} 同时调用。
  ///

  FutureOr<int> enableAudioSend(boolean enable) async {
    return await nativeCall('enableAudioSend', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 是否接收某个特定用户的音频流。关闭声音接收不会影响扬声器或其他音频输出设备的状态。
  /// @param userId 用户 ID，最大长度为128字节的非空字符串。支持的字符集范围为: <br>
  ///            1. 26个大写字母 A ~ Z<br>
  ///            2. 26个小写字母 a ~ z<br>
  ///            3. 10个数字 0 ~ 9<br>
  ///            4. 下划线"_", at符"\@", 减号"-"
  /// @param enable 是否接收指定用户的音频流：<br>
  ///               - true：接收该用户的音频流。即允许该用户的音频数据被传递到本地设备并播放。
  ///               - false：默认设置，不接收该用户的音频流，即不播放该用户的声音。但不会关闭扬声器，扬声器仍可用于其他音频输出。
  /// @return
  ///        - 0：接口调用成功
  ///        - -2：传入的用户 ID 为空字符串。
  ///

  FutureOr<int> enableAudioReceive(String userId, boolean enable) async {
    return await nativeCall('enableAudioReceive', [userId, enable]);
  }

  /// @detail api
  /// @brief 加入 RTS 房间。 <br>
  ///        调用 createRTSRoom{@link #RTCEngine#createRTSRoom} 创建房间后，调用此方法加入房间，同房间内其他用户进行音视频通话。
  /// @param token 动态密钥。用于对进房用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///        使用不同 AppID 的 App 是不能互通的。 <br>
  ///        请务必保证生成 Token 使用的 AppID 和创建引擎时使用的 AppID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  /// @return
  ///        - 0：方法调用成功。
  ///        -  0: 成功。触发以下回调：
  ///          - 本端收到房间状态通知 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调。
  ///        - -1：roomID / userInfo.uid 包含了无效的参数。
  ///        - -2：已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom{@link #RTSRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调告知。
  /// @note
  ///       - 同一个 App ID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后进房的用户会将先进房的用户踢出房间，并且先进房的用户会收到 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调通知，错误类型详见 ERROR_CODE_DUPLICATE_LOGIN{@link #ErrorCode#ERROR_CODE_DUPLICATE_LOGIN}。
  ///       - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 onConnectionStateChanged{@link #IRTCEngineEventHandler#onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调通知；如果加入房间的用户是可见用户，远端用户会收到 onUserJoined{@link #IRTSRoomEventHandler#onUserJoined} 回调通知。
  ///

  FutureOr<int> joinRTSRoom(String token, UserInfo userInfo) async {
    return await nativeCall('joinRTSRoom', [token, userInfo]);
  }

  /// @detail api
  /// @brief 通过设置 RTSRoom{@link #RTSRoom} 对象的事件句柄，监听此对象对应的回调事件。
  /// @param rtcRoomEventHandler 参看 IRTSRoomEventHandler{@link #IRTSRoomEventHandler}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> setRTSRoomEventHandler(
      IRTSRoomEventHandler rtcRoomEventHandler) async {
    return await nativeCall('setRTSRoomEventHandler', [rtcRoomEventHandler]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点文本消息（P2P）。
  /// @param userId 消息接收用户的 ID
  /// @param messageStr 发送的文本消息内容。消息不超过 64 KB。
  /// @param config 消息发送的可靠/有序类型，参看 MessageConfig{@link #MessageConfig}
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增
  ///        - -1：发送失败，RTCRoom 实例未创建
  ///        - -2：发送失败，uid 为空
  /// @note
  ///      - 在发送房间内文本消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///      - 调用后，会收到 onUserMessageSendResult{@link #IRTSRoomEventHandler#onUserMessageSendResult} 回调，通知消息发送成功或失败；
  ///      - 若消息发送成功，则 userId 所指定的用户会收到 onUserMessageReceived{@link #IRTSRoomEventHandler#onUserMessageReceived} 回调。
  ///

  FutureOr<long> sendUserMessage(
      String userId, String messageStr, MessageConfig config) async {
    return await nativeCall(
        'sendUserMessage', [userId, messageStr, config.$value]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点二进制消息（P2P）。
  /// @param userId 消息接收用户的 ID
  /// @param buffer 发送的二进制消息内容。消息不超过 64KB。
  /// @param config 消息发送的可靠/有序类型，参看 MessageConfig{@link #MessageConfig}。
  /// @note
  ///      - 在发送房间内二进制消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///      - 调用后，会收到 onUserMessageSendResult{@link #IRTSRoomEventHandler#onUserMessageSendResult} 回调，通知消息发送成功或失败；
  ///      - 若消息发送成功，则 userId 所指定的用户会收到 onUserBinaryMessageReceived{@link #IRTSRoomEventHandler#onUserBinaryMessageReceived} 回调。
  ///

  FutureOr<long> sendUserBinaryMessage(
      String userId, ArrayBuffer buffer, MessageConfig config) async {
    return await nativeCall(
        'sendUserBinaryMessage', [userId, buffer, config.$value]);
  }

  /// @detail api
  /// @brief 给房间内的所有其他用户群发文本消息。
  /// @param messageStr 发送的文本消息内容，消息不超过 64 KB。
  /// @note
  ///       - 在房间内广播文本消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///       - 调用后，会收到 onRoomMessageSendResult{@link #IRTSRoomEventHandler#onRoomMessageSendResult} 回调；
  ///       - 同一房间内的其他用户会收到 onRoomMessageReceived{@link #IRTSRoomEventHandler#onRoomMessageReceived} 回调。
  ///

  FutureOr<long> sendRoomMessage(String messageStr) async {
    return await nativeCall('sendRoomMessage', [messageStr]);
  }

  /// @detail api
  /// @brief 给房间内的所有其他用户群发二进制消息。
  /// @param buffer 发送的二进制消息内容，消息不超过 64KB。
  /// @note
  ///       - 在房间内广播二进制消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///       - 调用后，会收到 onRoomMessageSendResult{@link #IRTSRoomEventHandler#onRoomMessageSendResult} 回调；
  ///       - 同一房间内的其他用户会收到 onRoomBinaryMessageReceived{@link #IRTSRoomEventHandler#onRoomBinaryMessageReceived} 回调。
  ///

  FutureOr<long> sendRoomBinaryMessage(ArrayBuffer buffer) async {
    return await nativeCall('sendRoomBinaryMessage', [buffer]);
  }
}

class IRangeAudio extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.audio.IRangeAudio';
  static get codegen_$namespace => _$namespace;

  IRangeAudio([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @author chuzhongtao
  /// @brief 开启/关闭范围语音功能。 <br>
  ///        范围语音是指，在同一 RTC 房间中设定的音频接收距离范围内，本地用户收听到的远端用户音频音量会随着远端用户的靠近/远离而放大/衰减；若远端用户在房间内的位置超出设定范围，则本地用户无法接收其音频。音频接收范围设置参看 updateReceiveRange{@link #IRangeAudio#updateReceiveRange}。
  /// @param enable 是否开启范围语音功能： <br>
  ///        - true: 开启
  ///        - false: 关闭（默认）
  /// @note 该方法进房前后都可调用，为保证进房后范围语音效果的平滑切换，你需在该方法前先调用 updatePosition{@link #IRangeAudio#updatePosition} 设置自身位置坐标，然后开启该方法收听范围语音效果。
  ///

  FutureOr<void> enableRangeAudio(boolean enable) async {
    return await nativeCall('enableRangeAudio', [enable]);
  }

  /// @detail api
  /// @author chuzhongtao
  /// @brief 更新本地用户在房间内空间直角坐标系中的位置坐标。
  /// @param pos 三维坐标的值，默认为 [0, 0, 0]，参看 Position{@link #Position}
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - !0：失败。
  /// @note
  ///        - 调用该接口更新坐标后，你需调用 enableRangeAudio{@link #IRangeAudio#enableRangeAudio} 开启范围语音功能以收听范围语音效果。
  ///

  FutureOr<int> updatePosition(Position pos) async {
    return await nativeCall('updatePosition', [pos]);
  }

  /// @detail api
  /// @author chuzhongtao
  /// @brief 更新本地用户的音频收听范围。
  /// @param range 音频收听范围，参看 ReceiveRange{@link #ReceiveRange}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - !0: 失败。
  ///

  FutureOr<int> updateReceiveRange(ReceiveRange range) async {
    return await nativeCall('updateReceiveRange', [range]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 设置范围语音的音量衰减模式。
  /// @param type 音量衰减模式。默认为线性衰减。详见 AttenuationType{@link #AttenuationType}。
  /// @param coefficient 指数衰减模式下的音量衰减系数，默认值为 1。范围 [0.1,100]，推荐设置为 `50`。数值越大，音量的衰减速度越快。
  /// @return 调用是否成功 <br>
  ///         - `0`:调用成功
  ///         - `-1`:调用失败。原因为在调用 enableRangeAudio{@link #IRangeAudio#enableRangeAudio} 开启范围语音前或进房前调用本接口
  /// @note 音量衰减范围通过 updateReceiveRange{@link #IRangeAudio#updateReceiveRange} 进行设置。
  ///

  FutureOr<int> setAttenuationModel(
      AttenuationType type, float coefficient) async {
    return await nativeCall('setAttenuationModel', [type.$value, coefficient]);
  }

  /// @detail api
  /// @author chuzhongtao
  /// @brief 添加标签组，用于标记相互之间通话不衰减的用户组。 <br>
  ///        在同一个 RTC 房间中，如果多个用户的标签组之间有交集，那么，他们之间互相通话时，通话不衰减。 <br>
  ///        比如，用户身处多个队伍，队伍成员间通话不衰减。那么，可以为每个队伍绑定专属标签，每个用户的标签组包含用户所属各个队伍的标签。
  /// @param flags 标签组
  ///

  FutureOr<void> setNoAttenuationFlags(List<String> flags) async {
    return await nativeCall('setNoAttenuationFlags', [flags]);
  }
}

class IVideoEffect extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.IVideoEffect';
  static get codegen_$namespace => _$namespace;

  IVideoEffect([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @hidden for internal use only
  /// @detail api
  /// @author zhushufan.ref
  /// @brief 设置视频特效算法模型地址，并初始化特效模块。
  /// @param finder ResourceFinder 地址
  /// @param deleter ResourceDeleter 地址
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  ///

  FutureOr<int> setAlgoModelResourceFinder(long finder, long deleter) async {
    return await nativeCall('setAlgoModelResourceFinder', [finder, deleter]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 检查视频特效证书，设置算法模型路径，并初始化特效模块。
  /// @param licenseFile 证书文件的绝对路径，用于鉴权。
  /// @param algoModelDir 算法模型绝对路径，即存放特效 SDK 所有算法模型的目录。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 鉴权时，会检查 CV 服务端时间和本地设备的时间差异。你必须保证本地系统时间和实际时间一致。
  ///

  FutureOr<int> initCVResource(String licenseFile, String algoModelDir) async {
    return await nativeCall('initCVResource', [licenseFile, algoModelDir]);
  }

  /// @detail api
  /// @author liuyang.bd
  /// @brief 从特效 SDK 获取授权消息，用于获取在线许可证。
  /// @return 授权消息字符串地址, 如果获取失败返回 nullptr。
  /// @note
  ///        - 使用视频特效的功能前，你必须获取特效 SDK 的在线许可证。
  ///        - 通过此接口获取授权消息后，参考 [在线授权说明](https://www.volcengine.com/docs/6705/102012)，自行实现获取在线许可证的业务逻辑。获取许可证后，你必须调用 initCVResource{@link #IVideoEffect#initCVResource} 确认许可证有效。然后，你才可以使用 CV 功能。
  ///

  FutureOr<String> getAuthMessage() async {
    return await nativeCall('getAuthMessage', []);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 开启高级美颜、滤镜等视频特效。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///      - 调用本方法前，必须先调用 initCVResource{@link #IVideoEffect#initCVResource} 进行初始化。
  ///      - 调用该方法后，特效不直接生效，你还需调用 setEffectNodes{@link #IVideoEffect#setEffectNodes} 设置视频特效素材包或调用 setColorFilter{@link #IVideoEffect#setColorFilter} 设置滤镜。
  ///      - 调用 disableVideoEffect{@link #IVideoEffect#disableVideoEffect} 关闭视频特效。
  ///

  FutureOr<int> enableVideoEffect() async {
    return await nativeCall('enableVideoEffect', []);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author zhushufan.ref
  /// @brief 私有接口 <br>
  /// 设置贴纸的特效路径
  /// @param effectNodes 特效素材包路径数。
  /// @return
  ///      - 0: 调用成功。
  ///      - < 0: 调用失败
  ///

  FutureOr<int> applyStickerEffect(String tickerPath) async {
    return await nativeCall('applyStickerEffect', [tickerPath]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 关闭视频特效。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 调用 enableVideoEffect{@link #IVideoEffect#enableVideoEffect} 开启视频特效。
  ///

  FutureOr<int> disableVideoEffect() async {
    return await nativeCall('disableVideoEffect', []);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 设置视频特效素材包。
  /// @param effectNodes 特效素材包绝对路径数组。 <br>
  ///        要取消当前视频特效，将此参数设置为 null。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 调用本方法前，必须先调用 enableVideoEffect{@link #IVideoEffect#enableVideoEffect}。
  ///

  FutureOr<int> setEffectNodes(List<String>? effectNodes) async {
    return await nativeCall('setEffectNodes', [effectNodes]);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author zhushufan.ref
  /// @brief 叠加视频特效素材包。
  /// @param effectNodes 特效素材包路径数组。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 该接口会在 setEffectNodes{@link #IVideoEffect#setEffectNodes} 设置的特效基础上叠加特效。
  ///

  FutureOr<int> appendEffectNodes(List<String> effectNodes) async {
    return await nativeCall('appendEffectNodes', [effectNodes]);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author zhushufan.ref
  /// @brief 移除指定的视频特效资源。
  /// @param effectNodes 特效素材包路径数组。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 移除 setEffectNodes{@link #IVideoEffect#SetEffectNodes} 或 appendEffectNodes{@link #IVideoEffect#appendEffectNodes} 设置的视频特效资源。
  ///

  FutureOr<int> removeEffectNodes(List<String> effectNodes) async {
    return await nativeCall('removeEffectNodes', [effectNodes]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 设置特效强度。
  /// @param effectNode 特效素材包绝对路径，参考[素材包结构说明](https://www.volcengine.com/docs/6705/102039)。
  /// @param key 需要设置的素材 key 名称，参考[素材 key 对应说明](https://www.volcengine.com/docs/6705/102041)。
  /// @param value 特效强度值，取值范围 [0,1]，超出范围时设置无效。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  ///

  FutureOr<int> updateEffectNode(
      String effectNode, String key, float value) async {
    return await nativeCall('updateEffectNode', [effectNode, key, value]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 设置颜色滤镜。
  /// @param filterRes 滤镜资源包绝对路径。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 调用 setColorFilterIntensity{@link #IVideoEffect#setColorFilterIntensity} 设置已启用颜色滤镜的强度。设置强度为 0 时即关闭颜色滤镜。
  ///

  FutureOr<int> setColorFilter(String filterRes) async {
    return await nativeCall('setColorFilter', [filterRes]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 设置已启用颜色滤镜的强度。
  /// @param intensity 滤镜强度。取值范围 [0,1]，超出范围时设置无效。 <br>
  ///                  当设置滤镜强度为 0 时即关闭颜色滤镜。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  ///

  FutureOr<int> setColorFilterIntensity(float intensity) async {
    return await nativeCall('setColorFilterIntensity', [intensity]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 将摄像头采集画面中的人像背景替换为指定图片或纯色背景。
  /// @param backgroundStickerRes 背景贴纸特效素材绝对路径。
  /// @param source 背景贴纸对象，参看 VirtualBackgroundSource{@link #VirtualBackgroundSource}。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///      - 调用本方法前，必须先调用 initCVResource{@link #IVideoEffect#initCVResource} 进行初始化。
  ///      - 调用 disableVirtualBackground{@link #IVideoEffect#disableVirtualBackground} 关闭虚拟背景。
  ///

  FutureOr<int> enableVirtualBackground(
      String modelPath, VirtualBackgroundSource source) async {
    return await nativeCall('enableVirtualBackground', [modelPath, source]);
  }

  /// @detail api
  /// @author wangjunlin.3182
  /// @brief 关闭虚拟背景。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 调用 enableVirtualBackground{@link #IVideoEffect#enableVirtualBackground} 开启虚拟背景后，可以调用此接口关闭虚拟背景。
  ///

  FutureOr<int> disableVirtualBackground() async {
    return await nativeCall('disableVirtualBackground', []);
  }

  /// @detail api
  /// @author wangjunlin.3182
  /// @brief 开启人脸识别功能，并设置人脸检测结果回调观察者。 <br>
  ///        此观察者后，你会周期性收到 onFaceDetectResult{@link #IFaceDetectionObserver#onFaceDetectResult} 回调。
  /// @param observer 人脸检测结果回调观察者，参看 IFaceDetectionObserver{@link #IFaceDetectionObserver}。
  /// @param intervalMs 两次回调之间的最小时间间隔，必须大于 0，单位为毫秒。实际收到回调的时间间隔大于 interval_ms，小于 interval_ms+视频采集帧间隔。
  /// @param faceModelPath 人脸检测算法模型文件路径，一般为 ttfacemodel 文件夹中 tt_face_vXXX.model 文件的绝对路径。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - -1004: 初始化中，初始化完成后启动此功能。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  ///

  FutureOr<int> enableFaceDetection(
      IFaceDetectionObserver observer, int interval, String modelPath) async {
    return await nativeCall(
        'enableFaceDetection', [observer, interval, modelPath]);
  }

  /// @detail api
  /// @author wangjunlin.3182
  /// @brief 关闭人脸识别功能。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  ///

  FutureOr<int> disableFaceDetection() async {
    return await nativeCall('disableFaceDetection', []);
  }
}

class RTSRoom extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.RTSRoom';
  static get codegen_$namespace => _$namespace;

  RTSRoom([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @brief 退出并销毁调用 createRTSRoom{@link #RTCEngine#createRTSRoom} 所创建的房间实例。
  ///

  FutureOr<void> destroy() async {
    return await nativeCall('destroy', []);
  }

  /// @detail api
  /// @brief 加入 RTS 房间。 <br>
  ///        调用 createRTSRoom{@link #RTCEngine#createRTSRoom} 创建房间后，调用此方法加入房间，同房间内其他用户进行音视频通话。
  /// @param token 动态密钥。用于对进房用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///        使用不同 AppID 的 App 是不能互通的。 <br>
  ///        请务必保证生成 Token 使用的 AppID 和创建引擎时使用的 AppID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  /// @return
  ///        - 0：方法调用成功。
  ///        -  0: 成功。触发以下回调：
  ///          - 本端收到房间状态通知 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调。
  ///        - -1：roomID / userInfo.uid 包含了无效的参数。
  ///        - -2：已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom{@link #RTSRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调告知。
  /// @note
  ///       - 同一个 App ID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后进房的用户会将先进房的用户踢出房间，并且先进房的用户会收到 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调通知，错误类型详见 ERROR_CODE_DUPLICATE_LOGIN{@link #ErrorCode#ERROR_CODE_DUPLICATE_LOGIN}。
  ///       - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 onConnectionStateChanged{@link #IRTCEngineEventHandler#onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调通知；如果加入房间的用户是可见用户，远端用户会收到 onUserJoined{@link #IRTSRoomEventHandler#onUserJoined} 回调通知。
  ///

  FutureOr<int> joinRTSRoom(String token, UserInfo userInfo) async {
    return await nativeCall('joinRTSRoom', [token, userInfo]);
  }

  /// @detail api
  /// @brief 更新房间的 Token。 <br>
  ///        收到 onTokenWillExpire{@link #IRTCRoomEventHandler#onTokenWillExpire}，onPublishPrivilegeTokenWillExpire{@link #IRTCRoomEventHandler#onPublishPrivilegeTokenWillExpire}， 或 onSubscribePrivilegeTokenWillExpire{@link #IRTCRoomEventHandler#onSubscribePrivilegeTokenWillExpire} 时，你必须重新获取 Token，并调用此方法更新 Token，以保证通话的正常进行。
  /// @param token 重新获取的有效 Token。 <br>
  ///        如果 Token 无效，你会收到 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged}，错误码是 `-1010`。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ReturnStatus{@link #ReturnStatus}。
  /// @note
  ///      - 请勿同时调用 updateToken{@link #RTSRoom#updateToken} 和 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 方法更新 Token。若因 Token 过期或无效导致加入房间失败或已被移出房间，你应该在获取新的有效 Token 后调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 重新加入房间。
  ///

  FutureOr<int> updateToken(String token) async {
    return await nativeCall('updateToken', [token]);
  }

  /// @detail api
  /// @brief 离开 RTS 房间。 <br>
  ///        调用此方法结束通话过程，并释放所有通话相关的资源。
  /// @return
  ///        - 0: 调用成功。如果用户是房间内可见用户，触发以下回调：
  ///            - 远端用户收到 onUserLeave{@link #IRTSRoomEventHandler#onUserLeave} 回调通知。
  ///        - < 0: 调用失败，参看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 加入房间后，必须调用此方法结束通话，否则无法开始下一次通话。
  ///       - 此方法是异步操作，调用返回时并没有真正退出房间。真正退出房间后，本地会收到 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调通知。你必须在收到 onRoomStateChanged{@link #IRTSRoomEventHandler#onRoomStateChanged} 回调后，再销毁房间或引擎，或调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 再次加入房间。
  ///       - 调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 将自身设为可见的用户离开房间后，房间内其他用户会收到 onUserLeave{@link #IRTSRoomEventHandler#onUserLeave} 回调通知。
  ///

  FutureOr<int> leaveRoom() async {
    return await nativeCall('leaveRoom', []);
  }

  /// @detail api
  /// @brief 通过设置 RTSRoom{@link #RTSRoom} 对象的事件句柄，监听此对象对应的回调事件。
  /// @param rtcRoomEventHandler 参看 IRTSRoomEventHandler{@link #IRTSRoomEventHandler}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  ///

  FutureOr<int> setRTSRoomEventHandler(
      IRTSRoomEventHandler rtcRoomEventHandler) async {
    return await nativeCall('setRTSRoomEventHandler', [rtcRoomEventHandler]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点文本消息（P2P）。
  /// @param userId 消息接收用户的 ID
  /// @param messageStr 发送的文本消息内容。消息不超过 64 KB。
  /// @param config 消息发送的可靠/有序类型，参看 MessageConfig{@link #MessageConfig}
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增
  ///        - -1：发送失败，RTCRoom 实例未创建
  ///        - -2：发送失败，uid 为空
  /// @note
  ///      - 在发送房间内文本消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///      - 调用后，会收到 onUserMessageSendResult{@link #IRTSRoomEventHandler#onUserMessageSendResult} 回调，通知消息发送成功或失败；
  ///      - 若消息发送成功，则 userId 所指定的用户会收到 onUserMessageReceived{@link #IRTSRoomEventHandler#onUserMessageReceived} 回调。
  ///

  FutureOr<long> sendUserMessage(
      String userId, String messageStr, MessageConfig config) async {
    return await nativeCall(
        'sendUserMessage', [userId, messageStr, config.$value]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点二进制消息（P2P）。
  /// @param userId 消息接收用户的 ID
  /// @param buffer 发送的二进制消息内容。消息不超过 64KB。
  /// @param config 消息发送的可靠/有序类型，参看 MessageConfig{@link #MessageConfig}。
  /// @note
  ///      - 在发送房间内二进制消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///      - 调用后，会收到 onUserMessageSendResult{@link #IRTSRoomEventHandler#onUserMessageSendResult} 回调，通知消息发送成功或失败；
  ///      - 若消息发送成功，则 userId 所指定的用户会收到 onUserBinaryMessageReceived{@link #IRTSRoomEventHandler#onUserBinaryMessageReceived} 回调。
  ///

  FutureOr<long> sendUserBinaryMessage(
      String userId, ArrayBuffer buffer, MessageConfig config) async {
    return await nativeCall(
        'sendUserBinaryMessage', [userId, buffer, config.$value]);
  }

  /// @detail api
  /// @brief 给房间内的所有其他用户群发文本消息。
  /// @param messageStr 发送的文本消息内容，消息不超过 64 KB。
  /// @note
  ///       - 在房间内广播文本消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///       - 调用后，会收到 onRoomMessageSendResult{@link #IRTSRoomEventHandler#onRoomMessageSendResult} 回调；
  ///       - 同一房间内的其他用户会收到 onRoomMessageReceived{@link #IRTSRoomEventHandler#onRoomMessageReceived} 回调。
  ///

  FutureOr<long> sendRoomMessage(String messageStr) async {
    return await nativeCall('sendRoomMessage', [messageStr]);
  }

  /// @detail api
  /// @brief 给房间内的所有其他用户群发二进制消息。
  /// @param buffer 发送的二进制消息内容，消息不超过 64KB。
  /// @note
  ///       - 在房间内广播二进制消息前，必须先调用 joinRTSRoom{@link #RTSRoom#joinRTSRoom} 加入房间。
  ///       - 调用后，会收到 onRoomMessageSendResult{@link #IRTSRoomEventHandler#onRoomMessageSendResult} 回调；
  ///       - 同一房间内的其他用户会收到 onRoomBinaryMessageReceived{@link #IRTSRoomEventHandler#onRoomBinaryMessageReceived} 回调。
  ///

  FutureOr<long> sendRoomBinaryMessage(ArrayBuffer buffer) async {
    return await nativeCall('sendRoomBinaryMessage', [buffer]);
  }
}

class ISpatialAudio extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.audio.ISpatialAudio';
  static get codegen_$namespace => _$namespace;

  ISpatialAudio([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 开启/关闭空间音频功能。
  /// @param enable 是否开启空间音频功能： <br>
  ///        - true：开启
  ///        - false：关闭（默认）
  /// @note 该方法仅开启空间音频功能，你须调用 updateSelfPosition{@link #ISpatialAudio#updateSelfPosition} 设置自身位置坐标后方可收听空间音频效果。空间音频相关 API 和调用时序详见[空间音频](https://www.volcengine.com/docs/6348/93903)。
  ///

  FutureOr<void> enableSpatialAudio(boolean enable) async {
    return await nativeCall('enableSpatialAudio', [enable]);
  }

  /// @detail api
  /// @author luomingkang.264
  /// @brief 关闭本地用户朝向对本地用户发声效果的影响。 <br>
  ///        调用此接口后，房间内的其他用户收听本地发声时，声源都在收听者正面。
  /// @note
  ///        - 调用本接口关闭朝向功能后，在当前的空间音频实例的生命周期内无法再次开启。
  ///        - 调用此接口不影响本地用户收听朝向的音频效果。要改变本地用户收听朝向，参看 updateSelfPosition{@link #ISpatialAudio#updateSelfPosition} 和 updateRemotePosition{@link #ISpatialAudio#updateRemotePosition} 。
  ///

  FutureOr<void> disableRemoteOrientation() async {
    return await nativeCall('disableRemoteOrientation', []);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置本地用户在自建空间直角坐标系中的收听坐标和收听朝向，以实现本地用户预期的空间音频收听效果。
  /// @param positionInfo 空间音频位置信息。参看 PositionInfo{@link #PositionInfo}。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。
  ///        - -2: 失败，原因是校验本地用户的三维朝向信息时，三个向量没有两两垂直。
  /// @note
  ///        - 该方法需在进房后调用。
  ///        - 调用该接口更新坐标前，你需调用 enableSpatialAudio{@link #ISpatialAudio#enableSpatialAudio} 开启空间音频功能。空间音频相关 API 和调用时序详见[空间音频](https://www.volcengine.com/docs/6348/93903)。
  ///        - 调用此接口在本地进行的设定对其他用户的空间音频收听效果不会产生任何影响。
  ///

  FutureOr<int> updateSelfPosition(PositionInfo positionInfo) async {
    return await nativeCall('updateSelfPosition', [positionInfo]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置房间内某一远端用户在本地用户自建的空间音频坐标系中的发声位置和发声朝向，以实现本地用户预期的空间音频收听效果。
  /// @param uid 用户 ID
  /// @param positionInfo 远端用户的空间音频位置信息。参看 PositionInfo{@link #PositionInfo}。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。
  ///        - -2: 失败，原因是校验远端用户的三维朝向信息时，三个向量没有两两垂直。
  /// @note
  ///        - 该方法需在创建房间后调用。
  ///        - 调用此接口在本地进行的设定对其他用户的空间音频收听效果不会产生任何影响。
  ///

  FutureOr<int> updateRemotePosition(
      String uid, PositionInfo positionInfo) async {
    return await nativeCall('updateRemotePosition', [uid, positionInfo]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 移除调用 updateRemotePosition{@link #ISpatialAudio#updateRemotePosition} 为某一远端用户设置的空间音频效果。
  /// @param uid 远端用户 ID。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。
  ///

  FutureOr<int> removeRemotePosition(String uid) async {
    return await nativeCall('removeRemotePosition', [uid]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 移除调用 updateRemotePosition{@link #ISpatialAudio#updateRemotePosition} 为所有远端用户设置的空间音频效果。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。
  ///

  FutureOr<int> removeAllRemotePosition() async {
    return await nativeCall('removeAllRemotePosition', []);
  }
}
