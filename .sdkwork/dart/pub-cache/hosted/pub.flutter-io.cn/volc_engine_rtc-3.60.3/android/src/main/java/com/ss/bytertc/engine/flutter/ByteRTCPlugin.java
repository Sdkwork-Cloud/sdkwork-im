// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import androidx.annotation.NonNull;
import com.volcengine.VolcApiEngine.*;
import io.flutter.embedding.engine.plugins.FlutterPlugin;
import io.flutter.plugin.common.MethodCall;
import io.flutter.plugin.common.MethodChannel;
import io.flutter.plugin.common.MethodChannel.MethodCallHandler;
import io.flutter.plugin.common.MethodChannel.Result;
import java.util.HashMap;
import java.util.Map;

/** ByteRTCPlugin */
public class ByteRTCPlugin
  implements FlutterPlugin, MethodCallHandler, IEventReceiver {
  /// The MethodChannel that will the communication between Flutter and native
  /// Android
  ///
  /// This local reference serves to register the plugin with the Flutter Engine
  /// and unregister it when the Flutter Engine is detached from the Activity
  private MethodChannel channel;
  private VolcApiEngine apiEngine;
  private static final String Tag = "ByteRTCPlugin";
  static android.content.Context applicationContext;

  @Override
  public void
  onAttachedToEngine(@NonNull FlutterPluginBinding flutterPluginBinding) {
    channel = new MethodChannel(flutterPluginBinding.getBinaryMessenger(),
                                "com.volcengine.rtc.hybrid_runtime");
    channel.setMethodCallHandler(this);
    apiEngine =
        new VolcApiEngine(flutterPluginBinding.getApplicationContext(), this);
    applicationContext = flutterPluginBinding.getApplicationContext();
    NativeVariableManager.init(this.apiEngine.msgClient, applicationContext);
    ClassHelper.init();
    flutterPluginBinding
        .getPlatformViewRegistry()
        .registerViewFactory("ByteRTCSurfaceView", new ByteRTCViewFactory(flutterPluginBinding.getBinaryMessenger()));
  }

  @Override
  public void onMethodCall(@NonNull MethodCall call, @NonNull Result result) {
    switch (call.method) {
      case "callApi":
        String params = call.argument("params");
        try {
          // 获取主线程处理器
          android.os.Handler mainHandler =
                  new android.os.Handler(android.os.Looper.getMainLooper());
          // 在主线程上执行 API 调用
          mainHandler.post(() -> {
            try {
              String response = apiEngine.callApi(params);
              result.success(response);
            } catch (Exception e) {
              e.printStackTrace();
            }
          });
        } catch (Exception e) {
          result.error("CALL_API_ERROR", e.getMessage(), null);
        }
        break;
      default:
        result.notImplemented();
    }
  }

  @Override
  public void OnEvent(String event, String data) {
    if (channel != null) {
      // 使用 Map 包装事件数据
      Map<String, Object> eventData = new HashMap<>();
      eventData.put("event", event);
      eventData.put("data", data);

      // 发送事件到 Flutter 端
      try {
        // 获取主线程处理器
        android.os.Handler mainHandler =
                new android.os.Handler(android.os.Looper.getMainLooper());
        // 在主线程上执行 API 调用
        mainHandler.post(() -> {
          try {
            channel.invokeMethod("onEvent", eventData);
          } catch (Exception e) {
            e.printStackTrace();
          }
        });
      } catch (Exception e) {
        e.printStackTrace();
      }
    }
  }

  @Override
  public void onDetachedFromEngine(@NonNull FlutterPluginBinding binding) {
    channel.setMethodCallHandler(null);
    channel = null;
  }
}
