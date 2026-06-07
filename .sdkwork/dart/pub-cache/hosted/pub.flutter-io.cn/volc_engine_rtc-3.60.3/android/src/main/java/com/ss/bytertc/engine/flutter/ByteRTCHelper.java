// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import android.content.Intent;
import android.view.View;

import android.util.Log;

import com.ss.bytertc.engine.RTCEngine;
import com.ss.bytertc.engine.RemoteVideoRenderConfig;
import com.ss.bytertc.engine.VideoCanvas;
import com.ss.bytertc.engine.audio.IMediaPlayer;
import com.ss.bytertc.engine.data.AudioRoute;
import com.ss.bytertc.engine.data.PlayerState;
import com.ss.bytertc.engine.data.ScreenMediaType;
import com.ss.bytertc.engine.data.VirtualBackgroundSource;
import com.ss.bytertc.engine.data.VirtualBackgroundSourceType;
import com.ss.bytertc.engine.live.MixedStreamPushTargetConfig;
import com.ss.bytertc.engine.type.ProblemFeedbackInfo;
import com.ss.bytertc.engine.type.ProblemFeedbackOption;
import com.ss.bytertc.engine.video.RTCVideoEffect;
import com.ss.bytertc.engine.video.VideoCaptureConfig;
import com.ss.bytertc.ktv.KTVManagerImpl;
import com.ss.bytertc.ktv.data.MusicFilterType;
import com.volcengine.VolcApiEngine.view.VolcViewManager;
import com.ss.bytertc.engine.live.MixedStreamConfig;

import org.json.JSONObject;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * @locale zh
 * @author quemingyi
 * @brief Provide the helper for the ByteRTC Flutter SDK, been called in the dart code.
 */
public class ByteRTCHelper {
    final static String Tag = "ByteRTCHelper";
    static RTCEngine engineInstance;

    static Map<Long, String> currentSnapShotFilePathMap = new HashMap<>();

    /**
     * @locale zh
     * @brief Get the view by id.
     * @param id The id of the view.
     * @return The view.
     */
    public View getView(String id) {
        ByteRTCView view = (ByteRTCView) VolcViewManager.getViewById(id);
        return view.getView();
    }

    /**
     * @locale zh
     * @brief Get the application context.
     */
    public android.content.Context getContext() {
        return ByteRTCPlugin.applicationContext;
    }

    /**
     * @locale zh
     * @brief Set the view's z order on top.
     * @param id The id of the view.
     * @param onTop Whether the view is on top.
     * @return The status of the operation.
     */
    public int setZOrderOnTop(String id, Boolean onTop) {
        ByteRTCView view = (ByteRTCView) VolcViewManager.getViewById(id);
        if (view != null) {
            return view.setZOrderOnTop(onTop);
        }
        return ReturnStatus.VIEW_NOT_FOUND;
    }

    /**
     * @locale zh
     * @brief Set the view's z order media overlay.
     * @param id The id of the view.
     * @param isMediaOverlay Whether the view is media overlay.
     * @return The status of the operation.
     */
    public int setZOrderMediaOverlay(String id, Boolean isMediaOverlay) {
        ByteRTCView view = (ByteRTCView) VolcViewManager.getViewById(id);
        if (view != null) {
            return view.setZOrderMediaOverlay(isMediaOverlay);
        }
        return ReturnStatus.VIEW_NOT_FOUND;
    }

    /**
     * @locale zh
     * @brief Start the screen capture.
     * @param type The type of the screen capture.
     * @return The status of the operation.
     */
    public int startScreenCapture(RTCEngine engine, ScreenMediaType type) {
        try {
            engineInstance = engine;
            android.content.Context context = ByteRTCPlugin.applicationContext.getApplicationContext();
            Intent intent = new Intent(context.getPackageName() + ".action.REQUEST_SCREEN_CAPTURE");
            // setPackage is necessary
            intent.setPackage(context.getPackageName());
            intent.putExtra(BaseScreenCaptureRequestActivity.EXTRA_STREAM_TYPE, type);
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            context.startActivity(intent);
            return ReturnStatus.OK;
        } catch (Exception e){
            Log.e(Tag, "startScreenCapture failed: " + e.getMessage());
            return ReturnStatus.ERROR;
        }
    }

