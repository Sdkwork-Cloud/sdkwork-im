// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import android.graphics.Bitmap;

import com.ss.bytertc.engine.data.RemoteStreamKey;
import com.ss.bytertc.engine.data.StreamIndex;
import com.ss.bytertc.engine.video.ISnapshotResultCallback;
import com.ss.bytertc.ktv.IKTVManagerEventHandler;
import com.ss.bytertc.ktv.data.DownloadResult;
import com.ss.bytertc.ktv.data.HotMusicInfo;
import com.ss.bytertc.ktv.data.KTVErrorCode;
import com.ss.bytertc.ktv.data.MusicInfo;
import com.volcengine.VolcApiEngine.BeanFactory;

import java.io.FileOutputStream;
import java.io.IOException;

public class ByteRTCKTVManagerEventHandlerImpl extends IKTVManagerEventHandler implements BeanFactory.EventReceiver {

    static ByteRTCKTVManagerEventHandlerImpl observer;
    public BeanFactory.EventEmitter ee;

    public ByteRTCKTVManagerEventHandlerImpl(BeanFactory.EventEmitter ee) {
        this.ee = ee;
        observer = this;
    }

    @Override
    public void onMusicListResult(MusicInfo[] musicInfos, int totalSize, KTVErrorCode errorCode) {
        this.ee.sendEvent("onMusicListResult", musicInfos, totalSize, errorCode);
    }

    @Override
    public void onSearchMusicResult(MusicInfo[] musicInfos, int totalSize, KTVErrorCode errorCode) {
        this.ee.sendEvent("onSearchMusicResult", musicInfos, totalSize, errorCode);
    }

    @Override
    public void onHotMusicResult(HotMusicInfo[] hotMusics, KTVErrorCode errorCode) {
        this.ee.sendEvent("onHotMusicResult", hotMusics, errorCode);
    }

    @Override
    public void onMusicDetailResult(MusicInfo musicInfo, KTVErrorCode errorCode) {
        this.ee.sendEvent("onMusicDetailResult", musicInfo, errorCode);
    }

    @Override
    public void onDownloadSuccess(int downloadId, DownloadResult result) {
        this.ee.sendEvent("onDownloadSuccess", downloadId, result);
    }

    @Override
    public void onDownloadFailed(int downloadId, KTVErrorCode errorCode) {
        this.ee.sendEvent("onDownloadFailed", downloadId, errorCode);
    }

    @Override
    public void onDownloadMusicProgress(int downloadId, int downloadProgress) {
        this.ee.sendEvent("onDownloadMusicProgress", downloadId, downloadProgress);
    }

    @Override
    public void onClearCacheResult(KTVErrorCode errorCode) {
        this.ee.sendEvent("onClearCacheResult", errorCode);
    }
}
