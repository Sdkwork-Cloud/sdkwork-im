// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import com.ss.bytertc.engine.IWTNStreamEventHandler;
import com.ss.bytertc.engine.data.DataMessageSourceType;
import com.ss.bytertc.engine.data.VideoFrameInfo;
import com.ss.bytertc.engine.data.WTNSubscribeState;
import com.ss.bytertc.engine.data.WTNSubscribeStateChangeReason;
import com.ss.bytertc.engine.type.RemoteAudioStats;
import com.ss.bytertc.engine.type.RemoteVideoStats;
import com.volcengine.VolcApiEngine.BeanFactory;

import java.nio.ByteBuffer;

public class ByteRTCWTNEventHandlerImpl implements BeanFactory.EventReceiver, IWTNStreamEventHandler {

    public BeanFactory.EventEmitter ee;
    public ByteRTCWTNEventHandlerImpl(BeanFactory.EventEmitter ee) {
        this.ee = ee;
    }

    @Override
    public void onWTNRemoteVideoStats(String streamId, RemoteVideoStats stats) {
        this.ee.sendEvent("onWTNRemoteVideoStats", streamId, stats);
    }

    @Override
    public void onWTNRemoteAudioStats(String streamId, RemoteAudioStats stats) {
        this.ee.sendEvent("onWTNRemoteAudioStats", streamId, stats);
    }

    @Override
    public void onWTNVideoSubscribeStateChanged(String streamId, WTNSubscribeState stateCode, WTNSubscribeStateChangeReason reason) {
        this.ee.sendEvent("onWTNVideoSubscribeStateChanged", streamId, stateCode, reason);
    }

    @Override
    public void onWTNAudioSubscribeStateChanged(String streamId, WTNSubscribeState stateCode, WTNSubscribeStateChangeReason reason) {
        this.ee.sendEvent("onWTNAudioSubscribeStateChanged", streamId, stateCode, reason);

    }

    @Override
    public void onWTNFirstRemoteAudioFrame(String streamId) {
        this.ee.sendEvent("onWTNFirstRemoteAudioFrame", streamId);
    }

    @Override
    public void onWTNFirstRemoteVideoFrameDecoded(String streamId, VideoFrameInfo info) {
        this.ee.sendEvent("onWTNFirstRemoteVideoFrameDecoded", streamId, info);
    }

    @Override
    public void onWTNSEIMessageReceived(String streamId, int channelId, ByteBuffer message) {
        this.ee.sendEvent("onWTNSEIMessageReceived", streamId, channelId, message);
    }

    @Override
    public void onWTNDataMessageReceived(String streamId, ByteBuffer message, DataMessageSourceType sourceType) {
        this.ee.sendEvent("onWTNDataMessageReceived", streamId, message, sourceType);

    }
}