    /**
     * @locale zh
     * @brief Feedback
     * @return The status of the operation.
     */
    public int feedback(RTCEngine engine, List<Integer> opts, ProblemFeedbackInfo info) {
        List<ProblemFeedbackOption> lists = new ArrayList<>();
        ProblemFeedbackOption[] values = ProblemFeedbackOption.values();
        for (Integer index : opts) {
            if (index != null && index >= 0 && index < values.length) {
                lists.add(values[index]);
            }
        }
        return engine.feedback(lists, info);
    }

    /**
     * @locale zh
     * @brief Update remote stream video canvas
     */
    public int updateRemoteStreamVideoCanvas(RTCEngine engine, String streamId, int renderMode, int color) {
        RemoteVideoRenderConfig config = new RemoteVideoRenderConfig();
        config.renderMode = renderMode;
        config.backgroundColor = color;
        return engine.updateRemoteStreamVideoCanvas(streamId, config);
    }

    /**
     * @locale zh
     * @brief Remove remote video canvas.
     */
    public int removeRemoteVideo(RTCEngine engine, String streamId) {
        // Use a blank Canvas as Remove
        VideoCanvas videoCanvas = new VideoCanvas();
        return engine.setRemoteVideoCanvas(streamId, videoCanvas);
    }

    /**
     * @locale zh
     * @brief Take snapshot for remote user.
     */
    public int takeRemoteSnapshot(RTCEngine engine, String streamId, String filePath) {
        long taskId = engine.takeRemoteSnapshot(streamId, ByteRTCSnapShotResultEventHandlerImpl.observer);
        ByteRTCHelper.currentSnapShotFilePathMap.put(taskId, filePath);
        return (int)taskId;
    }

    /**
     * @locale zh
     * @brief Take snapshot for local user.
     */
    public int takeLocalSnapshot(RTCEngine engine, String filePath) {
        long taskId = engine.takeLocalSnapshot(ByteRTCSnapShotResultEventHandlerImpl.observer);
        ByteRTCHelper.currentSnapShotFilePathMap.put(taskId, filePath);
        return (int)taskId;
    }

    /**
     * @locale zh
     * @brief Remove local video canvas.
     */
    public int removeLocalVideo(RTCEngine engine) {
        // Use a blank Canvas as Remove
        VideoCanvas videoCanvas = new VideoCanvas();
        return engine.setLocalVideoCanvas(videoCanvas);
    }

    /**
     * @locale zh
     * @brief setVideoWatermark
     */
    public int setVideoWatermark(RTCEngine engine, String imagePath, Object arguments) {
        try {
            final ByteRTCTypes watermarkConfig = new ByteRTCTypes(arguments);
            return engine.setVideoWatermark(imagePath, ByteRTCTypeHelper.toRTCWatermarkConfig(watermarkConfig));
        } catch (Exception e) {
            Log.e(Tag, "setVideoWatermark failed: " + e.getMessage());
            return ReturnStatus.ERROR;
        }
    }

    /**
     * @locale zh
     * @brief setVideoCaptureConfig
     */
    public int setVideoCaptureConfig(RTCEngine engine, VideoCaptureConfig videoCaptureConfig) {
        try {
            return engine.setVideoCaptureConfig(videoCaptureConfig);
        } catch (Exception e) {
            Log.e(Tag, "setVideoCaptureConfig failed: " + e.getMessage());
            return ReturnStatus.ERROR;
        }
    }

