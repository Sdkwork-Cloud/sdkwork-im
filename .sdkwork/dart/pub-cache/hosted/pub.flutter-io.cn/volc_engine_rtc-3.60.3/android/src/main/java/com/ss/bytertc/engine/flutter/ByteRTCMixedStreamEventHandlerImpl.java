// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import com.ss.bytertc.engine.live.IClientMixedStreamObserver;
import com.ss.bytertc.engine.live.MixedStreamTaskErrorCode;
import com.ss.bytertc.engine.live.MixedStreamTaskEvent;
import com.ss.bytertc.engine.live.MixedStreamTaskInfo;
import com.ss.bytertc.engine.live.MixedStreamType;
import com.ss.bytertc.engine.video.IVideoFrame;
import com.volcengine.VolcApiEngine.BeanFactory;

public class ByteRTCMixedStreamEventHandlerImpl implements BeanFactory.EventReceiver, IClientMixedStreamObserver {
    public BeanFactory.EventEmitter ee;
    public ByteRTCMixedStreamEventHandlerImpl(BeanFactory.EventEmitter ee) {
        this.ee = ee;
    }
    @Override
    public void onClientMixedStreamEvent(MixedStreamTaskInfo info, MixedStreamType type, MixedStreamTaskEvent event, MixedStreamTaskErrorCode error) {
        this.ee.sendEvent("onClientMixedStreamEvent", info, type, event, error);
    }

    @Override
    public void onMixedAudioFrame(String taskId, byte[] audioFrame, int frameNum, long timeStampMs) {
        this.ee.sendEvent("onMixedAudioFrame", taskId, audioFrame, frameNum, timeStampMs);
    }

    @Override
    public void onMixedVideoFrame(String taskId, IVideoFrame videoFrame) {
//        this.ee.sendEvent("onMixedVideoFrame", taskId, videoFrame);
    }

    @Override
    public void onMixedDataFrame(String taskId, byte[] dataFrame, long time) {
        this.ee.sendEvent("onMixedDataFrame", taskId, dataFrame, time);
    }

    @Override
    public void onCacheSyncVideoFrames(String taskId, String[] userIds, IVideoFrame[] videoFrame, byte[][] dataFrame, int count) {
//        this.ee.sendEvent("onCacheSyncVideoFrames", taskId, userIds, videoFrame, dataFrame, count);
    }

    @Override
    public void onMixedFirstAudioFrame(String taskId) {
        this.ee.sendEvent("onMixedFirstAudioFrame", taskId);
    }

    @Override
    public void onMixedFirstVideoFrame(String taskId) {
//        this.ee.sendEvent("onMixedFirstVideoFrame", taskId);
    }
}
