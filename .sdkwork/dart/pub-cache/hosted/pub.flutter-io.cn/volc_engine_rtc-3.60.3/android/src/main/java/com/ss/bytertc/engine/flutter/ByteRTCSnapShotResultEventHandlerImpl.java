// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import android.graphics.Bitmap;

import com.ss.bytertc.engine.IVideoSource;
import com.ss.bytertc.engine.data.RemoteStreamKey;
import com.ss.bytertc.engine.data.StreamIndex;
import com.ss.bytertc.engine.data.StreamInfo;
import com.ss.bytertc.engine.video.ISnapshotResultCallback;
import com.volcengine.VolcApiEngine.BeanFactory;

import java.io.FileOutputStream;
import java.io.IOException;

public class ByteRTCSnapShotResultEventHandlerImpl implements ISnapshotResultCallback, BeanFactory.EventReceiver {

    static ByteRTCSnapShotResultEventHandlerImpl observer;
    public BeanFactory.EventEmitter ee;

    public static final int ERROR_WRITE_FILE_FAILED = -102;
    public static final int ERROR_IMAGE_FORMAT = -103;

    public ByteRTCSnapShotResultEventHandlerImpl(BeanFactory.EventEmitter ee) {
        this.ee = ee;
        observer = this;
    }

    public int storeImageToFile(Bitmap image, String filePath) {
        if (image == null) {
            return ERROR_IMAGE_FORMAT;
        }
        try (FileOutputStream fos = new FileOutputStream(filePath)) {
            image.compress(Bitmap.CompressFormat.JPEG, 100, fos);
            return ReturnStatus.OK;
        } catch (IOException e) {
            return ERROR_WRITE_FILE_FAILED;
        }
    }

    @Override
    public void onTakeLocalSnapshotResult(long taskId, IVideoSource videoSource, Bitmap image, int errorCode) {
        int res = ERROR_IMAGE_FORMAT;
        String filePath = ByteRTCHelper.currentSnapShotFilePathMap.get(taskId);
        if (filePath != null) {
            res = storeImageToFile(image, filePath);
        }
        this.ee.sendEvent("onTakeLocalSnapshotResult", taskId, image.getWidth(), image.getHeight(), filePath, res);
    }

    @Override
    public void onTakeRemoteSnapshotResult(long taskId, String streamId, StreamInfo streamInfo, Bitmap image, int errorCode) {
        int res = ERROR_IMAGE_FORMAT;
        String filePath = ByteRTCHelper.currentSnapShotFilePathMap.get(taskId);
        if (filePath != null) {
            res = storeImageToFile(image, filePath);
        }
        this.ee.sendEvent("onTakeRemoteSnapshotResult", taskId, streamId, streamInfo, image.getWidth(), image.getHeight(), filePath, res);
    }
}
