// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import androidx.annotation.NonNull;

import com.ss.bytertc.ktv.IKTVManagerEventHandler;
import com.ss.bytertc.ktv.IKTVPlayer;
import com.ss.bytertc.ktv.IKTVPlayerEventHandler;
import com.ss.bytertc.ktv.data.DownloadResult;
import com.ss.bytertc.ktv.data.HotMusicInfo;
import com.ss.bytertc.ktv.data.KTVErrorCode;
import com.ss.bytertc.ktv.data.KTVPlayerErrorCode;
import com.ss.bytertc.ktv.data.MusicInfo;
import com.ss.bytertc.ktv.data.PlayState;
import com.volcengine.VolcApiEngine.BeanFactory;

public class ByteRTCKTVPlayerEventHandlerImpl extends IKTVPlayerEventHandler implements BeanFactory.EventReceiver {

    static ByteRTCKTVPlayerEventHandlerImpl observer;
    public BeanFactory.EventEmitter ee;

    public ByteRTCKTVPlayerEventHandlerImpl(BeanFactory.EventEmitter ee) {
        this.ee = ee;
        observer = this;
    }


    @Override
    public void onPlayProgress(@NonNull String musicId, long progress) {
        this.ee.sendEvent("onPlayProgress", musicId, progress);
    }

    @Override
    public void onPlayStateChanged(@NonNull String musicId, @NonNull PlayState playState, KTVPlayerErrorCode errorCode) {
        this.ee.sendEvent("onPlayStateChanged", musicId, playState, errorCode);

    }
}
