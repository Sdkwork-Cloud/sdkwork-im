/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';
import 'types.dart';
import 'keytype.dart';
import 'callback.dart';
import 'external.dart';

class ByteRTCVideoEffect extends NativeClass {
  static const _$namespace = r'ByteRTCVideoEffect';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoEffect([NativeClassOptions? options])
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
  /// @brief 从特效 SDK 获取授权消息，用于获取在线许可证。
  /// @param ppmsg 授权消息字符串地址
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///        - 使用视频特效的功能前，你必须获取特效 SDK 的在线许可证。
  ///        - 通过此接口获取授权消息后，参考 [在线授权说明](https://www.volcengine.com/docs/6705/102012)，自行实现获取在线许可证的业务逻辑。获取许可证后，你必须调用 initCVResource:withAlgoModelDir:{@link #ByteRTCVideoEffect#initCVResource:withAlgoModelDir} 确认许可证有效。然后，你才可以使用 CV 功能。

  FutureOr<int> getAuthMessage(NSString ppmsg) async {
    return await nativeCall('getAuthMessage:', [ppmsg]);
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

  FutureOr<int> initCVResource(
      NSString licenseFile, NSString algoModelDir) async {
    return await nativeCall(
        'initCVResource:withAlgoModelDir:', [licenseFile, algoModelDir]);
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
  ///      - 调用本方法前，必须先调用 initCVResource:withAlgoModelDir:{@link #ByteRTCVideoEffect#initCVResource:withAlgoModelDir} 进行初始化。
  ///      - 调用该方法后，特效不直接生效，你还需调用 setEffectNodes:{@link #ByteRTCVideoEffect#setEffectNodes} 设置视频特效素材包或调用 setColorFilter:{@link #ByteRTCVideoEffect#setColorFilter} 设置滤镜。
  ///      - 调用 disableVideoEffect{@link #ByteRTCVideoEffect#disableVideoEffect} 关闭视频特效。

  FutureOr<int> enableVideoEffect() async {
    return await nativeCall('enableVideoEffect', []);
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
  /// @note 调用 enableVideoEffect{@link #ByteRTCVideoEffect#enableVideoEffect} 开启视频特效。

  FutureOr<int> disableVideoEffect() async {
    return await nativeCall('disableVideoEffect', []);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author zhushufan.ref
  /// @brief 返回视频特效句柄。私有接口。

  FutureOr<void> getVideoEffectHandle() async {
    return await nativeCall('getVideoEffectHandle', []);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author zhushufan.ref
  /// @brief 私有接口 <br>
  /// 设置视频特效素材包
  /// @param stickerPath 特效素材包绝对路径。 <br>
  ///        要取消当前视频特效，将此参数设置为 null。
  /// @return
  ///      - 0: 调用成功。
  ///      - 1000: 未集成特效 SDK。
  ///      - 1001: 特效 SDK 不支持该功能。
  ///      - < 0: 调用失败。具体错误码，参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note 在调用这个方法之前，你须先调用 enableVideoEffect{@link #ByteRTCVideoEffect#enableVideoEffect}。

  FutureOr<int> applyStickerEffect(NSString stickerPath) async {
    return await nativeCall('applyStickerEffect:', [stickerPath]);
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
  /// @note 调用本方法前，必须先调用 enableVideoEffect{@link #ByteRTCVideoEffect#enableVideoEffect}。

  FutureOr<int> setEffectNodes(NSArray<NSString>? effectNodes) async {
    return await nativeCall('setEffectNodes:', [effectNodes]);
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
  /// @note 该接口会在 setEffectNodes:{@link #ByteRTCVideoEffect#setEffectNodes} 设置的特效基础上叠加特效。

  FutureOr<int> appendEffectNodes(NSArray<NSString> effectNodes) async {
    return await nativeCall('appendEffectNodes:', [effectNodes]);
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
  /// @note 移除 setEffectNodes:{@link #ByteRTCVideoEffect#setEffectNodes} 或 appendEffectNodes:{@link #ByteRTCVideoEffect#appendEffectNodes} 设置的视频特效资源。

  FutureOr<int> removeEffectNodes(NSArray<NSString> effectNodes) async {
    return await nativeCall('removeEffectNodes:', [effectNodes]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 设置特效强度。
  /// @param node 特效素材包绝对路径，参考[素材包结构说明](https://www.volcengine.com/docs/6705/102039)。
  /// @param key 需要设置的素材 key 名称，参考[素材 key 对应说明](https://www.volcengine.com/docs/6705/102041)。
  /// @param value 特效强度值，取值范围 [0,1]，超出范围时设置无效。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。

  FutureOr<int> updateEffectNode(
      NSString node, NSString key, float value) async {
    return await nativeCall('updateEffectNode:key:value:', [node, key, value]);
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
  /// @note 调用 setColorFilterIntensity:{@link #ByteRTCVideoEffect#setColorFilterIntensity} 设置已启用颜色滤镜的强度。设置强度为 0 时即关闭颜色滤镜。

  FutureOr<int> setColorFilter(NSString filterRes) async {
    return await nativeCall('setColorFilter:', [filterRes]);
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

  FutureOr<int> setColorFilterIntensity(float intensity) async {
    return await nativeCall('setColorFilterIntensity:', [intensity]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 将摄像头采集画面中的人像背景替换为指定图片或纯色背景。
  /// @param backgroundStickerPath 背景贴纸特效素材绝对路径。
  /// @param source 背景贴纸对象，参看 ByteRTCVirtualBackgroundSource{@link #ByteRTCVirtualBackgroundSource}。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///      - 调用本方法前，必须先调用 initCVResource:withAlgoModelDir:{@link #ByteRTCVideoEffect#initCVResource:withAlgoModelDir} 进行初始化。
  ///      - 调用 disableVirtualBackground{@link #ByteRTCVideoEffect#disableVirtualBackground} 关闭虚拟背景。

  FutureOr<int> enableVirtualBackground(NSString backgroundStickerPath,
      ByteRTCVirtualBackgroundSource source) async {
    return await nativeCall(
        'enableVirtualBackground:withSource:', [backgroundStickerPath, source]);
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
  /// @note 调用 enableVirtualBackground:withSource:{@link #ByteRTCVideoEffect#enableVirtualBackground:withSource} 开启虚拟背景后，可以调用此接口关闭虚拟背景。

  FutureOr<int> disableVirtualBackground() async {
    return await nativeCall('disableVirtualBackground', []);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author zhushufan.ref
  /// @brief 开启人像属性检测。
  /// @param config 人像属性检测参数，参看 ByteRTCExpressionDetectConfig{@link #ByteRTCExpressionDetectConfig}。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。

  FutureOr<int> setVideoEffectExpressionDetect(
      ByteRTCExpressionDetectConfig config) async {
    return await nativeCall('setVideoEffectExpressionDetect:', [config]);
  }

  /// @detail api
  /// @author wangjunlin.3182
  /// @brief 开启人脸识别功能，并设置人脸检测结果回调观察者。 <br>
  ///        此观察者后，你会周期性收到 onFaceDetectResult:{@link #ByteRTCFaceDetectionObserver#onFaceDetectResult} 回调。
  /// @param observer 人脸检测结果回调观察者，参看 ByteRTCFaceDetectionObserver{@link #ByteRTCFaceDetectionObserver}。
  /// @param interval 两次回调之间的最小时间间隔，必须大于 0，单位为毫秒。实际收到回调的时间间隔大于 interval，小于 interval+视频采集帧间隔。
  /// @param path 人脸检测算法模型文件路径，一般为 ttfacemodel 文件夹中 tt_face_vXXX.model 文件的绝对路径。
  /// @return
  ///      - 0: 调用成功。
  ///      - –1000: 未集成特效 SDK。
  ///      - –1001: 特效 SDK 不支持该功能。
  ///      - –1002: 特效 SDK 版本不兼容。
  ///      - -1004: 初始化中，初始化完成后启动此功能。
  ///      - < 0: 调用失败，错误码对应具体描述参看 [错误码表](https://www.volcengine.com/docs/6705/102042)。

  FutureOr<int> enableFaceDetection(id<ByteRTCFaceDetectionObserver> observer,
      NSUInteger interval, NSString path) async {
    return await nativeCall('enableFaceDetection:withInterval:withModelPath:',
        [observer, interval, path]);
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

  FutureOr<int> disableFaceDetection() async {
    return await nativeCall('disableFaceDetection', []);
  }
}

class ByteRTCRoom extends NativeClass {
  static const _$namespace = r'ByteRTCRoom';
  static get codegen_$namespace => _$namespace;

  ByteRTCRoom([NativeClassOptions? options])
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

  /// @detail callback
  FutureOr<ByteRTCRoomDelegate?> get delegate async {
    try {
      final result = await sendInstanceGet<ByteRTCRoomDelegate?>("delegate");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCRoomDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegate(FutureOr<ByteRTCRoomDelegate?> value) {
    sendInstanceSet("delegate", value);
  }

  /// @detail callback
  FutureOr<ByteRTCRTSRoomDelegate?> get delegateRts async {
    try {
      final result =
          await sendInstanceGet<ByteRTCRTSRoomDelegate?>("delegateRts");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCRTSRoomDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegateRts(FutureOr<ByteRTCRTSRoomDelegate?> value) {
    sendInstanceSet("delegateRts", value);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 退出并销毁调用 createRTCRoom:{@link #ByteRTCEngine#createRTCRoom} 所创建的房间实例。

  FutureOr<void> destroy() async {
    return await nativeCall('destroy', []);
  }

  /// @valid since 3.53
  /// @detail api
  /// @author gechangwu
  /// @brief 获取房间 ID。
  /// @return 房间 ID。

  FutureOr<NSString> getRoomId() async {
    return await nativeCall('getRoomId', []);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 通过设置 ByteRTCRoom{@link #ByteRTCRoom} 对象的事件句柄，监听此对象对应的回调事件。
  /// @param roomDelegate 参见 ByteRTCRoomDelegate{@link #ByteRTCRoomDelegate}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> setRTCRoomDelegate(id<ByteRTCRoomDelegate> roomDelegate) async {
    return await nativeCall('setRTCRoomDelegate:', [roomDelegate]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 加入 RTC 房间。 <br>
  ///        调用 createRTCRoom:{@link #ByteRTCEngine#createRTCRoom} 创建房间后，调用此方法加入房间，同房间内其他用户进行音视频通话。
  /// @param token 动态密钥，用于对进房用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///        使用不同 AppID 的 App 是不能互通的。 <br>
  ///        请务必保证生成 Token 使用的 AppID 和创建引擎时使用的 AppID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息。参看 ByteRTCUserInfo{@link #ByteRTCUserInfo}。
  /// @param userVisibility 用户可见性。建议在进房时将用户可见性都设置为 `false`，并在用户需要发送音视频流时再通过 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 设置为 `true`。从而避免因房间内用户达到数量上限所导致的进房失败。默认情况下，一个 RTC 房间最多同时容纳 50 名可见用户，其中最多 30 人可同时上麦，更多信息参看[用户和媒体流上限](https://www.volcengine.com/docs/6348/257549)。
  /// @param roomConfig 房间参数配置，设置房间模式以及是否自动发布或订阅流。具体配置模式参看 ByteRTCRoomConfig{@link #ByteRTCRoomConfig}。
  /// @return 方法调用结果。 <br>
  ///        -  0: 成功。触发以下回调：
  ///          - 本端收到房间状态通知 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///          - 本端收到本地流发布状态通知 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason}。
  ///          - 本端收到流订阅状态通知 rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason}、rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason}。
  ///          - 本端收到房间内已发布流的通知 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}、rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish}。
  ///          - 如果本端用户为可见用户，房间内其他用户收到 rtcRoom:onUserJoined:{@link #ByteRTCRoomDelegate#rtcRoom:onUserJoined} 回调通知。
  ///        - -1: 参数无效
  ///        - -2: 已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom{@link #ByteRTCRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调告知。
  /// @note
  ///        - 同一个 AppID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后加入房间的用户会将先加入房间的用户踢出房间，并且先加入房间的用户会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知，错误类型为重复登录 ByteRTCErrorCodeDuplicateLogin。
  ///        - 本地用户调用此方法加入房间成功后，会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知。若本地用户同时为可见用户，加入房间时远端用户会收到 rtcRoom:onUserJoined:{@link #ByteRTCRoomDelegate#rtcRoom:onUserJoined} 回调通知。
  ///        - 房间内不可见用户的容量远远大于可见用户，而且用户默认可见，因此对于不参与互动的用户，你需要调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 更改为不可见用户。从而避免因房间内用户达到数量上限所导致的进房失败。默认情况下，一个 RTC 房间最多同时容纳 50 名可见用户，其中最多 30 人可同时上麦，更多信息参看[用户和媒体流上限](https://www.volcengine.com/docs/6348/257549)。
  ///        - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 rtcEngine:onConnectionStateChanged:{@link #ByteRTCEngineDelegate#rtcEngine:onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo}。

  FutureOr<int> joinRoom(NSString token, ByteRTCUserInfo userInfo,
      BOOL userVisibility, ByteRTCRoomConfig roomConfig) async {
    return await nativeCall('joinRoom:userInfo:userVisibility:roomConfig:',
        [token, userInfo, userVisibility, roomConfig]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 设置用户可见性。未调用该接口前，本地用户默认对他人可见。 <br>
  ///        默认情况下，一个 RTC 房间最多同时容纳 50 名可见用户，最多 30 人可同时上麦。更多信息参看[用户和媒体流上限](https://www.volcengine.com/docs/6348/257549)。
  /// @param enable 设置用户是否对房间内其他用户可见： <br>
  ///        - YES: 可见，用户可以在房间内发布音视频流，房间中的其他用户将收到用户的行为通知，例如进房、开启视频采集和退房。
  ///        - NO: 不可见，用户不可以在房间内发布音视频流，房间中的其他用户不会收到用户的行为通知，例如进房、开启视频采集和退房。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0: 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  ///       设置用户可见性，会收到设置成功/失败回调 rtcRoom:onUserVisibilityChanged:errorCode:{@link #ByteRTCRoomDelegate#rtcRoom:onUserVisibilityChanged:errorCode}。（v3.54 新增）
  ///        - 在加入房间前设置用户可见性，若设置的可见性与默认值不同，将在加入房间时触发本回调。 <br>
  ///        - 在加入房间后设置用户可见性，若可见性前后不同，会触发本回调。 <br>
  ///        - 在断网重连后，若可见性发生改变，会触发本回调。 <br>
  /// @note
  ///       - 在加入房间前后，用户均可调用此方法设置用户可见性。
  ///       - 在房间内，调用此方法成功切换用户可见性后，房间内其他用户会收到相应的回调。
  ///   &#x0020;  • 从可见换至不可见时，房间内其他用户会收到 rtcRoom:onUserLeave:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onUserLeave:reason}。 <br>
  ///   &#x0020;  • 从不可见切换至可见时，房间内其他用户会收到 rtcRoom:onUserJoined:{@link #ByteRTCRoomDelegate#rtcRoom:onUserJoined}。 <br>
  ///   &#x0020;  • 若调用该方法将可见性设为 `false`，此时尝试发布流会收到 `ByteRTCWarningCodeSubscribeStreamForbiden` 警告。

  FutureOr<int> setUserVisibility(BOOL enable) async {
    return await nativeCall('setUserVisibility:', [enable]);
  }

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 设置发流端音画同步。 <br>
  ///        当同一用户同时使用两个通话设备分别采集发送音频和视频时，有可能会因两个设备所处的网络环境不一致而导致发布的流不同步，此时你可以在视频发送端调用该接口，SDK 会根据音频流的时间戳自动校准视频流，以保证接收端听到音频和看到视频在时间上的同步性。
  /// @param audioUserId 音频发送端的用户 ID，将该参数设为空则可解除当前音视频的同步关系。
  /// @return
  ///        - 0: 调用成功。调用该接口后音画同步状态发生改变时，你会收到 rtcRoom:onAVSyncStateChange:{@link #ByteRTCRoomDelegate#rtcRoom:onAVSyncStateChange} 回调。
  ///        - < 0 : 调用失败。监听 rtcRoom:onAVSyncEvent:userId:eventCode:{@link #ByteRTCRoomDelegate#rtcRoom:onAVSyncEvent:userId:eventCode} 获取错误详情。同一 RTC 房间内允许存在多个音视频同步关系，但需注意单个音频源不支持与多个视频源同时同步。
  /// @note
  ///        - 该方法在进房前后均可调用。
  ///        - 进行音画同步的音频发布用户 ID 和视频发布用户 ID 须在同一个 RTC 房间内。
  ///        - 如需更换同步音频源，再次调用该接口传入新的 `audioUserId` 即可；如需更换同步视频源，需先解除当前的同步关系，后在新视频源端开启同步。

  FutureOr<int> setMultiDeviceAVSync(NSString audioUserId) async {
    return await nativeCall('setMultiDeviceAVSync:', [audioUserId]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 离开 RTC 房间。 <br>
  ///        调用此方法结束通话过程，并释放所有通话相关的资源。
  /// @return
  ///        - 0: 调用成功。如果用户是房间内可见用户，触发以下回调：
  ///            - 远端用户收到 rtcRoom:onUserLeave:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onUserLeave:reason} 回调通知。
  ///            - 正在发布的流会被取消发布。远端用户收到 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason} 回调通知。
  ///        - < 0: 调用失败，参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 加入房间后，必须调用此方法结束通话，否则无法开始下一次通话。
  ///       - 此方法是异步操作，调用返回时并没有真正退出房间。真正退出房间后，本地会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知。你必须在收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调后，再销毁房间或引擎，或调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 再次加入房间。
  ///       - 调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 将自身设为可见的用户离开房间后，房间内其他用户会收到 rtcRoom:onUserLeave:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onUserLeave:reason} 回调通知。

  FutureOr<int> leaveRoom() async {
    return await nativeCall('leaveRoom', []);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `publishStream` 和 `unpublishStream` 方法来实现下述功能。如果你如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此接口。
  /// @author xuyiling.x10
  /// @brief 手动发布/取消发布本地摄像头采集的视频流。
  /// @param publish 指定是否发布本地摄像头采集的视频流。<br>
  ///        - true: 发布。
  ///        - false: 取消发布。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果你已经在用户进房时通过调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 成功选择了自动发布，则无需再调用本接口。
  ///        - 调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 方法将自身设置为不可见后无法调用该方法，需将自身切换至可见后方可调用该方法发布摄像头视频流。
  ///        - 如果你需要发布麦克风采集到的音频流，调用 publishStreamAudio:{@link #ByteRTCRoom#publishStreamAudio}。
  ///        - 如果你需要向多个房间发布流，调用 startForwardStreamToRooms:{@link #ByteRTCRoom#startForwardStreamToRooms}。
  ///        - 调用此方法后，房间中的所有远端用户会收到 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish} 回调通知，订阅了视频流的远端用户会收到 rtcEngine:onFirstRemoteVideoFrameDecoded:info:withFrameInfo:{@link #ByteRTCEngineDelegate#rtcEngine:onFirstRemoteVideoFrameDecoded:info:withFrameInfo} 回调。

  FutureOr<int> publishStreamVideo(BOOL publish) async {
    return await nativeCall('publishStreamVideo:', [publish]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `publishStream` 和 `unpublishStream` 方法来实现下述功能。如果你如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此接口。
  /// @author xuyiling.x10
  /// @brief 发布/取消发布本地麦克风采集的音频流。
  /// @param publish 指定是否发布音频流。<br>
  ///        - true: 发布。
  ///        - false: 取消发布。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果你已经在用户进房时通过调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 成功选择了自动发布，则无需再调用本接口。
  ///        - 调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 方法将自身设置为不可见后无法调用该方法，需将自身切换至可见后方可调用该方法发布音频流。
  ///        - 如果你需要发布摄像头采集到的视频流，调用 publishStreamVideo:{@link #ByteRTCRoom#publishStreamVideo}。
  ///        - 如果你需要向多个房间发布流，调用 startForwardStreamToRooms:{@link #ByteRTCRoom#startForwardStreamToRooms}。
  ///        - 调用此方法后，房间中的所有远端用户会收到 rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish} 回调通知，其中成功收到了音频流的远端用户会收到 rtcEngine:onFirstRemoteAudioFrame:info:{@link #ByteRTCEngineDelegate#rtcEngine:onFirstRemoteAudioFrame:info} 回调。
  /// @order 0
  ///

  FutureOr<int> publishStreamAudio(BOOL publish) async {
    return await nativeCall('publishStreamAudio:', [publish]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeStream` 和 `unsubscribeStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用这两个方法，请迁移至该接口。
  /// @author xuyiling.x10
  /// @brief 订阅/取消订阅房间内指定的远端视频流（通过摄像头采集的）。
  /// @param streamId 目标远端视频流 ID。
  /// @param subscribe 指定是否订阅该视频流。<br>
  ///        - true: 订阅。
  ///        - false: 取消订阅。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 若当前用户在调用本接口时已经订阅该远端用户（手动订阅或自动订阅），则将根据本次传入的参数，更新订阅配置。
  ///        - 你必须先通过 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish} 回调获取当前房间里的远端摄像头流信息，然后调用本方法按需订阅。
  ///        - 调用该方法后，你会收到 rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason} 通知方法调用结果。
  ///        - 成功订阅远端用户的媒体流后，订阅关系将持续到调用本方法取消订阅或本端用户退房。
  ///        - 关于其他调用异常，你会收到 rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason} 回调通知，具体异常原因参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 3
  ///

  FutureOr<int> subscribeStreamVideo(NSString streamId, BOOL subscribe) async {
    return await nativeCall(
        'subscribeStreamVideo:subscribe:', [streamId, subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeStream` 和 `unsubscribeStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用这两个方法，请迁移至该接口。
  /// @author xuyiling.x10
  /// @brief 订阅/取消订阅房间内指定的远端音频流（通过麦克风采集的）。
  /// @param streamId 目标远端音频流 ID。
  /// @param subscribe 指定是否订阅该音频流。<br>
  ///        - true: 订阅。
  ///        - false: 取消订阅。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 若当前用户在调用本接口时已经订阅该远端用户（手动订阅或自动订阅），则将根据本次传入的参数，更新订阅配置。
  ///        - 你必须先通过 rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish} 回调获取当前房间里的远端麦克风流信息，然后调用本方法按需订阅。
  ///        - 调用该方法后，你会收到 rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason} 通知方法调用结果。
  ///        - 成功订阅远端用户的媒体流后，订阅关系将持续到调用 subscribeStreamAudio:subscribe:{@link #ByteRTCRoom#subscribeStreamAudio:subscribe} 取消订阅或本端用户退房。
  ///        - 关于其他调用异常，你会收到 rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason} 回调通知，具体异常原因参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 3
  ///

  FutureOr<int> subscribeStreamAudio(NSString streamId, BOOL subscribe) async {
    return await nativeCall(
        'subscribeStreamAudio:subscribe:', [streamId, subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeAllStreams` 和 `unsubscribeAllStreams` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用这两个方法，请迁移至该接口。
  /// @author yejing.luna
  /// @brief 订阅/取消订阅房间内所有远端视频流（通过摄像头采集的）。
  /// @param subscribe  是否订阅所有远端视频流。<br>
  ///        - true: 订阅。
  ///        - false: 取消订阅。
  /// @return
  ///        0: 方法调用成功 <br>
  ///        !0: 方法调用失败
  /// @note
  ///        - 多次调用订阅接口时，将根据末次调用接口和传入的参数，更新订阅配置。
  ///        - 开启音频选路后，如果房间内的媒体流超过上限，建议通过调用 subscribeStreamVideo:subscribe:{@link #ByteRTCRoom#subscribeStreamVideo:subscribe} 接口逐一指定需要订阅的媒体流。
  ///        - 调用该方法后，你会收到 rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason} 通知方法调用结果。
  ///        - 成功调用本接口后，订阅关系将持续到调用 subscribeAllStreamsVideo:{@link #ByteRTCRoom#subscribeAllStreamsVideo}  取消订阅或本端用户退房。
  ///        - 关于其他调用异常，你会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知，具体异常原因参看 ByteRTCErrorCode{@link #ByteRTCErrorCode}。

  FutureOr<int> subscribeAllStreamsVideo(BOOL subscribe) async {
    return await nativeCall('subscribeAllStreamsVideo:', [subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `subscribeAllStreams` 和 `unsubscribeAllStreams` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用这两个方法，请迁移至该接口。
  /// @author yejing.luna
  /// @brief 订阅或取消订阅所有远端音频流（通过麦克风采集的）。
  /// @param subscribe 是否订阅所有远端音频流。<br>
  ///        - true: 订阅。
  ///        - false: 取消订阅。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 多次调用订阅接口时，将根据末次调用接口和传入的参数，更新订阅配置。
  ///        - 开启音频选路后，如果房间内的媒体流超过上限，建议通过调用 subscribeStreamAudio:subscribe:{@link #ByteRTCRoom#subscribeStreamAudio:subscribe} 接口逐一指定需要订阅的媒体流。
  ///        - 调用该方法后，你会收到 rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason} 通知方法调用结果。
  ///        - 成功订阅远端用户的媒体流后，订阅关系将持续到调用 subscribeStreamAudio:subscribe:{@link #ByteRTCRoom#subscribeStreamAudio:subscribe} 取消订阅或本端用户退房。
  ///        - 关于其他调用异常，你会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知，具体异常原因参看 ByteRTCErrorCode{@link #ByteRTCErrorCode}。

  FutureOr<int> subscribeAllStreamsAudio(BOOL subscribe) async {
    return await nativeCall('subscribeAllStreamsAudio:', [subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `resumeAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @brief 暂停接收所有远端视频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该方法仅暂停远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。
  ///        - 若想恢复接收远端流，需调用 resumeAllSubscribedStreamVideo{@link #ByteRTCRoom#resumeAllSubscribedStreamVideo}。
  ///        - 多房间场景下，仅暂停接收发布在当前所在房间的流。

  FutureOr<int> pauseAllSubscribedStreamVideo() async {
    return await nativeCall('pauseAllSubscribedStreamVideo', []);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `pauseAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @author shenpengliang
  /// @brief 暂停接收所有远端音频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅暂停远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。
  ///        - 若想恢复接收远端流，需调用 resumeAllSubscribedStreamVideo{@link #ByteRTCRoom#resumeAllSubscribedStreamVideo}。
  ///        - 多房间场景下，仅暂停接收发布在当前所在房间的流。

  FutureOr<int> pauseAllSubscribedStreamAudio() async {
    return await nativeCall('pauseAllSubscribedStreamAudio', []);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `resumeAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @author shenpengliang
  /// @brief 恢复接收所有远端音频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该方法仅恢复远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。

  FutureOr<int> resumeAllSubscribedStreamVideo() async {
    return await nativeCall('resumeAllSubscribedStreamVideo', []);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `resumeAllSubscribedStream` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，且仍在使用该方法，请迁移至该接口。
  /// @author shenpengliang
  /// @brief 恢复接收所有远端音频流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅恢复远端流的接收，并不影响远端流的采集和发送；
  ///        - 该方法不改变用户的订阅状态以及订阅流的属性。

  FutureOr<int> resumeAllSubscribedStreamAudio() async {
    return await nativeCall('resumeAllSubscribedStreamAudio', []);
  }

  /// @detail api
  /// @brief 给房间内的所有其他用户群发文本消息。
  /// @param message <br>
  ///        发送的文本消息内容。 <br>
  ///        消息不超过 64 KB。
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note
  ///      - 在发送房间内文本消息前，必须先调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 加入房间。
  ///      - 调用该函数后会收到一次 rtcRoom:onRoomMessageSendResult:error:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomMessageSendResult:error} 回调，通知消息发送方发送成功或失败；
  ///      - 若文本消息发送成功，则房间内所有远端用户会收到 rtcRoom:onRoomMessageReceived:message:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomMessageReceived:message} 回调。
  ///

  FutureOr<NSInteger> sendRoomMessage(NSString message) async {
    return await nativeCall('sendRoomMessage:', [message]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 给房间内的所有其他用户群发二进制消息。
  /// @param message <br>
  ///        用户发送的二进制广播消息 <br>
  ///        消息不超过 64KB。
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note
  ///      - 在房间内广播二进制消息前，必须先调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 加入房间。
  ///      - 调用该函数后会收到一次 rtcRoom:onRoomMessageSendResult:error:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomMessageSendResult:error} 回调，通知消息发送方发送成功或失败；
  ///      - 若二进制消息发送成功，则房间内所有用户会收到 rtcRoom:onRoomBinaryMessageReceived:message:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomBinaryMessageReceived:message} 回调。
  ///

  FutureOr<NSInteger> sendRoomBinaryMessage(NSData message) async {
    return await nativeCall('sendRoomBinaryMessage:', [message]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 开始跨房间转发媒体流。 <br>
  ///        在调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 后调用本接口，实现向多个房间转发媒体流，适用于跨房间连麦等场景。
  /// @param configurations 跨房间媒体流转发指定房间的信息。参看 ByteRTCForwardStreamConfiguration{@link #ByteRTCForwardStreamConfiguration}。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 调用本方法后，将在本端触发 rtcRoom:onForwardStreamStateChanged:{@link #ByteRTCRoomDelegate#rtcRoom:onForwardStreamStateChanged} 回调。
  ///        - 调用本方法后，你可以通过监听 rtcRoom:onForwardStreamEvent:{@link #ByteRTCRoomDelegate#rtcRoom:onForwardStreamEvent} 回调来获取各个目标房间在转发媒体流过程中的相关事件。
  ///        - 开始转发后，目标房间中的用户将接收到本地用户进房 rtcRoom:onUserJoined:{@link #ByteRTCRoomDelegate#rtcRoom:onUserJoined} 和发流 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}、rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish} 回调。
  ///        - 调用本方法后，可以调用 updateForwardStreamToRooms:{@link #ByteRTCRoom#updateForwardStreamToRooms} 更新目标房间信息，例如，增加或减少目标房间等。
  ///        - 调用本方法后，可以调用 stopForwardStreamToRooms{@link #ByteRTCRoom#stopForwardStreamToRooms} 停止向所有房间转发媒体流。
  ///        - 调用本方法后，可以调用 pauseForwardStreamToAllRooms{@link #ByteRTCRoom#pauseForwardStreamToAllRooms} 暂停向所有房间转发媒体流。
  ///

  FutureOr<int> startForwardStreamToRooms(
      NSArray<ByteRTCForwardStreamConfiguration> configurations) async {
    return await nativeCall('startForwardStreamToRooms:', [configurations]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 更新跨房间媒体流转发信息。 <br>
  ///        通过 startForwardStreamToRooms:{@link #ByteRTCRoom#startForwardStreamToRooms} 发起媒体流转发后，可调用本方法增加或者减少目标房间，或更新房间密钥。 <br>
  ///        调用本方法增加或删减房间后，将在本端触发 rtcRoom:onForwardStreamStateChanged:{@link #ByteRTCRoomDelegate#rtcRoom:onForwardStreamStateChanged} 回调，包含发生了变动的目标房间中媒体流转发状态。
  /// @param configurations 跨房间媒体流转发目标房间信息。参看 ByteRTCForwardStreamConfiguration{@link #ByteRTCForwardStreamConfiguration}。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        增加或删减目标房间后，新增目标房间中的用户将接收到本地用户进房 rtcRoom:onUserJoined:{@link #ByteRTCRoomDelegate#rtcRoom:onUserJoined} 和发布 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}、rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish} 回调。
  ///

  FutureOr<int> updateForwardStreamToRooms(
      NSArray<ByteRTCForwardStreamConfiguration> configurations) async {
    return await nativeCall('updateForwardStreamToRooms:', [configurations]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 停止跨房间媒体流转发。 <br>
  ///        通过 startForwardStreamToRooms:{@link #ByteRTCRoom#startForwardStreamToRooms} 发起媒体流转发后，可调用本方法停止向所有目标房间转发媒体流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 调用本方法后，将在本端触发 rtcRoom:onForwardStreamStateChanged:{@link #ByteRTCRoomDelegate#rtcRoom:onForwardStreamStateChanged} 回调。
  ///        - 调用本方法后，原目标房间中的用户将接收到本地用户停止发布 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}、rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish} 回调和退房 rtcRoom:onUserLeave:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onUserLeave:reason} 的回调。
  ///        - 如果需要更改目标房间，请调用 updateForwardStreamToRooms:{@link #ByteRTCRoom#updateForwardStreamToRooms} 更新房间信息。
  ///        - 如果需要暂停转发，请调用 pauseForwardStreamToAllRooms{@link #ByteRTCRoom#pauseForwardStreamToAllRooms}，并在之后随时调用 resumeForwardStreamToAllRooms{@link #ByteRTCRoom#resumeForwardStreamToAllRooms} 快速恢复转发。

  FutureOr<int> stopForwardStreamToRooms() async {
    return await nativeCall('stopForwardStreamToRooms', []);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 暂停跨房间媒体流转发。 <br>
  ///        通过 startForwardStreamToRooms:{@link #ByteRTCRoom#startForwardStreamToRooms} 发起媒体流转发后，可调用本方法暂停向所有目标房间转发媒体流。 <br>
  ///        调用本方法暂停向所有目标房间转发后，你可以随时调用 resumeForwardStreamToAllRooms{@link #ByteRTCRoom#resumeForwardStreamToAllRooms} 快速恢复转发。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note 调用本方法后，目标房间中的用户将接收到本地用户停止发布 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}、rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish} 回调和退房 rtcRoom:onUserLeave:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onUserLeave:reason} 的回调。
  /// @order 13

  FutureOr<int> pauseForwardStreamToAllRooms() async {
    return await nativeCall('pauseForwardStreamToAllRooms', []);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 恢复跨房间媒体流转发。 <br>
  ///        调用 pauseForwardStreamToAllRooms{@link #ByteRTCRoom#pauseForwardStreamToAllRooms} 暂停转发之后，调用本方法恢复向所有目标房间转发媒体流。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        目标房间中的用户将接收到本地用户进房 rtcRoom:onUserJoined:{@link #ByteRTCRoomDelegate#rtcRoom:onUserJoined} 和发布 rtcRoom:onUserJoined:{@link #ByteRTCRoomDelegate#rtcRoom:onUserJoined} 的回调。

  FutureOr<int> resumeForwardStreamToAllRooms() async {
    return await nativeCall('resumeForwardStreamToAllRooms', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 获取范围语音接口实例。
  /// @return 方法调用结果： <br>
  ///        - ByteRTCRangeAudio：成功，返回一个 ByteRTCRangeAudio{@link #ByteRTCRangeAudio} 实例。
  ///        - NULL：失败，当前 SDK 不支持范围语音功能。
  /// @note 首次调用该方法须在创建房间后、加入房间前。范围语音相关 API 和调用时序详见[范围语音](https://www.volcengine.com/docs/6348/114727)。

  FutureOr<ByteRTCRangeAudio> getRangeAudio() async {
    final result = await nativeCall('getRangeAudio', []);
    return packObject(
        result,
        () =>
            ByteRTCRangeAudio(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 获取空间音频接口实例。
  /// @return 方法调用结果： <br>
  ///        - ByteRTCSpatialAudio：成功，返回一个 ByteRTCSpatialAudio{@link #ByteRTCSpatialAudio} 实例。
  ///        - NULL：失败，当前 SDK 不支持空间音频功能。
  /// @note
  ///        - 首次调用该方法须在创建房间后、加入房间前。 空间音频相关 API 和调用时序详见[空间音频](https://www.volcengine.com/docs/6348/93903)。
  ///        - 只有在使用支持真双声道播放的设备时，才能开启空间音频效果；
  ///        - 机型性能不足可能会导致音频卡顿，使用低端机时，不建议开启空间音频效果；
  ///        - SDK 最多支持 30 个用户同时开启空间音频功能。

  FutureOr<ByteRTCSpatialAudio> getSpatialAudio() async {
    final result = await nativeCall('getSpatialAudio', []);
    return packObject(
        result,
        () => ByteRTCSpatialAudio(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author zhangcaining
  /// @brief 调节某个房间内所有远端用户的音频播放音量。
  /// @param volume 音频播放音量值和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///              - 0: 静音
  ///              - 100: 原始音量，默认值
  ///              - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内， <br>
  ///        - 该方法与 setRemoteAudioPlaybackVolume:volume:{@link #ByteRTCEngine#setRemoteAudioPlaybackVolume:volume} 互斥，最新调用的任一方法设置的音量将覆盖此前已设置的音量，效果不叠加；
  ///        - 当该方法与 setPlaybackVolume:{@link #ByteRTCEngine#setPlaybackVolume} 方法共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。

  FutureOr<int> setRemoteRoomAudioPlaybackVolume(NSInteger volume) async {
    return await nativeCall('setRemoteRoomAudioPlaybackVolume:', [volume]);
  }

  /// @valid since 3.52.
  /// @detail api
  /// @author yejing.luna
  /// @brief 设置本端发布流在音频选路中的优先级。
  /// @param audioSelectionPriority 本端发布流在音频选路中的优先级，默认正常参与音频选路。参见 ByteRTCAudioSelectionPriority{@link #ByteRTCAudioSelectionPriority}。
  /// @note
  /// 在控制台上为本 appId 开启音频选路后，调用本接口才会生效。进房前后调用均可生效。更多信息参见[音频选路](https://www.volcengine.com/docs/6348/113547)。 <br>
  /// 如果本端用户同时加入不同房间，使用本接口进行的设置相互独立。

  FutureOr<int> setAudioSelectionConfig(
      ByteRTCAudioSelectionPriority audioSelectionPriority) async {
    return await nativeCall(
        'setAudioSelectionConfig:', [audioSelectionPriority.$value]);
  }

  /// @valid since 3.52.
  /// @detail api
  /// @author lichangfeng.rtc
  /// @brief 设置/更新房间附加信息，可用于标识房间状态或属性，或灵活实现各种业务逻辑。
  /// @param key 房间附加信息键值，长度小于 10 字节。 <br>
  ///        同一房间内最多可存在 5 个 key，超出则会从第一个 key 起进行替换。
  /// @param value 房间附加信息内容，长度小于 128 字节。
  /// @return
  ///        - 0: 方法调用成功，返回本次调用的任务编号；
  ///        - <0: 方法调用失败，具体原因详见 ByteRTCSetRoomExtraInfoResult{@link #ByteRTCSetRoomExtraInfoResult}。
  /// @note
  ///       - 在设置房间附加信息前，必须先调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 加入房间。
  ///       - 调用该方法后，会收到一次 rtcRoom:onSetRoomExtraInfoResult:result:{@link #ByteRTCRoomDelegate#rtcRoom:onSetRoomExtraInfoResult:result} 回调，提示设置结果。
  ///       - 调用该方法成功设置附加信息后，同一房间内的其他用户会收到关于该信息的回调 rtcRoom:onRoomExtraInfoUpdate:value:lastUpdateUserId:lastUpdateTimeMs:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomExtraInfoUpdate:value:lastUpdateUserId:lastUpdateTimeMs}。
  ///       - 新进房的用户会收到进房前房间内已有的全部附加信息通知。

  FutureOr<NSInteger> setRoomExtraInfo(NSString key, NSString value) async {
    return await nativeCall('setRoomExtraInfo:value:', [key, value]);
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

  FutureOr<NSInteger> setStreamExtraInfo(NSString extra_info) async {
    return await nativeCall('setStreamExtraInfo:', [extra_info]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author qiaoxingwang
  /// @brief 识别或翻译房间内所有用户的语音，形成字幕。 <br>
  ///        调用该方法时，可以在 ByteRTCSubtitleMode{@link #ByteRTCSubtitleMode} 中选择语音识别或翻译模式。如果选择识别模式，语音识别文本会通过 rtcRoom:onSubtitleMessageReceived:{@link #ByteRTCRoomDelegate#rtcRoom:onSubtitleMessageReceived} 事件回调给你； <br>
  ///        如果选择翻译模式，你会同时收到两个 rtcRoom:onSubtitleMessageReceived:{@link #ByteRTCRoomDelegate#rtcRoom:onSubtitleMessageReceived} 回调，分别包含字幕原文及字幕译文。 <br>
  ///        调用该方法后，用户会收到 rtcRoom:onSubtitleStateChanged:errorCode:errorMessage:{@link #ByteRTCRoomDelegate#rtcRoom:onSubtitleStateChanged:errorCode:errorMessage} 回调，通知字幕是否开启。
  /// @param subtitleConfig 字幕配置信息。参看 ByteRTCSubtitleConfig{@link #ByteRTCSubtitleConfig}。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 使用字幕功能前，你需要在 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle) 开启实时字幕功能。
  ///        - 如果你需要使用流式语音识别模式，你应在 [语音技术控制台](https://console.volcengine.com/speech/service/16) 创建流式语音识别应用。创建时，服务类型应选择 `流式语音识别`，而非 `音视频字幕生成`。创建后，在 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle) 上启动流式语音识别，并填写创建语音技术应用时获取的相关信息，包括：APP ID，Access Token，和 Cluster ID。
  ///        - 如果你需要使用实时语音翻译模式，你应开通机器翻译服务，参考 [开通服务](https://www.volcengine.com/docs/4640/130262)。完成开通后，在 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle) 上启用实时语音翻译模式。<br> *        + 此方法需要在进房后调用。
  ///        - 如需指定源语言，你需要在调用 `joinRoom` 接口进房时，通过 extraInfo 参数传入格式为`"语种英文名": "语种代号"` JSON 字符串，例如设置源语言为英文时，传入 `"source_language": "en"`。如未指定源语言，SDK 会将系统语种设定为源语言。如果你的系统语种不是中文、英文和日文，此时 SDK 会自动将中文设为源语言。
  ///          - 识别模式下，你可以传入 [RTC 控制台](https://console.volcengine.com/rtc/cloudRTC?tab=subtitle)上预设或自定义的语种英文名和语种代号。识别模式下支持的语言参看[识别模式语种支持](https://www.volcengine.com/docs/6561/109880#场景-语种支持)。
  ///          - 翻译模式下，你需要传入机器翻译规定的语种英文名和语种代号。翻译模式下支持的语言及对应的代号参看[翻译模式语言支持](https://www.volcengine.com/docs/4640/35107)。

  FutureOr<int> startSubtitle(ByteRTCSubtitleConfig subtitleConfig) async {
    return await nativeCall('startSubtitle:', [subtitleConfig]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author qiaoxingwang
  /// @brief 关闭字幕。 <br>
  ///        调用该方法后，用户会收到 rtcRoom:onSubtitleStateChanged:errorCode:errorMessage:{@link #ByteRTCRoomDelegate#rtcRoom:onSubtitleStateChanged:errorCode:errorMessage}  回调，通知字幕是否关闭。
  /// @return
  ///        -  0: 调用成功。
  ///        - !0: 调用失败。

  FutureOr<int> stopSubtitle() async {
    return await nativeCall('stopSubtitle', []);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @author zhoubohui
  /// @brief 设置期望订阅的远端视频流类型。
  /// @param streamId 目标要订阅的远端视频流 ID。
  /// @param streamType 远端视频流类型，参看 ByteRTCVideoSimulcastStreamType{@link #ByteRTCVideoSimulcastStreamType}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。
  /// @note
  ///        - 该方法仅在发布端调用 setVideoEncoderConfig:withParameters:{@link #ByteRTCEngine#setVideoEncoderConfig:withParameters} 开启了发送多路视频流的情况下生效。
  ///        - 若发布端开启了推送多路流功能，但订阅端不对流参数进行设置，则默认接受发送端设置的分辨率最大的一路视频流。该方法可在进房后调用。
  /// @order 1
  ///

  FutureOr<int> setRemoteSimulcastStreamType(
      NSString streamId, ByteRTCVideoSimulcastStreamType streamType) async {
    return await nativeCall('setRemoteSimulcastStreamType:streamType:',
        [streamId, streamType.$value]);
  }

  /// @detail api
  /// @brief 设置期望订阅的远端视频流的参数。
  /// @param streamId 期望配置订阅参数的远端视频流 ID。
  /// @param remoteVideoConfig 期望配置的远端视频流参数，参看 ByteRTCRemoteVideoConfig{@link #ByteRTCRemoteVideoConfig}。
  /// @return 方法调用结果： <br>
  ///        + 0：成功。<br>
  ///        + !0：失败。
  /// @note
  ///        + 若使用 342 及以前版本的 SDK，调用该方法前请联系技术支持人员开启按需订阅功能。  <br>
  ///        + 该方法仅在发布端调用 setLocalSimulcastMode:{@link #ByteRTCEngine#setLocalSimulcastMode} 开启了发送多路视频流的情况下生效，此时订阅端将收到来自发布端与期望设置的参数最相近的一路流；否则订阅端只会收到一路参数为分辨率 640px × 360px、帧率 15fps 的视频流。  <br>
  ///        + 若发布端开启了推送多路流功能，但订阅端不对流参数进行设置，则默认接受发送端设置的分辨率最大的一路视频流。  <br>
  ///        + 该方法需在进房后调用。  <br>
  ///        + SDK 会根据发布端和所有订阅端的设置灵活调整视频流的参数，具体调整策略详见[推送多路流](https://www.volcengine.com/docs/6348/70139)文档。

  FutureOr<int> setRemoteVideoConfig(
      NSString streamId, ByteRTCRemoteVideoConfig remoteVideoConfig) async {
    return await nativeCall('setRemoteVideoConfig:remoteVideoConfig:',
        [streamId, remoteVideoConfig]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @brief 获取通话 ID。
  ///        该方法需要在加入房间后调用。当创建一个房间开启音视频通话后，系统会为该房间生成一个对应的通话 ID，标识此房间的通话。
  /// @return 通话 ID。

  FutureOr<NSString> getCallId() async {
    return await nativeCall('getCallId', []);
  }

  /// @detail api
  /// @brief 通过设置 ByteRTCRTSRoomDelegate{@link #ByteRTCRTSRoomDelegate}代理，可以监听此 `ByteRTCRTSRoom` 对象对应的回调事件。
  /// @param roomDelegate 参见 ByteRTCRTSRoomDelegate{@link #ByteRTCRTSRoomDelegate}。
  /// @return  <br>
  ///        + 0: 调用成功。
  ///        + < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> setRTCRoomDelegateRts(
      id<ByteRTCRTSRoomDelegate> roomDelegate) async {
    return await nativeCall('setRTCRoomDelegateRts:', [roomDelegate]);
  }

  /// @detail api
  /// @brief 更新 Token。 <br>
  ///        收到 onTokenWillExpire:{@link #ByteRTCRoomDelegate#onTokenWillExpire}，onPublishPrivilegeTokenWillExpire:{@link #ByteRTCRoomDelegate#onPublishPrivilegeTokenWillExpire}，或 onSubscribePrivilegeTokenWillExpire:{@link #ByteRTCRoomDelegate#onSubscribePrivilegeTokenWillExpire} 时，你必须重新获取 Token，并调用此方法更新 Token，以保证通话的正常进行。
  /// @param token 重新获取的有效 Token。 <br>
  ///        如果 Token 无效，你会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo}，错误码是 `-1010`。
  /// @return
  ///        - 0：成功；
  ///        - !0：失败。
  /// @note 请勿同时调用 updateToken:{@link #ByteRTCRTSRoom#updateToken} 和 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 重新加入房间。

  FutureOr<int> updateToken(NSString token) async {
    return await nativeCall('updateToken:', [token]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点文本消息（P2P）。
  /// @param userId <br>
  ///        消息接收用户的 ID
  /// @param message <br>
  ///        发送的文本消息内容。 <br>
  ///        消息不超过 64 KB。
  /// @param config <br>
  ///        消息发送的可靠/有序类型，参看 ByteRTCMessageConfig{@link #ByteRTCMessageConfig}
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note
  ///      - 在发送房间内文本消息前，必须先调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 加入房间。
  ///      - 调用该函数后会收到一次 rtsRoom:onUserMessageSendResult:error:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserMessageSendResult:error} 回调，通知消息发送方发送成功或失败。
  ///      - 若文本消息发送成功，则 uid 所指定的用户会收到 rtsRoom:onUserMessageReceived:message:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserMessageReceived:message} 回调。
  ///

  FutureOr<int64_t> sendUserMessage(
      NSString userId, NSString message, ByteRTCMessageConfig config) async {
    return await nativeCall(
        'sendUserMessage:message:config:', [userId, message, config.$value]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点二进制消息（P2P）。
  /// @param uid <br>
  ///        消息接收用户的 ID
  /// @param message <br>
  ///        发送的二进制消息内容 <br>
  ///        消息不超过 64KB。
  /// @param config <br>
  ///        消息发送的可靠/有序类型，参看 ByteRTCMessageConfig{@link #ByteRTCMessageConfig}。
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note
  ///      - 在发送房间内二进制消息前，必须先调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 加入房间。
  ///      - 调用该函数后会收到一次 rtsRoom:onUserMessageSendResult:error:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserMessageSendResult:error} 回调，通知消息发送方发送成功或失败；
  ///      - 若二进制消息发送成功，则 uid 所指定的用户会收到 rtsRoom:onUserBinaryMessageReceived:message:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserBinaryMessageReceived:message} 回调。
  ///

  FutureOr<int64_t> sendUserBinaryMessage(
      NSString uid, NSData message, ByteRTCMessageConfig config) async {
    return await nativeCall(
        'sendUserBinaryMessage:message:config:', [uid, message, config.$value]);
  }

  /// @detail api
  /// @brief 加入 RTS 房间。 <br>
  ///        调用 createRTSRoom:{@link #ByteRTCEngine#createRTSRoom} 创建房间后，调用此方法加入房间，同房间内其他用户进行音视频通话。
  /// @param token 动态密钥，用于对进房用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///        使用不同 AppID 的 App 是不能互通的。 <br>
  ///        请务必保证生成 Token 使用的 AppID 和创建引擎时使用的 AppID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息。参看 ByteRTCUserInfo{@link #ByteRTCUserInfo}。
  /// @return 方法调用结果。 <br>
  ///        -  0: 成功。触发以下回调：
  ///          - 本端收到房间状态通知 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///        - -1: 参数无效
  ///        - -2: 已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom{@link #ByteRTCRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调告知。
  /// @note
  ///        - 同一个 AppID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后加入房间的用户会将先加入房间的用户踢出房间，并且先加入房间的用户会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知，错误类型为重复登录 ByteRTCErrorCodeDuplicateLogin。
  ///        - 本地用户调用此方法加入房间成功后，会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知。若本地用户同时为可见用户，加入房间时远端用户会收到 rtsRoom:onUserJoined:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserJoined} 回调通知。
  ///        - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 rtcEngine:onConnectionStateChanged:{@link #ByteRTCEngineDelegate#rtcEngine:onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo}。

  FutureOr<int> joinRTSRoom(NSString token, ByteRTCUserInfo userInfo) async {
    return await nativeCall('joinRTSRoom:userInfo:', [token, userInfo]);
  }
}

class ByteRTCRangeAudio extends NativeClass {
  static const _$namespace = r'ByteRTCRangeAudio';
  static get codegen_$namespace => _$namespace;

  ByteRTCRangeAudio([NativeClassOptions? options])
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
  ///        范围语音是指，在同一 RTC 房间中设定的音频接收距离范围内，本地用户收听到的远端用户音频音量会随着远端用户的靠近/远离而放大/衰减；若远端用户在房间内的位置超出设定范围，则本地用户无法接收其音频。音频接收范围设置参看 updateReceiveRange:{@link #ByteRTCRangeAudio#updateReceiveRange}。
  /// @param enable 是否开启范围语音功能： <br>
  ///        - YES: 开启
  ///        - NO: 关闭（默认）
  /// @note 该方法进房前后都可调用，为保证进房后范围语音效果的平滑切换，你需在该方法前先调用 updatePosition:{@link #ByteRTCRangeAudio#updatePosition} 设置自身位置坐标，然后开启该方法收听范围语音效果。

  FutureOr<void> enableRangeAudio(BOOL enable) async {
    return await nativeCall('enableRangeAudio:', [enable]);
  }

  /// @detail api
  /// @author chuzhongtao
  /// @brief 更新本地用户的音频收听范围。
  /// @param range 音频收听范围，参看 ByteRTCReceiveRange{@link #ByteRTCReceiveRange}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - !0: 失败。

  FutureOr<int> updateReceiveRange(ByteRTCReceiveRange range) async {
    return await nativeCall('updateReceiveRange:', [range]);
  }

  /// @detail api
  /// @author chuzhongtao
  /// @brief 更新本地用户在房间内空间直角坐标系中的位置坐标。
  /// @param pos 三维坐标的值，默认为 [0, 0, 0]，参看 ByteRTCPosition{@link #ByteRTCPosition}.
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - !0：失败。
  /// @note 调用该接口更新坐标后，你需调用 enableRangeAudio:{@link #ByteRTCRangeAudio#enableRangeAudio} 开启范围语音功能以收听范围语音效果。

  FutureOr<int> updatePosition(ByteRTCPosition pos) async {
    return await nativeCall('updatePosition:', [pos]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 设置范围语音的音量衰减模式。
  /// @param type 音量衰减模式。默认为线性衰减。详见 ByteRTCAttenuationType{@link #ByteRTCAttenuationType}。
  /// @param coefficient 指数衰减模式下的音量衰减系数，默认值为 1。范围 [0.1,100]，推荐设置为 `50`。数值越大，音量的衰减速度越快。
  /// @return 调用是否成功 <br>
  ///         - `0`:调用成功
  ///         - `-1`:调用失败。原因为在调用 enableRangeAudio:{@link #ByteRTCRangeAudio#enableRangeAudio} 开启范围语音前或进房前调用本接口
  /// @note 音量衰减范围通过 updateReceiveRange:{@link #ByteRTCRangeAudio#updateReceiveRange} 进行设置。

  FutureOr<int> setAttenuationModel(
      ByteRTCAttenuationType type, float coefficient) async {
    return await nativeCall(
        'setAttenuationModel:coefficient:', [type.$value, coefficient]);
  }

  /// @detail api
  /// @author chuzhongtao
  /// @brief 添加标签组，用于标记相互之间通话不衰减的用户组。 <br>
  ///        在同一个 RTC 房间中，如果多个用户的标签组之间有交集，那么，他们之间互相通话时，通话不衰减。 <br>
  ///        比如，用户身处多个队伍，队伍成员间通话不衰减。那么，可以为每个队伍绑定专属标签，每个用户的标签组包含用户所属各个队伍的标签。
  /// @param flags 标签组。

  FutureOr<void> setNoAttenuationFlags(NSArray<NSString> flags) async {
    return await nativeCall('setNoAttenuationFlags:', [flags]);
  }
}

class ByteRTCGameRoom extends NativeClass {
  static const _$namespace = r'ByteRTCGameRoom';
  static get codegen_$namespace => _$namespace;

  ByteRTCGameRoom([NativeClassOptions? options])
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

  /// @detail callback
  FutureOr<ByteRTCGameRoomDelegate?> get delegate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCGameRoomDelegate?>("delegate");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCGameRoomDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegate(FutureOr<ByteRTCGameRoomDelegate?> value) {
    sendInstanceSet("delegate", value);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 退出并销毁调用 createGameRoom:roomConfig:{@link #ByteRTCEngine#createGameRoom:roomConfig} 所创建的房间实例。

  FutureOr<void> destroy() async {
    return await nativeCall('destroy', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 通过设置 ByteRTCGameRoom{@link #ByteRTCGameRoom} 对象的事件句柄，监听此对象对应的回调事件。
  /// @param roomDelegate 参见 ByteRTCRoomDelegate{@link #ByteRTCRoomDelegate}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> setRTCGameRoomDelegate(
      id<ByteRTCGameRoomDelegate> roomDelegate) async {
    return await nativeCall('setRTCGameRoomDelegate:', [roomDelegate]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间。 <br>
  ///        调用 createGameRoom:roomConfig:{@link #ByteRTCEngine#createGameRoom:roomConfig} 创建房间后，调用此方法加入房间，同房间内其他用户进行音频通话。
  /// @param token 动态密钥，用于对登录用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在您的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///       - 使用不同 App ID 的 App 是不能互通的。
  ///       - 请务必保证生成 Token 使用的 App ID 和创建引擎时使用的 App ID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息，参看 ByteRTCUserInfo{@link #ByteRTCUserInfo}。
  /// @return
  ///        -  0: 成功。触发以下回调：
  ///          - 本端收到房间状态通知 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///          - 如果本端用户为可见用户，房间内其他用户收到 rtcRoom:onUserJoined:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserJoined} 回调通知。
  ///        - -1：room_id / user_info.uid 包含了无效的参数。
  ///        - -2：已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom:{@link #ByteRTCGameRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调告知。
  /// @note
  ///       - 同一个 App ID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后进房的用户会将先进房的用户踢出房间，并且先进房的用户会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知，错误类型详见 ByteRTCErrorCode{@link #ByteRTCErrorCode} 中的 ByteRTCErrorCodeDuplicateLogin。
  ///       - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 rtcEngine:onConnectionStateChanged:{@link #ByteRTCEngineDelegate#rtcEngine:onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知。

  FutureOr<int> joinRoom(NSString token, ByteRTCUserInfo userInfo) async {
    return await nativeCall('joinRoom:userInfo:', [token, userInfo]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 离开游戏房间。 <br>
  ///        调用此方法结束通话过程，并释放所有通话相关的资源。
  /// @return
  ///        - 0: 调用成功。如果用户是房间内可见用户，触发以下回调：
  ///            - 远端用户收到 rtcRoom:onUserLeave:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserLeave:reason} 回调通知。
  ///            - 正在发布的流会被取消发布。远端用户收到 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason}、rtcRoom:onScreenVideoPublishStateChanged:userId:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onScreenVideoPublishStateChanged:userId:state:reason} 和/或 rtcRoom:onScreenAudioPublishStateChanged:userId:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onScreenAudioPublishStateChanged:userId:state:reason} 回调通知。
  ///        - < 0: 调用失败，参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 加入房间后，必须调用此方法结束通话，否则无法开始下一次通话。
  ///       - 此方法是异步操作，调用返回时并没有真正退出房间。真正退出房间后，本地会收到 rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChangedWithReason:withUid:state:reason}  回调通知。你必须在收到 rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChangedWithReason:withUid:state:reason}  回调后，再销毁房间或引擎，或调用 joinRoom:userInfo:{@link #ByteRTCGameRoom#joinRoom:userInfo} 再次加入房间。

  FutureOr<int> leaveRoom() async {
    return await nativeCall('leaveRoom', []);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 更新游戏房间 Token。 <br>
  ///        收到 onTokenWillExpire:{@link #ByteRTCGameRoomDelegate#onTokenWillExpire}，onPublishPrivilegeTokenWillExpire:{@link #ByteRTCGameRoomDelegate#onPublishPrivilegeTokenWillExpire}，或 onSubscribePrivilegeTokenWillExpire:{@link #ByteRTCGameRoomDelegate#onSubscribePrivilegeTokenWillExpire} 时，你必须重新获取 Token，并调用此方法更新 Token，以保证通话的正常进行。
  /// @param token 重新获取的有效 Token。 <br>
  ///        如果 Token 无效，你会收到 rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChangedWithReason:withUid:state:reason} ，错误码是 `-1010`。
  /// @return
  ///        - 0：成功；
  ///        - !0：失败。
  /// @note 请勿同时调用 updateToken:{@link #ByteRTCGameRoom#updateToken} 和 joinRoom:userInfo:{@link #ByteRTCGameRoom#joinRoom:userInfo} 重新加入房间。

  FutureOr<int> updateToken(NSString token) async {
    return await nativeCall('updateToken:', [token]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 获取游戏房间的范围语音接口实例。
  /// @return 方法调用结果： <br>
  ///        - ByteRTCRangeAudio：成功，返回一个 ByteRTCRangeAudio{@link #ByteRTCRangeAudio} 实例。
  ///        - NULL：失败，当前 SDK 不支持范围语音功能。
  /// @note 首次调用该方法须在创建房间后、加入房间前。范围语音相关 API 和调用时序详见[范围语音](https://www.volcengine.com/docs/6348/114727)。

  FutureOr<ByteRTCRangeAudio> getRangeAudio() async {
    final result = await nativeCall('getRangeAudio', []);
    return packObject(
        result,
        () =>
            ByteRTCRangeAudio(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，调用该接口开启或关闭麦克风。同游戏房间其他用户会收到回调 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error}。
  /// @param enable 是否开启麦克风：<br>
  ///               - true：开启麦克风，采集并发布音频流。
  ///               - false：默认设置。关闭麦克风，停止采集和发布音频流。
  /// @return
  ///        - 0：接口调用成功。
  ///        - -3：接口调用失败。没有加入房间。
  /// @note 不可与 enableAudioSend:{@link #ByteRTCGameRoom#enableAudioSend} 同时调用。

  FutureOr<int> enableMicrophone(BOOL enable) async {
    return await nativeCall('enableMicrophone:', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 加入游戏房间后，开启或关闭扬声器。
  /// @param enable 是否开启声器：<br>
  ///                - true：开启扬声器。
  ///                - false：默认设置。关闭扬声器。
  /// @return
  ///         - 0：接口调用成功。
  ///         - -3：接口调用失败。没有加入房间。

  FutureOr<int> enableSpeakerphone(BOOL enable) async {
    return await nativeCall('enableSpeakerphone:', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 开始或停止发送音频流。调用此接口不影响音频采集。同游戏房间其他用户会收到相应的回调。
  /// @param enable 是否发送音频流：<br>
  ///               - true：发布音频流。
  ///               - false：默认设置。停止发布音频流（不会关闭麦克风），即静音。
  /// @return
  ///        - 0：接口调用成功。
  ///        - -3：接口调用失败，未加入房间。
  /// @note 不可与 enableMicrophone:{@link #ByteRTCGameRoom#enableMicrophone} 同时调用。

  FutureOr<int> enableAudioSend(BOOL enable) async {
    return await nativeCall('enableAudioSend:', [enable]);
  }

  /// @detail api
  /// @author luomingkang
  /// @brief 是否订阅指定用户的音频流。
  /// @param userId 用户 ID，最大长度为128字节的非空字符串。支持的字符集范围为:<br>
  ///            1. 26个大写字母 A ~ Z<br>
  ///            2. 26个小写字母 a ~ z<br>
  ///            3. 10个数字 0 ~ 9<br>
  ///            4. 下划线"_", at符"\@", 减号"-"
  /// @param enable 是否订阅音频流：<br>
  ///        true：订阅指定用户的音频流。
  ///        false：默认设置。不订阅指定用户的音频流。
  /// @return
  ///        - 0：接口调用成功
  ///        - -2：传入的用户 ID 为空字符串。

  FutureOr<int> enableAudioReceive(NSString userId, BOOL enable) async {
    return await nativeCall('enableAudioReceive:enable:', [userId, enable]);
  }

  /// @detail api
  /// @author zhangcaining
  /// @brief 调节某个游戏房间内所有远端用户的音频播放音量。
  /// @param volume 音频播放音量值和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///              - 0: 静音
  ///              - 100: 原始音量，默认值
  ///              - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内， <br>
  ///        - 该方法与 setRemoteAudioPlaybackVolume:volume:{@link #ByteRTCEngine#setRemoteAudioPlaybackVolume:volume} 互斥，最新调用的任一方法设置的音量将覆盖此前已设置的音量，效果不叠加；
  ///        - 当该方法与 setPlaybackVolume:{@link #ByteRTCEngine#setPlaybackVolume} 方法共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。

  FutureOr<int> setRemoteRoomAudioPlaybackVolume(NSInteger volume) async {
    return await nativeCall('setRemoteRoomAudioPlaybackVolume:', [volume]);
  }
}

class ByteRTCKTVPlayer extends NativeClass {
  static const _$namespace = r'ByteRTCKTVPlayer';
  static get codegen_$namespace => _$namespace;

  ByteRTCKTVPlayer([NativeClassOptions? options])
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

  FutureOr<ByteRTCKTVPlayerDelegate?> get delegate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCKTVPlayerDelegate?>("delegate");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCKTVPlayerDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegate(FutureOr<ByteRTCKTVPlayerDelegate?> value) {
    sendInstanceSet("delegate", value);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 播放歌曲。
  /// @param musicId 音乐 ID。 <br>
  ///        若同一 musicId 的歌曲正在播放，再次调用接口会从开始位置重新播放。若 musicId 对应的音频文件不存在会触发报错。
  /// @param trackType 原唱伴唱类型，参看 ByteRTCAudioTrackType{@link #ByteRTCAudioTrackType}。
  /// @param playType 音乐播放类型。参看 ByteRTCAudioPlayType{@link #ByteRTCAudioPlayType}。
  /// @note
  ///        - 调用接口后，你会收到 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调歌曲播放状态。
  ///        - 若音乐 ID 错误，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3023，playState 为 4。
  ///        - 若未进房，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3022，playState 为 4。
  ///        - 若音乐文件不存在，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3020，playState 为 4。

  FutureOr<void> playMusic(NSString musicId, ByteRTCAudioTrackType trackType,
      ByteRTCAudioPlayType playType) async {
    return await nativeCall('playMusic:audioTrackType:audioPlayType:',
        [musicId, trackType.$value, playType.$value]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 暂停播放歌曲。
  /// @param musicId 音乐 ID。
  /// @note
  ///        - 调用接口后，你会收到 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调歌曲播放状态。
  ///        - 若音乐 ID 错误，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3023，playState 为 4。
  ///        - 若未进房，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3022，playState 为 4。

  FutureOr<void> pauseMusic(NSString musicId) async {
    return await nativeCall('pauseMusic:', [musicId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 继续播放歌曲。
  /// @param musicId 音乐 ID。
  /// @note
  ///        - 调用接口后，你会收到 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调歌曲播放状态。
  ///        - 若音乐 ID 错误，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3023，playState 为 4。
  ///        - 若未进房，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3022，playState 为 4。

  FutureOr<void> resumeMusic(NSString musicId) async {
    return await nativeCall('resumeMusic:', [musicId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 停止播放歌曲。
  /// @param musicId 音乐 ID。
  /// @note
  ///        - 调用接口后，你会收到 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调歌曲播放状态。
  ///        - 若音乐 ID 错误，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3023，playState 为 4。
  ///        - 若未进房，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3022，playState 为 4。

  FutureOr<void> stopMusic(NSString musicId) async {
    return await nativeCall('stopMusic:', [musicId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 设置音乐文件的起始播放位置。
  /// @param musicId 音乐 ID。
  /// @param position 音乐起始位置，单位为毫秒，取值小于音乐文件总时长。
  /// @note
  ///        - 调用本接口时音乐必须处于播放中状态。
  ///        - 调用接口后，你会收到 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调歌曲播放状态。
  ///        - 若音乐 ID 错误，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3023，playState 为 4。
  ///        - 若未进房，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3022，playState 为 4。

  FutureOr<void> seekMusic(NSString musicId, int position) async {
    return await nativeCall('seekMusic:position:', [musicId, position]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 设置歌曲播放音量，只能在开始播放后进行设置。
  /// @param musicId 音乐 ID。
  /// @param volume 歌曲播放音量，调节范围：[0,400]。 <br>
  ///        - 0：静音。
  ///        - 100：原始音量。
  ///        - 400: 原始音量的 4 倍(自带溢出保护)。
  /// @note
  ///        - 调用本接口时音乐必须处于播放中状态。
  ///        - 若设置的音量大于 400，则按最大值 400 进行调整；若设置的音量小于 0，则按最小值 0 进行调整。
  ///        - 若音乐 ID 错误，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3023，playState 为 4。
  ///        - 若未进房，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3022，playState 为 4。

  FutureOr<void> setMusicVolume(NSString musicId, int volume) async {
    return await nativeCall('setMusicVolume:volume:', [musicId, volume]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 切换歌曲原唱伴唱。
  /// @param musicId 音乐 ID。
  /// @note 调用本接口时音乐必须处于播放中状态。

  FutureOr<void> switchAudioTrackType(NSString musicId) async {
    return await nativeCall('switchAudioTrackType:', [musicId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 对播放中的音乐设置升降调信息。
  /// @param musicId 音乐 ID。
  /// @param pitch 相对于音乐文件原始音调的升高/降低值，取值范围 [-12，12]，默认值为 0，即不做调整。 <br>
  ///              取值范围内每相邻两个值的音高距离相差半音，正值表示升调，负值表示降调，设置的绝对值越大表示音调升高或降低越多。
  /// @note
  ///        - 调用本接口时音乐必须处于播放中状态。
  ///        - 若设置的 pitch 大于 12，则按最大值 12 进行调整；若设置的 pitch 小于 –12，，则按最小值 –12 进行调整。
  ///        - 若音乐 ID 错误，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3023，playState 为 4。
  ///        - 若未进房，会触发 ktvPlayer:onPlayStateChanged:state:error:{@link #ByteRTCKTVPlayerDelegate#ktvPlayer:onPlayStateChanged:state:error} 回调，errorCode 为 –3022，playState 为 4。

  FutureOr<void> setMusicPitch(NSString musicId, int pitch) async {
    return await nativeCall('setMusicPitch:pitch:', [musicId, pitch]);
  }
}

class ByteRTCDeviceCollection extends NativeClass {
  static const _$namespace = r'ByteRTCDeviceCollection';
  static get codegen_$namespace => _$namespace;

  ByteRTCDeviceCollection([NativeClassOptions? options])
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
  /// @author dixing
  /// @brief 获取当前音视频设备数量
  /// @return 音视频设备数量

  FutureOr<int> getCount() async {
    return await nativeCall('getCount', []);
  }

  /// @detail api
  /// @author dixing
  /// @brief 根据索引号，获取设备信息
  /// @param index 设备索引号，从 0 开始，注意需小于 getCount{@link #ByteRTCDeviceCollection#getCount} 返回值。
  /// @param deviceName 设备名称
  /// @param deviceID 设备 ID
  /// @return
  ///        - 0：方法调用成功
  ///        - !0：方法调用失败

  FutureOr<int> getDevice(
      int index, NSString deviceName, NSString deviceID) async {
    return await nativeCall(
        'getDevice:DeviceName:DeviceID:', [index, deviceName, deviceID]);
  }
}

class ByteRTCSingScoringManager extends NativeClass {
  static const _$namespace = r'ByteRTCSingScoringManager';
  static get codegen_$namespace => _$namespace;

  ByteRTCSingScoringManager([NativeClassOptions? options])
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
  /// @author wangjunzheng
  /// @brief 初始化 K 歌评分。
  /// @param singScoringAppkey K 歌评分密钥，用于鉴权验证 K 歌功能是否开通。
  /// @param singScoringToken K 歌评分密钥，用于鉴权验证 K 歌功能是否开通。
  /// @param delegate K 歌评分事件回调类，详见 ByteRTCSingScoringDelegate{@link #ByteRTCSingScoringDelegate}。
  /// @return
  ///        - 0：配置成功。
  ///        - -1：接口调用失败。
  ///        - -2：未集成 K 歌评分模块。
  ///        - >0：其他错误，具体参看[错误码表](https://www.volcengine.com/docs/6489/148198)。
  /// @note 输入正确的鉴权信息才可以使用 K 歌评分相关的功能，鉴权方式为离线鉴权，根据包名（bundleID）绑定 Appkey 及 Token，K 歌评分密钥请联系技术支持人员申请。

  FutureOr<int> initSingScoring(
      NSString singScoringAppkey,
      NSString singScoringToken,
      id<ByteRTCSingScoringDelegate> delegate) async {
    return await nativeCall('initSingScoring:singScoringToken:delegate:',
        [singScoringAppkey, singScoringToken, delegate]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置 K 歌评分参数。
  /// @param config K 歌评分的各项参数，详见 ByteRTCSingScoringConfig{@link #ByteRTCSingScoringConfig}。
  /// @return
  ///        - 0：配置成功。
  ///        - -1：接口调用失败。
  ///        - -2：未集成 K 歌评分模块。
  ///        - >0：其他错误，具体参看[错误码表](https://www.volcengine.com/docs/6489/148198)。

  FutureOr<int> setSingScoringConfig(ByteRTCSingScoringConfig config) async {
    return await nativeCall('setSingScoringConfig:', [config]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 获取标准音高数据。
  /// @param midiFilepath 歌曲 midi 文件路径。
  /// @return ByteRTCStandardPitchInfo{@link #ByteRTCStandardPitchInfo} 标准音高数据数组。
  /// @note
  ///        - 请保证此接口传入的 midi 文件路径与 setSingScoringConfig:{@link #ByteRTCSingScoringManager#setSingScoringConfig} 接口中传入的路径一致。

  FutureOr<ByteRTCStandardPitchInfo> getStandardPitchInfo(
      NSString midiFilepath) async {
    final result = await nativeCall('getStandardPitchInfo:', [midiFilepath]);
    return packObject(
        result,
        () => ByteRTCStandardPitchInfo(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 开始 K 歌评分。
  /// @param position 开始评分时，音乐的播放进度，单位：ms。
  /// @param scoringInfoInterval 实时回调的时间间隔，单位：ms；默认 50 ms。最低间隔为 20 ms。
  /// @return
  ///        - 0：配置成功。
  ///        - -1：接口调用失败。
  ///        - -2：未集成 K 歌评分模块。
  ///        - >0：其他错误，具体参看[错误码表](https://www.volcengine.com/docs/6489/148198)。
  /// @note
  ///        - 在调用 initSingScoring:singScoringToken:delegate:{@link #ByteRTCSingScoringManager#initSingScoring:singScoringToken:delegate} 初始化 K 歌评分功能后调用该接口。
  ///        - 调用该接口后，将会根据设置的回调时间间隔，收到评分结果 onCurrentScoringInfo:{@link #ByteRTCSingScoringDelegate#onCurrentScoringInfo} 回调。
  ///        - 如果调用 startAudioMixing:filePath:config: 接口播放音频文件，请在收到 rtcEngine:onAudioMixingStateChanged:state:error:(ByteRTCAudioMixingStatePlaying) 之后调用此接口。

  FutureOr<int> startSingScoring(int position, int scoringInfoInterval) async {
    return await nativeCall('startSingScoring:scoringInfoInterval:',
        [position, scoringInfoInterval]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 停止 K 歌评分。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。

  FutureOr<int> stopSingScoring() async {
    return await nativeCall('stopSingScoring', []);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 获取上一句的演唱评分。调用 startSingScoring:scoringInfoInterval:{@link #ByteRTCSingScoringManager#startSingScoring:scoringInfoInterval} 开始评分后可以调用该接口。
  /// @return
  ///        - <0：获取评分失败。
  ///        - >=0：上一句歌词的演唱评分。

  FutureOr<int> getLastSentenceScore() async {
    return await nativeCall('getLastSentenceScore', []);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 获取当前演唱总分。调用 startSingScoring:scoringInfoInterval:{@link #ByteRTCSingScoringManager#startSingScoring:scoringInfoInterval} 开始评分后可以调用该接口。
  /// @return
  ///        - <0：获取总分失败。
  ///        - >=0：当前演唱总分。

  FutureOr<int> getTotalScore() async {
    return await nativeCall('getTotalScore', []);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 获取当前演唱歌曲的平均分。
  /// @return
  ///        - <0：获取平均分失败。
  ///        - >=0：当前演唱平均分。

  FutureOr<int> getAverageScore() async {
    return await nativeCall('getAverageScore', []);
  }
}

class ByteRTCVideoDeviceManager extends NativeClass {
  static const _$namespace = r'ByteRTCVideoDeviceManager';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoDeviceManager([NativeClassOptions? options])
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
  /// @author zhangzhenyu.samuel
  /// @brief 获取视频采集设备列表。
  /// @return 包含系统中所有视频采集设备的列表，参看 ByteRTCDeviceCollection{@link #ByteRTCDeviceCollection}。 <br>
  /// 等待超时后会返回空列表。超时时间默认为 10 s。建议通过 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 监听到 `ByteRTCMediaDeviceListUpdated` 后，再次调用本接口获取。
  /// @note 你可以在收到 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 了解设备变更后，重新调用本接口以获得新的设备列表。

  FutureOr<ByteRTCDeviceCollection> enumerateVideoCaptureDevices() async {
    final result = await nativeCall('enumerateVideoCaptureDevices', []);
    return packObject(
        result,
        () => ByteRTCDeviceCollection(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 获取当前 SDK 正在使用的视频采集设备信息
  /// @param deviceID 视频设备 ID
  /// @return
  ///        - 0：方法调用成功
  ///        - !0：方法调用失败

  FutureOr<int> getVideoCaptureDevice(NSString deviceID) async {
    return await nativeCall('getVideoCaptureDevice:', [deviceID]);
  }

  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前视频采集设备
  /// @param deviceID 视频设备 ID。调用 enumerateVideoCaptureDevices{@link #ByteRTCVideoDeviceManager#enumerateVideoCaptureDevices} 获取全量视频设备。
  /// @return
  ///        - 0：方法调用成功
  ///        - !0：方法调用失败

  FutureOr<int> setVideoCaptureDevice(NSString deviceID) async {
    return await nativeCall('setVideoCaptureDevice:', [deviceID]);
  }
}

class ByteRTCWTNStream extends NativeClass {
  static const _$namespace = r'ByteRTCWTNStream';
  static get codegen_$namespace => _$namespace;

  ByteRTCWTNStream([NativeClassOptions? options])
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
  /// @valid since 3.60. 自 3.60 起，此接口替代了 `startPlayPublicStream:` 和 `stopPlayPublicStream:` 方法来订阅/取消订阅指定 WTN 视频流，如果你使用了这两个方法，请迁移至此接口。
  /// @author hanchenchen
  /// @brief 订阅/取消订阅指定 WTN 视频流 <br>
  ///        无论用户是否在房间内，都可以调用本接口订阅/取消订阅指定的 WTN 音频流。
  /// @param streamId WTN 流 ID，如果指定流暂未发布，则本地客户端将在其开始发布后接收到流数据。
  /// @param subscribe 是否订阅 WTN 流 <br>
  ///       - true：订阅
  ///       - false：取消订阅
  /// @return
  ///        - 0: 成功。同时将收到
  /// onWTNAudioSubscribeStateChanged:state:reason:{@link #ByteRTCWTNStreamDelegate#onWTNAudioSubscribeStateChanged:state:reason} 回调。
  ///        - !0: 失败。当参数不合法或参数为空，调用失败。
  /// @note
  ///        - 一个客户端最多同时播放 5 路 WTN 流，请及时调用 subscribeWTNVideoStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNVideoStream:subscribe}/subscribeWTNAudioStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNAudioStream:subscribe} 取消订阅 WTN 流，避免订阅的 WTN 流数量超限。
  ///        - 在调用本接口之前，建议先绑定渲染视图。
  ///              - 调用 setWTNRemoteVideoCanvas:withCanvas:{@link #ByteRTCWTNStream#setWTNRemoteVideoCanvas:withCanvas} 绑定内部渲染视图
  ///              - 调用 setWTNRemoteVideoSink:withSink:withConfig:{@link #ByteRTCWTNStream#setWTNRemoteVideoSink:withSink:withConfig} 绑定自定义渲染视图
  ///        - 调用本接口后，可以通过 onWTNFirstRemoteVideoFrameDecoded:withFrameInfo:{@link #ByteRTCWTNStreamDelegate#onWTNFirstRemoteVideoFrameDecoded:withFrameInfo} 回调 WTN 视频流的首帧解码情况
  ///        - 调用本接口后，可以通过 onWTNSEIMessageReceived:andChannelId:andMessage:{@link #ByteRTCWTNStreamDelegate#onWTNSEIMessageReceived:andChannelId:andMessage} 回调 WTN 流中包含的 SEI 信息。
  /// @order 0

  FutureOr<int> subscribeWTNVideoStream(
      NSString streamId, bool subscribe) async {
    return await nativeCall(
        'subscribeWTNVideoStream:subscribe:', [streamId, subscribe]);
  }

  /// @author hanchenchen
  /// @detail api
  /// @valid since 3.60. 自 3.60 起，此接口替代了 `startPlayPublicStream:` 和 `stopPlayPublicStream:` 方法来订阅/取消订阅指定 WTN 音频流，如果你使用了这两个方法，请迁移至此接口。
  /// @brief 订阅/取消订阅指定 WTN 音频流 <br>
  ///        无论用户是否在房间内，都可以调用本接口订阅/取消订阅指定的 WTN 音频流。
  /// @param streamId WTN 流 ID，如果指定流暂未发布，则本地客户端将在其开始发布后接收到流数据。
  /// @param subscribe 是否订阅 WTN 流 <br>
  ///       - true：订阅
  ///       - false：取消订阅
  /// @return
  ///        - 0: 成功。同时将收到 onWTNVideoSubscribeStateChanged:state:reason:{@link #ByteRTCWTNStreamDelegate#onWTNVideoSubscribeStateChanged:state:reason} 回调。
  ///        - !0: 失败。当参数不合法或参数为空，调用失败。
  /// @note
  ///        - 一个客户端最多同时播放 5 路 WTN 流，请及时调用 subscribeWTNVideoStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNVideoStream:subscribe}/subscribeWTNAudioStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNAudioStream:subscribe} 取消订阅 WTN 流，避免订阅的 WTN 流数量超限。
  ///        - 在调用本接口之前，建议先绑定渲染视图。
  ///              - 调用 setWTNRemoteVideoCanvas:withCanvas:{@link #ByteRTCWTNStream#setWTNRemoteVideoCanvas:withCanvas}  绑定内部渲染视图
  ///              - 调用 setWTNRemoteVideoSink:withSink:withConfig:{@link #ByteRTCWTNStream#setWTNRemoteVideoSink:withSink:withConfig} 绑定自定义渲染视图
  ///        - 调用本接口后，可以通过 onWTNFirstRemoteAudioFrame:{@link #ByteRTCWTNStreamDelegate#onWTNFirstRemoteAudioFrame} 回调 WTN 音频流首帧解码情况。
  ///        - 调用本接口后，可以通过 onWTNSEIMessageReceived:andChannelId:andMessage:{@link #ByteRTCWTNStreamDelegate#onWTNSEIMessageReceived:andChannelId:andMessage} 回调 WTN 流中包含的 SEI 信息。
  /// @order 1

  FutureOr<int> subscribeWTNAudioStream(
      NSString streamId, bool subscribe) async {
    return await nativeCall(
        'subscribeWTNAudioStream:subscribe:', [streamId, subscribe]);
  }

  /// @detail api
  /// @valid since 3.60. Since version 3.60, this interface replaces `setPublicStreamVideoCanvas:withCanvas:` for the following function. If you have upgraded to version 3.60 or above and are still using this method, please migrate to this interface.
  /// @author hanchenchen
  /// @brief Assign a internal render view to the WTN stream
  /// @param streamId ID of the WTN stream
  /// @param canvas Internal render view. Set to be a blank view if you want to unbind. Refer to ByteRTCVideoCanvas{@link #ByteRTCVideoCanvas} for more details.
  /// @return
  ///        - 0: Success
  ///        - !0: Failure
  /// @order 2

  FutureOr<int> setWTNRemoteVideoCanvas(
      NSString streamId, ByteRTCVideoCanvas canvas) async {
    return await nativeCall(
        'setWTNRemoteVideoCanvas:withCanvas:', [streamId, canvas]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `setPublicStreamVideoSink:withSink:withPixelFormat:` 方法来实现下述功能。你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此接口。
  /// @author hanchenchen
  /// @brief 为指定 WTN 流绑定自定义渲染器。详见[自定义视频渲染](https://www.volcengine.com/docs/6348/81201)。
  /// @param streamId WTN 流 ID
  /// @param videoSink 自定义视频渲染器，需要释放渲染器资源时，将 videoSink 设置为 `null`。参看 ByteRTCVideoSinkDelegate{@link #ByteRTCVideoSinkDelegate}
  /// @param config 远端视频帧回调配置，参看 ByteRTCRemoteVideoSinkConfig{@link #ByteRTCRemoteVideoSinkConfig}
  /// @return
  ///        - 0: 成功
  ///        - <0: 失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @order 3

  FutureOr<int> setWTNRemoteVideoSink(
      NSString streamId,
      id<ByteRTCVideoSinkDelegate> videoSink,
      ByteRTCRemoteVideoSinkConfig config) async {
    return await nativeCall('setWTNRemoteVideoSink:withSink:withConfig:',
        [streamId, videoSink, config]);
  }

  /// @detail api
  /// @valid since 3.60. 自 3.60 起，该接口替代了 `setPublicStreamAudioPlaybackVolume:volume:` 方法来实现下述功能。你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此接口。
  /// @author hanchenchen
  /// @brief 调节 WTN 流的音频播放音量。
  /// @param streamId WTN 流 ID
  /// @param volume 音频播放音量值和原始音量值的比值，该比值的范围是 `[0, 400]`，单位为 \%，且自带溢出保护。为保证更好的音频质量，建议设定在 `[0, 100]` 之间，其中 100 为系统默认值。
  /// @return
  ///         - 0: 成功调用。
  ///         - -2: 参数错误。
  /// @order 4

  FutureOr<int> setWTNRemoteAudioPlaybackVolume(
      NSString streamId, NSInteger volume) async {
    return await nativeCall(
        'setWTNRemoteAudioPlaybackVolume:volume:', [streamId, volume]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 设置 WTN 流回调接口
  /// @param delegate WTN 回调类，参看 ByteRTCWTNStreamDelegate{@link #ByteRTCWTNStreamDelegate}。
  /// @order 5

  FutureOr<void> setWTNStreamDelegate(
      id<ByteRTCWTNStreamDelegate> delegate) async {
    return await nativeCall('setWTNStreamDelegate:', [delegate]);
  }
}

class ByteRTCSpatialAudio extends NativeClass {
  static const _$namespace = r'ByteRTCSpatialAudio';
  static get codegen_$namespace => _$namespace;

  ByteRTCSpatialAudio([NativeClassOptions? options])
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
  ///        - YES：开启
  ///        - NO：关闭（默认）
  /// @note 该方法仅开启空间音频功能，你须调用 updateSelfPosition:{@link #ByteRTCSpatialAudio#updateSelfPosition} 设置自身位置坐标后方可收听空间音频效果。空间音频相关 API 和调用时序详见[空间音频](https://www.volcengine.com/docs/6348/93903)。

  FutureOr<void> enableSpatialAudio(BOOL enable) async {
    return await nativeCall('enableSpatialAudio:', [enable]);
  }

  /// @detail api
  /// @author luomingkang.264
  /// @brief 关闭本地用户朝向对本地用户发声效果的影响。 <br>
  ///        调用此接口后，房间内的其他用户收听本地发声时，声源都在收听者正面。
  /// @note
  ///        - 调用本接口关闭朝向功能后，在当前的空间音频实例的生命周期内无法再次开启。
  ///        - 调用此接口不影响本地用户收听朝向的音频效果。要改变本地用户收听朝向，参看 updateSelfPosition:{@link #ByteRTCSpatialAudio#updateSelfPosition} 和 updateRemotePosition:positionInfo:{@link #ByteRTCSpatialAudio#updateRemotePosition:positionInfo}。

  FutureOr<void> disableRemoteOrientation() async {
    return await nativeCall('disableRemoteOrientation', []);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置本地用户在自建空间直角坐标系中的收听坐标和收听朝向，以实现本地用户预期的空间音频收听效果。
  /// @param positionInfo 空间音频位置信息。参看 ByteRTCPositionInfo{@link #ByteRTCPositionInfo}。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。
  ///        - -2: 失败，原因是校验本地用户的三维朝向信息时，三个向量没有两两垂直。
  /// @note
  ///        - 该方法需在进房后调用。
  ///        - 调用该接口更新坐标前，你需调用 enableSpatialAudio:{@link #ByteRTCSpatialAudio#enableSpatialAudio} 开启空间音频功能。空间音频相关 API 和调用时序详见[空间音频](https://www.volcengine.com/docs/6348/93903)。
  ///        - 调用此接口在本地进行的设定对其他用户的空间音频收听效果不会产生任何影响。

  FutureOr<int> updateSelfPosition(ByteRTCPositionInfo positionInfo) async {
    return await nativeCall('updateSelfPosition:', [positionInfo]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置房间内某一远端用户在本地用户自建的空间音频坐标系中的发声位置和发声朝向，以实现本地用户预期的空间音频收听效果。
  /// @param uid 用户 ID
  /// @param positionInfo 远端用户的空间音频位置信息。参看 ByteRTCPositionInfo{@link #ByteRTCPositionInfo}。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。
  ///        - -2: 失败，原因是校验远端用户的三维朝向信息时，三个向量没有两两垂直。
  /// @note
  ///        该方法需在创建房间后调用。 <br>
  ///        调用此接口在本地进行的设定对其他用户的空间音频收听效果不会产生任何影响。

  FutureOr<int> updateRemotePosition(
      NSString uid, ByteRTCPositionInfo positionInfo) async {
    return await nativeCall(
        'updateRemotePosition:positionInfo:', [uid, positionInfo]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 移除调用 updateRemotePosition:positionInfo:{@link #ByteRTCSpatialAudio#updateRemotePosition:positionInfo} 为某一远端用户设置的空间音频效果。
  /// @param uid 远端用户 ID。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。

  FutureOr<int> removeRemotePosition(NSString uid) async {
    return await nativeCall('removeRemotePosition:', [uid]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author wangjunzheng
  /// @brief 移除调用 updateRemotePosition:positionInfo:{@link #ByteRTCSpatialAudio#updateRemotePosition:positionInfo} 为所有远端用户设置的空间音频效果。
  /// @return
  ///        - 0：成功。
  ///        - <0：失败。

  FutureOr<int> removeAllRemotePosition() async {
    return await nativeCall('removeAllRemotePosition', []);
  }
}

class ByteRTCRTSRoom extends NativeClass {
  static const _$namespace = r'ByteRTCRTSRoom';
  static get codegen_$namespace => _$namespace;

  ByteRTCRTSRoom([NativeClassOptions? options])
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

  /// @detail callback
  FutureOr<ByteRTCRTSRoomDelegate?> get delegateRts async {
    try {
      final result =
          await sendInstanceGet<ByteRTCRTSRoomDelegate?>("delegateRts");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCRTSRoomDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegateRts(FutureOr<ByteRTCRTSRoomDelegate?> value) {
    sendInstanceSet("delegateRts", value);
  }

  /// @detail api
  /// @brief 退出并销毁调用 createRTSRoom:{@link #ByteRTCEngine#createRTSRoom} 所创建的 RTS 房间实例。

  FutureOr<void> destroy() async {
    return await nativeCall('destroy', []);
  }

  /// @detail api
  /// @brief 通过设置 ByteRTCRTSRoomDelegate{@link #ByteRTCRTSRoomDelegate}代理，可以监听此 `ByteRTCRTSRoom` 对象对应的回调事件。
  /// @param roomDelegate 参见 ByteRTCRTSRoomDelegate{@link #ByteRTCRTSRoomDelegate}。
  /// @return  <br>
  ///        + 0: 调用成功。
  ///        + < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> setRTCRoomDelegateRts(
      id<ByteRTCRTSRoomDelegate> roomDelegate) async {
    return await nativeCall('setRTCRoomDelegateRts:', [roomDelegate]);
  }

  /// @detail api
  /// @brief 离开房间。 <br>
  ///        用户调用此方法离开房间，结束实时消息通信，释放所有通信相关的资源。
  /// @return 方法调用结果。 <br>
  ///        + 0: 方法调用成功 <br>
  ///        + < 0: 方法调用失败 <br>
  /// @note <br>
  ///       + 可见的用户离开房间后，房间内其他用户会收到 rtsRoom:onUserLeave:reason:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserLeave:reason} 回调通知；  <br>
  ///       + 此方法是异步操作，调用返回时并没有真正退出房间。真正退出房间后，本地会收到 rtsRoom:OnLeaveRoom:{@link #ByteRTCRTSRoomDelegate#rtsRoom:OnLeaveRoom} 回调通知。  <br>
  ///       + 如果调用此方法后立即销毁引擎，SDK 将无法触发 rtsRoom:OnLeaveRoom:{@link #ByteRTCRTSRoomDelegate#rtsRoom:OnLeaveRoom} 回调。

  FutureOr<int> leaveRoom() async {
    return await nativeCall('leaveRoom', []);
  }

  /// @detail api
  /// @brief 给房间内的所有其他用户发送文本消息。
  /// @param message  <br>
  ///        发送的文本消息内容。  <br>
  ///        消息不超过 64KB。
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note  <br>
  ///      + 在发送房间内文本消息前，必须先调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 加入房间。  <br>
  ///      + 调用该函数后会收到一次 rtsRoom:onRoomMessageSendResult:error:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomMessageSendResult:error} 回调，通知消息发送方发送成功或失败；  <br>
  ///      + 若文本消息发送成功，则房间内远端用户会收到 rtsRoom:onRoomMessageReceived:message:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomMessageReceived:message} 回调。
  ///

  FutureOr<int64_t> sendRoomMessage(NSString message) async {
    return await nativeCall('sendRoomMessage:', [message]);
  }

  /// @detail api
  /// @brief 给房间内的所有其他用户发送二进制消息。
  /// @param message  <br>
  ///        用户发送的二进制广播消息  <br>
  ///        消息不超过 64KB。
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note  <br>
  ///      + 在房间内广播二进制消息前，必须先调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 加入房间。  <br>
  ///      + 调用该函数后会收到一次 rtsRoom:onRoomMessageSendResult:error:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomMessageSendResult:error} 回调，通知消息发送方发送成功或失败；  <br>
  ///      + 若二进制消息发送成功，则房间内所有用户会收到 rtsRoom:onRoomBinaryMessageReceived:message:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomBinaryMessageReceived:message} 回调。
  ///

  FutureOr<int64_t> sendRoomBinaryMessage(NSData message) async {
    return await nativeCall('sendRoomBinaryMessage:', [message]);
  }

  /// @detail api
  /// @brief 更新 Token。 <br>
  ///        收到 onTokenWillExpire:{@link #ByteRTCRoomDelegate#onTokenWillExpire}，onPublishPrivilegeTokenWillExpire:{@link #ByteRTCRoomDelegate#onPublishPrivilegeTokenWillExpire}，或 onSubscribePrivilegeTokenWillExpire:{@link #ByteRTCRoomDelegate#onSubscribePrivilegeTokenWillExpire} 时，你必须重新获取 Token，并调用此方法更新 Token，以保证通话的正常进行。
  /// @param token 重新获取的有效 Token。 <br>
  ///        如果 Token 无效，你会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo}，错误码是 `-1010`。
  /// @return
  ///        - 0：成功；
  ///        - !0：失败。
  /// @note 请勿同时调用 updateToken:{@link #ByteRTCRTSRoom#updateToken} 和 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 重新加入房间。

  FutureOr<int> updateToken(NSString token) async {
    return await nativeCall('updateToken:', [token]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点文本消息（P2P）。
  /// @param userId <br>
  ///        消息接收用户的 ID
  /// @param message <br>
  ///        发送的文本消息内容。 <br>
  ///        消息不超过 64 KB。
  /// @param config <br>
  ///        消息发送的可靠/有序类型，参看 ByteRTCMessageConfig{@link #ByteRTCMessageConfig}
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note
  ///      - 在发送房间内文本消息前，必须先调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 加入房间。
  ///      - 调用该函数后会收到一次 rtsRoom:onUserMessageSendResult:error:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserMessageSendResult:error} 回调，通知消息发送方发送成功或失败。
  ///      - 若文本消息发送成功，则 uid 所指定的用户会收到 rtsRoom:onUserMessageReceived:message:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserMessageReceived:message} 回调。
  ///

  FutureOr<int64_t> sendUserMessage(
      NSString userId, NSString message, ByteRTCMessageConfig config) async {
    return await nativeCall(
        'sendUserMessage:message:config:', [userId, message, config.$value]);
  }

  /// @detail api
  /// @brief 给房间内指定的用户发送点对点二进制消息（P2P）。
  /// @param uid <br>
  ///        消息接收用户的 ID
  /// @param message <br>
  ///        发送的二进制消息内容 <br>
  ///        消息不超过 64KB。
  /// @param config <br>
  ///        消息发送的可靠/有序类型，参看 ByteRTCMessageConfig{@link #ByteRTCMessageConfig}。
  /// @return 这次发送消息的编号，从 1 开始递增。
  /// @note
  ///      - 在发送房间内二进制消息前，必须先调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 加入房间。
  ///      - 调用该函数后会收到一次 rtsRoom:onUserMessageSendResult:error:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserMessageSendResult:error} 回调，通知消息发送方发送成功或失败；
  ///      - 若二进制消息发送成功，则 uid 所指定的用户会收到 rtsRoom:onUserBinaryMessageReceived:message:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserBinaryMessageReceived:message} 回调。
  ///

  FutureOr<int64_t> sendUserBinaryMessage(
      NSString uid, NSData message, ByteRTCMessageConfig config) async {
    return await nativeCall(
        'sendUserBinaryMessage:message:config:', [uid, message, config.$value]);
  }

  /// @detail api
  /// @brief 加入 RTS 房间。 <br>
  ///        调用 createRTSRoom:{@link #ByteRTCEngine#createRTSRoom} 创建房间后，调用此方法加入房间，同房间内其他用户进行音视频通话。
  /// @param token 动态密钥，用于对进房用户进行鉴权验证。 <br>
  ///        进入房间需要携带 Token。测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。 <br>
  ///        使用不同 AppID 的 App 是不能互通的。 <br>
  ///        请务必保证生成 Token 使用的 AppID 和创建引擎时使用的 AppID 相同，否则会导致加入房间失败。
  /// @param userInfo 用户信息。参看 ByteRTCUserInfo{@link #ByteRTCUserInfo}。
  /// @return 方法调用结果。 <br>
  ///        -  0: 成功。触发以下回调：
  ///          - 本端收到房间状态通知 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///        - -1: 参数无效
  ///        - -2: 已经在房间内。接口调用成功后，只要收到返回值为 0 ，且未调用 leaveRoom{@link #ByteRTCRoom#leaveRoom} 成功，则再次调用进房接口时，无论填写的房间 ID 和用户 ID 是否重复，均触发此返回值。
  ///        调用失败时，具体失败原因会通过 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调告知。
  /// @note
  ///        - 同一个 AppID 的同一个房间内，每个用户的用户 ID 必须是唯一的。如果两个用户的用户 ID 相同，则后加入房间的用户会将先加入房间的用户踢出房间，并且先加入房间的用户会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知，错误类型为重复登录 ByteRTCErrorCodeDuplicateLogin。
  ///        - 本地用户调用此方法加入房间成功后，会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo} 回调通知。若本地用户同时为可见用户，加入房间时远端用户会收到 rtsRoom:onUserJoined:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onUserJoined} 回调通知。
  ///        - 用户加入房间成功后，在本地网络状况不佳的情况下，SDK 可能会与服务器失去连接，并触发 rtcEngine:onConnectionStateChanged:{@link #ByteRTCEngineDelegate#rtcEngine:onConnectionStateChanged} 回调。此时 SDK 会自动重试，直到成功重连。重连成功后，本地会收到 rtsRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRTSRoomDelegate#rtsRoom:onRoomStateChanged:withUid:state:extraInfo}。

  FutureOr<int> joinRTSRoom(NSString token, ByteRTCUserInfo userInfo) async {
    return await nativeCall('joinRTSRoom:userInfo:', [token, userInfo]);
  }
}

class ByteRTCAudioEffectPlayer extends NativeClass {
  static const _$namespace = r'ByteRTCAudioEffectPlayer';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioEffectPlayer([NativeClassOptions? options])
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
  ///        如果使用相同的 ID 重复调用本方法后，上一个音效会停止，下一个音效开始，并收到 onAudioEffectPlayerStateChanged:state:error:{@link #ByteRTCAudioEffectPlayerEventHandler#onAudioEffectPlayerStateChanged:state:error}。
  /// @param filePath 音效文件路径。 <br>
  ///        支持在线文件的 URL、本地文件的 URI、或本地文件的绝对路径。对于在线文件的 URL，仅支持 https 协议。 <br>
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
  /// @param config 音效配置，详见 ByteRTCAudioEffectPlayerConfig{@link #ByteRTCAudioEffectPlayerConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 如果已经通过 preload:filePath:{@link #ByteRTCAudioEffectPlayer#preload:filePath} 将文件加载至内存，确保此处的 ID 与 `preload` 设置的 ID 相同。
  ///       - 开始播放音效文件后，可以调用 stop:{@link #ByteRTCAudioEffectPlayer#stop} 方法停止播放音效文件。

  FutureOr<int> start(int effectId, NSString filePath,
      ByteRTCAudioEffectPlayerConfig config) async {
    return await nativeCall(
        'start:filePath:config:', [effectId, filePath, config]);
  }

  /// @detail api
  /// @brief 停止播放音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 方法开始播放音效文件后，可以调用本方法停止播放音效文件。
  ///       - 调用本方法停止播放音效文件后，该音效文件会被自动卸载。

  FutureOr<int> stop(int effectId) async {
    return await nativeCall('stop:', [effectId]);
  }

  /// @detail api
  /// @brief 停止播放所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 方法开始播放音效文件后，可以调用本方法停止播放所有音效文件。
  ///       - 调用本方法停止播放所有音效文件后，该音效文件会被自动卸载。

  FutureOr<int> stopAll() async {
    return await nativeCall('stopAll', []);
  }

  /// @detail api
  /// @brief 预加载指定音乐文件到内存中，以避免频繁播放同一文件时的重复加载，减少 CPU 占用。
  /// @param effectId 音效 ID。用于标识音效，请保证音效 ID 唯一性。 <br>
  ///        如果使用相同的 ID 重复调用本方法，后一次会覆盖前一次。 <br>
  ///        如果先调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config}，再使用相同的 ID 调用本方法 ，会收到回调 onAudioEffectPlayerStateChanged:state:error:{@link #ByteRTCAudioEffectPlayerEventHandler#onAudioEffectPlayerStateChanged:state:error}，通知前一个音效停止，然后加载下一个音效。 <br>
  ///        调用本方法预加载 A.mp3 后，如果需要使用相同的 ID 调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 播放 B.mp3，请先调用 unload:{@link #ByteRTCAudioEffectPlayer#unload} 卸载 A.mp3 ，否则会报错 AUDIO_MIXING_ERROR_LOAD_CONFLICT。
  /// @param filePath 音效文件路径。支持本地文件的 URI、或本地文件的绝对路径。 <br>
  ///                 预加载的文件长度不得超过 20s。 <br>
  ///                 不同平台支持的音效文件格式和 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 一致。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 本方法只是预加载指定音效文件，只有调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 方法才开始播放指定音效文件。
  ///       - 调用本方法预加载的指定音效文件可以通过 unload:{@link #ByteRTCAudioEffectPlayer#unload} 卸载。

  FutureOr<int> preload(int effectId, NSString filePath) async {
    return await nativeCall('preload:filePath:', [effectId, filePath]);
  }

  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 卸载指定音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 调用本方法卸载该文件后，关于当前的混音状态，如果设置了 setEventHandler:{@link #ByteRTCAudioEffectPlayer#setEventHandler}，会收到回调 `onAudioEffectPlayerStateChanged`。

  FutureOr<int> unload(int effectId) async {
    return await nativeCall('unload:', [effectId]);
  }

  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 卸载所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 调用本方法卸载该文件后，关于当前的混音状态，如果设置了 setEventHandler:{@link #ByteRTCAudioEffectPlayer#setEventHandler}，会收到回调 `onAudioEffectPlayerStateChanged`。

  FutureOr<int> unloadAll() async {
    return await nativeCall('unloadAll', []);
  }

  /// @detail api
  /// @brief 暂停播放音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 方法开始播放音效文件后，可以通过调用本方法暂停播放音效文件。
  ///       - 调用本方法暂停播放音效文件后，可调用 resume:{@link #ByteRTCAudioEffectPlayer#resume} 方法恢复播放。

  FutureOr<int> pause(int effectId) async {
    return await nativeCall('pause:', [effectId]);
  }

  /// @detail api
  /// @brief 暂停播放所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 方法开始播放音效文件后，可以通过调用本方法暂停播放所有音效文件。
  ///       - 调用本方法暂停播放所有音效文件后，可调用 resumeAll{@link #ByteRTCAudioEffectPlayer#resumeAll} 方法恢复所有播放。

  FutureOr<int> pauseAll() async {
    return await nativeCall('pauseAll', []);
  }

  /// @detail api
  /// @brief 恢复播放音效文件。
  /// @param effectId 音效 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 调用 pause:{@link #ByteRTCAudioEffectPlayer#pause} 方法暂停播放音效文件后，可以通过调用本方法恢复播放。

  FutureOr<int> resume(int effectId) async {
    return await nativeCall('resume:', [effectId]);
  }

  /// @detail api
  /// @brief 恢复播放所有音效文件。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 调用 pauseAll{@link #ByteRTCAudioEffectPlayer#pauseAll} 方法暂停所有正在播放音效文件后，可以通过调用本方法恢复播放。

  FutureOr<int> resumeAll() async {
    return await nativeCall('resumeAll', []);
  }

  /// @detail api
  /// @brief 设置音效文件的起始播放位置。
  /// @param effectId 音效 ID
  /// @param position 音效文件起始播放位置，单位为毫秒。 <br>
  ///        你可以通过 getDuration:{@link #ByteRTCAudioEffectPlayer#getDuration} 获取音效文件总时长，position 的值应小于音效文件总时长。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 在播放在线文件时，调用此接口可能造成播放延迟的现象。
  ///        - 仅在调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 后调用此接口。

  FutureOr<int> setPosition(int effectId, int position) async {
    return await nativeCall('setPosition:position:', [effectId, position]);
  }

  /// @detail api
  /// @brief 获取音效文件播放进度。
  /// @param effectId 音效 ID
  /// @return
  ///        - >0: 成功, 音效文件播放进度，单位为毫秒。
  ///        - < 0: 失败
  /// @note
  ///        - 在播放在线文件时，调用此接口可能造成播放延迟的现象。
  ///        - 仅在调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 后调用此接口。

  FutureOr<int> getPosition(int effectId) async {
    return await nativeCall('getPosition:', [effectId]);
  }

  /// @detail api
  /// @brief 调节指定音效的音量大小，包括音效文件和 PCM 音频。
  /// @param effectId 音效 ID
  /// @param volume 播放音量相对原音量的比值。单位为 \%。范围为 `[0, 400]`，建议范围是 `[0, 100]`。带溢出保护。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 仅在调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 后调用此接口。

  FutureOr<int> setVolume(int effectId, int volume) async {
    return await nativeCall('setVolume:volume:', [effectId, volume]);
  }

  /// @detail api
  /// @brief 设置所有音效的音量大小，包括音效文件和 PCM 音效。
  /// @param volume 播放音量相对原音量的比值。单位为 \%。范围为 `[0, 400]`，建议范围是 `[0, 100]`。带溢出保护。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 该接口的优先级低于 setVolume:volume:{@link #ByteRTCAudioEffectPlayer#setVolume:volume}，即通过 `setVolume` 单独设置了音量的音效 ID，不受该接口设置的影响。

  FutureOr<int> setVolumeAll(int volume) async {
    return await nativeCall('setVolumeAll:', [volume]);
  }

  /// @detail api
  /// @brief 获取当前音量。
  /// @param effectId 音效 ID
  /// @return
  ///        - >0: 成功, 当前音量值。
  ///        - < 0: 失败
  /// @note 仅在调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 后调用此接口。

  FutureOr<int> getVolume(int effectId) async {
    return await nativeCall('getVolume:', [effectId]);
  }

  /// @detail api
  /// @brief 获取音效文件时长。
  /// @param effectId 音效 ID
  /// @return
  ///        - >0: 成功, 音效文件时长，单位为毫秒。
  ///        - < 0: 失败
  /// @note 仅在调用 start:filePath:config:{@link #ByteRTCAudioEffectPlayer#start:filePath:config} 后调用此接口。

  FutureOr<int> getDuration(int effectId) async {
    return await nativeCall('getDuration:', [effectId]);
  }

  /// @detail api
  /// @brief 设置回调句柄。
  /// @param handler 参看 ByteRTCAudioEffectPlayerEventHandler{@link #ByteRTCAudioEffectPlayerEventHandler}。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。

  FutureOr<int> setEventHandler(
      id<ByteRTCAudioEffectPlayerEventHandler> handler) async {
    return await nativeCall('setEventHandler:', [handler]);
  }
}

enum ByteRTCUserOfflineReason {
  /// @brief 远端用户调用 `leaveRoom` 方法主动退出房间。
  ///
  ByteRTCUserOfflineReasonQuit(0),

  /// @brief 远端用户因网络等原因掉线。
  ///
  ByteRTCUserOfflineReasonDropped(1),

  /// @brief 远端用户切换至隐身状态。
  ///
  ByteRTCUserOfflineReasonSwitchToInvisible(2),

  /// @brief 远端用户被踢出出房间。 <br>
  ///        因调用踢出用户的 OpenAPI，远端用户被踢出房间。
  ///
  ByteRTCUserOfflineReasonKickedByAdmin(3);

  final dynamic $value;
  const ByteRTCUserOfflineReason([this.$value]);
}

enum ByteRTCAudioCodecType {
  /// @brief 未知编码类型
  ///
  ByteRTCAudioCodecTypeNone(0),

  /// @brief Opus 编码类型
  ///
  ByteRTCAudioCodecTypeOpus(1),

  /// @hidden currently not available
  ///
  ByteRTCAudioCodecTypeAAC(2),

  /// @hidden currently not available
  ///
  ByteRTCAudioCodecTypeAACLC(2),

  /// @hidden currently not available
  ///
  ByteRTCAudioCodecTypeAACHEv1(3),

  /// @hidden currently not available
  ///
  ByteRTCAudioCodecTypeAACHEv2(4),

  /// @hidden currently not available
  ///
  ByteRTCAudioCodecTypeAACLCadts(5);

  final dynamic $value;
  const ByteRTCAudioCodecType([this.$value]);
}

class ByteRTCAudioSource extends NativeClass {
  static const _$namespace = r'ByteRTCAudioSource';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioSource([NativeClassOptions? options])
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
}

class ByteRTCClientMixedStreamConfig extends NativeClass {
  static const _$namespace = r'ByteRTCClientMixedStreamConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCClientMixedStreamConfig([NativeClassOptions? options])
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

  /// @brief 客户端合流是否使用混音，默认为 true。
  FutureOr<BOOL?> get useAudioMixer async {
    return await sendInstanceGet<BOOL?>("useAudioMixer");
  }

  set useAudioMixer(FutureOr<BOOL?> value) {
    sendInstanceSet("useAudioMixer", value);
  }

  /// @brief 客户端合流回调视频格式，参看 ByteRTCMixedStreamClientMixVideoFormat{@link #ByteRTCMixedStreamClientMixVideoFormat}。
  FutureOr<ByteRTCMixedStreamClientMixVideoFormat?> get videoFormat async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamClientMixVideoFormat?>(
              "videoFormat");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamClientMixVideoFormat.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoFormat(FutureOr<ByteRTCMixedStreamClientMixVideoFormat?> value) {
    sendInstanceSet("videoFormat", value);
  }
}

class ByteRTCMediaPlayer extends NativeClass {
  static const _$namespace = r'ByteRTCMediaPlayer';
  static get codegen_$namespace => _$namespace;

  ByteRTCMediaPlayer([NativeClassOptions? options])
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
  ///        要播放 PCM 格式的音频数据，参看 openWithCustomSource:config:{@link #ByteRTCMediaPlayer#openWithCustomSource:config}。`openWithCustomSource` 和此 API 互斥。
  /// @param filePath 音乐文件路径。 <br>
  ///        支持在线文件的 URL、本地文件的 URI、或本地文件的绝对路径。对于在线文件的 URL，仅支持 https 协议。 <br>
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
  /// @param config 详见 ByteRTCMediaPlayerConfig{@link #ByteRTCMediaPlayerConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> open(NSString filePath, ByteRTCMediaPlayerConfig config) async {
    return await nativeCall('open:config:', [filePath, config]);
  }

  /// @detail api
  /// @brief 播放音乐。你仅需要在调用 open:config:{@link #ByteRTCMediaPlayer#open:config}，且未开启自动播放时，调用此方法。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  /// - 要播放 PCM 格式的音频数据，参看 openWithCustomSource:config:{@link #ByteRTCMediaPlayer#openWithCustomSource:config}。`openWithCustomSource` 和此 API 互斥。
  /// - 调用本方法播放音频文件后，可调用 stop{@link #ByteRTCMediaPlayer#stop} 方法暂停播放。

  FutureOr<int> start() async {
    return await nativeCall('start', []);
  }

  /// @detail api
  /// @brief 启动音频裸数据混音。 <br>
  ///        要播放音乐文件，参看 open:config:{@link #ByteRTCMediaPlayer#open:config}。`open` 与此 API 互斥。
  /// @param source 数据源，详见 ByteRTCMediaPlayerCustomSource{@link #ByteRTCMediaPlayerCustomSource}
  /// @param config 详见 ByteRTCMediaPlayerConfig{@link #ByteRTCMediaPlayerConfig}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用本方法启动后，再调用 pushExternalAudioFrame:{@link #ByteRTCEngine#pushExternalAudioFrame} 推送音频数据，才会开始混音。
  ///       - 如要结束 PCM 音频数据混音，调用 stop{@link #ByteRTCMediaPlayer#stop}。

  FutureOr<int> openWithCustomSource(ByteRTCMediaPlayerCustomSource source,
      ByteRTCMediaPlayerConfig config) async {
    return await nativeCall('openWithCustomSource:config:', [source, config]);
  }

  /// @detail api
  /// @brief 调用 open:config:{@link #ByteRTCMediaPlayer#open:config}, start{@link #ByteRTCMediaPlayer#start}, 或 openWithCustomSource:config:{@link #ByteRTCMediaPlayer#openWithCustomSource:config} 开始播放后，可以调用本方法停止。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> stop() async {
    return await nativeCall('stop', []);
  }

  /// @detail api
  /// @brief 调用 open:config:{@link #ByteRTCMediaPlayer#open:config}，或 start{@link #ByteRTCMediaPlayer#start} 开始播放音频文件后，调用本方法暂停播放。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  /// - 调用本方法暂停播放后，可调用 resume{@link #ByteRTCMediaPlayer#resume} 恢复播放。
  /// - 此接口仅支持音频文件，不支持 PCM 数据。

  FutureOr<int> pause() async {
    return await nativeCall('pause', []);
  }

  /// @detail api
  /// @brief 调用 pause{@link #ByteRTCMediaPlayer#pause} 暂停音频播放后，调用本方法恢复播放。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        此接口仅支持音频文件，不支持 PCM 数据。

  FutureOr<int> resume() async {
    return await nativeCall('resume', []);
  }

  /// @detail api
  /// @brief 调节指定混音的音量大小，包括音乐文件混音和 PCM 混音。
  /// @param volume 播放音量相对原音量的比值。单位为 \%。范围为 `[0, 400]`，建议范围是 `[0, 100]`。带溢出保护。
  /// @param type 详见 ByteRTCAudioMixingType{@link #ByteRTCAudioMixingType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。

  FutureOr<int> setVolume(int volume, ByteRTCAudioMixingType type) async {
    return await nativeCall('setVolume:type:', [volume, type.$value]);
  }

  /// @detail api
  /// @brief 获取当前音量
  /// @param type 详见 ByteRTCAudioMixingType{@link #ByteRTCAudioMixingType}。
  /// @return
  ///        - >0: 成功, 当前音量值。
  ///        - < 0: 失败
  /// @note 仅在音频播放进行状态时，调用此方法。包括音乐文件混音和 PCM 混音。

  FutureOr<int> getVolume(ByteRTCAudioMixingType type) async {
    return await nativeCall('getVolume:', [type.$value]);
  }

  /// @detail api
  /// @brief 获取音乐文件时长。
  /// @return
  ///        - >0: 成功, 音乐文件时长，单位为毫秒。
  ///        - < 0: 失败
  /// @note
  ///        - 仅在音频播放进行状态时，调用此方法。
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。

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
  ///        - 仅在音频播放进行状态，且 setProgressInterval:{@link #ByteRTCMediaPlayer#setProgressInterval} 设置间隔大于 `0` 时，调用此方法。
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。

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

  FutureOr<int> getPosition() async {
    return await nativeCall('getPosition', []);
  }

  /// @valid since 3.59
  /// @detail api
  /// @author wangfeng.1004
  /// @brief 获取播放器状态
  /// @return 播放器当前状态，参看 ByteRTCPlayerState{@link #ByteRTCPlayerState}。
  /// @note 仅在音频实例创建后，调用此方法。

  FutureOr<ByteRTCPlayerState> getState() async {
    return await nativeCall('getState', []);
  }

  /// @detail api
  /// @brief 开启变调功能，多用于 K 歌场景。
  /// @param pitch 与音乐文件原始音调相比的升高/降低值，取值范围为 `[-12，12]`，默认值为 0。每相邻两个值的音高距离相差半音，正值表示升调，负值表示降调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 仅支持音乐文件混音，不支持 PCM 数据。

  FutureOr<int> setAudioPitch(int pitch) async {
    return await nativeCall('setAudioPitch:', [pitch]);
  }

  /// @detail api
  /// @brief 设置音乐文件的起始播放位置。
  /// @param position 音乐文件起始播放位置，单位为毫秒。 <br>
  ///        你可以通过 getTotalDuration{@link #ByteRTCMediaPlayer#getTotalDuration} 获取音乐文件总时长，position 的值应小于音乐文件总时长。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 此接口仅支持音频文件，不支持 PCM 数据。
  ///        - 在播放在线文件时，调用此接口可能造成播放延迟的现象。
  ///        - 调用本接口后，会收到 onMediaPlayerEvent:event:message:{@link #ByteRTCMediaPlayerEventHandler#onMediaPlayerEvent:event:message} 回调。

  FutureOr<int> setPosition(int position) async {
    return await nativeCall('setPosition:', [position]);
  }

  /// @detail api
  /// @brief 设置当前音乐文件的声道模式
  /// @param mode 声道模式。默认的声道模式和源文件一致，详见 ByteRTCAudioMixingDualMonoMode{@link #ByteRTCAudioMixingDualMonoMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 仅支持音频文件，不支持 PCM 数据。

  FutureOr<int> setAudioDualMonoMode(
      ByteRTCAudioMixingDualMonoMode mode) async {
    return await nativeCall('setAudioDualMonoMode:', [mode.$value]);
  }

  /// @detail api
  /// @brief 获取当前音乐文件的音轨数
  /// @return + >= 0：成功，返回当前音乐文件的音轨数
  ///         - < 0：方法调用失败
  /// @note
  ///        - 仅在音频播放进行状态时，调用此方法。
  ///        - 此方法仅支持音乐文件，不支持 PCM 数据。

  FutureOr<int> getAudioTrackCount() async {
    return await nativeCall('getAudioTrackCount', []);
  }

  /// @detail api
  /// @brief 指定当前音乐文件的播放音轨
  /// @param index 指定的播放音轨，从 0 开始，取值范围为 `[0, getAudioTrackCount()-1]`。 <br>
  ///        设置的参数值需要小于 getAudioTrackCount{@link #ByteRTCMediaPlayer#getAudioTrackCount} 的返回值
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 仅在音频播放进行状态时，调用此方法。
  ///        - 此方法仅支持音乐文件，不支持 PCM 数据。
  ///        - 调用本接口后，会收到 onMediaPlayerEvent:event:message:{@link #ByteRTCMediaPlayerEventHandler#onMediaPlayerEvent:event:message} 回调。

  FutureOr<int> selectAudioTrack(int index) async {
    return await nativeCall('selectAudioTrack:', [index]);
  }

  /// @detail api
  /// @brief 设置播放速度
  /// @param speed 播放速度与原始文件速度的比例，单位：\%，取值范围为 `[50,200]`，默认值为 100。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 此方法对音频文件可用，不支持 PCM 数据。

  FutureOr<int> setPlaybackSpeed(int speed) async {
    return await nativeCall('setPlaybackSpeed:', [speed]);
  }

  /// @detail api
  /// @brief 设置音频文件混音时，收到 onMediaPlayerPlayingProgress:progress:{@link #ByteRTCMediaPlayerEventHandler#onMediaPlayerPlayingProgress:progress} 的间隔。
  /// @param interval 时间间隔，单位毫秒。 <br>
  ///       - interval > 0 时，触发回调。实际间隔为 10 的倍数。如果输入数值不能被 10 整除，将自动向上取整。例如传入 `52`，实际间隔为 60 ms。
  ///       - interval <= 0 时，不会触发回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 此方法仅支持音频文件，不支持 PCM 数据。

  FutureOr<int> setProgressInterval(int64_t interval) async {
    return await nativeCall('setProgressInterval:', [interval]);
  }

  /// @detail api
  /// @brief 如果你需要使用 enableVocalInstrumentBalance:{@link #ByteRTCEngine#enableVocalInstrumentBalance} 对音频文件/PCM 音频数据设置音量均衡，你必须通过此接口传入其原始响度。
  /// @param loudness 原始响度，单位：lufs，取值范围为 `[-70.0, 0.0]`。 <br>
  ///        当设置的值小于 -70.0lufs 时，则默认调整为 -70.0lufs，大于 0.0lufs 时，则不对该响度做音量均衡处理。默认值为 1.0lufs，即不做处理。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 如果在起播前调用此接口，设置的参数值将被缓存，在起播后生效。
  ///        - 此方法对音频文件和音频裸数据播放都可用。

  FutureOr<int> setLoudness(float loudness) async {
    return await nativeCall('setLoudness:', [loudness]);
  }

  /// @detail api
  /// @brief 注册回调句柄以在本地音乐文件混音时，收到相关回调。
  /// @param observer 参看 ByteRTCMediaPlayerAudioFrameObserver{@link #ByteRTCMediaPlayerAudioFrameObserver}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        此接口仅支持音频文件，不支持 PCM 数据。

  FutureOr<int> registerAudioFrameObserver(
      id<ByteRTCMediaPlayerAudioFrameObserver> observer) async {
    return await nativeCall('registerAudioFrameObserver:', [observer]);
  }

  /// @detail api
  /// @brief 推送用于混音的 PCM 音频帧数据
  /// @param audioFrame 音频帧，详见 ByteRTCAudioFrame{@link #ByteRTCAudioFrame}。 <br>
  ///        - 音频采样格式必须为 S16。音频缓冲区内的数据格式必须为 PCM，其容量大小应该为 audioFrame.samples × audioFrame.channel × 2。
  ///        - 必须指定具体的采样率和声道数，不支持设置为自动。
  /// @return
  ///       - 0: 成功
  ///       - < 0: 失败
  /// @note
  ///      - 调用该方法前，须通过 openWithCustomSource:config:{@link #ByteRTCMediaPlayer#openWithCustomSource:config} 启动外部音频流混音。
  ///      - 使用参考建议：首次推送数据，请在应用侧先缓存一定数据（如 200 毫秒），然后一次性推送过去；此后的推送操作定时 10 毫秒一次，并且每次的音频数据量为 10 毫秒数据量。
  ///      - 如果要暂停播放，暂停推送即可。

  FutureOr<int> pushExternalAudioFrame(ByteRTCAudioFrame audioFrame) async {
    return await nativeCall('pushExternalAudioFrame:', [audioFrame]);
  }

  /// @detail api
  /// @brief 设置回调句柄。
  /// @param handler 参看 ByteRTCMediaPlayerEventHandler{@link #ByteRTCMediaPlayerEventHandler}。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。

  FutureOr<int> setEventHandler(
      id<ByteRTCMediaPlayerEventHandler> handler) async {
    return await nativeCall('setEventHandler:', [handler]);
  }
}

class ByteRTCAudioDeviceManager extends NativeClass {
  static const _$namespace = r'ByteRTCAudioDeviceManager';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioDeviceManager([NativeClassOptions? options])
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
  /// @author dixing
  /// @brief 获取当前系统内音频播放设备列表。
  /// @return 所有音频播放设备的列表，参看 ByteRTCDeviceCollection{@link #ByteRTCDeviceCollection}。 <br>
  /// 等待超时后会返回空列表。超时时间默认为 10 s。建议通过 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 监听到 `ByteRTCMediaDeviceListUpdated` 后，再次调用本接口获取。
  /// @note 你可以在收到 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 了解设备变更后，重新调用本接口以获得新的设备列表。 <br>

  FutureOr<ByteRTCDeviceCollection> enumerateAudioPlaybackDevices() async {
    final result = await nativeCall('enumerateAudioPlaybackDevices', []);
    return packObject(
        result,
        () => ByteRTCDeviceCollection(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取音频采集设备列表。
  /// @return 音频采集设备列表。详见 ByteRTCDeviceCollection{@link #ByteRTCDeviceCollection}。 <br>
  /// 等待超时后会返回空列表。超时时间默认为 10 s。建议通过 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 监听到 `ByteRTCMediaDeviceListUpdated` 后，再次调用本接口获取。
  /// @note 你可以在收到 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 了解设备变更后，重新调用本接口以获得新的设备列表。

  FutureOr<ByteRTCDeviceCollection> enumerateAudioCaptureDevices() async {
    final result = await nativeCall('enumerateAudioCaptureDevices', []);
    return packObject(
        result,
        () => ByteRTCDeviceCollection(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author yezijian.me
  /// @brief 设置音频采集路由是否跟随系统。
  /// @param followed <br>
  ///        - true: 跟随。此时，调用 setAudioCaptureDevice:{@link #ByteRTCAudioDeviceManager#setAudioCaptureDevice} 会失败。默认值。
  ///        - false: 不跟随系统。此时，可以调用 setAudioCaptureDevice:{@link #ByteRTCAudioDeviceManager#setAudioCaptureDevice} 进行设置。

  FutureOr<void> followSystemCaptureDevice(BOOL followed) async {
    return await nativeCall('followSystemCaptureDevice:', [followed]);
  }

  /// @detail api
  /// @author yezijian.me
  /// @brief 设置音频播放路由是否跟随系统。
  /// @param followed <br>
  ///        - true: 跟随。此时，调用 setAudioPlaybackDevice:{@link #ByteRTCAudioDeviceManager#setAudioPlaybackDevice} 会失败。默认值。
  ///        - false: 不跟随系统。此时，可以调用 setAudioPlaybackDevice:{@link #ByteRTCAudioDeviceManager#setAudioPlaybackDevice} 进行设置。

  FutureOr<void> followSystemPlaybackDevice(BOOL followed) async {
    return await nativeCall('followSystemPlaybackDevice:', [followed]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 设置音频播放设备。
  /// @param deviceID 音频播放设备 ID，可通过 enumerateAudioPlaybackDevices{@link #ByteRTCAudioDeviceManager#enumerateAudioPlaybackDevices} 获取。
  /// @return
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note 当你调用 followSystemPlaybackDevice:{@link #ByteRTCAudioDeviceManager#followSystemPlaybackDevice} 设置音频播放设备跟随系统后，将无法调用此接口设置音频播放设备。

  FutureOr<int> setAudioPlaybackDevice(NSString deviceID) async {
    return await nativeCall('setAudioPlaybackDevice:', [deviceID]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取当前音频播放设备。
  /// @param deviceID 设备 ID
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> getAudioPlaybackDevice(NSString deviceID) async {
    return await nativeCall('getAudioPlaybackDevice:', [deviceID]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 设置音频采集设备。
  /// @param deviceID 音频采集设备 ID。你可调用 enumerateAudioCaptureDevices{@link #ByteRTCAudioDeviceManager#EnumerateAudioCaptureDevices} 获取可用设备列表。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note 当你调用 followSystemCaptureDevice:{@link #ByteRTCAudioDeviceManager#followSystemCaptureDevice} 设置音频采集设备跟随系统后，将无法调用此接口设置音频采集设备。

  FutureOr<int> setAudioCaptureDevice(NSString deviceID) async {
    return await nativeCall('setAudioCaptureDevice:', [deviceID]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取当前音频采集设备。
  /// @param deviceID 音频采集设备 ID。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> getAudioCaptureDevice(NSString deviceID) async {
    return await nativeCall('getAudioCaptureDevice:', [deviceID]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 设置当前音频采集设备静音状态，默认为非静音。
  /// @param mute <br>
  ///       - true：静音
  ///       - false：非静音
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///      - 该方法用于静音整个系统的音频采集。你也可以仅对麦克风采集到的音频信号做静音处理，而不影响媒体播放器的音乐声音，具体参看 muteAudioCapture:{@link #ByteRTCEngine#muteAudioCapture} 方法说明。
  ///      - 设该方法为 `true` 静音后仍可通过 setAudioCaptureDeviceVolume:{@link #ByteRTCAudioDeviceManager#setAudioCaptureDeviceVolume} 调整采集音量，调整后的音量会在取消静音后生效。

  FutureOr<int> setAudioCaptureDeviceMute(bool mute) async {
    return await nativeCall('setAudioCaptureDeviceMute:', [mute]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取当前音频采集设备是否静音的信息。
  /// @param mute <br>
  ///       - true：静音
  ///       - false：非静音
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> getAudioCaptureDeviceMute(bool mute) async {
    return await nativeCall('getAudioCaptureDeviceMute:', [mute]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 设置当前音频播放设备静音状态，默认为非静音。
  /// @param mute <br>
  ///       - true：静音
  ///       - false：非静音
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> setAudioPlaybackDeviceMute(bool mute) async {
    return await nativeCall('setAudioPlaybackDeviceMute:', [mute]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取当前音频播放设备是否静音的信息。
  /// @param mute <br>
  ///       - true：静音
  ///       - false：非静音
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> getAudioPlaybackDeviceMute(bool mute) async {
    return await nativeCall('getAudioPlaybackDeviceMute:', [mute]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 设置当前音频采集设备音量
  /// @param volume 音频采集设备音量，取值范围为 [0,255]。 <br>
  ///       - [0,25] 接近无声；
  ///       - [25,75] 为低音量；
  ///       - [76,204] 为中音量；
  ///       - [205,255] 为高音量。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功。将影响 rtcEngine:onLocalAudioPropertiesReport:{@link #ByteRTCEngineDelegate#rtcEngine:onLocalAudioPropertiesReport} 回调的音量信息。
  ///        - < 0：方法调用失败
  /// @note 调用 setAudioCaptureDeviceMute:{@link #ByteRTCAudioDeviceManager#setAudioCaptureDeviceMute} 设为 `true` 静音采集设备后的音量调节会在取消静音后生效。

  FutureOr<int> setAudioCaptureDeviceVolume(int volume) async {
    return await nativeCall('setAudioCaptureDeviceVolume:', [volume]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取当前音频采集设备音量
  /// @param volume 音频采集设备音量，取值范围是 [0,255] <br>
  ///       - [0,25] 接近无声；
  ///       - [25,75] 为低音量；
  ///       - [76,204] 为中音量；
  ///       - [205,255] 为高音量。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> getAudioCaptureDeviceVolume(int volume) async {
    return await nativeCall('getAudioCaptureDeviceVolume:', [volume]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 设置当前音频播放设备音量
  /// @param volume 音频播放设备音量，取值范围为 [0,255] <br>
  ///       - [0,25] 接近无声；
  ///       - [25,75] 为低音量；
  ///       - [76,204] 为中音量；
  ///       - [205,255] 为高音量。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> setAudioPlaybackDeviceVolume(int volume) async {
    return await nativeCall('setAudioPlaybackDeviceVolume:', [volume]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 获取当前音频播放设备音量
  /// @param volume 音频播放设备音量，取值范围是 [0,255] <br>
  ///       - [0,25] 接近无声；
  ///       - [25,75] 为低音量；
  ///       - [76,204] 为中音量；
  ///       - [205,255] 为高音量。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败

  FutureOr<int> getAudioPlaybackDeviceVolume(int volume) async {
    return await nativeCall('getAudioPlaybackDeviceVolume:', [volume]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 启动音频播放设备测试。 <br>
  ///        该方法测试播放设备是否能正常工作。SDK 播放指定的音频文件，测试者如果能听到声音，说明播放设备能正常工作。
  /// @param testAudioFilePath 音频文件的绝对路径，路径字符串使用 UTF-8 编码格式，支持以下音频格式: mp3，aac，m4a，3gp，wav。
  /// @param interval 音频设备播放测试音量回调的间隔
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///       - 该方法必须在进房前调用，且不可与其它音频设备测试功能同时应用。
  ///       - 调用 stopAudioPlaybackDeviceTest{@link #ByteRTCAudioDeviceManager#stopAudioPlaybackDeviceTest} 停止测试。

  FutureOr<int> startAudioPlaybackDeviceTest(
      NSString testAudioFilePath, int interval) async {
    return await nativeCall('startAudioPlaybackDeviceTest:interval:',
        [testAudioFilePath, interval]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 停止音频播放设备测试。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note 调用 startAudioPlaybackDeviceTest:interval:{@link #ByteRTCAudioDeviceManager#startAudioPlaybackDeviceTest:interval} 后，需调用本方法停止测试。

  FutureOr<int> stopAudioPlaybackDeviceTest() async {
    return await nativeCall('stopAudioPlaybackDeviceTest', []);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author dixing
  /// @brief 开始音频采集设备和音频播放设备测试。
  /// @param interval 测试中会收到 `rtcEngine:onLocalAudioPropertiesReport:` 回调，本参数指定了该周期回调的时间间隔，单位为毫秒。建议设置到大于 200 毫秒。最小不得少于 10 毫秒。
  /// @return 方法调用结果 <br>
  ///       - 0：方法调用成功
  ///       - < 0：方法调用失败
  /// @note
  ///       - 该方法在进房前后均可调用。且不可与其它音频设备测试功能同时应用。
  ///       - 调用本接口 30 s 后，采集自动停止，并开始播放采集到的声音。录音播放完毕后，设备测试流程自动结束。你也可以在 30 s 内调用 stopAudioDeviceRecordAndPlayTest{@link #ByteRTCAudioDeviceManager#stopAudioDeviceRecordAndPlayTest}  来停止采集并开始播放此前采集到的声音。
  ///       - 调用 stopAudioDevicePlayTest{@link #ByteRTCAudioDeviceManager#stopAudioDevicePlayTest} 可以停止音频设备采集和播放测试。
  ///       - 你不应在测试过程中，调用 `enableAudioPropertiesReport:` 注册音量提示回调。
  ///       - 该方法仅在本地进行音频设备测试，不涉及网络连接。

  FutureOr<int> startAudioDeviceRecordTest(int interval) async {
    return await nativeCall('startAudioDeviceRecordTest:', [interval]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author dixing
  /// @brief 停止采集本地音频，并开始播放采集到的声音。录音播放完毕后，设备测试流程结束。 <br>
  /// 调用 startAudioDeviceRecordTest:{@link #ByteRTCAudioDeviceManager#startAudioDeviceRecordTest} 30s 内调用本接口来停止采集并开始播放此前采集到的声音。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///        - 该方法依赖 rtc 引擎，只有通过成员方法 getAudioDeviceManager{@link #ByteRTCEngine#getAudioDeviceManager} 创建的 ByteRTCAudioDeviceManager，该方法才是有效的
  ///        - 调用本接口开始播放录音后，可以在播放过程中调用 stopAudioDevicePlayTest{@link #ByteRTCAudioDeviceManager#stopAudioDevicePlayTest} 停止播放。

  FutureOr<int> stopAudioDeviceRecordAndPlayTest() async {
    return await nativeCall('stopAudioDeviceRecordAndPlayTest', []);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author dixing
  /// @brief 停止由调用 startAudioDeviceRecordTest:{@link #ByteRTCAudioDeviceManager#startAudioDeviceRecordTest} 开始的音频播放设备测试。 <br>
  ///        在音频播放设备测试自动结束前，可调用本接口停止音频采集与播放测试。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///        - 该方法依赖 rtc 引擎，只有通过成员方法 getAudioDeviceManager{@link #ByteRTCEngine#getAudioDeviceManager} 创建的 ByteRTCAudioDeviceManager，该方法才是有效的

  FutureOr<int> stopAudioDevicePlayTest() async {
    return await nativeCall('stopAudioDevicePlayTest', []);
  }

  /// @detail api
  /// @author dixing
  /// @brief 尝试初始化音频播放设备，以检测设备不存在、权限被拒绝/禁用等异常问题。
  /// @param deviceID 设备索引号
  /// @return 设备状态错误码 <br>
  ///        - 0: 设备检测结果正常
  ///        - -1: 接口调用失败
  ///        - -2: 设备无权限，尝试初始化设备失败
  ///        - -3: 设备不存在，当前没有设备或设备被移除时返回
  ///        - -4: 设备音频格式不支持
  ///        - -5: 其它原因错误
  /// @note
  ///        - 该接口需在进房前调用；
  ///        - 检测成功不代表设备一定可以启动成功，还可能因设备被其他应用进程独占，或 CPU/内存不足等原因导致启动失败。

  FutureOr<int> initAudioPlaybackDeviceForTest(NSString deviceID) async {
    return await nativeCall('initAudioPlaybackDeviceForTest:', [deviceID]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 尝试初始化音频采集设备，以检测设备不存在、权限被拒绝/禁用等异常问题。
  /// @param deviceID 设备索引号
  /// @return 设备状态错误码 <br>
  ///        - 0: 设备检测结果正常
  ///        - -1: 接口调用失败
  ///        - -2: 设备无权限，尝试初始化设备失败
  ///        - -3: 设备不存在，当前没有设备或设备被移除时返回
  ///        - -4: 设备音频格式不支持
  ///        - -5: 其它原因错误
  /// @note
  ///        - 该接口需在进房前调用；
  ///        - 检测成功不代表设备一定可以启动成功，还可能因设备被其他应用进程独占，或 CPU/内存不足等原因导致启动失败。

  FutureOr<int> initAudioCaptureDeviceForTest(NSString deviceID) async {
    return await nativeCall('initAudioCaptureDeviceForTest:', [deviceID]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 启动音频采集设备测试。 <br>
  ///        该方法测试音频采集设备是否能正常工作。启动测试后，会收到 rtcEngine:onLocalAudioPropertiesReport:{@link #ByteRTCEngineDelegate#rtcEngine:onLocalAudioPropertiesReport} 回调上报的音量信息。
  /// @param indicationInterval 获取回调的时间间隔，单位为毫秒。建议设置到大于 200 毫秒。最小不得少于 10 毫秒。小于 10 毫秒行为未定义。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///       - 该方法不依赖 rtc 引擎
  ///       - 该方法必须在进房前调用，且不可与其它音频设备测试功能同时应用。
  ///       - 你需调用 stopAudioRecordingDeviceTest{@link #ByteRTCAudioDeviceManager#stopAudioRecordingDeviceTest} 停止测试。

  FutureOr<int> startAudioRecordingDeviceTest(int indicationInterval) async {
    return await nativeCall(
        'startAudioRecordingDeviceTest:', [indicationInterval]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 停止音频采集设备测试。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///        - 该方法不依赖 rtc 引擎
  ///        - 调用 startAudioRecordingDeviceTest:{@link #ByteRTCAudioDeviceManager#startAudioRecordingDeviceTest} 后，需调用本方法停止测试。

  FutureOr<int> stopAudioRecordingDeviceTest() async {
    return await nativeCall('stopAudioRecordingDeviceTest', []);
  }

  /// @detail api
  /// @author dixing
  /// @brief 开始音频设备回路测试。 <br>
  ///        该方法测试音频采集设备和音频播放设备是否能正常工作。一旦测试开始，音频采集设备会采集本地声音并通过音频播放设备播放出来，同时会收到 rtcEngine:onLocalAudioPropertiesReport:{@link #ByteRTCEngineDelegate#rtcEngine:onLocalAudioPropertiesReport}。
  /// @param indicationInterval 收到回调的时间间隔，单位为 ms。建议设置到大于 200 ms。最小不得少于 10 ms。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///       - 该方法不依赖 rtc 引擎
  ///       - 该方法必须在进房前调用。且不可与其它音频设备测试功能同时应用。
  ///       - 你需调用 stopAudioDeviceLoopbackTest{@link #ByteRTCAudioDeviceManager#stopAudioDeviceLoopbackTest} 停止测试。
  ///       - 该方法仅在本地进行音频设备测试，不涉及网络连接。

  FutureOr<int> startAudioDeviceLoopbackTest(int indicationInterval) async {
    return await nativeCall(
        'startAudioDeviceLoopbackTest:', [indicationInterval]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 停止音频设备回路测试。
  /// @return 方法调用结果 <br>
  ///        - 0：方法调用成功
  ///        - < 0：方法调用失败
  /// @note
  ///        - 该方法不依赖 rtc 引擎
  ///        - 调用 startAudioDeviceLoopbackTest:{@link #ByteRTCAudioDeviceManager#startAudioDeviceLoopbackTest} 后，需调用本方法停止测试。

  FutureOr<int> stopAudioDeviceLoopbackTest() async {
    return await nativeCall('stopAudioDeviceLoopbackTest', []);
  }
}

class ByteRTCEngine extends NativeClass {
  static const _$namespace = r'ByteRTCEngine';
  static get codegen_$namespace => _$namespace;

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 创建引擎对象。 <br>
  ///        如果当前进程中未创建引擎实例，那么你必须先使用此方法，以使用 RTC 提供的各种音视频能力。 <br>
  ///        如果当前进程中已创建了引擎实例，再次调用此方法时，会返回已创建的引擎实例。
  /// @param config 创建引擎参数配置，详见 ByteRTCEngineConfig{@link #ByteRTCEngineConfig}
  /// @param delegate SDK 回调给应用层的 delegate，详见 ByteRTCEngineDelegate{@link #ByteRTCEngineDelegate}
  /// @return 可用的 ByteRTCEngine{@link #ByteRTCEngine} 实例

  static FutureOr<ByteRTCEngine> createRTCEngine(
      ByteRTCEngineConfig config, id<ByteRTCEngineDelegate> delegate) async {
    try {
      final result = await NativeClassUtils.nativeStaticCall(
        _$namespace,
        'createRTCEngine:delegate:',
        [config, delegate],
        'com.volcengine.rtc.hybrid_runtime',
      );
      return packObject(result,
          () => ByteRTCEngine(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      rethrow;
    }
  }

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 销毁由 createRTCEngine:delegate:{@link #ByteRTCEngine#createRTCEngine:delegate} 所创建的引擎实例，并释放所有相关资源。
  /// @note
  ///      - 请确保和需要销毁的 ByteRTCEngine{@link #ByteRTCEngine} 实例相关的业务场景全部结束后，才调用此方法
  ///      - 该方法在调用之后，会销毁所有和此 ByteRTCEngine{@link #ByteRTCEngine} 实例相关的内存，并且停止与媒体服务器的任何交互
  ///      - 调用本方法会启动 SDK 退出逻辑。引擎线程会保留，直到退出逻辑完成。因此，不要在回调线程中直接调用此 API，会导致死锁。同时此方法是耗时操作，不建议在主线程调用本方法，避免主线程阻塞。
  ///      - 可以通过 Objective-C 的 ARC 机制，在 dealloc 时自动触发销毁逻辑

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

  static FutureOr<NSString> getSDKVersion() async {
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
  /// @param logConfig 本地日志参数，参看 ByteRTCLogConfig{@link #ByteRTCLogConfig}。
  /// @return
  ///        - 0：成功。
  ///        - –1：失败，本方法必须在创建引擎前调用。
  ///        - –2：失败，参数填写错误。
  /// @note 本方法必须在调用 createRTCEngine:delegate:{@link #ByteRTCEngine#createRTCEngine:delegate} 之前调用。

  static FutureOr<int> setLogConfig(ByteRTCLogConfig logConfig) async {
    return await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'setLogConfig:',
      [logConfig],
      'com.volcengine.rtc.hybrid_runtime',
    );
  }

  ByteRTCEngine([NativeClassOptions? options])
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

  /// @detail callback
  FutureOr<ByteRTCEngineDelegate?> get delegate async {
    try {
      final result = await sendInstanceGet<ByteRTCEngineDelegate?>("delegate");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCEngineDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegate(FutureOr<ByteRTCEngineDelegate?> value) {
    sendInstanceSet("delegate", value);
  }

  FutureOr<ByteRTCMonitorDelegate?> get monitorDelegate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMonitorDelegate?>("monitorDelegate");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCMonitorDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set monitorDelegate(FutureOr<ByteRTCMonitorDelegate?> value) {
    sendInstanceSet("monitorDelegate", value);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 切换音频采集方式
  /// @param type 音频数据源，详见 ByteRTCAudioSourceType{@link #ByteRTCAudioSourceType}。 <br>
  ///             默认使用内部音频采集。音频采集和渲染方式无需对应。
  /// @return 方法调用结果： <br>
  ///        - =0: 切换成功。
  ///        - <0：切换失败。
  /// @note
  ///      - 进房前后调用此方法均有效。
  ///      - 如果你调用此方法由内部采集切换至自定义采集，SDK 会自动关闭内部采集。然后，调用 pushExternalAudioFrame:{@link #ByteRTCEngine#pushExternalAudioFrame} 推送自定义采集的音频数据到 RTC SDK 用于传输。
  ///      - 如果你调用此方法由自定义采集切换至内部采集，你必须再调用 startAudioCapture{@link #ByteRTCEngine#startAudioCapture} 手动开启内部采集。

  FutureOr<int> setAudioSourceType(ByteRTCAudioSourceType type) async {
    return await nativeCall('setAudioSourceType:', [type.$value]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 切换音频渲染方式
  /// @param type 音频输出类型，详见 ByteRTCAudioRenderType{@link #ByteRTCAudioRenderType} <br>
  ///             默认使用内部音频渲染。音频采集和渲染方式无需对应。
  /// @return 方法调用结果： <br>
  ///        - =0: 切换成功。
  ///        - <0：切换失败。
  /// @note
  ///      - 进房前后调用此方法均有效。
  ///      - 如果你调用此方法切换至自定义渲染，调用 pullExternalAudioFrame:{@link #ByteRTCEngine#pullExternalAudioFrame} 获取音频数据。

  FutureOr<int> setAudioRenderType(ByteRTCAudioRenderType type) async {
    return await nativeCall('setAudioRenderType:', [type.$value]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 开启内部音频采集。默认为关闭状态。 <br>
  ///        内部采集是指：使用 RTC SDK 内置的音频采集机制进行音频采集。 <br>
  ///        调用该方法开启后，本地用户会收到 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 的回调。 <br>
  ///        非隐身用户进房后调用该方法，房间中的其他用户会收到 rtcEngine:onUserStartAudioCapture:info:{@link #ByteRTCEngineDelegate#rtcEngine:onUserStartAudioCapture:info} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 若未取得当前设备的麦克风权限，调用该方法后会触发 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 回调，对应的错误码为 `ByteRTCMediaDeviceError.ByteRTCMediaDeviceErrorDeviceNoPermission = 1`。
  ///       - 调用 stopAudioCapture{@link #ByteRTCEngine#stopAudioCapture} 可以关闭音频采集设备，否则，SDK 只会在销毁引擎的时候自动关闭设备。
  ///       - 由于不同硬件设备初始化响应时间不同，频繁调用 stopAudioCapture{@link #ByteRTCEngine#stopAudioCapture} 和本接口闭麦/开麦可能出现短暂无声问题，建议使用 publishStreamAudio:{@link #ByteRTCRoom#publishStreamAudio} 实现临时闭麦和重新开麦。
  ///       - 创建引擎后，无论是否发布音频数据，你都可以调用该方法开启音频采集，并且调用后方可发布音频。
  ///       - 如果需要从自定义音频采集切换为内部音频采集，你必须先停止发布流，调用 setAudioSourceType:{@link #ByteRTCEngine#setAudioSourceType} 关闭自定义采集，再调用此方法手动开启内部采集。

  FutureOr<int> startAudioCapture() async {
    return await nativeCall('startAudioCapture', []);
  }

  /// @detail api
  /// @author dixing
  /// @brief 关闭内部音频采集。默认为关闭状态。 <br>
  ///        内部采集是指：使用 RTC SDK 内置的音频采集机制进行音频采集。 <br>
  ///        调用该方法，本地用户会收到 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 的回调。 <br>
  ///        非隐身用户进房后调用该方法，房间中的其他用户会收到 rtcEngine:onUserStopAudioCapture:info:{@link #ByteRTCEngineDelegate#rtcEngine:onUserStopAudioCapture:info} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 startAudioCapture{@link #ByteRTCEngine#startAudioCapture} 可以开启音频采集设备。
  ///       - 如果不调用本方法停止内部视频采集，则只有当销毁引擎实例时，内部音频采集才会停止。
  ///

  FutureOr<int> stopAudioCapture() async {
    return await nativeCall('stopAudioCapture', []);
  }

  /// @hidden(macOS)
  /// @valid since 3.60.
  /// @detail api
  /// @author gongzhengduo
  /// @brief 设置音频场景类型。 <br>
  ///        选择音频场景后，SDK 会自动根据场景切换对应的音量模式（通话音量/媒体音量）和改场景下的最佳音频配置。 <br>
  /// @param audioScenario 音频场景类型，参看 ByteRTCAudioScenarioType{@link #ByteRTCAudioScenarioType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 此接口在进房前后调用都有效。
  ///        - 通话音量更适合通话、会议等对信息准确度更高的场景。通话音量会激活系统硬件信号处理，使通话声音更清晰。同时，音量无法降低到 0。
  ///        - 媒体音量更适合娱乐场景，因其声音的表现力会更强。媒体音量下，最低音量可以为 0。

  FutureOr<int> setAudioScenario(ByteRTCAudioScenarioType audioScenario) async {
    return await nativeCall('setAudioScenario:', [audioScenario.$value]);
  }

  /// @detail api
  /// @author dixing
  /// @brief 设置音质档位。 <br>
  ///        当所选的 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 中的音频参数无法满足你的场景需求时，调用本接口切换的音质档位。
  /// @param audioProfile 音质档位，参看 ByteRTCAudioProfileType{@link #ByteRTCAudioProfileType}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法在进房前后均可调用；
  ///        - 支持通话过程中动态切换音质档位。

  FutureOr<int> setAudioProfile(ByteRTCAudioProfileType audioProfile) async {
    return await nativeCall('setAudioProfile:', [audioProfile.$value]);
  }

  /// @valid since 3.52
  /// @detail api
  /// @author liuchuang
  /// @brief 支持根据业务场景，设置通话中的音频降噪模式。
  /// @param ansMode 降噪模式。具体参见 ByteRTCAnsMode{@link #ByteRTCAnsMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该接口进房前后均可调用，可重复调用，仅最后一次调用生效。
  ///        - 降噪算法包含传统降噪和 AI 降噪。传统降噪主要是抑制平稳噪声，比如空调声、风扇声等。而 AI 降噪主要是抑制非平稳噪声，比如键盘敲击声、桌椅碰撞声等。
  ///        - 只有以下 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 场景时，调用本接口可以开启 AI 降噪。其余场景的 AI 降噪不会生效。
  ///                 -  游戏语音模式： `ByteRTCRoomProfileGame`
  ///                 -  高音质游戏模式： `ByteRTCRoomProfileGameHD`
  ///                 -  云游戏模式： `ByteRTCRoomProfileCloudGame`
  ///                 -  1 vs 1 音视频通话： `ByteRTCRoomProfileChat`
  ///                 -  多端同步播放音视频：`ByteRTCRoomProfileLwTogether`
  ///                 -  云端会议中的个人设备：`ByteRTCRoomProfileMeeting`
  ///                 -  课堂互动模式：`ByteRTCRoomProfileClassroom`
  ///                 -  云端会议中的会议室终端：`ByteRTCRoomProfileMeetingRoom`

  FutureOr<int> setAnsMode(ByteRTCAnsMode ansMode) async {
    return await nativeCall('setAnsMode:', [ansMode.$value]);
  }

  /// @hidden(iOS)
  /// @valid since 3.51
  /// @detail api
  /// @author liuchuang
  /// @brief 打开/关闭 AGC(Analog Automatic Gain Control)模拟自动增益控制功能。 <br>
  ///        开启该功能后，SDK 会自动调节麦克风的采集音量，确保音量稳定。
  /// @param enable 是否打开 AGC 功能: <br>
  ///        - true: 打开 AGC 功能。
  ///        - false: 关闭 AGC 功能。
  /// @return
  ///        -  0: 调用成功。
  ///        - -1: 调用失败。
  /// @note
  ///         该方法在进房前后均可调用。如果你需要在进房前使用 AGC 功能，请联系技术支持获得私有参数，传入对应 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 。 <br>
  ///         要想在进房后开启 AGC 功能，你需要把 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 设为 `ByteRTCRoomProfileMeeting`、`ByteRTCRoomProfileMeetingRoom` 或`ByteRTCRoomProfileClassroom` 。 <br>
  ///         AGC 功能生效后，不建议再调用 `setAudioCaptureDeviceVolume:` 来调节设备麦克风的采集音量。

  FutureOr<int> enableAGC(BOOL enable) async {
    return await nativeCall('enableAGC:', [enable]);
  }

  /// @valid since 3.32
  /// @detail api
  /// @author luomingkang
  /// @brief 设置变声特效类型
  /// @param voiceChanger 变声特效类型，参看 ByteRTCVoiceChangerType{@link #ByteRTCVoiceChangerType}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 如需使用该功能，需集成 SAMI 动态库，详情参看[按需集成插件](#1108726)文档。
  ///        - 在进房前后都可设置。
  ///        - 对 RTC SDK 内部采集的音频和自定义采集的音频都生效。
  ///        - 只对单声道音频生效。
  ///        - 与 setVoiceReverbType:{@link #ByteRTCEngine#setVoiceReverbType} 互斥，后设置的特效会覆盖先设置的特效。

  FutureOr<int> setVoiceChangerType(
      ByteRTCVoiceChangerType voiceChanger) async {
    return await nativeCall('setVoiceChangerType:', [voiceChanger.$value]);
  }

  /// @valid since 3.32
  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置混响特效类型
  /// @param voiceReverb 混响特效类型，参看 ByteRTCVoiceReverbType{@link #ByteRTCVoiceReverbType}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 在进房前后都可设置。
  ///        - 对 RTC SDK 内部采集的音频和自定义采集的音频都生效。
  ///        - 只对单声道音频生效。
  ///        - 与 setVoiceChangerType:{@link #ByteRTCEngine#setVoiceChangerType} 互斥，后设置的特效会覆盖先设置的特效。

  FutureOr<int> setVoiceReverbType(ByteRTCVoiceReverbType voiceReverb) async {
    return await nativeCall('setVoiceReverbType:', [voiceReverb.$value]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置本地采集语音的均衡效果。包含内部采集和外部采集，但不包含混音音频文件。
  /// @param config 语音均衡效果，参看 ByteRTCVoiceEqualizationConfig{@link #ByteRTCVoiceEqualizationConfig}
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 根据奈奎斯特采样率，音频采样率必须大于等于设置的中心频率的两倍，否则，设置不生效。

  FutureOr<int> setLocalVoiceEqualization(
      ByteRTCVoiceEqualizationConfig config) async {
    return await nativeCall('setLocalVoiceEqualization:', [config]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 设置本地采集音频的混响效果。包含内部采集和外部采集，但不包含混音音频文件。
  /// @param param 混响效果，参看 ByteRTCVoiceReverbConfig{@link #ByteRTCVoiceReverbConfig}
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 调用 enableLocalVoiceReverb:{@link #ByteRTCEngine#enableLocalVoiceReverb} 开启混响效果。

  FutureOr<int> setLocalVoiceReverbParam(ByteRTCVoiceReverbConfig param) async {
    return await nativeCall('setLocalVoiceReverbParam:', [param]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 开启本地音效混响效果
  /// @param enable 是否开启
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 调用 setLocalVoiceReverbParam:{@link #ByteRTCEngine#setLocalVoiceReverbParam} 设置混响效果。

  FutureOr<int> enableLocalVoiceReverb(bool enable) async {
    return await nativeCall('enableLocalVoiceReverb:', [enable]);
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
  ///        - < 0 : 调用失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 该方法用于设置是否使用静音数据替换设备采集到的音频数据进行推流，不影响 SDK 音频流的采集发布状态。对于 macOS 平台，如有需要你也可以选择静音整个系统的音频采集，具体参看 setAudioCaptureDeviceMute:{@link #ByteRTCAudioDeviceManager#setAudioCaptureDeviceMute} 方法说明。
  ///        - 静音后通过 setCaptureVolume:{@link #ByteRTCEngine#setCaptureVolume} 调整音量不会取消静音状态，音量状态会保存至取消静音。
  ///        - 调用 startAudioCapture{@link #ByteRTCEngine#startAudioCapture} 开启音频采集前后，都可以使用此接口设置采集音量。

  FutureOr<int> muteAudioCapture(bool mute) async {
    return await nativeCall('muteAudioCapture:', [mute]);
  }

  /// @valid since 3.60.
  /// @detail api
  /// @author shiyayun
  /// @brief 静音或静音/取消静音屏幕共享时采集的音频。<br>
  ///        调用此方法后，SDK 将发送静音数据来代替真实的屏幕音频数据，不影响本端音频设备的采集状态和 SDK 音频流的采集发布状态。
  /// @param mute 是否静音屏幕音频。 <br>
  ///        - True：静音。远端用户听不到来自你屏幕共享的声音。
  ///        - False：（默认）取消静音。恢复发送屏幕共享的音频。
  /// @return
  ///        - 0：调用成功。
  ///        - < 0：调用失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 该方法用于设置是否使用静音数据替换设备采集到的音频数据进行推流，不影响 SDK 音频流的采集发布状态。对于 macOS 平台，如有需要你也可以选择静音整个系统的音频采集，具体参看 setAudioCaptureDeviceMute:{@link #ByteRTCAudioDeviceManager#setAudioCaptureDeviceMute} 方法说明。
  ///        - 静音后通过 setCaptureVolume:{@link #ByteRTCEngine#setCaptureVolume} 调整音量不会取消静音状态，音量状态会保存至取消静音。
  ///        - 调用 startAudioCapture{@link #ByteRTCEngine#startAudioCapture} 开启音频采集前后，都可以使用此接口设置采集音量。

  FutureOr<int> muteScreenAudioCapture(bool mute) async {
    return await nativeCall('muteScreenAudioCapture:', [mute]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 调节音频采集音量
  /// @param volume 采集的音量值和原始音量的百分比，范围是 [0, 400]，单位为 \%，自带溢出保护。 <br>
  ///        为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///       - 0：静音
  ///       - 100：原始音量
  ///       - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 在开启音频采集前后，你都可以使用此接口设定采集音量。

  FutureOr<int> setCaptureVolume(int volume) async {
    return await nativeCall('setCaptureVolume:', [volume]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 调节屏幕共享时采集的音频音量。<br>
  ///        只改变音频数据的音量信息，不影响麦克风采集的音量，也不会改变本端音频设备本身的音量。
  /// @param volume 采集的音量值和原始音量的百分比，范围是 [0, 400]，单位为 \%，自带溢出保护。<br>
  ///               为保证更好的通话质量，建议将 volume 值设为 [0, 100]。<br>
  ///               + 0：静音
  ///               + 100：原始音量
  ///               + 400：最大可为原始音量的 4 倍（自带溢出保护）
  /// @return
  ///        + 0: 调用成功。<br>
  ///        + <0：调用失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 在开启屏幕音频采集前后，你都可以使用此接口设定采集音量。

  FutureOr<int> setScreenCaptureVolume(int volume) async {
    return await nativeCall('setScreenCaptureVolume:', [volume]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 调节本地播放的所有远端用户音频混音后的音量，混音内容包括远端人声、音乐、音效等。 <br>
  ///        播放音频前或播放音频时，你都可以使用此接口设定播放音量。
  /// @param volume 音频播放音量值和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。 <br>
  ///        为保证更好的通话质量，建议将 volume 值设为 [0,100]。 <br>
  ///       - 0: 静音
  ///       - 100: 原始音量
  ///       - 400: 最大可为原始音量的 4 倍(自带溢出保护)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内，当该方法与 setRemoteAudioPlaybackVolume:volume:{@link #ByteRTCEngine#setRemoteAudioPlaybackVolume:volume} 或 setRemoteRoomAudioPlaybackVolume:{@link #ByteRTCRoom#setRemoteRoomAudioPlaybackVolume} 共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。

  FutureOr<int> setPlaybackVolume(NSInteger volume) async {
    return await nativeCall('setPlaybackVolume:', [volume]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 启用音频信息提示。启用后，你可以收到 rtcEngine:onLocalAudioPropertiesReport:{@link #ByteRTCEngineDelegate#rtcEngine:onLocalAudioPropertiesReport}，rtcEngine:onRemoteAudioPropertiesReport:totalRemoteVolume:{@link #ByteRTCEngineDelegate#rtcEngine:onRemoteAudioPropertiesReport:totalRemoteVolume}，和 rtcEngine:onActiveSpeaker:uid:{@link #ByteRTCEngineDelegate#rtcEngine:onActiveSpeaker:uid}。
  /// @param config 详见 ByteRTCAudioPropertiesConfig{@link #ByteRTCAudioPropertiesConfig}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> enableAudioPropertiesReport(
      ByteRTCAudioPropertiesConfig config) async {
    return await nativeCall('enableAudioPropertiesReport:', [config]);
  }

  /// @detail api
  /// @hidden 3.60 for internal use only
  /// @region 音频管理
  /// @author gengjunjie
  /// @brief 启用音频人声识别能力。开启提示后，你会收到 rtcEngine:onAudioVADStateUpdate。
  /// @param interval 回调间隔，单位毫秒。<br>
  ///       + `<= 0`: 关闭人声识别能力回调。
  ///       + `[100, 3000]`: 开启人声识别能力回调，并将信息提示间隔设置为此值。
  ///       + 不合法的 interval 值：小于 100 设置为 100，超出 3000 设置为 3000。
  /// @return
  ///        + 0: 调用成功。
  ///        + < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。

  FutureOr<int> enableAudioVADReport(NSInteger interval) async {
    return await nativeCall('enableAudioVADReport:', [interval]);
  }

  /// @hidden 3.60 for internal use only
  /// @detail api
  /// @region 音频管理
  /// @author shiyayun
  /// @brief 启用AED检测。启用后，你可以收到 rtcEngine:onAudioAEDStateUpdate。
  /// @param interval 回调间隔，单位毫秒。<br>
  ///       + `<= 0`: 关闭回调。
  ///       + `[100, 3000]`: 开启回调，并将回调间隔设置为该值。推荐设置为 2000。
  ///       + 不合法的 interval 值：小于 100 设置为 100，超出 3000 设置为 3000。
  /// @return
  ///        + 0：调用成功。
  ///        + <0：调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。

  FutureOr<int> enableAudioAEDReport(NSInteger interval) async {
    return await nativeCall('enableAudioAEDReport:', [interval]);
  }

  /// @detail api
  /// @author huanghao
  /// @brief 调节本端播放收到的远端流时的音量。你必须在进房后进行设置。流的发布状态改变不影响设置生效。
  /// @param streamId 远端流 ID。
  /// @param volume 音量值和原始音量的比值，范围是 [0, 400]，单位为 \%，自带溢出保护。 <br>
  ///               为保证更好的通话质量，建议将 volume 值设为 [0,100]。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 假设某远端用户 A 始终在被调节的目标用户范围内： <br>
  ///        - 当该方法与 setRemoteRoomAudioPlaybackVolume:{@link #ByteRTCRoom#setRemoteRoomAudioPlaybackVolume} 共同使用时，本地收听用户 A 的音量为后调用的方法设置的音量；
  ///        - 当该方法与 setPlaybackVolume:{@link #ByteRTCEngine#setPlaybackVolume} 方法共同使用时，本地收听用户 A 的音量将为两次设置的音量效果的叠加。
  ///        - 当你调用该方法设置远端流音量后，如果远端退房，接口设置失效。

  FutureOr<int> setRemoteAudioPlaybackVolume(
      NSString streamId, int volume) async {
    return await nativeCall(
        'setRemoteAudioPlaybackVolume:volume:', [streamId, volume]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @brief 开启/关闭耳返功能。
  /// @param mode 是否开启耳返功能，参看 ByteRTCEarMonitorMode{@link #ByteRTCEarMonitorMode}。默认关闭。
  /// @param filter 是否经过本地音频处理，参看 ByteRTCEarMonitorAudioFilter{@link #ByteRTCEarMonitorAudioFilter}。默认不经过音频处理。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 耳返功能仅适用于由 RTC SDK 内部采集的音频。
  ///        - 使用耳返必须佩戴耳机。为保证低延时耳返最佳体验，建议佩戴有线耳机。
  ///        - 对于 iOS，仅支持软件耳返功能。
  ///        - 对于 macOS，耳返功能仅支持设备通过 3.5mm 接口、USB 接口、或蓝牙方式直连耳机时可以使用。对于通过 HDMI 或 USB-C 接口连接显示器转接耳机，或通过连接 OTG 外接声卡再连接的耳机，不支持耳返功能。

  FutureOr<int> setEarMonitorMode(
      ByteRTCEarMonitorMode mode, ByteRTCEarMonitorAudioFilter filter) async {
    return await nativeCall(
        'setEarMonitorMode:filter:', [mode.$value, filter.$value]);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 设置耳返的音量。
  /// @param volume 耳返的音量，取值范围：[0,100]，单位：\%
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note 设置耳返音量前，你必须先调用 setEarMonitorMode:{@link #ByteRTCEngine#setEarMonitorMode} 打开耳返功能。

  FutureOr<int> setEarMonitorVolume(NSInteger volume) async {
    return await nativeCall('setEarMonitorVolume:', [volume]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author dixing
  /// @brief 在纯媒体音频场景下,切换 iOS 设备与耳机之间的蓝牙传输协议。
  /// @param mode 蓝牙传输协议。详见 ByteRTCBluetoothMode{@link #ByteRTCBluetoothMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 以下场景你会收到 rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning} 回调：1）当前不支持设置 HFP；2）非纯媒体音频场景，建议在调用此接口前调用 setAudioScenario:{@link #ByteRTCEngine#setAudioScenario} 设置纯媒体音频场景。

  FutureOr<int> setBluetoothMode(ByteRTCBluetoothMode mode) async {
    return await nativeCall('setBluetoothMode:', [mode.$value]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 开启本地语音变调功能，多用于 K 歌场景。 <br>
  ///        使用该方法，你可以对本地语音的音调进行升调或降调等调整。
  /// @param pitch 相对于语音原始音调的升高/降低值，取值范围[-12，12]，默认值为 0，即不做调整。 <br>
  ///        取值范围内每相邻两个值的音高距离相差半音，正值表示升调，负值表示降调，设置的绝对值越大表示音调升高或降低越多。 <br>
  ///        超出取值范围则设置失败，并且会触发 rtcEngine:onWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onWarning} 回调，提示 ByteRTCWarningCode{@link #ByteRTCWarningCode} 错误码为 `WARNING_CODE_SET_SCREEN_STREAM_INVALID_VOICE_PITCH` 设置语音音调不合法
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> setLocalVoicePitch(NSInteger pitch) async {
    return await nativeCall('setLocalVoicePitch:', [pitch]);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 开启/关闭音量均衡功能。 <br>
  ///        开启音量均衡功能后，人声的响度会调整为 -16lufs。如果已调用 setAudioMixingLoudness:loudness: 传入了混音音乐的原始响度，此音乐播放时，响度会调整为 -20lufs。
  /// @param enable 是否开启音量均衡功能： <br>
  ///       - YES: 是
  ///       - NO: 否
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 该接口须在调用 startAudioMixing:filePath:config: 开始播放音频文件之前调用。

  FutureOr<int> enableVocalInstrumentBalance(BOOL enable) async {
    return await nativeCall('enableVocalInstrumentBalance:', [enable]);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 打开/关闭音量闪避功能，适用于在 RTC 通话过程中会同时播放短视频或音乐的场景，如“一起看”、“在线 KTV”等。 <br>
  ///        开启该功能后，当检测到远端人声时，RTC 的本地的媒体播放音量会自动减弱，从而保证远端人声的清晰可辨；当远端人声消失时，RTC 的本地媒体音量会恢复到闪避前的音量水平。
  /// @param enable 是否开启音量闪避： <br>
  ///        - YES: 是
  ///        - NO: 否
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> enablePlaybackDucking(BOOL enable) async {
    return await nativeCall('enablePlaybackDucking:', [enable]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @deprecated since 3.57, use setLocalVideoSink:withLocalRenderConfig:{@link #ByteRTCEngine#setLocalVideoSink:withLocalRenderConfig} instead.
  /// @region 自定义视频采集渲染
  /// @author sunhang.io
  /// @brief 将本地视频流与自定义渲染器绑定。
  /// @param videoSink 自定义视频渲染器，参看 ByteRTCVideoSinkDelegate{@link #ByteRTCVideoSinkDelegate}
  /// @param requiredFormat videoSink 适用的视频帧编码格式，参看 ByteRTCVideoSinkPixelFormat{@link #ByteRTCVideoSinkPixelFormat}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - RTC SDK 默认使用 RTC SDK 自带的渲染器（内部渲染器）进行视频渲染。
  ///        - 如果需要解除绑定，必须将 videoSink 设置为 null。退房时将清除绑定状态。
  ///        - 一般在收到 rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo:{@link #ByteRTCEngineDelegate#rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo} 回调通知完成本地视频首帧采集后，调用此方法为视频流绑定自定义渲染器；然后加入房间。
  ///        - 本方法获取的是前处理后的视频帧。

  FutureOr<int> setLocalVideoSink(id<ByteRTCVideoSinkDelegate> videoSink,
      ByteRTCVideoSinkPixelFormat requiredFormat) async {
    return await nativeCall('setLocalVideoSink:withPixelFormat:',
        [videoSink, requiredFormat.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @deprecated since 3.57, use setRemoteVideoSink:withSink:withRemoteRenderConfig:{@link #ByteRTCEngine#setRemoteVideoSink:withSink:withRemoteRenderConfig} instead.
  /// @region 自定义视频采集渲染
  /// @author sunhang.io
  /// @brief 将远端视频流与自定义渲染器绑定。
  /// @param streamId 远端流 ID。
  /// @param videoSink 自定义视频渲染器，参看 ByteRTCVideoSinkDelegate{@link #ByteRTCVideoSinkDelegate}
  /// @param requiredFormat videoSink 适用的视频帧编码格式，参看 ByteRTCVideoSinkPixelFormat{@link #ByteRTCVideoSinkPixelFormat}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - RTC SDK 默认使用 RTC SDK 自带的渲染器（内部渲染器）进行视频渲染。
  ///        - 该方法进房前后均可以调用。若想在进房前调用，你需要在加入房间前获取远端流信息；若无法预先获取远端流信息，你可以在加入房间并通过 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish} 回调获取到远端流信息之后，再调用该方法。
  ///        - 如果需要解除绑定，必须将 videoSink 设置为 null。退房时将清除绑定状态。
  ///        - 本方法获取的是后处理后的视频帧。

  FutureOr<int> setRemoteVideoSink(
      NSString streamId,
      id<ByteRTCVideoSinkDelegate> videoSink,
      ByteRTCVideoSinkPixelFormat requiredFormat) async {
    return await nativeCall('setRemoteVideoSink:withSink:withPixelFormat:',
        [streamId, videoSink, requiredFormat.$value]);
  }

  /// @hidden(macOS) for internal use
  /// @valid since 3.54
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author yinkaisheng
  /// @brief 设置远端视频超分模式。
  /// @param streamId 远端流 ID。
  /// @param mode 超分模式，参看 ByteRTCVideoSuperResolutionMode{@link #ByteRTCVideoSuperResolutionMode}。
  /// @return
  ///        - 0: ByteRTCReturnStatusSuccess，SDK 调用成功，并不代表超分模式实际状态，需要根据回调 rtcEngine:onRemoteVideoSuperResolutionModeChanged:info:withMode:withReason:{@link #ByteRTCEngineDelegate#rtcEngine:onRemoteVideoSuperResolutionModeChanged:info:withMode:withReason} 判断实际状态。
  ///        - -1: ByteRTCReturnStatusNativeInValid，native library 未加载。
  ///        - -2: ByteRTCReturnStatusParameterErr，参数非法，指针为空或字符串为空。
  ///        - -9: ByteRTCReturnStatusScreenNotSupport，不支持对屏幕流开启超分。
  ///        其他错误码参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 如需使用该功能，需集成超分插件 ByteRTCVideoSRExtension.xcframework 及依赖库 bmf_mods_shared.xcframework，详情参看[按需集成插件](#1108726)文档。
  ///        - 该方法须进房后调用。
  ///        - 远端用户视频流的原始分辨率不能超过 640 × 360 px。
  ///        - 支持对一路远端流开启超分，不支持对多路流开启超分。

  FutureOr<int> setRemoteVideoSuperResolution(
      NSString streamId, ByteRTCVideoSuperResolutionMode mode) async {
    return await nativeCall(
        'setRemoteVideoSuperResolution:withMode:', [streamId, mode.$value]);
  }

  /// @hidden not available on iOS
  /// @valid since 3.54
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author Yujianli
  /// @brief 设置视频降噪模式。
  /// @param mode 视频降噪模式。参看 ByteRTCVideoDenoiseMode{@link #ByteRTCVideoDenoiseMode}。
  /// @return
  ///        - 0: API 调用成功。 用户可以根据回调函数 rtcEngine:onVideoDenoiseModeChanged:withReason:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDenoiseModeChanged:withReason} 判断视频降噪是否开启。
  ///        - < 0: API 调用失败。
  /// @note 如需使用该功能，需集成降噪插件 ByteRTCVideoDenoiseExtension.xcframework 及依赖库 bmf_mods_shared.xcframework，详情参看[按需集成插件](#1108726)文档。

  FutureOr<int> setVideoDenoiser(ByteRTCVideoDenoiseMode mode) async {
    return await nativeCall('setVideoDenoiser:', [mode.$value]);
  }

  /// @hidden(iOS)
  /// @valid since 3.57
  /// @detail api
  /// @author zhoubohui
  /// @brief 设置视频暗光增强模式。 <br>
  ///        对于光线不足、照明不均匀或背光等场景下推荐开启，可有效改善画面质量。
  /// @param mode 默认不开启。参看 ByteRTCVideoEnhancementMode{@link #ByteRTCVideoEnhancementMode}。
  /// @return
  ///        - 0: API 调用成功。会立即生效，但需要等待下载和检测完成后才能看到增强后的效果。
  ///        - < 0: API 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 开启后会影响设备性能，应根据实际需求和设备性能决定是否开启。
  ///        - 对 RTC SDK 内部采集的视频和自定义采集的视频都生效。

  FutureOr<int> setLowLightAdjusted(ByteRTCVideoEnhancementMode mode) async {
    return await nativeCall('setLowLightAdjusted:', [mode.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhaomingliang
  /// @brief 设置本端采集的视频帧的旋转角度。 <br>
  ///        当摄像头倒置或者倾斜安装时，可调用本接口进行调整。对于手机等普通设备，可调用 setVideoRotationMode:{@link #ByteRTCEngine#setVideoRotationMode} 实现旋转。
  /// @param rotation 相机朝向角度，默认为 `ByteRTCVideoRotation0`，无旋转角度。详见 ByteRTCVideoRotation{@link #ByteRTCVideoRotation}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 对于内部采集的视频画面，如果已调用 setVideoRotationMode:{@link #ByteRTCEngine#setVideoRotationMode} 设置了旋转方向，会在此基础上叠加旋转角度。
  ///        - 调用本接口也将对自定义采集视频画面生效，在原有的旋转角度基础上叠加本次设置。
  ///        - 视频贴纸特效或通过 enableVirtualBackground{@link #ByteRTCVideoEffect#enableVirtualBackground:withSource} 增加的虚拟背景，也会跟随本接口的设置进行旋转。
  ///        - 本地渲染视频和发送到远端的视频都会相应旋转，但不会应用到单流转推中。如果希望在单流转推的视频中应用旋转，调用 setVideoOrientation:{@link #ByteRTCEngine#setVideoOrientation}。

  FutureOr<int> setVideoCaptureRotation(ByteRTCVideoRotation rotation) async {
    return await nativeCall('setVideoCaptureRotation:', [rotation.$value]);
  }

  /// @hidden currently not available
  /// @author wangyu.1705
  /// @brief iOS 和 Mac 不支持 Fov，对齐其他端预留接口。

  FutureOr<int> setVideoEncoderConfig(
      ByteRTCVideoEncoderConfig encoderConfig, NSDictionary parameters) async {
    return await nativeCall(
        'setVideoEncoderConfig:withParameters:', [encoderConfig, parameters]);
  }

  /// @valid since 3.60.
  /// @detail api
  /// @author zhoubohui
  /// @brief 发布端进行大小流(simulcast)设置。
  /// @param mode 详见 ByteRTCVideoSimulcastMode{@link #ByteRTCVideoSimulcastMode}。默认为只发送单流。你应在进房前调用修改本参数。
  /// @param streamConfig 小流参数。最多可设置 3 路。分辨率按照从小到大顺序，且每路流参数分辨率需小于大流 setVideoEncoderConfig:withParameters:{@link #ByteRTCEngine#setVideoEncoderConfig:withParameters} 设置参数。否则可能会设置失败。参看 ByteRTCVideoEncoderConfig{@link #ByteRTCVideoEncoderConfig}。
  ///        其余模式下，默认小流参数为 160px × 90px, 码率为 50kpbs。
  /// @return 方法调用结果： <br>
  ///        - 0：成功
  ///        - !0：失败
  /// @note
  ///        - 调用本方法前，SDK 默认仅发布一条分辨率为 640px × 360px \@15fps 的视频流。
  ///        - 本方法适用于摄像头采集的视频流。
  ///        - 更多信息详见[推送多路流](https://www.volcengine.com/docs/6348/70139)文档。
  ///

  FutureOr<int> setLocalSimulcastMode(ByteRTCVideoSimulcastMode mode,
      NSArray<ByteRTCVideoEncoderConfig> streamConfig) async {
    return await nativeCall(
        'setLocalSimulcastMode:config:', [mode.$value, streamConfig]);
  }

  /// @hidden(macOS)
  /// @valid since 3.58
  /// @detail api
  /// @author zhuhongshuyu
  /// @brief 开启自定义采集视频帧的 Alpha 通道编码功能。 <br>
  ///        适用于需要分离推流端视频主体与背景，且在拉流端可自定义渲染背景的场景。
  /// @param alphaLayout 分离后的 Alpha 通道相对于 RGB 通道信息的排列位置。当前仅支持 ByteRTCAlphaLayout.ByteRTCAlphaLayoutTop，即置于 RGB 通道信息上方。
  /// @return 方法调用结果： <br>
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该接口仅作用于自定义采集的 ByteRTCVideoPixelFormat.ByteRTCVideoPixelFormatCVPixelBuffer 格式视频帧。
  ///        - 该接口须在发布视频流之前调用。
  ///        - 调用本接口开启 Alpha 通道编码后，你需调用 pushExternalVideoFrame:{@link #ByteRTCEngine#pushExternalVideoFrame} 把自定义采集的视频帧推送至 RTC SDK。若推送了不支持的视频帧格式，则调用 pushExternalVideoFrame:{@link #ByteRTCEngine#pushExternalVideoFrame} 时会返回错误码 ByteRTCReturnStatus.ByteRTCReturnStatusParameterErr。

  FutureOr<int> enableAlphaChannelVideoEncode(
      ByteRTCAlphaLayout alphaLayout) async {
    return await nativeCall(
        'enableAlphaChannelVideoEncode:', [alphaLayout.$value]);
  }

  /// @hidden(macOS)
  /// @valid since 3.58
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhuhongshuyu
  /// @brief 关闭外部采集视频帧的 Alpha 通道编码功能。
  /// @return 方法调用结果： <br>
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note 该接口须在停止发布视频流之后调用。

  FutureOr<int> disableAlphaChannelVideoEncode() async {
    return await nativeCall('disableAlphaChannelVideoEncode', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 设置 RTC SDK 内部采集时的视频采集参数。 <br>
  ///        如果你的项目使用了 SDK 内部采集模块，可以通过本接口指定视频采集参数包括模式、分辨率、帧率。
  /// @param captureConfig 视频采集参数。参看: ByteRTCVideoCaptureConfig{@link #ByteRTCVideoCaptureConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  /// - 本接口在引擎创建后可调用，调用后立即生效。建议在调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 前调用本接口。
  /// - 建议同一设备上的不同引擎使用相同的视频采集参数。
  /// - 如果调用本接口前使用内部模块开始视频采集，采集参数默认为 Auto 模式。

  FutureOr<int> setVideoCaptureConfig(
      ByteRTCVideoCaptureConfig captureConfig) async {
    return await nativeCall('setVideoCaptureConfig:', [captureConfig]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author sunhang.io
  /// @brief 设置本地视频渲染时使用的视图，并设置渲染模式。
  /// @param canvas 视图信息和渲染模式，参看 ByteRTCVideoCanvas{@link #ByteRTCVideoCanvas}
  /// @return
  ///        - 0：成功。
  ///        - -2: 参数错误。
  ///        - -12: 本方法不支持在 Audio SDK 中使用。
  /// @note
  ///        - 你应在加入房间前，绑定本地视图。退出房间后，此设置仍然有效。
  ///        - 如果需要解除绑定，你可以调用本方法传入空视图。

  FutureOr<int> setLocalVideoCanvas(ByteRTCVideoCanvas canvas) async {
    return await nativeCall('setLocalVideoCanvas:', [canvas]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfujun.911
  /// @brief 修改本地视频渲染模式和背景色。
  /// @param renderMode 渲染模式。参看 ByteRTCRenderMode{@link #ByteRTCRenderMode}
  /// @param backgroundColor 背景颜色。参看 ByteRTCVideoCanvas{@link #ByteRTCVideoCanvas}.backgroundColor
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note 你可以在本地视频渲染过程中，调用此接口。调用结果会实时生效。

  FutureOr<int> updateLocalVideoCanvas(
      ByteRTCRenderMode renderMode, NSUInteger backgroundColor) async {
    return await nativeCall('updateLocalVideoCanvas:withBackgroundColor:',
        [renderMode.$value, backgroundColor]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author sunhang.io
  /// @brief 渲染来自指定远端用户 uid 的视频流时，设置使用的视图和渲染模式。 <br>
  ///        如果需要解除视频的绑定视图，把 `canvas.view` 设置为空。(`canvas` 中其他参数不能为空。)
  /// @param streamId 远端流 ID。
  /// @param canvas 视图信息和渲染模式，参看 ByteRTCVideoCanvas{@link #ByteRTCVideoCanvas}。3.56 版本起支持通过 `renderRotation` 设置远端视频渲染的旋转角度。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 本地用户离开房间时，会解除调用此 API 建立的绑定关系；远端用户离开房间则不会影响。

  FutureOr<int> setRemoteVideoCanvas(
      NSString streamId, ByteRTCVideoCanvas canvas) async {
    return await nativeCall(
        'setRemoteVideoCanvas:withCanvas:', [streamId, canvas]);
  }

  /// @deprecated since 3.56, and will be deleted in 3.62. Use updateRemoteStreamVideoCanvas:withRemoteVideoRenderConfig:{@link #ByteRTCEngine#updateRemoteStreamVideoCanvas:withRemoteVideoRenderConfig} instead.
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfujun.911
  /// @brief 修改远端视频帧的渲染设置，包括渲染模式和背景颜色。
  /// @param streamId 远端流 ID。
  /// @param renderMode 渲染模式，参看 ByteRTCRenderMode{@link #ByteRTCRenderMode}
  /// @param backgroundColor 背景颜色，参看 ByteRTCVideoCanvas{@link #ByteRTCVideoCanvas}.backgroundColor
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 你可以在远端视频渲染过程中，调用此接口。调用结果会实时生效。

  FutureOr<int> updateRemoteStreamVideoCanvas(NSString streamId,
      ByteRTCRenderMode renderMode, NSUInteger backgroundColor) async {
    return await nativeCall(
        'updateRemoteStreamVideoCanvas:withRenderMode:withBackgroundColor:',
        [streamId, renderMode.$value, backgroundColor]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhangzhenyu.samuel
  /// @brief 立即开启内部视频采集。默认为关闭状态。 <br>
  ///        内部视频采集指：使用 RTC SDK 内置视频采集模块，进行采集。 <br>
  ///        调用该方法后，本地用户会收到 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 的回调。 <br>
  ///        非隐身用户进房后调用该方法，房间中的其他用户会收到 rtcEngine:onUserStartVideoCapture:info:{@link #ByteRTCEngineDelegate#rtcEngine:onUserStartVideoCapture:info} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 stopVideoCapture{@link #ByteRTCEngine#stopVideoCapture} 可以停止内部视频采集。否则，只有当销毁引擎实例时，内部视频采集才会停止。
  ///       - 创建引擎后，无论是否发布视频数据，你都可以调用该方法开启内部视频采集。只有当（内部或外部）视频采集开始以后视频流才会发布。
  ///       - 如果需要从自定义视频采集切换为内部视频采集，你必须先停止发布流，关闭自定义采集，再调用此方法手动开启内部采集。
  ///       - 内部视频采集使用的摄像头由 switchCamera:{@link #ByteRTCEngine#switchCamera} 接口指定。（macOS 不支持）
  ///       - 自 v3.37.0 升级版本，你需要在应用中向用户申请摄像头权限后才能开始采集。
  ///

  FutureOr<int> startVideoCapture() async {
    return await nativeCall('startVideoCapture', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhangzhenyu.samuel
  /// @brief 立即关闭内部视频采集。默认为关闭状态。 <br>
  ///        内部视频采集指：使用 RTC SDK 内置视频采集模块，进行采集。 <br>
  ///        调用该方法后，本地用户会收到 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 的回调。 <br>
  ///        非隐身用户进房后调用该方法，房间中的其他用户会收到 rtcEngine:onUserStopVideoCapture:info:{@link #ByteRTCEngineDelegate#rtcEngine:onUserStopVideoCapture:info} 的回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 可以开启内部视频采集。
  ///       - 如果不调用本方法停止内部视频采集，则只有当销毁引擎实例时，内部视频采集才会停止。
  ///

  FutureOr<int> stopVideoCapture() async {
    return await nativeCall('stopVideoCapture', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 为采集到的视频流开启镜像
  /// @param mirrorType 镜像类型，参看 ByteRTCMirrorType{@link #ByteRTCMirrorType}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
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

  FutureOr<int> setLocalVideoMirrorType(ByteRTCMirrorType mirrorType) async {
    return await nativeCall('setLocalVideoMirrorType:', [mirrorType.$value]);
  }

  /// @valid since 3.57
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @brief 使用内部渲染时，为远端流开启镜像。
  /// @param streamId 远端流 ID。
  /// @param mirrorType 远端流的镜像类型，参看 ByteRTCRemoteMirrorType{@link #ByteRTCRemoteMirrorType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0: 调用失败，参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。

  FutureOr<int> setRemoteVideoMirrorType(
      NSString streamId, ByteRTCRemoteMirrorType mirrorType) async {
    return await nativeCall('setRemoteVideoMirrorType:withMirrorType:',
        [streamId, mirrorType.$value]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 设置采集视频的旋转模式。默认以 App 方向为旋转参考系。 <br>
  ///        接收端渲染视频时，将按照和发送端相同的方式进行旋转。
  /// @param rotationMode 视频旋转参考系为 App 方向或重力方向，参看 ByteRTCVideoRotationMode{@link #ByteRTCVideoRotationMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 旋转仅对内部视频采集生效，不适用于外部视频源和屏幕源。
  ///        - 调用该接口时已开启视频采集，将立即生效；调用该接口时未开启视频采集，则将在采集开启后生效。
  ///        - 更多信息请参考[视频采集方向](https://www.volcengine.com/docs/6348/106458)。

  FutureOr<int> setVideoRotationMode(
      ByteRTCVideoRotationMode rotationMode) async {
    return await nativeCall('setVideoRotationMode:', [rotationMode.$value]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 在自定义视频前处理及编码前，设置 RTC 链路中的视频帧朝向，默认为 Adaptive 模式。 <br>
  ///        移动端开启视频特效贴纸，或使用自定义视频前处理时，建议固定视频帧朝向为 Portrait 模式。单流转推场景下，建议根据业务需要固定视频帧朝向为 Portrait 或 Landscape 模式。不同模式的具体显示效果参看[视频帧朝向](https://www.volcengine.com/docs/6348/128787)。
  /// @param orientation 视频帧朝向，参看 ByteRTCVideoOrientation{@link #ByteRTCVideoOrientation}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 视频帧朝向设置仅适用于内部采集视频源。对于自定义采集视频源，设置视频帧朝向可能会导致错误，例如宽高对调。屏幕源不支持设置视频帧朝向。
  ///        - 编码分辨率的更新与视频帧处理是异步操作，进房后切换视频帧朝向可能导致画面出现短暂的裁切异常，因此建议在进房前设置视频帧朝向，且不在进房后进行切换。

  FutureOr<int> setVideoOrientation(ByteRTCVideoOrientation orientation) async {
    return await nativeCall('setVideoOrientation:', [orientation.$value]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhangzhenyu.samuel
  /// @brief 切换视频内部采集时使用的前置/后置摄像头 <br>
  ///        调用此接口后，在本地会触发 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 回调。
  /// @param cameraId 摄像头类型，参看 ByteRTCCameraID{@link #ByteRTCCameraID}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 默认使用前置摄像头。
  ///        - 如果你正在使用相机进行视频采集，切换操作当即生效；如果相机未启动，后续开启内部采集时，会打开设定的摄像头。

  FutureOr<int> switchCamera(ByteRTCCameraID cameraId) async {
    return await nativeCall('switchCamera:', [cameraId.$value]);
  }

  /// @detail api
  /// @author zhushufan.ref
  /// @brief 获取视频特效接口。
  /// @return 视频特效接口，参看 ByteRTCVideoEffect{@link #ByteRTCVideoEffect}。

  FutureOr<ByteRTCVideoEffect> getVideoEffectInterface() async {
    final result = await nativeCall('getVideoEffectInterface', []);
    return packObject(
        result,
        () => ByteRTCVideoEffect(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 开启/关闭基础美颜。
  /// @param enable 基础美颜开关 <br>
  ///        - YES: 开启基础美颜
  ///        - NO: 关闭基础美颜（默认）
  /// @return
  ///        - 0: 调用成功。
  ///        - –1001: RTC SDK 版本不支持此功能。
  ///        - -12: 本方法不支持在 Audio SDK 中使用。
  ///        - <0: 调用失败，特效 SDK 内部错误，具体错误码请参考[错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///        - 本方法不能与高级视频特效接口共用。如已购买高级视频特效，建议参看[集成指南](https://www.volcengine.com/docs/6348/114717)使用高级美颜、特效、贴纸功能等。
  ///        - 使用此功能需要集成特效 SDK，建议使用特效 SDK v4.4.2+ 版本。更多信息参看 [Native 端基础美颜](https://www.volcengine.com/docs/6348/372605)。
  ///        - 调用 setBeautyIntensity:withIntensity:{@link #ByteRTCEngine#setBeautyIntensity:withIntensity} 设置基础美颜强度。若在调用本方法前没有设置美颜强度，则使用默认强度。各基础美颜模式的强度默认值分别为：美白 0.7，磨皮 0.8，锐化 0.5，清晰 0.7。
  ///        - 本方法仅适用于视频源，不适用于屏幕源。

  FutureOr<int> enableEffectBeauty(BOOL enable) async {
    return await nativeCall('enableEffectBeauty:', [enable]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangjunlin.3182
  /// @brief 调整基础美颜强度。
  /// @param beautyMode 基础美颜模式，参看 ByteRTCEffectBeautyMode{@link #ByteRTCEffectBeautyMode}。
  /// @param intensity 美颜强度，取值范围为 [0,1]。强度为 0 表示关闭。 <br>
  ///                  各基础美颜模式的强度默认值分别为：美白 0.7，磨皮 0.8，锐化 0.5，清晰 0.7。
  /// @return
  ///        - 0: 调用成功。
  ///        - –2: `intensity` 范围超限。
  ///        - –1001: RTC SDK 版本不支持此功能。
  ///        - <0: 调用失败，特效 SDK 内部错误，具体错误码请参考[错误码表](https://www.volcengine.com/docs/6705/102042)。
  /// @note
  ///        - 若在调用 enableVideoEffect{@link #ByteRTCVideoEffect#enableVideoEffect} 前设置美颜强度，则对应美颜功能的强度初始值会根据设置更新。
  ///        - 销毁引擎后，美颜功能强度恢复默认值。

  FutureOr<int> setBeautyIntensity(
      ByteRTCEffectBeautyMode beautyMode, float intensity) async {
    return await nativeCall(
        'setBeautyIntensity:withIntensity:', [beautyMode.$value, intensity]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头（前置/后置）的变焦倍数
  /// @param zoomRatio 变焦倍数。取值范围是 [1, <最大变焦倍数>]。 <br>
  ///                 最大变焦倍数可以通过调用 getCameraZoomMaxRatio{@link #ByteRTCEngine#getCameraZoomMaxRatio} 获取。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能设置摄像头变焦倍数。
  ///        - 设置结果在调用 stopVideoCapture{@link #ByteRTCEngine#stopVideoCapture} 关闭内部采集后失效。
  ///        - 你可以调用 setVideoDigitalZoomConfig:size:{@link #ByteRTCEngine#setVideoDigitalZoomConfig:size} 设置数码变焦参数，调用 setVideoDigitalZoomControl:{@link #ByteRTCEngine#setVideoDigitalZoomControl} 进行数码变焦。

  FutureOr<int> setCameraZoomRatio(float zoomRatio) async {
    return await nativeCall('setCameraZoomRatio:', [zoomRatio]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 获取当前使用的摄像头（前置/后置）的最大变焦倍数
  /// @return 最大变焦倍数
  /// @note 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检测摄像头最大变焦倍数。

  FutureOr<float> getCameraZoomMaxRatio() async {
    return await nativeCall('getCameraZoomMaxRatio', []);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检测当前使用的摄像头（前置/后置），是否支持变焦（数码/光学变焦）。
  /// @return
  ///        - true: 支持
  ///        - false: 不支持
  /// @note 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检测摄像头变焦能力。

  FutureOr<bool> isCameraZoomSupported() async {
    return await nativeCall('isCameraZoomSupported', []);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检测当前使用的摄像头（前置/后置），是否支持闪光灯。
  /// @return
  ///        - true: 支持
  ///        - false: 不支持
  /// @note 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检测闪光能力。

  FutureOr<bool> isCameraTorchSupported() async {
    return await nativeCall('isCameraTorchSupported', []);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 打开/关闭当前使用的摄像头（前置/后置）的闪光灯
  /// @param torchState 打开/关闭。参看 ByteRTCTorchState{@link #ByteRTCTorchState}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能设置闪光灯。
  ///        - 设置结果在调用 stopVideoCapture{@link #ByteRTCEngine#stopVideoCapture} 关闭内部采集后失效。

  FutureOr<int> setCameraTorch(ByteRTCTorchState torchState) async {
    return await nativeCall('setCameraTorch:', [torchState.$value]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检查当前使用的摄像头是否支持手动对焦。
  /// @return
  ///        - true: 支持。
  ///        - false: 不支持。
  /// @note 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集，才能检查摄像头是否支持手动对焦。

  FutureOr<bool> isCameraFocusPositionSupported() async {
    return await nativeCall('isCameraFocusPositionSupported', []);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头的对焦点。
  /// @param position 对焦点坐标。以本地预览画布的左上为坐标原点，`position`的`x`字段为对焦点水平方向归一化坐标，`y`字段为对焦点垂直方向归一化坐标，取值范围为 [0, 1]。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集，并且使用 SDK 内部渲染时，才能设置对焦点。
  ///        - 对焦点设置为画布中央（即`x`和`y`均取 0.5）时，恢复系统默认值。
  ///        - 调用 stopVideoCapture{@link #ByteRTCEngine#stopVideoCapture} 关闭内部采集后，设置的对焦点失效。

  FutureOr<int> setCameraFocusPosition(CGPoint position) async {
    return await nativeCall('setCameraFocusPosition:', [position]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 检查当前使用的摄像头是否支持手动设置曝光点。
  /// @return
  ///        - true: 支持。
  ///        - false: 不支持。
  /// @note 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能检查曝光点设置能力。

  FutureOr<bool> isCameraExposurePositionSupported() async {
    return await nativeCall('isCameraExposurePositionSupported', []);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头的曝光点。
  /// @param position 曝光点坐标。以本地预览画布的左上为坐标原点，`position`的`x`字段为曝光点水平方向归一化坐标，`y`字段为曝光点垂直方向归一化坐标，取值范围为 [0, 1]。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集，并且使用 SDK 内部渲染时，才能设置曝光点。
  ///        - 曝光点设置为画布中央（即`x`和`y`均取 0.5）时，恢复系统默认值。
  ///        - 调用 stopVideoCapture{@link #ByteRTCEngine#stopVideoCapture} 关闭内部采集后，设置的曝光点失效。

  FutureOr<int> setCameraExposurePosition(CGPoint position) async {
    return await nativeCall('setCameraExposurePosition:', [position]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 设置当前使用的摄像头的曝光补偿。
  /// @param val 曝光补偿值，取值范围 [-1, 1]，0 为系统默认值(没有曝光补偿)。
  /// @return
  ///        - 0: 成功。
  ///        - < 0: 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 必须已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 使用 SDK 内部采集模块进行视频采集时，才能设置曝光补偿。
  ///        - 调用 stopVideoCapture{@link #ByteRTCEngine#stopVideoCapture} 关闭内部采集后，设置的曝光补偿失效。

  FutureOr<int> setCameraExposureCompensation(float val) async {
    return await nativeCall('setCameraExposureCompensation:', [val]);
  }

  /// @hidden(macOS)
  /// @valid since 353
  /// @detail api
  /// @author yinkaisheng
  /// @brief 启用或禁用内部采集时人脸自动曝光模式。此模式会改善强逆光下，脸部过暗的问题；但也会导致 ROI 以外区域过亮/过暗的问题。
  /// @param enable 是否启用。iOS默认开启，Android默认关闭。
  /// @return
  ///        - 0: 成功.
  ///        - !0: 失败.
  /// @note 在采集前或采集中调用此接口均可生效。

  FutureOr<int> enableCameraAutoExposureFaceMode(bool enable) async {
    return await nativeCall('enableCameraAutoExposureFaceMode:', [enable]);
  }

  /// @hidden(macOS)
  /// @valid since 353
  /// @detail api
  /// @author yinkaisheng
  /// @brief 设置内部采集适用动态帧率时，帧率的最小值。
  /// @param framerate 最小值。单位为 fps。默认值是 7。 <br>
  ///                  动态帧率的最大帧率是通过 setVideoCaptureConfig:{@link #ByteRTCEngine#setVideoCaptureConfig} 设置的帧率值。当传入参数大于最大帧率时，使用固定帧率模式，帧率为最大帧率；当传入参数小于最大帧率时，使用动态帧率。
  /// @return
  ///        - 0: 成功.
  ///        - !0: 失败.
  /// @note
  ///        - 你必须在调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 开启内部采集前，调用此接口方可生效。
  ///        - 如果由于性能降级、静态适配等原因导致采集最大帧率变化时，已设置的最小帧率值会与新的采集最大帧率值重新比较。比较结果变化可能导致固定/动态帧率模式切换。
  ///        - 对 Android，默认开启动态帧率模式
  ///        - 对 iOS，默认使用固定帧率模式

  FutureOr<int> setCameraAdaptiveMinimumFrameRate(int framerate) async {
    return await nativeCall('setCameraAdaptiveMinimumFrameRate:', [framerate]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 通过视频帧发送 SEI 数据。 <br>
  ///        在视频通话场景下，SEI 数据会随视频帧发送；在语音通话场景下，SDK 会自动生成一路 16px × 16px 的黑帧视频流用来发送 SEI 数据。
  /// @param message SEI 消息，建议每帧 SEI 数据总长度长度不超过 4 KB。
  /// @param repeatCount 消息发送重复次数。取值范围是 [0, max{29, \%{视频帧率}-1}]。推荐范围 [2,4]。 <br>
  ///                    调用此接口后，SEI 数据会添加到从当前视频帧开始的连续 `repeatCount+1` 个视频帧中。
  /// @param mode SEI 发送模式，参看 ByteRTCSEICountPerFrame{@link #ByteRTCSEICountPerFrame}。
  /// @return
  ///        - >= 0: 将被添加到视频帧中的 SEI 的数量。
  ///        - < 0: 发送失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 每秒发送的 SEI 消息数量建议不超过当前的视频帧率。在语音通话场景下，黑帧帧率为 15 fps。
  ///        - 语音通话场景中，仅支持在内部采集模式下调用该接口发送 SEI 数据。
  ///        - 视频通话场景中，使用自定义采集并通过 pushExternalVideoFrame:{@link #ByteRTCEngine#pushExternalVideoFrame} 推送至 SDK 的视频帧，若本身未携带 SEI 数据，也可通过本接口发送 SEI 数据；若原视频帧中已添加了 SEI 数据，则调用此方法不生效。
  ///        - 视频帧仅携带前后 2s 内收到的 SEI 数据；语音通话场景下，若调用此接口后 1min 内未有 SEI 数据发送，则 SDK 会自动取消发布视频黑帧。
  ///        - 消息发送成功后，远端会收到 rtcEngine:onSEIMessageReceived:info:andMessage:{@link #ByteRTCEngineDelegate#rtcEngine:onSEIMessageReceived:info:andMessage} 回调。
  ///        - 语音通话切换至视频通话时，会停止使用黑帧发送 SEI 数据，自动转为用采集到的正常视频帧发送 SEI 数据。

  FutureOr<int> sendSEIMessage(
      NSData message, int repeatCount, ByteRTCSEICountPerFrame mode) async {
    return await nativeCall('sendSEIMessage:andRepeatCount:andCountPerFrame:',
        [message, repeatCount, mode.$value]);
  }

  /// @hidden for internal use only
  /// @valid since 3.56
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief WTN 流视频帧发送 SEI 数据。
  /// @param channelId SEI 的消息传输通道，取值范围 [0 - 255]。通过此参数，你可以为不同接受方设置不同的 ChannelID，这样不同接收方可以根据回调中的 ChannelID 选择应关注的 SEI 信息。
  /// @param message SEI 消息。
  /// @param repeatCount 消息发送重复次数。取值范围是 [0, max{29, \%{视频帧率}-1}]。推荐范围 [2,4]。 <br>
  ///                    调用此接口后，SEI 数据会添加到从当前视频帧开始的连续 `repeat_count+1` 个视频帧中。
  /// @param mode SEI 发送模式，参看 ByteRTCSEICountPerFrame{@link #ByteRTCSEICountPerFrame}。
  /// @return
  ///        - < 0：说明调用失败
  ///        - = 0：说明当前发送队列已满，无法发送
  ///        - > 0: 说明调用成功，该数值为已经发送 SEI 的数量
  /// @note
  ///        - 每秒发送的 SEI 消息数量建议不超过当前的视频帧率
  ///        - 视频通话场景中，使用自定义采集并通过 pushExternalVideoFrame:{@link #ByteRTCEngine#pushExternalVideoFrame} 推送至 SDK 的视频帧，若本身未携带 SEI 数据，也可通过本接口发送 SEI 数据；若原视频帧中已添加了 SEI 数据，则调用此方法不生效。
  ///        - 视频帧仅携带前后 2s 内收到的 SEI 数据
  ///        - 消息发送成功后，远端会收到 rtcEngine:onPublicStreamSEIMessageReceivedWithChannel:andChannelId:andMessage:回调。
  ///        - 调用失败时，本地及远端都不会收到回调。

  FutureOr<int> sendPublicStreamSEIMessage(int channelId, NSData message,
      int repeatCount, ByteRTCSEICountPerFrame mode) async {
    return await nativeCall(
        'sendPublicStreamSEIMessage:andMessage:andRepeatCount:andCountPerFrame:',
        [channelId, message, repeatCount, mode.$value]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 设置本地摄像头数码变焦参数，包括缩放倍数，移动步长。
  /// @param type 数码变焦参数类型，缩放系数或移动步长。参看 ByteRTCZoomConfigType{@link #ByteRTCZoomConfigType}。必填。
  /// @param size 缩放系数或移动步长，保留到小数点后三位。默认值为 0。必填。 <br>
  ///                  选择不同 `type` 时有不同的取值范围。当计算后的结果超过缩放和移动边界时，取临界值。 <br>
  ///                  - `ByteRTCZoomConfigTypeFocusOffset`：缩放系数增量，范围为 [0, 7]。例如，设置为 0.5 时，如果调用 setVideoDigitalZoomControl:{@link #ByteRTCEngine#setVideoDigitalZoomControl} 选择 Zoom in，则缩放系数增加 0.5。缩放系数范围 [1，8]，默认为 `1`，原始大小。
  ///                  - `ByteRTCZoomConfigTypeMoveOffset`：移动百分比，范围为 [0, 0.5]，默认为 0，不移动。如果调用 setVideoDigitalZoomControl:{@link #ByteRTCEngine#setVideoDigitalZoomControl} 选择的是左右移动，则移动距离为 size x 原始视频宽度；如果选择的是上下移动，则移动距离为 size x 原始视频高度。例如，视频帧边长为 1080 px，设置为 0.5 时，实际移动距离为 0.5 x 1080 px = 540 px。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 每次调用本接口只能设置一种参数。如果缩放系数和移动步长都需要设置，分别调用本接口传入相应参数。
  ///        - 由于移动步长的默认值为 `0` ，在调用 setVideoDigitalZoomControl:{@link #ByteRTCEngine#setVideoDigitalZoomControl} 或 startVideoDigitalZoomControl:{@link #ByteRTCEngine#startVideoDigitalZoomControl} 进行数码变焦操作前，应先调用本接口。

  FutureOr<int> setVideoDigitalZoomConfig(
      ByteRTCZoomConfigType type, float size) async {
    return await nativeCall(
        'setVideoDigitalZoomConfig:size:', [type.$value, size]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 控制本地摄像头数码变焦，缩放或移动一次。设置对本地预览画面和发布到远端的视频都生效。
  /// @param direction 数码变焦操作类型，参看 ByteRTCZoomDirectionType{@link #ByteRTCZoomDirectionType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 由于默认步长为 `0`，调用该方法前需通过 setVideoDigitalZoomConfig:size:{@link #ByteRTCEngine#setVideoDigitalZoomConfig:size} 设置参数。
  ///        - 调用该方法进行移动前，应先使用本方法或 startVideoDigitalZoomControl:{@link #ByteRTCEngine#startVideoDigitalZoomControl} 进行放大，否则无法移动。
  ///        - 当数码变焦操作超出范围时，将置为临界值。例如，移动到了图片边界、放大到了 8 倍、缩小到原图大小。
  ///        - 如果你希望实现持续数码变焦操作，调用 startVideoDigitalZoomControl:{@link #ByteRTCEngine#startVideoDigitalZoomControl}。
  ///        - 移动端可对摄像头进行光学变焦控制，参看 `setCameraZoomRatio:`.

  FutureOr<int> setVideoDigitalZoomControl(
      ByteRTCZoomDirectionType direction) async {
    return await nativeCall('setVideoDigitalZoomControl:', [direction.$value]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 开启本地摄像头持续数码变焦，缩放或移动。设置对本地预览画面和发布到远端的视频都生效。
  /// @param direction 数码变焦操作类型，参看 ByteRTCZoomDirectionType{@link #ByteRTCZoomDirectionType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 由于默认步长为 `0`，调用该方法前需通过 setVideoDigitalZoomConfig:size:{@link #ByteRTCEngine#setVideoDigitalZoomConfig:size} 设置参数。
  ///        - 调用该方法进行移动前，应先使用本方法或 setVideoDigitalZoomControl:{@link #ByteRTCEngine#setVideoDigitalZoomControl} 进行放大，否则无法移动。
  ///        - 当数码变焦操作超出范围时，将置为临界值并停止操作。例如，移动到了图片边界、放大到了 8 倍、缩小到原图大小。
  ///        - 你也可以调用 stopVideoDigitalZoomControl{@link #ByteRTCEngine#stopVideoDigitalZoomControl} 手动停止控制。
  ///        - 如果你希望实现单次数码变焦操作，调用 setVideoDigitalZoomControl:{@link #ByteRTCEngine#setVideoDigitalZoomControl}。
  ///        - 如果你希望实现单次数码变焦操作，调用 `setVideoDigitalZoomControl:`。

  FutureOr<int> startVideoDigitalZoomControl(
      ByteRTCZoomDirectionType direction) async {
    return await nativeCall(
        'startVideoDigitalZoomControl:', [direction.$value]);
  }

  /// @valid since 3.51
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author likai.666
  /// @brief 停止本地摄像头持续数码变焦。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note 关于开始数码变焦，参看 startVideoDigitalZoomControl:{@link #ByteRTCEngine#startVideoDigitalZoomControl}。

  FutureOr<int> stopVideoDigitalZoomControl() async {
    return await nativeCall('stopVideoDigitalZoomControl', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhushufan.ref
  /// @brief 设置自定义视频前处理器。 <br>
  ///        使用这个视频前处理器，你能够调用 processVideoFrame:{@link #ByteRTCVideoProcessorDelegate#processVideoFrame} 对 RTC SDK 采集得到的视频帧进行前处理，并将处理后的视频帧用于 RTC 音视频通信。
  /// @param processor 自定义视频处理器，详见 ByteRTCVideoProcessorDelegate{@link #ByteRTCVideoProcessorDelegate}。如果传入 null，则不对 RTC SDK 采集得到的视频帧进行前处理。 <br>
  ///        SDK 只持有 processor 的弱引用，你应保证其生命周期。 <br>
  ///        在设计 `processor` 时，应从 ByteRTCVideoFrame{@link #ByteRTCVideoFrame} 的 `textureBuf` 字段获取视频帧数据； <br>
  ///        处理后返回的视频帧数据格式应为 ByteRTCVideoPixelFormat{@link #ByteRTCVideoPixelFormat} 中的 `ByteRTCVideoPixelFormatCVPixelBuffer`，且必须存放在返回帧数据的 `textureBuf` 字段中。
  /// @param config 自定义视频前处理器适用的设置，详见 ByteRTCVideoPreprocessorConfig{@link #ByteRTCVideoPreprocessorConfig}。 <br>
  ///               当前，`config` 中的 `required_pixel_format` 仅支持：`ByteRTCVideoPixelFormatI420` 和 `ByteRTCVideoPixelFormatUnknown`： <br>
  ///               - 设置为 `Unknown` 时，RTC SDK 给出供 processor 处理的视频帧格式即采集的格式。
  ///               - 设置为 `ByteRTCVideoPixelFormatI420` 时，RTC SDK 会将采集得到的视频转变为对应的格式，供前处理使用。
  ///               - 设置为其他值时，此方法调用失败。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 重复调用此接口时，仅最后一次调用生效。效果不会叠加。
  ///        - 对于 iOS 平台，将 ByteRTCVideoPreprocessorConfig{@link #ByteRTCVideoPreprocessorConfig} 中的 requiredPixelFormat 设置为 `kVideoPixelFormatUnknown`，可以通过避免格式转换带来一些性能优化。

  FutureOr<int> registerLocalVideoProcessor(
      id<ByteRTCVideoProcessorDelegate> processor,
      ByteRTCVideoPreprocessorConfig config) async {
    return await nativeCall(
        'registerLocalVideoProcessor:withConfig:', [processor, config]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 注册本地视频帧监测器。 <br>
  ///        无论使用内部采集还是自定义采集，调用该方法后，SDK 每监测到一帧本地视频帧时，都会将视频帧信息通过 onLocalEncodedVideoFrame:Frame:{@link #ByteRTCLocalEncodedVideoFrameObserver#onLocalEncodedVideoFrame:Frame} 回调给用户
  /// @param frameObserver 本地视频帧监测器，参看 ByteRTCLocalEncodedVideoFrameObserver{@link #ByteRTCLocalEncodedVideoFrameObserver}。将参数设置为 nullptr 则取消注册。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 该方法可在进房前后任意时间调用，在进房前调用可保证尽可能早地监测视频帧并触发回调

  FutureOr<int> registerLocalEncodedVideoFrameObserver(
      id<ByteRTCLocalEncodedVideoFrameObserver> frameObserver) async {
    return await nativeCall(
        'registerLocalEncodedVideoFrameObserver:', [frameObserver]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author yezijian.me
  /// @brief 强制切换当前的音频播放路由。默认使用 setDefaultAudioRoute:{@link #ByteRTCEngine#setDefaultAudioRoute} 中设置的音频路由。 <br>
  ///        音频播放路由发生变化时，会收到 rtcEngine:onAudioRouteChanged:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioRouteChanged} 回调。
  /// @param audioRoute 音频播放路由，参见 ByteRTCAudioRoute{@link #ByteRTCAudioRoute}。仅支持扬声器和默认路由设备。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///      - 对于绝大多数音频场景，使用 setDefaultAudioRoute:{@link #ByteRTCEngine#setDefaultAudioRoute} 设置默认音频路由，并借助 RTC SDK 的音频路由自动切换逻辑即可完成。切换逻辑参见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836)。你应仅在例外的场景下，使用此接口，比如在接入外接音频设备时，手动切换音频路由。
  ///      - 本接口仅支持在 `ByteRTCAudioScenarioCommunication` 音频场景下使用。你可以通过调用 setAudioScenario:{@link #ByteRTCEngine#setAudioScenario} 切换音频场景。
  ///      - 不同音频场景中，音频路由和发布订阅状态到音量类型的映射关系详见 ByteRTCAudioScenarioType{@link #ByteRTCAudioScenarioType} 。

  FutureOr<int> setAudioRoute(ByteRTCAudioRoute audioRoute) async {
    return await nativeCall('setAudioRoute:', [audioRoute.$value]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author dixing
  /// @brief 获取当前使用的音频播放路由。
  /// @return 详见 ByteRTCAudioRoute{@link #ByteRTCAudioRoute}
  /// @note 要设置音频路由，详见 setAudioRoute:{@link #ByteRTCEngine#setAudioRoute}，仅适用于移动端。

  FutureOr<ByteRTCAudioRoute> getAudioRoute() async {
    return await nativeCall('getAudioRoute', []);
  }

  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 启用匹配外置声卡的音频处理模式
  /// @param enable <br>
  ///        - true: 开启
  ///        - false: 不开启(默认)
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 当采用外接声卡进行音频采集时，建议开启此模式，以获得更好的音质。
  ///        - 开启此模式时，仅支持耳机播放。如果需要使用扬声器或者外置音箱播放，关闭此模式。

  FutureOr<int> enableExternalSoundCard(bool enable) async {
    return await nativeCall('enableExternalSoundCard:', [enable]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author yezijian.me
  /// @brief 将默认的音频播放设备设置为听筒或扬声器。
  /// @param audioRoute 音频播放设备。参看 ByteRTCAudioRoute{@link #ByteRTCAudioRoute}。仅支持听筒或扬声器。
  /// @return
  ///        - 0: 方法调用成功。
  ///        - < 0: 方法调用失败。
  /// @note 对于音频路由切换逻辑，参见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836)。
  ///

  FutureOr<int> setDefaultAudioRoute(ByteRTCAudioRoute audioRoute) async {
    return await nativeCall('setDefaultAudioRoute:', [audioRoute.$value]);
  }

  FutureOr<int> pushClientMixedStreamExternalVideoFrame(
      NSString uid, ByteRTCVideoFrameData frame) async {
    return await nativeCall(
        'pushClientMixedStreamExternalVideoFrame:withFrame:', [uid, frame]);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @region 转推 CDN/WTN
  /// @author liujingchao
  /// @brief 设置客户端合流的观察者
  /// @param observer 客户端转推直播观察者。详见 ByteRTCClientMixedStreamDelegate{@link #ByteRTCClientMixedStreamDelegate}。 <br>
  ///        通过注册 observer 接收转推直播相关的回调。
  /// @return 方法调用结果。 <br>
  ///         -  0：方法调用成功
  ///         - < 0：方法调用失败

  FutureOr<int> setClientMixedStreamObserver(
      id<ByteRTCClientMixedStreamDelegate> observer) async {
    return await nativeCall('setClientMixedStreamObserver:', [observer]);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao

  FutureOr<int> startClientMixedStream(
      NSString taskId,
      ByteRTCMixedStreamConfig config,
      ByteRTCClientMixedStreamConfig extraConfig) async {
    return await nativeCall(
        'startClientMixedStream:withMixedConfig:withExtraConfig:',
        [taskId, config, extraConfig]);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao

  FutureOr<int> updateClientMixedStream(
      NSString taskId,
      ByteRTCMixedStreamConfig config,
      ByteRTCClientMixedStreamConfig extraConfig) async {
    return await nativeCall(
        'updateClientMixedStream:withMixedConfig:withExtraConfig:',
        [taskId, config, extraConfig]);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao

  FutureOr<int> stopClientMixedStream(NSString taskId) async {
    return await nativeCall('stopClientMixedStream:', [taskId]);
  }

  /// @valid since 3.60.
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author lizheng
  /// @brief 指定房间中的媒体流，合成后一路流发布到 CDN 或发布一路 WTN 流。
  /// @param taskId 转推直播任务 ID，长度不超过 126 字节。 当 ByteRTCMixedStreamConfig{@link #ByteRTCMixedStreamConfig} 中的 `PushTargetType = 0` 时， 用于标识转推直播任务。你可以在同一房间内发起多个转推直播任务，并用不同的 ID 加以区分。当你需要发起多个转推直播任务时，应使用多个 ID；当你仅需发起一个转推直播任务时，建议使用空字符串。
  /// 当 `PushTargetType = 1` 时，设置无效，传空即可。
  /// @param pushTargetConfig 推流目标配置参数，比如设置推流地址、WTN流 ID。参看 ByteRTCMixedStreamPushTargetConfig{@link #ByteRTCMixedStreamPushTargetConfig}。
  /// @param config 转推直播配置参数，比如设置合流的图片、视频视图布局和音频属性。参看 ByteRTCMixedStreamConfig{@link #ByteRTCMixedStreamConfig}。
  /// @return
  ///        - 0: 成功。你可以通过 rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode:{@link #ByteRTCEngineDelegate#rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode} 回调获取启动结果和推流过程中的事件。
  ///        - !0: 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 在[控制台](https://console.volcengine.com/rtc/cloudRTC?tab=callback)配置了转推直播和 WTN 流的服务端回调后，调用本接口会收到相应回调。重复调用该接口时，第二次调用会同时触发 [TranscodeStarted](https://www.volcengine.com/docs/6348/75125#transcodestarted) 和 [TranscodeUpdated](https://www.volcengine.com/docs/6348/75125#transcodeupdated)。
  ///       - 调用 stopPushMixedStream:withPushTargetType:{@link #ByteRTCEngine#stopPushMixedStream:withPushTargetType} 停止转推直播。
  ///       - 调用 updatePushMixedStream:withPushTargetConfig:withMixedConfig:{@link #ByteRTCEngine#updatePushMixedStream:withPushTargetConfig:withMixedConfig} 可以更新部分任务参数。
  ///       - 调用 startPushSingleStream:singleStream:{@link #ByteRTCEngine#startPushSingleStream:singleStream} 可以转推单路流到 CDN。
  /// @order 0
  ///

  FutureOr<int> startPushMixedStream(
      NSString taskId,
      ByteRTCMixedStreamPushTargetConfig pushTargetConfig,
      ByteRTCMixedStreamConfig config) async {
    return await nativeCall(
        'startPushMixedStream:withPushTargetConfig:withMixedConfig:',
        [taskId, pushTargetConfig, config]);
  }

  /// @valid since 3.60.
  /// @hidden(Linux)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author lizheng
  /// @brief 更新推 CDN/WTN 流转推参数，会收到 rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode:{@link #ByteRTCEngineDelegate#rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode} 回调。 <br>
  ///        使用 startPushMixedStream:withPushTargetConfig:withMixedConfig:{@link #ByteRTCEngine#startPushMixedStream:withPushTargetConfig:withMixedConfig} 启用转推直播功能后，使用此方法更新功能配置参数。
  /// @param taskId 转推直播任务 ID。指定想要更新参数设置的转推直播任务。
  /// @param pushTargetConfig 推流目标配置参数，比如设置推流地址、WTN流 ID。参看 ByteRTCMixedStreamPushTargetConfig{@link #ByteRTCMixedStreamPushTargetConfig}。
  /// @param config 转推直播配置参数，参看 ByteRTCMixedStreamConfig{@link #ByteRTCMixedStreamConfig}。除特殊说明外，均支持过程中更新。 <br>
  ///        调用时，结构体中没有传入值的属性，会被更新为默认值。
  /// @return
  ///        - 0: 成功。
  ///        - !0: 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @order 4
  ///

  FutureOr<int> updatePushMixedStream(
      NSString taskId,
      ByteRTCMixedStreamPushTargetConfig pushTargetConfig,
      ByteRTCMixedStreamConfig config) async {
    return await nativeCall(
        'updatePushMixedStream:withPushTargetConfig:withMixedConfig:',
        [taskId, pushTargetConfig, config]);
  }

  /// @valid since 3.60.
  /// @author lizheng
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @brief 停止 WTN 流或合流转推 CDN 任务。
  /// @param taskId 转推直播任务 ID。指定想要更新参数设置的转推直播任务。
  /// @param pushTargetType 参看 ByteRTCMixedStreamPushTargetType{@link #ByteRTCMixedStreamPushTargetType}。
  /// @return
  ///        + 0: 成功
  ///        + !0: 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 关于启动合流转推直播或 WTN 流，参看 startPushMixedStream:withPushTargetConfig:withMixedConfig:{@link #ByteRTCEngine#startPushMixedStream:withPushTargetConfig:withMixedConfig}。
  /// @order 3
  ///

  FutureOr<int> stopPushMixedStream(
      NSString taskId, ByteRTCMixedStreamPushTargetType pushTargetType) async {
    return await nativeCall('stopPushMixedStream:withPushTargetType:',
        [taskId, pushTargetType.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao
  /// @brief 新增单流转推直播任务。
  /// @param taskId 任务 ID。 <br>
  ///               你可以发起多个转推直播任务，并用不同的任务 ID 加以区分。当你需要发起多个转推直播任务时，应使用多个 ID；当你仅需发起一个转推直播任务时，建议使用空字符串。
  /// @param singleStream 转推直播配置参数。详见 ByteRTCPushSingleStreamParam{@link #ByteRTCPushSingleStreamParam}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用该方法后，关于启动结果和推流过程中的错误，会收到 rtcEngine:onSingleStreamEvent:withTaskId:withErrorCode:{@link #ByteRTCEngineDelegate#rtcEngine:onSingleStreamEvent:withTaskId:withErrorCode} 回调。
  ///       - 调用 stopPushSingleStream:{@link #ByteRTCEngine#stopPushSingleStream} 停止任务。
  ///       - 由于本功能不进行编解码，所以推到 RTMP 的视频流会根据推流端的分辨率、编码方式、关闭摄像头等变化而变化。

  FutureOr<int> startPushSingleStream(
      NSString taskId, ByteRTCPushSingleStreamParam singleStream) async {
    return await nativeCall(
        'startPushSingleStream:singleStream:', [taskId, singleStream]);
  }

  /// @detail api
  /// @author liujingchao
  /// @brief 停止单流转推直播任务。 <br>
  /// @param taskId 任务 ID。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 关于启动单流转推直播，参看 startPushSingleStream:singleStream:{@link #ByteRTCEngine#startPushSingleStream:singleStream}。
  ///        - 关于启动合流转推直播，参看 startPushMixedStream:withPushTargetConfig:withMixedConfig:{@link #ByteRTCEngine#startPushMixedStream:withPushTargetConfig:withMixedConfig}。

  FutureOr<int> stopPushSingleStream(NSString taskId) async {
    return await nativeCall('stopPushSingleStream:', [taskId]);
  }

  /// @hidden internal use only
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao
  /// @brief 开启缓存同步功能。开启后，会缓存收到的实时音视频数据，并对齐不同数据中的时间戳完成同步。此功能会影响音视频数据消费的实时性。
  /// @param config 参看 ByteRTCChorusCacheSyncConfig{@link #ByteRTCChorusCacheSyncConfig}。
  /// @param observer 事件和数据观察者，参看 ByteRTCChorusCacheSyncObserver{@link #ByteRTCChorusCacheSyncObserver}。
  /// @return 查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 要关闭缓存同步功能，调用 stopChorusCacheSync{@link #ByteRTCEngine#stopChorusCacheSync}。
  ///

  FutureOr<int> startChorusCacheSync(ByteRTCChorusCacheSyncConfig config,
      id<ByteRTCChorusCacheSyncObserver> observer) async {
    return await nativeCall(
        'startChorusCacheSync:observer:', [config, observer]);
  }

  /// @hidden internal use only
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liujingchao
  /// @brief 关闭缓存同步功能。
  /// @return 查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  ///

  FutureOr<int> stopChorusCacheSync() async {
    return await nativeCall('stopChorusCacheSync', []);
  }

  FutureOr<ByteRTCWTNStream> getWTNStream() async {
    final result = await nativeCall('getWTNStream', []);
    return packObject(
        result,
        () =>
            ByteRTCWTNStream(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 推送外部视频帧。
  /// @param frame 该视频帧包含待 SDK 编码的视频数据，参考 ByteRTCVideoFrame{@link #ByteRTCVideoFrame}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 推送外部视频帧前，必须调用 setVideoSourceType:{@link #ByteRTCEngine#setVideoSourceType} 开启外部视频源采集。 <br>
  ///       支持格式：raw NV12

  FutureOr<int> pushExternalVideoFrame(ByteRTCVideoFrameData frame) async {
    return await nativeCall('pushExternalVideoFrame:', [frame]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 设置并开启指定的音频数据帧回调
  /// @param method 音频回调方法，参看 ByteRTCAudioFrameCallbackMethod{@link #ByteRTCAudioFrameCallbackMethod}。 <br>
  ///               当音频回调方法设置为 `0`、`1`、`2`、`5`时，你需要在参数 `format` 中指定准确的采样率和声道，暂不支持设置为自动。 <br>
  ///               当音频回调方法设置为 `3` 时，将 `format` 中的各个字段设置为默认值。
  /// @param format 音频参数格式，参看 ByteRTCAudioFormat{@link #ByteRTCAudioFormat}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 开启音频回调并调用 registerAudioFrameObserver:{@link #ByteRTCMediaPlayer#registerAudioFrameObserver} 后，ByteRTCAudioFrameObserver{@link #ByteRTCAudioFrameObserver} 会收到对应的音频回调。两者调用顺序没有限制且相互独立。

  FutureOr<int> enableAudioFrameCallback(
      ByteRTCAudioFrameCallbackMethod method, ByteRTCAudioFormat format) async {
    return await nativeCall(
        'enableAudioFrameCallback:format:', [method.$value, format]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 关闭音频回调
  /// @param method 音频回调方法，参看 ByteRTCAudioFrameCallbackMethod{@link #ByteRTCAudioFrameCallbackMethod}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 该方法需要在调用 enableAudioFrameCallback:format:{@link #ByteRTCEngine#enableAudioFrameCallback:format} 之后调用。

  FutureOr<int> disableAudioFrameCallback(
      ByteRTCAudioFrameCallbackMethod method) async {
    return await nativeCall('disableAudioFrameCallback:', [method.$value]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 注册音频数据回调观察者。
  /// @param audioFrameObserver 音频数据观察者，参看 ByteRTCAudioFrameObserver{@link #ByteRTCAudioFrameObserver}。如果传入 null，则取消注册。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 注册音频数据回调观察者并调用 enableAudioFrameCallback:format:{@link #ByteRTCEngine#enableAudioFrameCallback:format} 后，ByteRTCAudioFrameObserver{@link #ByteRTCAudioFrameObserver} 会收到对应的音频回调。对回调中收到的音频数据进行处理，不会影响 RTC 的编码发送或渲染。

  FutureOr<int> registerAudioFrameObserver(
      id<ByteRTCAudioFrameObserver> audioFrameObserver) async {
    return await nativeCall(
        'registerAudioFrameObserver:', [audioFrameObserver]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 注册自定义音频处理器。 <br>
  ///        注册完成后，你可以调用 enableAudioProcessor:audioFormat:{@link #ByteRTCEngine#enableAudioProcessor:audioFormat}，对本地采集到的音频进行处理，RTC SDK 将对处理后的音频进行编码和发送。也可以对接收到的远端音频进行自定义处理，RTC SDK 将对处理后的音频进行渲染。
  /// @param processor 自定义音频处理器，详见 ByteRTCAudioFrameProcessor{@link #ByteRTCAudioFrameProcessor}。 <br>
  ///        SDK 只持有 processor 的弱引用，你应保证其生命周期。需要取消注册时，设置此参数为 nullptr。 <br>
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  /// - 重复调用此接口时，仅最后一次调用生效。
  /// - 更多相关信息，详见[音频自定义处理](https://www.volcengine.com/docs/6348/80635)。

  FutureOr<int> registerAudioProcessor(
      id<ByteRTCAudioFrameProcessor> processor) async {
    return await nativeCall('registerAudioProcessor:', [processor]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 设置并开启指定的音频帧回调，进行自定义处理。
  /// @param method 音频帧类型，参看 ByteRTCAudioFrameMethod{@link #ByteRTCAudioFrameMethod}。可多次调用此接口，处理不同类型的音频帧。 <br>
  ///        选择不同类型的音频帧将收到对应的回调： <br>
  ///        - 选择本地采集的音频时，会收到 onProcessRecordAudioFrame:{@link #ByteRTCAudioFrameProcessor#onProcessRecordAudioFrame}。
  ///        - 选择远端音频流的混音音频时，会收到 onProcessPlayBackAudioFrame:{@link #ByteRTCAudioFrameProcessor#onProcessPlayBackAudioFrame}。
  ///        - 选择远端音频流时，会收到 onProcessRemoteUserAudioFrame:info:audioFrame:{@link #ByteRTCAudioFrameProcessor#onProcessRemoteUserAudioFrame:info:audioFrame}。
  ///        - 选择软件耳返音频时，会收到 onProcessEarMonitorAudioFrame:{@link #ByteRTCAudioFrameProcessor#onProcessEarMonitorAudioFrame}。(仅适用于 iOS 平台)
  ///        - 选择屏幕共享音频流时，会收到 onProcessScreenAudioFrame:{@link #ByteRTCAudioFrameProcessor#onProcessScreenAudioFrame}。
  /// @param format 设定自定义处理时获取的音频帧格式，参看 ByteRTCAudioFormat{@link #ByteRTCAudioFormat}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 在调用此接口前，你需要调用 registerAudioProcessor:{@link #ByteRTCEngine#registerAudioProcessor} 注册自定义音频处理器。
  ///        - 要关闭音频自定义处理，调用 disableAudioProcessor:{@link #ByteRTCEngine#disableAudioProcessor}。

  FutureOr<int> enableAudioProcessor(
      ByteRTCAudioFrameMethod method, ByteRTCAudioFormat format) async {
    return await nativeCall(
        'enableAudioProcessor:audioFormat:', [method.$value, format]);
  }

  /// @detail api
  /// @author gongzhengduo
  /// @brief 关闭自定义音频处理。
  /// @param method 音频帧类型，参看 ByteRTCAudioFrameMethod{@link #ByteRTCAudioFrameMethod}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> disableAudioProcessor(ByteRTCAudioFrameMethod method) async {
    return await nativeCall('disableAudioProcessor:', [method.$value]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 推送自定义采集的音频数据到 RTC SDK。
  /// @param audioFrame 音频数据帧，详见 ByteRTCAudioFrame{@link #ByteRTCAudioFrame} <br>
  ///        - 音频采样格式为 S16。音频缓冲区内的数据格式必须为 PCM 数据，其容量大小应该为 audioFrame.samples × audioFrame.channel × 2。
  ///        - 必须指定具体的采样率和声道数，不支持设置为自动。
  /// @return 方法调用结果 <br>
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///       - 推送外部音频数据前，必须先调用 setAudioSourceType:{@link #ByteRTCEngine#setAudioSourceType} 开启自定义采集。
  ///       - 你必须每隔 10 毫秒推送一次外部采集的音频数据。单次推送的 samples (音频采样点个数）应该为 audioFrame.sampleRate / 100。比如设置采样率为 48000 时， 每次应该推送 480 个采样点。

  FutureOr<int> pushExternalAudioFrame(ByteRTCAudioFrame audioFrame) async {
    return await nativeCall('pushExternalAudioFrame:', [audioFrame]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 拉取下行音频数据用于自定义音频渲染。 <br>
  ///        调用该方法后，SDK 会主动拉取待播放的音频数据，包括远端已解码和混音后的音频数据，用于外部播放。
  /// @param audioFrame 音频数据帧，详见 ByteRTCAudioFrame{@link #ByteRTCAudioFrame}
  /// @return 方法调用结果 <br>
  ///        - 0: 设置成功
  ///        - < 0: 设置失败
  /// @note
  ///       - 拉取外部音频数据前，必须先调用 setAudioRenderType:{@link #ByteRTCEngine#setAudioRenderType} 开启自定义渲染。
  ///       - 由于 RTC SDK 的帧长为 10 毫秒，你应当每隔 10 毫秒拉取一次音频数据。确保音频采样点数（sample）x 拉取频率等于 audioFrame 的采样率 （sampleRate）。如设置采样率为 48000 时，每 10 毫秒调用本接口拉取数据，每次应拉取 480 个采样点。
  ///       - 音频采样格式为 S16。音频缓冲区内的数据格式必须为 PCM 数据，其容量大小应该为 audioFrame.samples × audioFrame.channel × 2。

  FutureOr<int> pullExternalAudioFrame(ByteRTCAudioFrame audioFrame) async {
    return await nativeCall('pullExternalAudioFrame:', [audioFrame]);
  }

  FutureOr<int> pushReferenceAudioPCMData(ByteRTCAudioFrame audioFrame) async {
    return await nativeCall('pushReferenceAudioPCMData:', [audioFrame]);
  }

  /// @hidden for internal use only
  /// @region 自定义音频采集渲染
  /// @brief 推送opus编码音频数据到 RTC SDK，RTC SDK纯转发。
  /// @param audio_stream 对应的opus音频数据。详见 EncodedAudioFrameData{@link #EncodedAudioFrameData}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///       - 推送音频数据前，必须先调用 enableAudioEncoding{@link #ByteRTCEngine#enableAudioEncoding} 关闭音频编码。

  FutureOr<int> pushExternalEncodedAudioFrame(
      ByteRTCEncodedAudioFrameData audioFrame) async {
    return await nativeCall('pushExternalEncodedAudioFrame:', [audioFrame]);
  }

  /// @hidden for internal use only
  /// @region 自定义音频采集渲染
  /// @brief 是否使用sdk音频编码功能。
  /// @param enable 是否使用sdk音频编码功能。
  ///      true: 打开音频编码（默认）
  ///      false: 关闭音频编码直接转推。
  /// @note
  ///       - 在pushExternalEncodedAudioFrame{@link #ByteRTCEngine#pushExternalEncodedAudioFrame}之前调用。

  FutureOr<void> enableAudioEncoding(bool enable) async {
    return await nativeCall('enableAudioEncoding:', [enable]);
  }

  /// @hidden for internal use only
  /// @region 自定义音频采集渲染
  /// @brief 是否使用sdk音频解码功能。
  /// @param enable 是否使用sdk音频解码功能。
  ///      true: 打开音频解码功能（默认）
  ///      false: 关闭音频解码功能直接转推。
  /// @note
  ///       - 在registerRemoteEncodedAudioFrameObserver之前调用。

  FutureOr<void> enableAudioDecoding(bool enable) async {
    return await nativeCall('enableAudioDecoding:', [enable]);
  }

  /// @detail api
  /// @hidden for internal use only
  /// @brief 注册远端音频帧监测器。 <br>
  ///        调用该方法后，SDK 每监测到一帧远端音频帧时，都会将音频帧信息通过 onRemoteEncodedAudioFrame:info:audioFrame: 回调给用户
  /// @param observer 远端音频帧监测器，参看 IRemoteEncodedAudioFrameObserver。
  /// @note
  ///       - 该方法建议在进房前调用。
  ///       - 将参数设置为 nullptr 则取消注册。
  ///       - 调用前，必须先调用 enableAudioDecoding{@link #ByteRTCEngine#enableAudioDecoding} 关闭音频解码功能。

  FutureOr<void> registerRemoteEncodedAudioFrameObserver(
      id<ByteRTCRemoteEncodedAudioFrameObserver> observer) async {
    return await nativeCall(
        'registerRemoteEncodedAudioFrameObserver:', [observer]);
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
  ///        - -2： 输入非法，合法字符包括所有小写字母、大写字母和数字，除此外还包括四个独立字符分别是：英文句号，短横线，下划线和 \@ 。
  /// @note
  ///        - 需要在调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 进房之前调用，进房之后调用该方法无效。

  FutureOr<int> setBusinessId(NSString businessId) async {
    return await nativeCall('setBusinessId:', [businessId]);
  }

  /// @detail api
  /// @author wangzhanqiang
  /// @brief 通话结束，将用户反馈的问题上报到 RTC。
  /// @param types 预设问题列表，参看 ByteRTCProblemFeedbackOption{@link #ByteRTCProblemFeedbackOption}
  /// @param info 预设问题以外的其他问题的具体描述，房间信息。参看 ByteRTCProblemFeedbackInfo{@link #ByteRTCProblemFeedbackInfo}
  /// @return
  ///         - 0: 成功。
  ///         - -3: 失败。
  /// @note
  ///         - 你可以在 [RTC 控制台](https://console.volcengine.com/rtc/callQualityRTC/feedback)上查看用户通过此接口提交的反馈详情和整体趋势。
  ///         - 如果用户上报时在房间内，那么问题会定位到用户当前所在的一个或多个房间；如果用户上报时不在房间内，那么问题会定位到引擎此前退出的房间。

  FutureOr<int> feedback(ByteRTCProblemFeedbackOption types,
      ByteRTCProblemFeedbackInfo info) async {
    return await nativeCall('feedback:info:', [types.$value, info]);
  }

  /// @valid since 353
  /// @detail api
  /// @author likai.666
  /// @valid since 353
  /// @brief 获取 C++ 层 [IRTCEngine 句柄](https://www.volcengine.com/docs/6348/70095#irtcengine)。
  /// @return
  ///         - >0：方法调用成功, 返回 C++ 层 `IRTCEngine` 的地址。
  ///         - NULL：方法调用失败
  /// @note 在一些场景下，获取 C++ 层 `IRTCEngine`，并通过其完成操作，相较于通过 OC 封装层完成有显著更高的执行效率。典型的场景有：视频/音频帧自定义处理，音视频通话加密等。

  FutureOr<void> getNativeHandle() async {
    return await nativeCall('getNativeHandle', []);
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置发布的音视频流的回退选项。 <br>
  ///        你可以调用该接口设置网络不佳或设备性能不足时从大流起进行降级处理，以保证通话质量。
  /// @param option 本地发布的音视频流回退选项，参看 ByteRTCPublishFallbackOption{@link #ByteRTCPublishFallbackOption}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅在调用 setLocalSimulcastMode:{@link #ByteRTCEngine#setlocalsimulcastmode} 开启了发送多路视频流的情况下生效。
  ///        - 该方法必须在进房前设置，进房后设置或更改设置无效。
  ///        - 调用该方法后，如因性能或网络不佳产生发布性能回退或恢复，本端会提前收到 rtcEngine:onPerformanceAlarms:info:mode:reason:sourceWantedData:{@link #ByteRTCEngineDelegate#rtcEngine:onPerformanceAlarms:info:mode:reason:sourceWantedData} 回调发出的告警，以便采集设备配合调整。
  ///        - 设置回退后，本地发布的音视频流发生回退或从回退中恢复时，远端会收到 rtcEngine:onSimulcastSubscribeFallback:info:event:{@link #ByteRTCEngineDelegate#rtcEngine:onSimulcastSubscribeFallback:info:event} 回调，通知该情况。
  ///        - 你可以调用客户端 API 或者在服务端下发策略设置回退。当使用服务端下发配置实现时，下发配置优先级高于在客户端使用 API 设定的配置。

  FutureOr<int> setPublishFallbackOption(
      ByteRTCPublishFallbackOption option) async {
    return await nativeCall('setPublishFallbackOption:', [option.$value]);
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置订阅的音视频流的回退选项。 <br>
  ///        你可调用该接口设置网络不佳或设备性能不足时允许订阅流进行降级或只订阅音频流，以保证通话流畅。
  /// @param option 订阅的音视频流回退选项，参看 ByteRTCSubscribeFallbackOption{@link #ByteRTCSubscribeFallbackOption}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 你必须在进房前设置，进房后设置或更改设置无效。
  ///        - 设置回退选项后，订阅的音视频流发生回退或从回退中恢复时，会收到 rtcEngine:onSimulcastSubscribeFallback:info:event:{@link #ByteRTCEngineDelegate#rtcEngine:onSimulcastSubscribeFallback:info:event} 和 rtcEngine:onRemoteVideoSizeChanged:info:withFrameInfo:{@link #ByteRTCEngineDelegate#rtcEngine:onRemoteVideoSizeChanged:info:withFrameInfo} 回调通知。
  ///        - 你可以调用 API 或者在服务端下发策略设置回退。当使用服务端下发配置实现时，下发配置优先级高于在客户端使用 API 设定的配置。

  FutureOr<int> setSubscribeFallbackOption(
      ByteRTCSubscribeFallbackOption option) async {
    return await nativeCall('setSubscribeFallbackOption:', [option.$value]);
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置用户优先级。
  /// @param priority 远端用户的优先级, 详见枚举类型 ByteRTCRemoteUserPriority{@link #ByteRTCRemoteUserPriority}
  /// @param roomId 房间 ID
  /// @param uid 远端用户的 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 该方法与 setSubscribeFallbackOption:{@link #ByteRTCEngine#setSubscribeFallbackOption}  搭配使用。
  ///        - 如果开启了订阅流回退选项，弱网或性能不足时会优先保证收到的高优先级用户的流的质量。
  ///        - 该方法在进房前后都可以使用，可以修改远端用户的优先级。

  FutureOr<int> setRemoteUserPriority(
      ByteRTCRemoteUserPriority priority, NSString roomId, NSString uid) async {
    return await nativeCall(
        'setRemoteUserPriority:InRoomId:uid:', [priority.$value, roomId, uid]);
  }

  /// @detail api
  /// @author wangjunlin.3182
  /// @brief 设置传输时使用内置加密的方式。
  /// @param encrypt_type 内置加密算法，详见 ByteRTCEncryptType{@link #ByteRTCEncryptType}
  /// @param key 加密密钥，长度限制为 36 位，超出部分将会被截断
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 使用传输时内置加密时，使用此方法；如果需要使用传输时自定义加密，参看 onEncryptData:{@link #ByteRTCEncryptHandler#onEncryptData}。
  ///         内置加密和自定义加密互斥，根据最后一个调用的方法确定传输加密的方案。 <br>
  ///       - 该方法必须在进房之前调用，可重复调用，以最后调用的参数作为生效参数。

  FutureOr<int> setEncryptInfo(
      ByteRTCEncryptType encrypt_type, NSString key) async {
    return await nativeCall('setEncryptInfo:key:', [encrypt_type.$value, key]);
  }

  /// @detail api
  /// @author wangjunlin.3182
  /// @brief 设置自定义加密和解密方式。
  /// @param handler 自定义加密 handler，需要实现里面的加密和解密方法。参看 ByteRTCEncryptHandler{@link #ByteRTCEncryptHandler}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 该方法与 setEncryptInfo:key:{@link #ByteRTCEngine#setEncryptInfo:key} 为互斥关系，即按照调用顺序，最后一个调用的方法为最终生效的版本。
  ///       - 该方法必须在调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 之前调用，可重复调用，以最后调用的参数作为生效参数。
  ///       - 无论加密或者解密，其对原始数据的长度修改，需要控制在 180\% 之间，即如果输入数据为 100 字节，则处理完成后的数据必须不超过 180 字节，如果加密或解密结果超出该长度限制，则该音视频帧可能会被丢弃。
  ///       - 数据加密/解密为串行执行，因而视实现方式不同，可能会影响到最终渲染效率，是否使用该方法，需要由使用方谨慎评估。

  FutureOr<int> setCustomizeEncryptHandler(
      id<ByteRTCEncryptHandler> handler) async {
    return await nativeCall('setCustomizeEncryptHandler:', [handler]);
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 创建 RTC 房间实例。 <br>
  ///        调用此方法仅返回一个房间实例，你仍需调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 才能真正地创建/加入房间。 <br>
  ///        多次调用此方法以创建多个 ByteRTCRoom{@link #ByteRTCRoom} 实例。分别调用各 ByteRTCRoom 实例中的 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 方法，同时加入多个房间。 <br>
  ///        多房间模式下，用户可以同时订阅各房间的音视频流。
  /// @param roomId 标识通话房间的房间 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。
  /// @return 创建的 ByteRTCRoom{@link #ByteRTCRoom} 房间实例。
  ///         返回 NULL 时，请确认指定房间是否已经存在或 roomId 格式错误。
  /// @note
  ///        - 如果需要加入的房间已存在，你仍需先调用本方法来获取 ByteRTCRoom 实例，再调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 加入房间。
  ///        - 请勿使用同样的 roomId 创建多个房间，否则后创建的房间实例会替换先创建的房间实例。
  ///        - 如果你需要在多个房间发布音视频流，无须创建多房间，直接调用 startForwardStreamToRooms:{@link #ByteRTCRoom#startForwardStreamToRooms} 开始跨房间转发媒体流。

  FutureOr<ByteRTCRoom> createRTCRoom(NSString roomId) async {
    final result = await nativeCall('createRTCRoom:', [roomId]);
    return packObject(result,
        () => ByteRTCRoom(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author shenpengliang
  /// @brief 创建游戏房间实例。 <br>
  ///        调用此方法仅返回一个房间实例，你仍需调用 joinRoom:userInfo:{@link #ByteRTCGameRoom#joinRoom:userInfo} 才能真正地创建/加入房间。 <br>
  ///        多次调用此方法以创建多个 ByteRTCGameRoom{@link #ByteRTCGameRoom} 实例。分别调用各 GameRTCRoom 实例中的 joinRoom:userInfo:{@link #ByteRTCGameRoom#joinRoom:userInfo} 方法，同时加入多个房间。 <br>
  ///        多房间模式下，用户可以同时订阅各房间的音视频流。
  /// @param roomId 标识通话房间的房间 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。
  /// @param roomConfig 房间配置，参看 GameRoomConfig{@link #GameRoomConfig}。
  /// @return 创建的 ByteRTCGameRoom{@link #ByteRTCGameRoom} 房间实例。
  ///         返回 NULL 时，请确认指定房间是否已经存在或 roomId 格式错误或当前场景是否是游戏场景。
  /// @note
  ///        - 如果需要加入的房间已存在，你仍需先调用本方法来获取 GameRTCRoom 实例，再调用 joinRoom:userInfo:{@link #ByteRTCGameRoom#joinRoom:userInfo} 加入房间。
  ///        - 请勿使用同样的 roomId 创建多个房间，否则后创建的房间实例会替换先创建的房间实例。

  FutureOr<ByteRTCGameRoom> createGameRoom(
      NSString roomId, GameRoomConfig roomConfig) async {
    final result =
        await nativeCall('createGameRoom:roomConfig:', [roomId, roomConfig]);
    return packObject(result,
        () => ByteRTCGameRoom(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @brief 创建 RTS 房间实例。 <br>
  ///        调用此方法仅返回一个RTS房间实例，你仍需调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 才能真正地创建/加入房间。 <br>
  ///        多次调用此方法以创建多个 ByteRTCRoom{@link #ByteRTCRoom} 实例。分别调用各 ByteRTCRTSRoom 实例中的 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 方法，同时加入多个房间。 <br>
  ///        多房间模式下，用户可以同时收发各房间的消息。
  /// @param roomId 标识通话房间的房间 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。
  /// @return 创建的 ByteRTCRoom{@link #ByteRTCRoom} 房间实例。
  ///         返回 NULL 时，请确认指定房间是否已经存在或 roomId 格式错误。
  /// @note
  ///        - 如果需要加入的房间已存在，你仍需先调用本方法来获取 ByteRTCRTSRoom 实例，再调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 加入房间。
  ///        - 请勿使用同样的 roomId 创建多个房间，否则后创建的房间实例会替换先创建的房间实例。

  FutureOr<ByteRTCRTSRoom> createRTSRoom(NSString roomId) async {
    final result = await nativeCall('createRTSRoom:', [roomId]);
    return packObject(result,
        () => ByteRTCRTSRoom(const NativeClassOptions([], disableInit: true)));
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 设置 Extension 配置项。你必须在使用屏幕内部采集功能前，设置使用的 Extension。
  /// @param groupId 你的应用和 Extension 应该归属于同一个 App Group，此处需要传入 Group Id。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 必须在调用 createRTCEngine:delegate:{@link #ByteRTCEngine#createRTCEngine:delegate} 之后立即调用此方法。在引擎实例的生命周期中，此方法只需要调用一次。

  FutureOr<int> setExtensionConfig(NSString groupId) async {
    return await nativeCall('setExtensionConfig:', [groupId]);
  }

  /// @detail api
  /// @author panjian.fishing
  /// @brief 设置运行时的参数
  /// @param parameters 保留参数
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 该接口需在 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 和 startAudioCapture{@link #ByteRTCEngine#startAudioCapture} 之前调用。

  FutureOr<int> setRuntimeParameters(NSDictionary parameters) async {
    return await nativeCall('setRuntimeParameters:', [parameters]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author liyi.000
  /// @brief 获取共享对象(应用窗口和桌面)列表。
  /// @return 共享对象(应用窗口和桌面)列表。参看 ByteRTCScreenCaptureSourceInfo{@link #ByteRTCScreenCaptureSourceInfo}。 <br>
  ///         枚举值可作为调用 startScreenVideoCapture:captureParameters:{@link #ByteRTCEngine#startScreenVideoCapture:captureParameters} 开启屏幕共享时的输入参数。
  /// @note 仅桌面端可用。

  FutureOr<ByteRTCScreenCaptureSourceInfo> getScreenCaptureSourceList() async {
    final result = await nativeCall('getScreenCaptureSourceList', []);
    return packObject(
        result,
        () => ByteRTCScreenCaptureSourceInfo(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @hidden(iOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liyi.000
  /// @brief 采集屏幕视频流，用于共享。屏幕视频流包括：屏幕上显示的内容，或应用窗口中显示的内容。
  /// @param sourceInfo 待共享的屏幕源，参看 ByteRTCScreenCaptureSourceInfo{@link #ByteRTCScreenCaptureSourceInfo}。 <br>
  ///                   你可以调用 getScreenCaptureSourceList{@link #ByteRTCEngine#getScreenCaptureSourceList} 获得所有可以共享的屏幕源。
  /// @param captureParameters 共享参数。参看 ByteRTCScreenCaptureParam{@link #ByteRTCScreenCaptureParam}。
  /// @return
  ///        - 0: 成功
  ///        - -1: 失败
  /// @note
  ///       - 调用本接口时，采集模式应为内部模式。在外部采集模式下调用无效，并将触发 rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning} 回调。
  ///       - 调用此方法仅开启屏幕流视频采集，不会发布采集到的视频。发布屏幕流视频需要调用 publishScreenVideo:{@link #ByteRTCRoom#publishScreenVideo}。
  ///       - 调用 stopScreenVideoCapture{@link #ByteRTCEngine#stopScreenVideoCapture} 关闭屏幕视频源采集。
  ///       - 本地用户通过 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 的回调获取屏幕采集状态，包括开始、暂停、恢复、错误等。
  ///       - 调用成功后，本端会收到 rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo:{@link #ByteRTCEngineDelegate#rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo} 回调。
  ///       - 调用此接口前，你可以调用 setScreenVideoEncoderConfig:{@link #ByteRTCEngine#setScreenVideoEncoderConfig} 设置屏幕视频流的采集帧率和编码分辨率。
  ///       - 在收到 rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo:{@link #ByteRTCEngineDelegate#rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo} 回调后，通过调用 setLocalVideoCanvas:withCanvas:{@link #ByteRTCEngine#setLocalVideoCanvas:withCanvas} 或 setLocalVideoSink:withSink:withPixelFormat:{@link #ByteRTCEngine#setLocalVideoSink:withSink:withPixelFormat} 函数设置本地屏幕共享视图。
  ///       - 再开启采集屏幕视频流后，你可以调用 updateScreenCaptureHighlightConfig:{@link #ByteRTCEngine#updateScreenCaptureHighlightConfig} 更新边框高亮设置，调用 updateScreenCaptureMouseCursor:{@link #ByteRTCEngine#updateScreenCaptureMouseCursor} 更新对鼠标的处理设置，PC 端还可以调用 updateScreenCaptureFilterConfig:{@link #ByteRTCEngine#updateScreenCaptureFilterConfig} 设置需要过滤的窗口。

  FutureOr<int> startScreenVideoCapture(
      ByteRTCScreenCaptureSourceInfo sourceInfo,
      ByteRTCScreenCaptureParam captureParameters) async {
    return await nativeCall('startScreenVideoCapture:captureParameters:',
        [sourceInfo, captureParameters]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liyi.000
  /// @brief 停止屏幕视频流采集。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用本接口时，采集模式应为内部模式。在外部采集模式下调用无效，并将触发 rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning} 回调。
  ///       - 要开启屏幕视频流采集，调用 startScreenVideoCapture:captureParameters:{@link #ByteRTCEngine#startScreenVideoCapture:captureParameters}。
  ///       - 调用后，本地用户会收到 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 的回调。
  ///       - 调用此接口不影响屏幕视频流发布。

  FutureOr<int> stopScreenVideoCapture() async {
    return await nativeCall('stopScreenVideoCapture', []);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liyi.000
  /// @brief 内部屏幕流采集时，更新采集区域。
  /// @param regionRect 采集区域相对 startScreenVideoCapture:captureParameters:{@link #ByteRTCEngine#startScreenVideoCapture:captureParameters} 中设定区域的值。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 调用此接口前，必须先通过调用 startScreenVideoCapture:captureParameters:{@link #ByteRTCEngine#startScreenVideoCapture:captureParameters} 开启了内部屏幕流采集。

  FutureOr<int> updateScreenCaptureRegion(CGRect regionRect) async {
    return await nativeCall('updateScreenCaptureRegion:', [regionRect]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liyi.000
  /// @brief 内部屏幕流采集时，更新边框高亮设置。默认展示边框。
  /// @param config 边框高亮设置。参见 ByteRTCHighlightConfig{@link #ByteRTCHighlightConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 调用此接口前，必须已通过调用 startScreenVideoCapture:captureParameters:{@link #ByteRTCEngine#startScreenVideoCapture:captureParameters} 开启了内部屏幕流采集。

  FutureOr<int> updateScreenCaptureHighlightConfig(
      ByteRTCHighlightConfig config) async {
    return await nativeCall('updateScreenCaptureHighlightConfig:', [config]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liyi.000
  /// @brief 内部屏幕流采集时，更新对鼠标的处理设置。默认采集鼠标。
  /// @param mouseCursorCaptureState 参看 ByteRTCMouseCursorCaptureState{@link #ByteRTCMouseCursorCaptureState}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 调用此接口前，必须已通过调用 startScreenVideoCapture:captureParameters:{@link #ByteRTCEngine#startScreenVideoCapture:captureParameters} 开启了内部屏幕流采集。

  FutureOr<int> updateScreenCaptureMouseCursor(
      ByteRTCMouseCursorCaptureState mouseCursorCaptureState) async {
    return await nativeCall(
        'updateScreenCaptureMouseCursor:', [mouseCursorCaptureState.$value]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liyi.000
  /// @brief 通过 RTC SDK 提供的采集模块采集屏幕视频流时，设置需要过滤的窗口。
  /// @param excludedWindowList 过滤掉的窗口列表。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用此接口前，必须已通过调用 startScreenVideoCapture:captureParameters:{@link #ByteRTCEngine#startScreenVideoCapture:captureParameters} 开启了内部屏幕流采集。
  ///       - 本函数在屏幕源类别是屏幕而非应用窗体时才起作用。详见：ByteRTCScreenCaptureSourceType{@link #ByteRTCScreenCaptureSourceType}。
  ///       - 调用本接口排除指定窗口时，共享视频的帧率无法达到 30fps。

  FutureOr<int> updateScreenCaptureFilterConfig(
      NSArray<NSNumber> excludedWindowList) async {
    return await nativeCall(
        'updateScreenCaptureFilterConfig:', [excludedWindowList]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author liyi.000
  /// @brief 获取屏幕采集对象缩略图
  /// @param sourceType 屏幕采集对象的类型。详见 ByteRTCScreenCaptureSourceType{@link #ByteRTCScreenCaptureSourceType}。
  /// @param sourceId 屏幕分享时，共享对象的 ID。可通过 getScreenCaptureSourceList{@link #ByteRTCEngine#getScreenCaptureSourceList} 返回的`ByteRTCScreenCaptureSourceInfo`共享对象列表中获取。
  /// @param maxWidth 最大宽度。保持采集对象本身的宽高比不变，将缩略图缩放到指定范围内的最大宽高。如果给出的尺寸与共享对象比例不同，得到的缩略图会有黑边。
  /// @param maxHeight 最大高度。参见 maxWidth 的说明。
  /// @return 屏幕采集对象缩略图。缩略图由屏幕共享对象等比缩放而来。缩略图的大小小于等于此接口设定的尺寸。

  FutureOr<ByteRTCImage> getThumbnail(ByteRTCScreenCaptureSourceType sourceType,
      intptr_t sourceId, int maxWidth, int maxHeight) async {
    return await nativeCall('getThumbnail:sourceId:maxWidth:maxHeight:',
        [sourceType.$value, sourceId, maxWidth, maxHeight]);
  }

  /// @hidden(iOS)
  /// @brief 获取应用窗体所属应用的图标。
  /// @region 屏幕共享
  /// @author liyi.000
  /// @param sourceId 屏幕共享对象的 ID，可通过 getScreenCaptureSourceList{@link #ByteRTCEngine#getScreenCaptureSourceList} 返回的`ByteRTCScreenCaptureSourceInfo`共享对象列表中获取。
  /// @param width 最大宽度。返回的图标将是宽高相等的，输入的宽高不等时，取二者较小值。宽高范围为 [32,256]，超出该范围将返回 `nullptr`，默认输出 100 x 100 的图像。
  /// @param height 最大高度。参见 `width` 的说明。
  /// @return 应用图标。当屏幕共享对象为应用窗体时有效，否则返回 `nullptr`。

  FutureOr<ByteRTCImage> getWindowAppIcon(
      intptr_t sourceId, int width, int height) async {
    return await nativeCall(
        'getWindowAppIcon:width:height:', [sourceId, width, height]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 使用 RTC SDK 内部采集模块开始采集屏幕音频流和（或）视频流。
  /// @param type 媒体类型，参看 ByteRTCScreenMediaType{@link #ByteRTCScreenMediaType}。
  /// @param bundleId 绑定 Extension 的 Bundle ID，绑定后应用中共享屏幕的选择列表中只展示你的 Extension 可供选择。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///      - 调用本接口时，采集模式应为内部模式。在外部采集模式下调用无效，并将触发 rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning} 或 rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning} 回调。
  ///      - 当从 iOS 控制中心发起屏幕采集时无需调用本方法。
  ///      - 采集后，你还需要调用 publishScreenVideo:{@link #ByteRTCRoom#publishScreenVideo} 和/或 publishScreenAudio:{@link #ByteRTCRoom#publishScreenAudio} 发布采集到的屏幕音视频。
  ///      - 开启屏幕音频/视频采集成功后，本地用户会收到 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 和 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 回调。

  FutureOr<int> startScreenCapture(
      ByteRTCScreenMediaType type, NSString bundleId) async {
    return await nativeCall(
        'startScreenCapture:bundleId:', [type.$value, bundleId]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 更新内部屏幕采集时采集的媒体类型。
  /// @param type 媒体类型，参看 ByteRTCScreenMediaType{@link #ByteRTCScreenMediaType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///      - 你需在开启屏幕视频流采集后调用该方法。
  ///      - 本地用户会收到 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 或 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 回调。

  FutureOr<int> updateScreenCapture(ByteRTCScreenMediaType type) async {
    return await nativeCall('updateScreenCapture:', [type.$value]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 在屏幕共享时，停止使用 RTC SDK 内部采集方式采集屏幕音视频。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///     - 调用本接口时，采集模式应为内部模式。在外部采集模式下调用无效，并将触发 rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning} 或 rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning} 回调。
  ///     - 当从 iOS 控制中心发起屏幕采集时无需调用本方法。
  ///     - 本方法只会停止本地屏幕采集，并不会影响屏幕流的发布状态。
  ///     - 本地用户会收到 rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error} 和 rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error} 的回调。

  FutureOr<int> stopScreenCapture() async {
    return await nativeCall('stopScreenCapture', []);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 向屏幕共享 Extension 发送自定义消息
  /// @param messsage 发送给 Extension 的消息内容
  /// @return
  ///        - 0: Success.
  ///        - < 0 : Fail. See ByteRTCReturnStatus{@link #ByteRTCReturnStatus} for more details
  /// @note
  ///       - 在 startScreenCapture:bundleId:{@link #ByteRTCEngine#startScreenCapture:bundleId} 后调用该方法。
  ///       - 通过 onReceiveMessageFromApp:{@link #ByteRtcScreenCapturerExtDelegate#onReceiveMessageFromApp} 回调发送的消息。

  FutureOr<int> sendScreenCaptureExtensionMessage(NSData messsage) async {
    return await nativeCall('sendScreenCaptureExtensionMessage:', [messsage]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author dixing
  /// @brief 创建音频设备管理实例
  /// @return ByteRTCAudioDeviceManager{@link #ByteRTCAudioDeviceManager}

  FutureOr<ByteRTCAudioDeviceManager> getAudioDeviceManager() async {
    final result = await nativeCall('getAudioDeviceManager', []);
    return packObject(
        result,
        () => ByteRTCAudioDeviceManager(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author zhangzhenyu.samuel
  /// @brief 创建视频设备管理实例
  /// @return 视频设备管理实例，详见 ByteRTCVideoDeviceManager{@link #ByteRTCVideoDeviceManager}

  FutureOr<ByteRTCVideoDeviceManager> getVideoDeviceManager() async {
    final result = await nativeCall('getVideoDeviceManager', []);
    return packObject(
        result,
        () => ByteRTCVideoDeviceManager(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 该方法将通话过程中的音视频数据录制到本地的文件中。
  /// @param recordingConfig 本地录制参数配置，参看 ByteRTCRecordingConfig{@link #ByteRTCRecordingConfig}
  /// @param recordingType 本地录制的媒体类型，参看 ByteRTCRecordingType{@link #ByteRTCRecordingType}
  /// @return
  ///        - 0: 正常
  ///        - -1: 参数设置异常
  ///        - -2: 当前版本 SDK 不支持该特性，请联系技术支持人员
  /// @note
  ///        - 该方法需在进房后调用。
  ///        - 调用该方法后，你会收到 rtcEngine:onRecordingStateUpdate:state:error_code:recording_info:{@link #ByteRTCEngineDelegate#rtcEngine:onRecordingStateUpdate:state:error_code:recording_info} 回调。
  ///        - 如果录制正常，系统每秒钟会通过 rtcEngine:onRecordingProgressUpdate:process:recording_info:{@link #ByteRTCEngineDelegate#rtcEngine:onRecordingProgressUpdate:process:recording_info} 回调通知录制进度。

  FutureOr<int> startFileRecording(ByteRTCRecordingConfig recordingConfig,
      ByteRTCRecordingType recordingType) async {
    return await nativeCall(
        'startFileRecording:type:', [recordingConfig, recordingType.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 停止本地录制
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用 startFileRecording:type:{@link #ByteRTCEngine#startFileRecording:type} 开启本地录制后，你必须调用该方法停止录制。
  ///        - 调用该方法后，你会收到 rtcEngine:onRecordingStateUpdate:state:error_code:recording_info:{@link #ByteRTCEngineDelegate#rtcEngine:onRecordingStateUpdate:state:error_code:recording_info} 回调提示录制结果。

  FutureOr<int> stopFileRecording() async {
    return await nativeCall('stopFileRecording', []);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 开启录制语音通话，生成本地文件。 <br>
  ///        在进房前后开启录制，如果未打开麦克风采集，录制任务正常进行，只是不会将数据写入生成的本地文件；只有调用 startAudioCapture{@link #ByteRTCEngine#startAudioCapture} 接口打开麦克风采集后，才会将录制数据写入本地文件。
  /// @param recordingConfig 参看 ByteRTCAudioRecordingConfig{@link #ByteRTCAudioRecordingConfig}
  /// @return
  ///        - 0: 正常
  ///        - -2: 参数设置异常
  ///        - -3: 当前版本 SDK 不支持该特性，请联系技术支持人员
  /// @note
  ///        - 录制包含各种音频效果。但不包含背景音乐。
  ///        - 调用 stopAudioRecording{@link #ByteRTCEngine#stopAudioRecording} 关闭录制。
  ///        - 加入房间前后均可调用。在进房前调用该方法，退房之后，录制任务不会自动停止，需调用 stopAudioRecording{@link #ByteRTCEngine#stopAudioRecording} 关闭录制。在进房后调用该方法，退房之后，录制任务会自动被停止。如果加入了多个房间，录制的文件中会包含各个房间的音频。
  ///        - 调用该方法后，你会收到 rtcEngine:onAudioRecordingStateUpdate:error_code:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioRecordingStateUpdate:error_code} 回调。

  FutureOr<int> startAudioRecording(
      ByteRTCAudioRecordingConfig recordingConfig) async {
    return await nativeCall('startAudioRecording:', [recordingConfig]);
  }

  /// @detail api
  /// @author huangshouqin
  /// @brief 停止音频文件录制
  /// @return
  ///         - 0: 正常
  ///         - -3: 当前版本 SDK 不支持该特性，请联系技术支持人员
  /// @note 调用 startAudioRecording:{@link #ByteRTCEngine#startAudioRecording} 开启本地录制。

  FutureOr<int> stopAudioRecording() async {
    return await nativeCall('stopAudioRecording', []);
  }

  /// @valid since 3.53
  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 创建音效播放器实例。
  /// @return 音效播放器。详见 ByteRTCAudioEffectPlayer{@link #ByteRTCAudioEffectPlayer}。
  ///

  FutureOr<ByteRTCAudioEffectPlayer> getAudioEffectPlayer() async {
    final result = await nativeCall('getAudioEffectPlayer', []);
    return packObject(
        result,
        () => ByteRTCAudioEffectPlayer(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @valid since 3.53
  /// @detail api
  /// @author zhangyuanyuan.0101
  /// @brief 创建音乐播放器实例。
  /// @param playerId 音乐播放器实例 id。取值范围为 `[0, 3]`。最多同时存在 4 个实例，超出取值范围时返回 nullptr。
  /// @return 音乐播放器实例，详见 ByteRTCMediaPlayer{@link #ByteRTCMediaPlayer}
  ///

  FutureOr<ByteRTCMediaPlayer> getMediaPlayer(int playerId) async {
    final result = await nativeCall('getMediaPlayer:', [playerId]);
    return packObject(
        result,
        () => ByteRTCMediaPlayer(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 登陆 RTS 服务器。 <br>
  ///        必须先登录，才能调用 sendUserMessageOutsideRoom:message:config:{@link #ByteRTCEngine#sendUserMessageOutsideRoom:message:config} 和 sendServerMessage:{@link #ByteRTCEngine#sendServerMessage} 发送房间外点对点消息和向应用服务器发送消息 <br>
  ///        在调用本接口登录后，如果想要登出，需要调用 logout{@link #ByteRTCEngine#logout}
  /// @param token 用户登录必须携带的 Token，用于鉴权验证。 <br>
  ///               测试时可使用[控制台](https://console.volcengine.com/rtc/listRTC)生成临时 Token，`roomId` 填任意值。 <br>
  ///               正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token，`roomId` 置空，Token 有效期及生成方式参看[使用 Token 完成鉴权](#70121)。
  /// @param uid 用户 ID，在 appid 的维度下是唯一的。
  /// @return
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note 本地用户调用此方法登录后，会收到 rtcEngine:onLoginResult:errorCode:elapsed:{@link #ByteRTCEngineDelegate#rtcEngine:onLoginResult:errorCode:elapsed} 回调通知登录结果，远端用户不会收到通知。

  FutureOr<int> login(NSString token, NSString uid) async {
    return await nativeCall('login:uid:', [token, uid]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 登出 RTS 服务器。 <br>
  ///        调用本接口登出后，无法调用房间外消息以及端到服务器消息相关的方法或收到相关回调。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 调用本接口登出前，必须先调用 login:uid:{@link #ByteRTCEngine#login:uid} 登录
  ///       - 本地用户调用此方法登出后，会收到 rtcEngine:onLogout:{@link #ByteRTCEngineDelegate#rtcEngine:onLogout}  回调通知结果，远端用户不会收到通知。

  FutureOr<int> logout() async {
    return await nativeCall('logout', []);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 更新用户用于登录的 Token <br>
  ///        Token 有一定的有效期，当 Token 过期时，需调用此方法更新登录的 Token 信息。 <br>
  ///        调用 login:uid:{@link #ByteRTCEngine#login:uid} 方法登录时，如果使用了过期的 Token 将导致登录失败，并会收到 rtcEngine:onLoginResult:errorCode:elapsed:{@link #ByteRTCEngineDelegate#rtcEngine:onLoginResult:errorCode:elapsed} 回调通知，错误码为 ByteRTCLoginErrorCodeInvalidToken。此时需要重新获取 Token，并调用此方法更新 Token。
  /// @param token <br>
  ///        更新的动态密钥
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 如果 Token 无效导致登录失败，则调用此方法更新 Token 后，SDK 会自动重新登录，而用户不需要自己调用 login:uid:{@link #ByteRTCEngine#login:uid} 方法。
  ///       - Token 过期时，如果已经成功登录，则不会受到影响。Token 过期的错误会在下一次使用过期 Token 登录时，或因本地网络状况不佳导致断网重新登录时通知给用户。

  FutureOr<int> updateLoginToken(NSString token) async {
    return await nativeCall('updateLoginToken:', [token]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 设置应用服务器参数 <br>
  ///        客户端调用 sendServerMessage:{@link #ByteRTCEngine#sendServerMessage} 或 sendServerBinaryMessage:{@link #ByteRTCEngine#sendServerBinaryMessage} 发送消息给应用服务器之前，必须需要设置有效签名和应用服务器地址。
  /// @param signature 动态签名，应用服务器可使用该签名验证消息来源。 <br>
  ///        签名需自行定义，可传入任意非空字符串，建议将 uid 等信息编码为签名。 <br>
  ///        设置的签名会以 post 形式发送至通过本方法中 url 参数设置的应用服务器地址。
  /// @param url 应用服务器的地址
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 用户必须调用 login:uid:{@link #ByteRTCEngine#login:uid} 登录后，才能调用本接口。
  ///       - 调用本接口后，SDK 会使用 rtcEngine:onServerParamsSetResult:{@link #ByteRTCEngineDelegate#rtcEngine:onServerParamsSetResult} 返回相应结果。

  FutureOr<int> setServerParams(NSString signature, NSString url) async {
    return await nativeCall('setServerParams:url:', [signature, url]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 查询对端用户或本端用户的登录状态
  /// @param peerUserId <br>
  ///        需要查询的用户 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 必须调用 login:uid:{@link #ByteRTCEngine#login:uid} 登录后，才能调用本接口。
  ///       - 调用本接口后，SDK 会使用 rtcEngine:onGetPeerOnlineStatus:status:{@link #ByteRTCEngineDelegate#rtcEngine:onGetPeerOnlineStatus:status} 回调通知查询结果。
  ///       - 在发送房间外消息之前，用户可以通过本接口了解对端用户是否登录，从而决定是否发送消息。也可以通过本接口查询自己查看自己的登录状态。

  FutureOr<int> getPeerOnlineStatus(NSString peerUserId) async {
    return await nativeCall('getPeerOnlineStatus:', [peerUserId]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 给房间外指定的用户发送文本消息（P2P）
  /// @param userId <br>
  ///        消息接收用户的 ID
  /// @param messageStr <br>
  ///        发送的文本消息内容 <br>
  ///        消息不超过 64 KB。
  /// @param config 消息类型，参看 ByteRTCMessageConfig{@link #ByteRTCMessageConfig}。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  /// @note
  ///       - 在发送房间外文本消息前，必须先调用 login:uid:{@link #ByteRTCEngine#login:uid} 完成登录。
  ///       - 用户调用本接口发送文本信息后，会收到一次 rtcEngine:onUserMessageSendResultOutsideRoom:error:{@link #ByteRTCEngineDelegate#rtcEngine:onUserMessageSendResultOutsideRoom:error} 回调，得知消息是否成功发送；
  ///       - 若文本消息发送成功，则 userId 所指定的用户会通过 rtcEngine:onUserMessageReceivedOutsideRoom:message:{@link #ByteRTCEngineDelegate#rtcEngine:onUserMessageReceivedOutsideRoom:message} 回调收到该消息。

  FutureOr<NSInteger> sendUserMessageOutsideRoom(
      NSString userId, NSString messageStr, ByteRTCMessageConfig config) async {
    return await nativeCall('sendUserMessageOutsideRoom:message:config:',
        [userId, messageStr, config.$value]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 给房间外指定的用户发送二进制消息（P2P）
  /// @param userId <br>
  ///        消息接收用户的 ID
  /// @param messageStr <br>
  ///        发送的二进制消息内容 <br>
  ///        消息不超过 64KB。
  /// @param config 消息类型，参看 ByteRTCMessageConfig{@link #ByteRTCMessageConfig}。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  ///        - -1：发送失败。消息为空。
  /// @note
  ///       - 在发送房间外二进制消息前，必须先调用 login:uid:{@link #ByteRTCEngine#login:uid} 完成登录。
  ///       - 用户调用本接口发送二进制消息后，会收到一次 rtcEngine:onUserMessageSendResultOutsideRoom:error:{@link #ByteRTCEngineDelegate#rtcEngine:onUserMessageSendResultOutsideRoom:error} 回调，通知消息是否发送成功；
  ///       - 若二进制消息发送成功，则 userId 所指定的用户会通过 rtcEngine:onUserBinaryMessageReceivedOutsideRoom:message:{@link #ByteRTCEngineDelegate#rtcEngine:onUserBinaryMessageReceivedOutsideRoom:message}  回调收到该条消息。

  FutureOr<NSInteger> sendUserBinaryMessageOutsideRoom(
      NSString userId, NSData messageStr, ByteRTCMessageConfig config) async {
    return await nativeCall('sendUserBinaryMessageOutsideRoom:message:config:',
        [userId, messageStr, config.$value]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 客户端给应用服务器发送文本消息（P2Server）
  /// @param messageStr <br>
  ///        发送的文本消息内容 <br>
  ///        消息不超过 64 KB。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  /// @note
  ///       - 在向应用服务器发送文本消息前，必须先调用 login:uid:{@link #ByteRTCEngine#login:uid} 完成登录，随后调用 setServerParams:url:{@link #ByteRTCEngine#setServerParams:url} 设置应用服务器。
  ///       - 调用本接口后，会收到一次 rtcEngine:onServerMessageSendResult:error:message:{@link #ByteRTCEngineDelegate#rtcEngine:onServerMessageSendResult:error:message} 回调，通知消息发送方是否发送成功。
  ///       - 若文本消息发送成功，则之前调用 setServerParams:url:{@link #ByteRTCEngine#setServerParams:url} 设置的应用服务器会收到该条消息。

  FutureOr<NSInteger> sendServerMessage(NSString messageStr) async {
    return await nativeCall('sendServerMessage:', [messageStr]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 客户端给应用服务器发送二进制消息（P2Server）
  /// @param messageStr <br>
  ///        发送的二进制消息内容 <br>
  ///        消息不超过 64KB。
  /// @return
  ///        - >0：发送成功，返回这次发送消息的编号，从 1 开始递增。
  ///        - -1：发送失败。消息为空。
  /// @note
  ///       - 在向应用服务器发送二进制消息前，先调用 login:uid:{@link #ByteRTCEngine#login:uid} 完成登录，随后调用 setServerParams:url:{@link #ByteRTCEngine#setServerParams:url} 设置应用服务器。
  ///       - 调用本接口后，会收到一次 rtcEngine:onServerMessageSendResult:error:message:{@link #ByteRTCEngineDelegate#rtcEngine:onServerMessageSendResult:error:message} 回调，通知消息发送方发送成功或失败；
  ///       - 若二进制消息发送成功，则之前调用 setServerParams:url:{@link #ByteRTCEngine#setServerParams:url} 设置的应用服务器会收到该条消息。

  FutureOr<NSInteger> sendServerBinaryMessage(NSData messageStr) async {
    return await nativeCall('sendServerBinaryMessage:', [messageStr]);
  }

  /// @detail api
  /// @author hanchenchen.c
  /// @brief 开始通话前网络探测
  /// @param isTestUplink 是否探测上行带宽
  /// @param expectedUplinkBitrate 期望上行带宽，单位：kbps<br>范围为 {0, [100-10000]}，其中， `0` 表示由 SDK 指定最高码率。
  /// @param isTestDownlink 是否探测下行带宽
  /// @param expectedDownlinkBitrate 期望下行带宽，单位：kbps<br>范围为 {0, [100-10000]}，其中， `0` 表示由 SDK 指定最高码率。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 成功调用本接口后，会在 3s 内收到一次 rtcEngine:onNetworkDetectionResult:quality:rtt:lostRate:bitrate:jitter:{@link #ByteRTCEngineDelegate#rtcEngine:onNetworkDetectionResult:quality:rtt:lostRate:bitrate:jitter} 回调，此后每 2s 会收到一次该回调，通知探测结果；
  ///       - 若探测停止，则会收到一次 rtcEngine:onNetworkDetectionStopped:{@link #ByteRTCEngineDelegate#rtcEngine:onNetworkDetectionStopped} 通知探测停止。

  FutureOr<int> startNetworkDetection(
      bool isTestUplink,
      int expectedUplinkBitrate,
      bool isTestDownlink,
      int expectedDownlinkBitrate) async {
    return await nativeCall(
        'startNetworkDetection:uplinkBandwidth:downlink:downlinkBandwidth:', [
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
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       调用本接口后，会收到一次 rtcEngine:onNetworkDetectionStopped:{@link #ByteRTCEngineDelegate#rtcEngine:onNetworkDetectionStopped} 通知探测停止。

  FutureOr<int> stopNetworkDetection() async {
    return await nativeCall('stopNetworkDetection', []);
  }

  /// @detail api
  /// @author liyi.000
  /// @brief 在屏幕共享时，设置屏幕音频的采集方式（内部采集/自定义采集）
  /// @param sourceType 屏幕音频输入源类型, 参看 ByteRTCAudioSourceType{@link #ByteRTCAudioSourceType}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///      - 默认采集方式是 RTC SDK 内部采集。
  ///      - 如果设定为内部采集，你必须重新开始采集。
  ///      - 如果设定为自定义采集，你必须再调用 pushScreenAudioFrame:{@link #ByteRTCEngine#pushScreenAudioFrame} 将自定义采集到的屏幕音频帧推送到 RTC SDK。
  ///      - 无论是内部采集还是自定义采集，你都必须调用 publishScreenAudio: 将采集到的屏幕音频发布给远端。
  /// @order 6

  FutureOr<int> setScreenAudioSourceType(
      ByteRTCAudioSourceType sourceType) async {
    return await nativeCall('setScreenAudioSourceType:', [sourceType.$value]);
  }

  /// @detail api
  /// @author liyi.000
  /// @brief 使用自定义采集方式，采集屏幕共享时的屏幕音频时，将音频帧推送至 RTC SDK 处进行编码等处理。
  /// @param audioFrame 音频数据帧，参见 ByteRTCAudioFrame{@link #ByteRTCAudioFrame} <br>
  ///                   - 音频采样格式为 S16。音频缓冲区内的数据格式必须为 PCM 数据，其容量大小应该为 samples × frame.channel × 2。
  ///                   - 必须指定具体的采样率和声道数，不支持设置为自动。
  /// @return 方法调用结果 <br>
  ///        - 0: 设置成功。
  ///        - < 0: 设置失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明。
  /// @note
  ///        - 调用此接口推送屏幕共享时的自定义采集的音频数据前，必须调用 setScreenAudioSourceType:{@link #ByteRTCEngine#setScreenAudioSourceType} 开启屏幕音频自定义采集。
  ///        - 你应每隔 10 毫秒，调用一次此方法推送一次自定义采集的音频帧。一次推送的音频帧中应包含 frame.sample_rate / 100 个音频采样点。比如，假如采样率为 48000Hz，则每次应该推送 480 个采样点。
  ///        - 调用此接口将自定义采集的音频帧推送到 RTC SDK 后，你必须调用 publishScreenAudio: 将采集到的屏幕音频推送到远端。在调用 publishScreenAudio: 前，推送到 RTC SDK 的音频帧信息会丢失。
  /// @order 9

  FutureOr<int> pushScreenAudioFrame(ByteRTCAudioFrame audioFrame) async {
    return await nativeCall('pushScreenAudioFrame:', [audioFrame]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author yezijian.me
  /// @brief 在屏幕共享时，开始使用 RTC SDK 内部采集方式，采集屏幕音频
  /// @param deviceId 虚拟设备 ID
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 本接口仅对内部采集生效，RTC SDK 默认使用内部采集模块采集屏幕音频。若已调用 setScreenAudioSourceType:{@link #ByteRTCEngine#setScreenAudioSourceType} 将音频输入源设置为 `ByteRTCAudioSourceTypeExternal` 自定义采集，需先切换为 `ByteRTCAudioSourceTypeInternal` 内部采集，否则该接口调用无效，并将触发 rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning} 回调。
  ///        - 采集后，你还需要调用 publishScreenAudio: 将采集到的屏幕音频推送到远端。
  ///        - 要关闭屏幕音频内部采集，调用 stopScreenAudioCapture{@link #ByteRTCEngine#stopScreenAudioCapture}。

  FutureOr<int> startScreenAudioCapture(NSString deviceId) async {
    return await nativeCall('startScreenAudioCapture:', [deviceId]);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author liyi.000
  /// @brief 在屏幕共享时，停止使用 RTC SDK 内部采集方式，采集屏幕音频。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用本接口时，采集模式应为内部模式。在外部采集模式下调用无效，并将触发 rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning} 回调。
  ///        - 要开始屏幕音频内部采集，调用 startScreenAudioCapture:{@link #ByteRTCEngine#startScreenAudioCapture}。

  FutureOr<int> stopScreenAudioCapture() async {
    return await nativeCall('stopScreenAudioCapture', []);
  }

  /// @hidden(iOS)
  /// @detail api
  /// @author zhangcaining
  /// @brief 在屏幕共享时，设置屏幕音频流的声道数
  /// @param channel 声道数，参看 ByteRTCAudioChannel{@link #ByteRTCAudioChannel}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 当你调用 setScreenAudioStreamIndex: 并设置屏幕音频流和麦克风音频流混流时，此接口不生效，音频通道数由 setAudioProfile:{@link #ByteRTCEngine#setAudioProfile} 控制。

  FutureOr<int> setScreenAudioChannel(ByteRTCAudioChannel channel) async {
    return await nativeCall('setScreenAudioChannel:', [channel.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 设置向 SDK 输入的视频源，包括屏幕流 <br>
  ///        默认使用内部采集。内部采集指：使用 RTC SDK 内置的视频采集机制进行视频采集。
  /// @param type 视频输入源类型，参看 ByteRTCVideoSourceType{@link #ByteRTCVideoSourceType}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法进房前后均可调用。
  ///        - 当你已调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 开启内部采集后，再调用此方法切换至自定义采集时，SDK 会自动关闭内部采集。
  ///        - 当你调用此方法开启自定义采集后，想要切换至内部采集，你必须先调用此方法关闭自定义采集，然后调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 手动开启内部采集。
  ///        - 当你需要向 SDK 推送自定义编码后的视频帧，你需调用该方法将视频源切换至自定义编码视频源。

  FutureOr<int> setVideoSourceType(ByteRTCVideoSourceType type) async {
    return await nativeCall('setVideoSourceType:', [type.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 注册自定义编码帧推送事件回调
  /// @param handler 自定义编码帧回调类，参看 ByteRTCExternalVideoEncoderEventHandler{@link #ByteRTCExternalVideoEncoderEventHandler}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 该方法需在进房前调用。
  ///       - 引擎销毁前需取消注册，调用该方法将参数设置为 nullptr 即可。

  FutureOr<int> setExternalVideoEncoderEventHandler(
      id<ByteRTCExternalVideoEncoderEventHandler> handler) async {
    return await nativeCall('setExternalVideoEncoderEventHandler:', [handler]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 推送自定义编码后的视频流
  /// @param videoIndex 对应的编码流下标，从 0 开始，如果调用 setVideoEncoderConfig:{@link #ByteRTCEngine#setVideoEncoderConfig} 设置了多路流，此处数量须与之保持一致
  /// @param videoFrame 编码流视频帧信息，参看 ByteRTCEncodedVideoFrame{@link #ByteRTCEncodedVideoFrame}。
  /// @return 方法调用结果： <br>
  ///        - 0：成功；
  ///        - <0：失败。具体失败原因参看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus}。
  /// @note
  ///        - 目前仅支持推送 H264 和 ByteVC1 格式的视频帧，且视频流协议格式须为 Annex B 格式。
  ///        - 该函数运行在用户调用线程内
  ///        - 推送自定义编码视频帧前，必须调用 setVideoSourceType:{@link #ByteRTCEngine#setVideoSourceType} 将视频输入源切换至自定义编码视频源。

  FutureOr<int> pushExternalEncodedVideoFrame(
      NSInteger videoIndex, ByteRTCEncodedVideoFrame videoFrame) async {
    return await nativeCall(
        'pushExternalEncodedVideoFrame:withEncodedVideoFrame:',
        [videoIndex, videoFrame]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 在订阅远端视频流之前，设置远端视频数据解码方式
  /// @param streamId 远端流 ID。
  /// @param config 视频解码方式，参看 ByteRTCVideoDecoderConfig{@link #ByteRTCVideoDecoderConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 当你想要对远端流进行自定义解码时，你需要先调用 registerRemoteEncodedVideoFrameObserver:{@link #ByteRTCEngine#registerRemoteEncodedVideoFrameObserver} 注册远端视频流监测器，然后再调用该接口将解码方式设置为自定义解码。监测到的视频数据会通过 onRemoteEncodedVideoFrame:info:withEncodedVideoFrame:{@link #ByteRTCRemoteEncodedVideoFrameObserver#onRemoteEncodedVideoFrame:info:withEncodedVideoFrame} 回调出来。
  ///        - 自 3.56 起，要用于自动订阅场景下，你可以设置 `streamId` 为特定值（若有对应逻辑），此时，通过此接口设置的解码方式根据 `streamId` 的相关逻辑，适用于所有的远端主流或屏幕流的解码方式。

  FutureOr<int> setVideoDecoderConfig(
      NSString streamId, ByteRTCVideoDecoderConfig config) async {
    return await nativeCall('setVideoDecoderConfig:withVideoDecoderConfig:',
        [streamId, config.$value]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 在订阅远端视频流之后，向远端请求关键帧
  /// @param streamId 远端流 ID。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 该方法仅适用于手动订阅模式，并且在成功订阅远端流之后使用。
  ///        - 该方法适用于调用 setVideoDecoderConfig:withVideoDecoderConfig:{@link #ByteRTCEngine#setVideoDecoderConfig:withVideoDecoderConfig} 开启自定义解码功能后，并且自定义解码失败的情况下使用

  FutureOr<int> requestRemoteVideoKeyFrame(NSString streamId) async {
    return await nativeCall('requestRemoteVideoKeyFrame:', [streamId]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangzhanqiang
  /// @brief 注册远端编码后视频数据回调。 <br>
  ///        完成注册后，当 SDK 监测到远端编码后视频帧时，会触发 onRemoteEncodedVideoFrame:info:withEncodedVideoFrame:{@link #ByteRTCRemoteEncodedVideoFrameObserver#onRemoteEncodedVideoFrame:info:withEncodedVideoFrame} 回调
  /// @param observer 远端编码后视频数据监测器，参看 ByteRTCRemoteEncodedVideoFrameObserver{@link #ByteRTCRemoteEncodedVideoFrameObserver}
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///       - 更多自定义解码功能说明参看 [自定义视频编解码](https://www.volcengine.com/docs/6348/82921#\%E8\%87\%AA\%E5\%AE\%9A\%E4\%B9\%89\%E8\%A7\%86\%E9\%A2\%91\%E8\%A7\%A3\%E7\%A0\%81)。
  ///       - 该方法适用于手动订阅，并且进房前后均可调用，建议在进房前调用。
  ///       - 引擎销毁前需取消注册，调用该方法将参数设置为 nullptr 即可。

  FutureOr<int> registerRemoteEncodedVideoFrameObserver(
      id<ByteRTCRemoteEncodedVideoFrameObserver> observer) async {
    return await nativeCall(
        'registerRemoteEncodedVideoFrameObserver:', [observer]);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 发送音频流同步信息。将消息通过音频流发送到远端，并实现与音频流同步，该接口调用成功后，远端用户会收到 rtcEngine:onStreamSyncInfoReceived:info:streamType:data:{@link #ByteRTCEngineDelegate#rtcEngine:onStreamSyncInfoReceived:info:streamType:data} 回调。
  /// @param data 消息内容。
  /// @param config 媒体流信息同步的相关配置，详见 ByteRTCStreamSyncInfoConfig{@link #ByteRTCStreamSyncInfoConfig} 。
  /// @return
  ///        - >=0: 消息发送成功。返回成功发送的次数。
  ///        - -1: 消息发送失败。消息长度大于 255 字节。
  ///        - -2: 消息发送失败。传入的消息内容为空。
  ///        - -3: 消息发送失败。通过屏幕流进行消息同步时，此屏幕流还未发布。
  ///        - -4: 消息发送失败。通过用麦克风或自定义设备采集到的音频流进行消息同步时，此音频流还未发布，详见错误码 ByteRTCErrorCode{@link #ByteRTCErrorCode}。
  /// @note
  /// - 调用本接口的频率建议不超过 50 次每秒。
  /// - 在 `ByteRTCRoomProfileInteractivePodcast` 房间模式下，此消息一定会送达。在其他房间模式下，如果本地用户未说话，此消息不一定会送达。

  FutureOr<int> sendStreamSyncInfo(
      NSData data, ByteRTCStreamSyncInfoConfig config) async {
    return await nativeCall('sendStreamSyncInfo:config:', [data, config]);
  }

  /// @detail api
  /// @author qipengxiang
  /// @brief 开启音视频回路测试。 <br>
  ///        在进房前，用户可调用该接口对音视频通话全链路进行检测，包括对音视频设备以及用户上下行网络的检测，从而帮助用户判断是否可以正常发布和接收音视频流。 <br>
  ///        开始检测后，SDK 会录制你声音或视频。如果你在设置的延时范围内收到了回放，则视为音视频回路测试正常。
  /// @param echoConfig 回路测试参数设置，参看 ByteRTCEchoTestConfig{@link #ByteRTCEchoTestConfig}。
  /// @param delayTime 音视频延迟播放的时间间隔，用于指定在开始检测多长时间后期望收到回放。取值范围为 [2,10]，单位为秒，默认为 2 秒。
  /// @return 方法调用结果： <br>
  ///        - 0：成功
  ///        - -2：失败，参数异常
  ///        - -4：失败，用户已进房
  ///        - -6：失败，当前用户已经在检测中
  ///        - -7：失败，音视频均不检查
  ///        - -8：失败，已经存在相同 roomId 的房间
  /// @note
  ///        - 调用该方法开始音视频回路检测后，你可以调用 stopEchoTest{@link #ByteRTCEngine#stopEchoTest} 立即结束测试，也可等待测试 60s 后自动结束，以更换设备进行下一次测试，或进房。
  ///        - 在该方法之前调用的所有跟设备控制、流控制相关的方法均在开始检测时失效，在结束检测后恢复生效。
  ///        - 在调用 startEchoTest:playDelay:{@link #ByteRTCEngine#startEchoTest:playDelay} 和 stopEchoTest{@link #ByteRTCEngine#stopEchoTest} 之间调用的所有跟设备采集、流控制、进房相关的方法均不生效，并会收到 rtcEngine:onWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onWarning} 回调，提示警告码为 `ByteRTCWarningCodeInEchoTestMode`。
  ///        - 音视频回路检测的结果会通过 rtcEngine:onEchoTestResult:{@link #ByteRTCEngineDelegate#rtcEngine:onEchoTestResult} 回调通知。

  FutureOr<int> startEchoTest(
      ByteRTCEchoTestConfig echoConfig, NSInteger delayTime) async {
    return await nativeCall(
        'startEchoTest:playDelay:', [echoConfig, delayTime]);
  }

  /// @detail api
  /// @author qipengxiang
  /// @brief 停止音视频回路测试。 <br>
  ///        调用 startEchoTest:playDelay:{@link #ByteRTCEngine#startEchoTest:playDelay} 开启音视频回路检测后，你必须调用该方法停止检测。
  /// @return 方法调用结果： <br>
  ///        - 0：成功
  ///        - -3：失败，未开启回路检测。
  /// @note 音视频回路检测结束后，所有对系统设备及音视频流的控制均会恢复到开始检测前的状态。

  FutureOr<int> stopEchoTest() async {
    return await nativeCall('stopEchoTest', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhushufan.ref
  /// @brief 在指定视频流上添加水印。
  /// @param imagePath 水印图片路径，仅支持本地文件绝对路径，长度限制为 512 字节。 <br>
  ///        水印图片为 PNG 或 JPG 格式。
  /// @param rtcWatermarkConfig 水印参数，参看 ByteRTCVideoWatermarkConfig{@link #ByteRTCVideoWatermarkConfig}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 调用 clearVideoWatermark{@link #ByteRTCEngine#clearVideoWatermark} 移除指定视频流的水印。
  ///        - 同一视频流只能设置一个水印，新设置的水印会替换上一次的设置。你可以多次调用本方法来设置不同视频流的水印。
  ///        - 进入房间前后均可调用此方法。
  ///        - 若开启本地预览镜像，或开启本地预览和编码传输镜像，则远端水印均不镜像；在开启本地预览水印时，本端水印会镜像。
  ///        - 开启大小流后，水印对大小流均生效，且针对小流进行等比例缩小。

  FutureOr<int> setVideoWatermark(NSString imagePath,
      ByteRTCVideoWatermarkConfig rtcWatermarkConfig) async {
    return await nativeCall('setVideoWatermark:withRtcWatermarkConfig:',
        [imagePath, rtcWatermarkConfig]);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author zhushufan.ref
  /// @brief 移除指定视频流的水印。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明

  FutureOr<int> clearVideoWatermark() async {
    return await nativeCall('clearVideoWatermark', []);
  }

  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfujun.911
  /// @brief 截取本地视频画面
  /// @param callback 本地截图的回调。参看 ByteRTCVideoSnapshotCallbackDelegate{@link #ByteRTCVideoSnapshotCallbackDelegate}。
  /// @return 本地截图任务的编号，从 `1` 开始递增。
  /// @note
  ///        - 对截取的画面，包含本地视频处理的全部效果，包含旋转，镜像，美颜等。
  ///        - 不管采用 SDK 内部采集，还是自定义采集，都可以进行截图。

  FutureOr<NSInteger> takeLocalSnapshot(
      id<ByteRTCVideoSnapshotCallbackDelegate> callback) async {
    return await nativeCall('takeLocalSnapshot:', [callback]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @author wangfujun.911
  /// @brief 截取本地的视频流画面，生成 JPG 文件，并保存到本地指定路径。 <br>
  ///        调用该方法后，SDK 会触发回调 rtcEngine:onLocalSnapshotTakenToFile:filePath:width:height:errorCode:taskId:{@link #ByteRTCEngineDelegate#rtcEngine:onLocalSnapshotTakenToFile:filePath:width:height:errorCode:taskId} 报告截图是否成功，以及截取的图片信息。
  /// @param filePath 截图的本地保存路径（绝对路径），需精确到文件名及格式，文件扩展名必须为 `.jpg`，并请确保路径存在且可写。示例：`/Users/YourName/Pictures/snapshot.jpg`。
  /// @return 本地截图任务的编号，从 `1` 开始递增。此编号可用于追踪任务状态或进行其他管理操作。

  FutureOr<NSInteger> takeLocalSnapshotToFile(NSString filePath) async {
    return await nativeCall('takeLocalSnapshotToFile:', [filePath]);
  }

  /// @detail api
  /// @author wangfujun.911
  /// @brief 截取远端视频画面，并保存到本地指定路径。
  /// @param streamId 截图的视频流对应的 ID。
  /// @param callback 参看 ByteRTCVideoSnapshotCallbackDelegate{@link #ByteRTCVideoSnapshotCallbackDelegate}。
  /// @return 远端截图任务的编号，从 `1` 开始递增。

  FutureOr<NSInteger> takeRemoteSnapshot(NSString streamId,
      id<ByteRTCVideoSnapshotCallbackDelegate> callback) async {
    return await nativeCall(
        'takeRemoteSnapshot:callback:', [streamId, callback]);
  }

  /// @detail api
  /// @valid since 3.60.
  /// @author wangfujun.911
  /// @brief 截取远端的视频流画面，生成 JPG 文件，并保存到本地指定路径。 <br>
  ///        调用该方法后，SDK 会触发回调 rtcEngine:onRemoteSnapshotTakenToFile:info:filePath:width:height:errorCode:taskId:{@link #ByteRTCEngineDelegate#rtcEngine:onRemoteSnapshotTakenToFile:info:filePath:width:height:errorCode:taskId} 报告截图是否成功，以及截取的图片信息。
  /// @param streamId 待截取的远端视频流 ID。
  /// @param filePath 截图的本地保存路径（绝对路径），需精确到文件名及格式，文件扩展名必须为 `.jpg`，并请确保路径存在且可写。示例：`/Users/YourName/Pictures/snapshot.jpg`。
  /// @return 远端截图任务的编号，从 `1` 开始递增。此编号可用于追踪任务状态或进行其他管理操作。

  FutureOr<NSInteger> takeRemoteSnapshotToFile(
      NSString streamId, NSString filePath) async {
    return await nativeCall(
        'takeRemoteSnapshotToFile:filePath:', [streamId, filePath]);
  }

  /// @detail api
  /// @author daining.nemo
  /// @brief 开启云代理
  /// @param cloudProxiesInfo 云代理服务器信息列表。参看 ByteRTCCloudProxyInfo{@link #ByteRTCCloudProxyInfo}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 在加入房间前调用此接口
  ///        - 在开启云代理后，进行通话前网络探测
  ///        - 开启云代理后，并成功链接云代理服务器后，会收到 rtcEngine:onCloudProxyConnected:{@link #ByteRTCEngineDelegate#rtcEngine:onCloudProxyConnected}。
  ///        - 要关闭云代理，调用 stopCloudProxy{@link #ByteRTCEngine#stopCloudProxy}。

  FutureOr<int> startCloudProxy(
      NSArray<ByteRTCCloudProxyInfo> cloudProxiesInfo) async {
    return await nativeCall('startCloudProxy:', [cloudProxiesInfo]);
  }

  /// @detail api
  /// @author daining.nemo
  /// @brief 关闭云代理
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note 要开启云代理，调用 startCloudProxy:{@link #ByteRTCEngine#startCloudProxy}

  FutureOr<int> stopCloudProxy() async {
    return await nativeCall('stopCloudProxy', []);
  }

  /// @detail api
  /// @author wangjunzheng
  /// @brief 创建 K 歌评分管理接口。
  /// @return K 歌评分管理接口,详见 ByteRTCSingScoringManager{@link #ByteRTCSingScoringManager}。
  /// @note 如需使用 K 歌评分功能，即调用该方法以及 `ByteRTCSingScoringManager` 类下全部方法，需集成 SAMI 动态库，详情参看[按需集成插件](#1108726)文档。

  FutureOr<ByteRTCSingScoringManager> getSingScoringManager() async {
    final result = await nativeCall('getSingScoringManager', []);
    return packObject(
        result,
        () => ByteRTCSingScoringManager(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author liuyangyang
  /// @brief 摄像头处于关闭状态时，使用静态图片填充本地推送的视频流。 <br>
  ///        调用 `stopVideoCapture` 接口时，会开始推静态图片。若要停止发送图片，可传入空字符串或启用内部摄像头采集。 <br>
  ///        可重复调用该接口来更新图片。
  /// @param filePath 设置静态图片的路径。 <br>
  ///        支持本地文件绝对路径，不支持网络链接，长度限制为 512 字节。 <br>
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

  FutureOr<int> setDummyCaptureImagePath(NSString filePath) async {
    return await nativeCall('setDummyCaptureImagePath:', [filePath]);
  }

  /// @detail api
  /// @author songxiaomeng.19
  /// @brief 通过 NTP 协议，获取网络时间。
  /// @return 网络时间。参看 ByteRTCNetworkTimeInfo{@link #ByteRTCNetworkTimeInfo}。
  /// @note
  ///        - 第一次调用此接口会启动网络时间同步功能，并返回 `0`。同步完成后，会收到 rtcEngineOnNetworkTimeSynchronized:{@link #ByteRTCEngineDelegate#rtcEngineOnNetworkTimeSynchronized}，此后，再次调用此 API，即可获取准确的网络时间。
  ///        - 在合唱场景下，合唱参与者应在相同的网络时间播放背景音乐。

  FutureOr<ByteRTCNetworkTimeInfo> getNetworkTimeInfo() async {
    final result = await nativeCall('getNetworkTimeInfo', []);
    return packObject(
        result,
        () => ByteRTCNetworkTimeInfo(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @hidden internal use only
  /// @author majun.lvhiei
  /// @brief 在听众端，设置订阅的所有远端音频流精准对齐后播放。
  /// @param streamId 作为对齐基准的远端音频流对应的id。 <br>
  ///                  一般选择主唱的音频流。 <br>
  ///                  你必须在收到 rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish}，确认此音频流已发布后，调用此 API。
  /// @param mode 是否对齐，默认不对齐。参看 ByteRTCAudioAlignmentMode{@link #ByteRTCAudioAlignmentMode}。
  /// @return
  ///        - 0: 调用成功。
  ///        - < 0 : 调用失败。查看 ByteRTCReturnStatus{@link #ByteRTCReturnStatus} 获得更多错误说明
  /// @note
  ///        - 你必须在实时合唱场景下使用此功能。在加入房间时，所有人应设置 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 为 `ByteRTCRoomProfileChorus`。
  ///        - 订阅的所有远端流必须通过 startAudioMixing:filePath:config: 开启了背景音乐混音，并将 ByteRTCAudioMixingConfig 中的 `syncProgressToRecordFrame` 设置为 `true`。
  ///        - 如果订阅的某个音频流延迟过大，可能无法实现精准对齐。
  ///        - 合唱的参与者不应调用此 API，因为调用此 API 会增加延迟。如果希望从听众变为合唱参与者，应关闭对齐功能。

  FutureOr<int> setAudioAlignmentProperty(
      NSString streamId, ByteRTCAudioAlignmentMode mode) async {
    return await nativeCall(
        'setAudioAlignmentProperty:withMode:', [streamId, mode.$value]);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 创建 KTV 管理接口。
  /// @return KTV 管理接口，参看 ByteRTCKTVManager{@link #ByteRTCKTVManager}。

  FutureOr<ByteRTCKTVManager> getKTVManager() async {
    final result = await nativeCall('getKTVManager', []);
    return packObject(
        result,
        () =>
            ByteRTCKTVManager(const NativeClassOptions([], disableInit: true)));
  }

  /// @detail api
  /// @author zhangcaining
  /// @brief 开启通话前回声检测
  /// @param testAudioFilePath 用于回声检测的音频文件的绝对路径，路径字符串使用 UTF-8 编码格式，支持以下音频格式: mp3，aac，m4a，3gp，wav。 <br>
  ///         音频文件不为静音文件，推荐时长为 10 ～ 20 秒。
  /// @return 方法调用结果： <br>
  ///        - 0: 成功。
  ///        - -1：失败。上一次检测未结束，请先调用 stopHardwareEchoDetection{@link #ByteRTCEngine#stopHardwareEchoDetection} 停止检测 后重新调用本接口。
  ///        - -2：失败。路径不合法或音频文件格式不支持。
  /// @note
  ///        - 只有当 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 为 `ByteRTCRoomProfileMeeting` 和 `ByteRTCRoomProfileMeetingRoom` 时支持开启本功能。
  ///        - 开启检测前，你需要向用户获取音频设备的使用权限。
  ///        - 开启检测前，请确保音频设备没有被静音，采集和播放音量正常。
  ///        - 调用本接口后监听 rtcEngine:onHardwareEchoDetectionResult:{@link #ByteRTCEngineDelegate#rtcEngine:onHardwareEchoDetectionResult} 获取检测结果。
  ///        - 检测期间，进程将独占音频设备，无法使用其他音频设备测试接口： startEchoTest:playDelay:{@link #ByteRTCEngine#startEchoTest:playDelay}、startAudioDeviceRecordTest:{@link #ByteRTCAudioDeviceManager#startAudioDeviceRecordTest} 或 startAudioPlaybackDeviceTest:interval:{@link #ByteRTCAudioDeviceManager#startAudioPlaybackDeviceTest:interval}。
  ///        - 调用 stopHardwareEchoDetection{@link #ByteRTCEngine#stopHardwareEchoDetection} 停止检测，释放对音频设备的占用。

  FutureOr<int> startHardwareEchoDetection(NSString testAudioFilePath) async {
    return await nativeCall('startHardwareEchoDetection:', [testAudioFilePath]);
  }

  /// @detail api
  /// @author zhangcaining
  /// @brief 停止通话前回声检测
  /// @return 方法调用结果： <br>
  ///        - 0: 成功。
  ///        - -1：失败。
  /// @note
  ///       - 关于开启通话前回声检测，参看 startHardwareEchoDetection:{@link #ByteRTCEngine#startHardwareEchoDetection} 。
  ///       - 建议在收到 rtcEngine:onHardwareEchoDetectionResult:{@link #ByteRTCEngineDelegate#rtcEngine:onHardwareEchoDetectionResult} 通知的检测结果后，调用本接口停止检测。
  ///       - 在用户进入房间前结束回声检测，释放对音频设备的占用，以免影响正常通话。

  FutureOr<int> stopHardwareEchoDetection() async {
    return await nativeCall('stopHardwareEchoDetection', []);
  }

  /// @hidden(macOS)
  /// @detail api
  /// @hiddensdk(audiosdk)
  /// @author wangfeng.1004
  /// @brief 启用蜂窝网络辅助增强，改善通话质量。
  /// @param config 参看 ByteRTCMediaTypeEnhancementConfig{@link #ByteRTCMediaTypeEnhancementConfig}。
  /// @return 方法调用结果： <br>
  ///        - 0: 成功。
  ///        - -1：失败，内部错误。
  ///        - -2: 失败，输入参数错误。
  /// @note 此功能默认不开启。

  FutureOr<int> setCellularEnhancement(
      ByteRTCMediaTypeEnhancementConfig config) async {
    return await nativeCall('setCellularEnhancement:', [config]);
  }

  /// @detail api
  /// @author keshixing.rtc
  /// @brief 设置本地代理。
  /// @param configurations 本地代理配置参数。参看 ByteRTCLocalProxyInfo{@link #ByteRTCLocalProxyInfo}。 <br>
  ///        你可以根据自己的需要选择同时设置 Http 隧道 和 Socks5 两类代理，或者单独设置其中一类代理。如果你同时设置了 Http 隧道 和 Socks5 两类代理，此时，媒体和信令采用 Socks5 代理， Http 请求采用 Http 隧道代理；如果只设置 Http 隧道 或 Socks5 一类代理，媒体、信令和 Http 请求均采用已设置的代理。 <br>
  ///        调用此接口设置本地代理后，若想清空当前已有的代理设置，可再次调用此接口，选择不设置任何代理即可清空。
  /// @note
  ///       - 该方法需要在进房前调用。
  ///       - 调用该方法设置本地代理后，SDK 会触发 rtcEngine:onLocalProxyStateChanged:withProxyState:withProxyError:{@link #ByteRTCEngineDelegate#rtcEngine:onLocalProxyStateChanged:withProxyState:withProxyError} ，返回代理连接的状态。
  ///

  FutureOr<int> setLocalProxy(
      NSArray<ByteRTCLocalProxyInfo> configurations) async {
    return await nativeCall('setLocalProxy:', [configurations]);
  }
}

class ByteRTCHttpClientProtocol extends NativeObserverClass {
  static const _$namespace = r'ByteRTCHttpClientProtocol';

  ByteRTCHttpClientProtocol([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"getAsync$timeoutMillisecond$withCallback":
                      r"getAsync:timeoutMillisecond:withCallback:",
                  r"postAsync$content$timeoutMillisecond$withCallback":
                      r"postAsync:content:timeoutMillisecond:withCallback:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"getAsync:timeoutMillisecond:withCallback:",
        getAsync$timeoutMillisecond$withCallback);

    registerEvent(r"postAsync:content:timeoutMillisecond:withCallback:",
        postAsync$content$timeoutMillisecond$withCallback);
  }

  /// @detail api
  /// @author weirongbin
  /// @brief 需要实现的 HTTP 异步 GET 接口。
  /// @param url GET 请求地址。
  /// @param timeout 超时时间。
  /// @param callback GET 请求结果回调函数。

  FutureOr<void> getAsync$timeoutMillisecond$withCallback(NSString url,
      int timeout, void Function(int code, String data) callback) async {}

  /// @detail api
  /// @author weirongbin
  /// @brief 需要实现的 HTTP 异步 POST 接口。
  /// @param url  POST 请求地址。
  /// @param content POST 请求的内容。
  /// @param timeout 超时时间。
  /// @param callback  POST 请求结果回调函数。

  FutureOr<void> postAsync$content$timeoutMillisecond$withCallback(
      NSString url,
      NSString content,
      int timeout,
      void Function(int code, String data) callback) async {}
}

enum ByteRTCProblemFeedbackOption {
  /// @brief 没有问题
  ///
  ByteRTCProblemFeedbackOptionNone(0),

  /// @brief 其他问题
  ///
  ByteRTCProblemFeedbackOptionOtherMessage(1),

  /// @brief 连接失败
  ///
  ByteRTCProblemFeedbackOptionDisconnected(2),

  /// @brief 耳返延迟大
  ///
  ByteRTCProblemFeedbackOptionEarBackDelay(4),

  /// @brief 本端有杂音
  ///
  ByteRTCProblemFeedbackOptionLocalNoise(1024),

  /// @brief 本端声音卡顿
  ///
  ByteRTCProblemFeedbackOptionLocalAudioLagging(2048),

  /// @brief 本端无声音
  ///
  ByteRTCProblemFeedbackOptionLocalNoAudio(4096),

  /// @brief 本端声音大/小
  ///
  ByteRTCProblemFeedbackOptionLocalAudioStrength(8192),

  /// @brief 本端有回声
  ///
  ByteRTCProblemFeedbackOptionLocalEcho(16384),

  /// @brief 本端视频模糊
  ///
  ByteRTCProblemFeedbackOptionLocalVideoFuzzy(16777216),

  /// @brief 本端音视频不同步
  ///
  ByteRTCProblemFeedbackOptionLocalNotSync(33554432),

  /// @brief 本端视频卡顿
  ///
  ByteRTCProblemFeedbackOptionLocalVideoLagging(67108864),

  /// @brief 本端无画面
  ///
  ByteRTCProblemFeedbackOptionLocalNoVideo(134217728),

  /// @brief 远端有杂音
  ///
  ByteRTCProblemFeedbackOptionRemoteNoise(32),

  /// @brief 远端声音卡顿
  ///
  ByteRTCProblemFeedbackOptionRemoteAudioLagging(64),

  /// @brief 远端无声音
  ///
  ByteRTCProblemFeedbackOptionRemoteNoAudio(128),

  /// @brief 远端声音大/小
  ///
  ByteRTCProblemFeedbackOptionRemoteAudioStrength(256),

  /// @brief 远端有回声
  ///
  ByteRTCProblemFeedbackOptionRemoteEcho(512),

  /// @brief 远端视频模糊
  ///
  ByteRTCProblemFeedbackOptionRemoteVideoFuzzy(524288),

  /// @brief 远端音视频不同步
  ///
  ByteRTCProblemFeedbackOptionRemoteNotSync(1048576),

  /// @brief 远端视频卡顿
  ///
  ByteRTCProblemFeedbackOptionRemoteVideoLagging(2097152),

  /// @brief 远端无画面
  ///
  ByteRTCProblemFeedbackOptionRemoteNoVideo(4194304);

  final dynamic $value;
  const ByteRTCProblemFeedbackOption([this.$value]);
}

class ByteRTCVideoSource extends NativeClass {
  static const _$namespace = r'ByteRTCVideoSource';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoSource([NativeClassOptions? options])
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
}

enum ByteRTCAggregationOption {
  /// @brief 流聚合向下取值  （默认策略）
  ///
  ByteRTCAggregationOptionMin(0),

  /// @brief 流聚合向上取值
  ///
  ByteRTCAggregationOptionMax(1),

  /// @brief 流聚合按比例取值，比例相同时，向下取值
  ///
  ByteRTCAggregationOptionMajority(2);

  final dynamic $value;
  const ByteRTCAggregationOption([this.$value]);
}

class ByteRTCKTVManager extends NativeClass {
  static const _$namespace = r'ByteRTCKTVManager';
  static get codegen_$namespace => _$namespace;

  ByteRTCKTVManager([NativeClassOptions? options])
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

  FutureOr<ByteRTCKTVManagerDelegate?> get delegate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCKTVManagerDelegate?>("delegate");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCKTVManagerDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegate(FutureOr<ByteRTCKTVManagerDelegate?> value) {
    sendInstanceSet("delegate", value);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 获取歌曲列表。
  /// @param pageNumber 页码，默认从 1 开始。
  /// @param pageSize 每页显示歌曲的最大数量，取值范围 [1,99]。
  /// @param filterType 歌曲过滤方式，参看 ByteRTCMusicFilterType{@link #ByteRTCMusicFilterType}。多个过滤方式可以按位或组合。
  /// @note 调用接口后，你会收到 ktvManager:onMusicListResult:totalSize:errorCode:{@link #ByteRTCKTVManagerDelegate#ktvManager:onMusicListResult:totalSize:errorCode} 回调歌曲列表。

  FutureOr<void> getMusicList(
      int pageNumber, int pageSize, int filterType) async {
    return await nativeCall('getMusicList:pageSize:filterType:',
        [pageNumber, pageSize, filterType]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 根据关键词搜索歌曲。
  /// @param keyWord 关键词，字符串长度最大为 20 个字符。
  /// @param pageNumber 页码，默认从 1 开始。
  /// @param pageSize 每页显示歌曲的最大数量，取值范围 [1,99]。
  /// @param filterType 歌曲过滤方式，参看 ByteRTCMusicFilterType{@link #ByteRTCMusicFilterType}。多个过滤方式可以按位或组合。
  /// @note 调用接口后，你会收到 ktvManager:onSearchMusicResult:totalSize:errorCode:{@link #ByteRTCKTVManagerDelegate#ktvManager:onSearchMusicResult:totalSize:errorCode} 回调歌曲列表。

  FutureOr<void> searchMusic(
      NSString keyWord, int pageNumber, int pageSize, int filterType) async {
    return await nativeCall('searchMusic:pageNumber:pageSize:filterType:',
        [keyWord, pageNumber, pageSize, filterType]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 指定歌曲榜单，并获取其歌曲列表。
  /// @param customHotlistId 榜单 ID 列表。<br>
  ///                        默认榜单 ID 有：`ContentCenter` 和 `Project`，分别是火山内容中心热歌榜和项目热歌榜。如果你需要其他榜单，请联系技术支持人员。
  /// @param filterType 歌曲过滤方式，参看 ByteRTCMusicFilterType{@link #ByteRTCMusicFilterType}。多个过滤方式可以按位或组合。
  /// @note 调用接口后，你会收到 ktvManager:onHotMusicResult:errorCode:{@link #ByteRTCKTVManagerDelegate#ktvManager:onHotMusicResult:errorCode} 回调歌曲列表。

  FutureOr<void> getHotMusic(NSArray<NSString> customHotlistId,
      ByteRTCMusicFilterType filterType) async {
    return await nativeCall(
        'getHotMusic:filterType:', [customHotlistId, filterType.$value]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 获取音乐详细信息。
  /// @param musicId 音乐 ID。
  /// @note 调用接口后，你会收到 ktvManager:onMusicDetailResult:errorCode:{@link #ByteRTCKTVManagerDelegate#ktvManager:onMusicDetailResult:errorCode} 回调。

  FutureOr<void> getMusicDetail(NSString musicId) async {
    return await nativeCall('getMusicDetail:', [musicId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 下载音乐。
  /// @param musicId 音乐 ID。
  /// @return 下载任务 ID。
  /// @note
  ///       - 若音乐下载成功，你会收到 ktvManager:onDownloadSuccess:downloadResult:{@link #ByteRTCKTVManagerDelegate#ktvManager:onDownloadSuccess:downloadResult} 回调。
  ///       - 若音乐下载失败，你会收到 ktvManager:onDownloadFailed:errorCode:{@link #ByteRTCKTVManagerDelegate#ktvManager:onDownloadFailed:errorCode} 回调。
  ///       - 音乐下载进度更新时，你会收到 ktvManager:onDownloadMusicProgress:progress:{@link #ByteRTCKTVManagerDelegate#ktvManager:onDownloadMusicProgress:progress} 回调。

  FutureOr<int> downloadMusic(NSString musicId) async {
    return await nativeCall('downloadMusic:', [musicId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 下载歌词。
  /// @param musicId 音乐 ID。
  /// @param lyricType 歌词文件类型，参看 ByteRTCDownloadLyricType{@link #ByteRTCDownloadLyricType}。
  /// @return 下载任务 ID。
  /// @note
  ///       - 若歌词下载成功，你会收到 ktvManager:onDownloadSuccess:downloadResult:{@link #ByteRTCKTVManagerDelegate#ktvManager:onDownloadSuccess:downloadResult} 回调。
  ///       - 若歌词下载失败，你会收到 ktvManager:onDownloadFailed:errorCode:{@link #ByteRTCKTVManagerDelegate#ktvManager:onDownloadFailed:errorCode} 回调。

  FutureOr<int> downloadLyric(
      NSString musicId, ByteRTCDownloadLyricType lyricType) async {
    return await nativeCall(
        'downloadLyric:lyricType:', [musicId, lyricType.$value]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 下载 MIDI 文件。
  /// @param musicId 音乐 ID。
  /// @return 下载任务 ID。
  /// @note
  ///       - 若文件下载成功，你会收到 ktvManager:onDownloadSuccess:downloadResult:{@link #ByteRTCKTVManagerDelegate#ktvManager:onDownloadSuccess:downloadResult} 回调。
  ///       - 若文件下载失败，你会收到 ktvManager:onDownloadFailed:errorCode:{@link #ByteRTCKTVManagerDelegate#ktvManager:onDownloadFailed:errorCode} 回调。

  FutureOr<int> downloadMidi(NSString musicId) async {
    return await nativeCall('downloadMidi:', [musicId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 取消下载任务。
  /// @param downloadId 下载任务 ID。

  FutureOr<void> cancelDownload(int downloadId) async {
    return await nativeCall('cancelDownload:', [downloadId]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 清除当前音乐缓存文件，包括音乐音频和歌词。

  FutureOr<void> clearCache() async {
    return await nativeCall('clearCache', []);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 设置歌曲文件最大占用的本地缓存。
  /// @param maxCacheSizeMB 本地缓存，单位 MB。 <br>
  ///        设置值小于等于 0 时，使用默认值 1024 MB。

  FutureOr<void> setMaxCacheSize(int maxCacheSizeMB) async {
    return await nativeCall('setMaxCacheSize:', [maxCacheSizeMB]);
  }

  /// @detail api
  /// @author lihuan.wuti2ha
  /// @brief 获取 KTV 播放器。
  /// @return KTV 播放器接口，参看 ByteRTCKTVPlayer{@link #ByteRTCKTVPlayer}。

  FutureOr<ByteRTCKTVPlayer> getKTVPlayer() async {
    final result = await nativeCall('getKTVPlayer', []);
    return packObject(
        result,
        () =>
            ByteRTCKTVPlayer(const NativeClassOptions([], disableInit: true)));
  }
}
