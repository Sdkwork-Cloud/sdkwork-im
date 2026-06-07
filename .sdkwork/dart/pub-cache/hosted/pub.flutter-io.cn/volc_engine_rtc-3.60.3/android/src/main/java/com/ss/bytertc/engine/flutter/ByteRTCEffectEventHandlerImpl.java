// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import com.ss.bytertc.engine.video.ExpressionDetectResult;
import com.ss.bytertc.engine.video.FaceDetectionResult;
import com.ss.bytertc.engine.video.IFaceDetectionObserver;
import com.volcengine.VolcApiEngine.BeanFactory;

public class ByteRTCEffectEventHandlerImpl implements BeanFactory.EventReceiver, IFaceDetectionObserver {
    public BeanFactory.EventEmitter ee;
    public ByteRTCEffectEventHandlerImpl(BeanFactory.EventEmitter ee) {
        this.ee = ee;
    }

    @Override
    public void onFaceDetectResult(FaceDetectionResult result) {
        this.ee.sendEvent("onFaceDetectResult", result);
    }

    @Override
    public void onExpressionDetectResult(ExpressionDetectResult result) {
        this.ee.sendEvent("onExpressionDetectResult", result);
    }
}
