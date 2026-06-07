// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import android.content.Context;
import android.util.Log;
import android.view.SurfaceView;
import android.view.TextureView;
import android.view.View;

import androidx.annotation.NonNull;

import java.util.Map;

import io.flutter.plugin.platform.PlatformView;

public class ByteRTCView implements PlatformView {

    final String TAG = "ByteRTCView";

    @NonNull
    private final View mRenderView;

    ByteRTCView(Context context, String id, Object args) {
        mRenderView = createRenderView(context, args);
    }

    @Override
    public View getView() {
        return mRenderView;
    }
    @Override
    public void dispose() {
        // 释放资源，如果有的话
    }

    @NonNull
    private View createRenderView(Context context, Object args) {
        String type = args instanceof Map ? (String) ((Map) args).get("type") : "texture";
        if ("surface".equals(type)) {
            return new SurfaceView(context);
        }
        if ("texture".equals(type)) {
            TextureView view = new TextureView(context);
            // not support ?
            // view.setBackgroundColor(0xFF000000);
            return view;
        }
        // default as TextureView
        Log.d(TAG, "Create view, but not view type provided, use texture view as default.");
        return new TextureView(context);
    }

    public int setZOrderOnTop(Boolean onTop) {
        if (mRenderView instanceof SurfaceView) {
            ((SurfaceView) mRenderView).setZOrderOnTop(onTop);
            return ReturnStatus.OK;
        }
        return ReturnStatus.ERROR;
    }

    public int setZOrderMediaOverlay(Boolean isMediaOverlay) {
        if (mRenderView instanceof SurfaceView) {
            ((SurfaceView) mRenderView).setZOrderMediaOverlay(isMediaOverlay);
            return ReturnStatus.OK;
        }
        return ReturnStatus.ERROR;
    }
}