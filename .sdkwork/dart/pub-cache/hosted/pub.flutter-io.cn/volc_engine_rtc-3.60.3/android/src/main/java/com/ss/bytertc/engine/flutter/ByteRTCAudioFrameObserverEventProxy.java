// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import com.ss.bytertc.engine.IAudioFrameObserver;
import com.ss.bytertc.engine.data.StreamInfo;
import com.ss.bytertc.engine.utils.IAudioFrame;
import com.volcengine.VolcApiEngine.BeanFactory;

import java.lang.reflect.Array;
import java.nio.Buffer;
import java.nio.ByteBuffer;
import java.util.HashMap;

public class ByteRTCAudioFrameObserverEventProxy implements BeanFactory.EventReceiver, IAudioFrameObserver {

    public BeanFactory.EventEmitter ee;
    public ByteRTCAudioFrameObserverEventProxy(BeanFactory.EventEmitter ee) {
        this.ee = ee;
    }

    private HashMap<String, Object> wrapperFrameData(IAudioFrame audioFrame) {
        HashMap<String, Object> map = new HashMap<>();
        ByteBuffer buffer = audioFrame.getDataBuffer();
        map.put("channel", audioFrame.channel().value());
        map.put("buffer", buffer);
        map.put("samples", audioFrame.data_size());
        map.put("sampleRate", audioFrame.sample_rate().value());
        return map;
    }

    @Override
    public void onRecordAudioFrame(IAudioFrame audioFrame) {
        this.ee.sendEvent("onRecordAudioFrame", wrapperFrameData(audioFrame));
    }

    @Override
    public void onPlaybackAudioFrame(IAudioFrame audioFrame) {
        this.ee.sendEvent("onPlaybackAudioFrame", wrapperFrameData(audioFrame));
    }

    @Override
    public void onRemoteUserAudioFrame(String streamId, StreamInfo streamInfo, IAudioFrame audioFrame) {
        this.ee.sendEvent("onRemoteUserAudioFrame", streamId, streamInfo, wrapperFrameData(audioFrame));
    }

    @Override
    public void onMixedAudioFrame(IAudioFrame audioFrame) {
        this.ee.sendEvent("onMixedAudioFrame", wrapperFrameData(audioFrame));
    }

    @Override
    public void onCaptureMixedAudioFrame(IAudioFrame audioFrame) {
        this.ee.sendEvent("onCaptureMixedAudioFrame", wrapperFrameData(audioFrame));
    }
}
