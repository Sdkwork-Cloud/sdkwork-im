## CalledByNative
-keep @interface com.bytedance.realx.base.CalledByNative

# Webrtc、RtcEngine
-keep class com.bytedance.services.** {*;}
-keep class com.ss.bytertc.**{*;}

# Hybrid_runtime
-keep class com.volcengine.** {*;}
-keep class com.volcengine.VolcApiEngine.** {*;}
-keep class com.volcengine.rtc.** {*;}
-keep class com.volcengine.volc_engine_rtc_flutter.** {*;}


# CallByNaitve
-keepclasseswithmembers class * {
    @com.bytedance.realx.base.CalledByNative <methods>;
}

-keep class com.ss.bytertc.audio.device.router.device.HnEarBackDeviceSupport { *; }
-keep class com.ss.bytertc.audio.device.hwearback.** { *; }

-keep class com.pandora.common.**{*;}
-keep class com.pandora.common.applog.**{*;}
-keep class com.pandora.ttuploader2.** {*;}
-keep class com.ss.bduploader.** {*;}
-keep class com.pandora.ttlicense2.**{*;}