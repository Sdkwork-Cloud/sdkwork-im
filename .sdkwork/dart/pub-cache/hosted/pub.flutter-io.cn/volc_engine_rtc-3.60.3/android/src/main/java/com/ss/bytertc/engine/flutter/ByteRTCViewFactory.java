// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import android.content.Context;

import io.flutter.plugin.common.BinaryMessenger;
import io.flutter.plugin.common.StandardMessageCodec;
import io.flutter.plugin.platform.PlatformView;
import io.flutter.plugin.platform.PlatformViewFactory;
import com.volcengine.VolcApiEngine.view.*;

public class ByteRTCViewFactory extends PlatformViewFactory {
    private final BinaryMessenger messenger;

    public ByteRTCViewFactory(BinaryMessenger messenger) {
        super(StandardMessageCodec.INSTANCE);
        this.messenger = messenger;
    }

    @Override
    public PlatformView create(Context context, int id, Object args) {
        final ByteRTCView view = new ByteRTCView(context, String.valueOf(id), args);
        VolcViewManager.putViewById(String.valueOf(id), view);
        return view;
    }
}