    public int enableVirtualBackground(RTCVideoEffect effect, String modelPath, Map<String, Object> arguments) {
        VirtualBackgroundSource source = new VirtualBackgroundSource();
        if (arguments.containsKey("sourcePath")) {
            Object pathObj = arguments.get("sourcePath");
            if (pathObj instanceof String) {
                source.sourcePath = (String) pathObj;
            }
        }

        if (arguments.containsKey("sourceColor")) {
            Object colorObj = arguments.get("sourceColor");
            if (colorObj instanceof Number) {
                source.sourceColor = ((Number) colorObj).intValue();
            }
        } else {
            source.sourceColor = 0;
        }

        if (arguments.containsKey("sourceType")) {
            Object typeObj = arguments.get("sourceType");
            if (typeObj instanceof Number) {
                int typeValue = ((Number) typeObj).intValue();
                if (typeValue == 0) {
                    source.sourceType = VirtualBackgroundSourceType.COLOR;
                } else if (typeValue == 1) {
                    source.sourceType = VirtualBackgroundSourceType.IMAGE;
                }
            }
        }
        return effect.enableVirtualBackground(modelPath, source);
    }


    public int getMusicList(KTVManagerImpl ktv, int pageNum, int pageSize, int[] filters) {
        try {
            MusicFilterType[] filterTypes = new MusicFilterType[filters.length];
            for (int i = 0; i < filters.length; i++) {
                filterTypes[i] = MusicFilterType.fromId(filters[i]);
            }
            ktv.getMusicList(pageNum, pageSize, filterTypes);
            return ReturnStatus.OK;
        } catch (Exception e) {
            Log.e(Tag, "getMusicList failed: " + e.getMessage());
            return ReturnStatus.ERROR;
        }
    }

    public int searchMusic(KTVManagerImpl ktv, String key, int pageNum, int pageSize, int[] filters) {
        try {
            MusicFilterType[] filterTypes = new MusicFilterType[filters.length];
            for (int i = 0; i < filters.length; i++) {
                filterTypes[i] = MusicFilterType.fromId(filters[i]);
            }
            ktv.searchMusic(key, pageNum, pageSize, filterTypes);
            return ReturnStatus.OK;
        } catch (Exception e) {
            Log.e(Tag, "searchMusic failed: " + e.getMessage());
            return ReturnStatus.ERROR;
        }
    }

    public int startPushMixedStream(
            RTCEngine engine,
            String taskId,
            MixedStreamPushTargetConfig pushTargetConfig,
            JSONObject mixedConfig
    ) {
        try {
            MixedStreamConfig config = DataTransformer.MixedStreamConfig(mixedConfig);
            return engine.startPushMixedStream(taskId, pushTargetConfig, config);
        } catch (Exception e) {
            Log.e(Tag, "startPushMixedStream failed: " + e.getMessage());
            return ReturnStatus.ERROR;
        }
    }

    public int updatePushMixedStream(
            RTCEngine engine,
            String taskId,
            MixedStreamPushTargetConfig pushTargetConfig,
            JSONObject mixedConfig
    ) {
        try {
            MixedStreamConfig config = DataTransformer.MixedStreamConfig(mixedConfig);
            return engine.updatePushMixedStream(taskId, pushTargetConfig, config);
        } catch (Exception e) {
            Log.e(Tag, "updatePushMixedStream failed: " + e.getMessage());
            return ReturnStatus.ERROR;
        }
    }
    
    public AudioRoute getAudioRoute(RTCEngine engine) {
        try {
            return engine.getAudioRoute();
        } catch (Exception e) {
            Log.e(Tag, "getAudioRoute failed: " + e.getMessage());
            return AudioRoute.AUDIO_ROUTE_DEFAULT;
        }
    }

    public PlayerState getState(IMediaPlayer mediaPlayer) {
        try {
            return mediaPlayer.getState();
        } catch (Exception e) {
            Log.e(Tag, "getState failed: " + e.getMessage());
            return PlayerState.FAILED;
        }
    }
}
