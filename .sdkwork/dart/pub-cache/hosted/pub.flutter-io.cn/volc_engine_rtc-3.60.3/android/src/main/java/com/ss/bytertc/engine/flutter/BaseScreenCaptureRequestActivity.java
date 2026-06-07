// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.media.projection.MediaProjectionManager;
import android.os.Build;
import android.os.Bundle;

import androidx.annotation.Nullable;
import androidx.annotation.RequiresApi;

import com.ss.bytertc.base.media.screen.RXScreenCaptureService;
import com.ss.bytertc.engine.RTCEngine;
import com.ss.bytertc.engine.data.ScreenMediaType;

import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public abstract class BaseScreenCaptureRequestActivity extends Activity {
    private static final int REQUEST_CODE_SCREEN_CAPTURE = 9679;
    static final String EXTRA_STREAM_TYPE = "type";

    @Override
    protected void onCreate(@Nullable Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        MediaProjectionManager mediaProjectionManager = null;
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.LOLLIPOP) {
            mediaProjectionManager = (MediaProjectionManager) getSystemService(Context.MEDIA_PROJECTION_SERVICE);
            Intent screenCaptureIntent = mediaProjectionManager.createScreenCaptureIntent();
            startActivityForResult(screenCaptureIntent, REQUEST_CODE_SCREEN_CAPTURE);
            return;
        }
        throw new RuntimeException("Android version is too low to support screen capture.");
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        if (requestCode == REQUEST_CODE_SCREEN_CAPTURE) {
            if (resultCode == Activity.RESULT_OK) {
                Intent intent = getIntent();
                ScreenMediaType type = (ScreenMediaType) intent.getSerializableExtra(EXTRA_STREAM_TYPE);

                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                    startRXScreenCaptureService(data);
                }

                RTCEngine engine = ByteRTCHelper.engineInstance;
                if (engine == null) {
                    throw new RuntimeException("[ScreenCapture] Engine is not initialized.");
                }

                if (type == ScreenMediaType.SCREEN_MEDIA_TYPE_AUDIO_ONLY) {
                    // back up.
                    ScheduledExecutorService scheduler =
                            Executors.newSingleThreadScheduledExecutor();
                    scheduler.schedule(() -> {
                        engine.startScreenCapture(type, data);
                    }, 500, TimeUnit.MILLISECONDS);
                } else {
                    engine.startScreenCapture(type, data);
                }

            } else {
                // TODO User rejected
                throw new RuntimeException("[ScreenCapture] User rejected screen capture.");
            }
            finish();
        } else {
            super.onActivityResult(requestCode, resultCode, data);
        }
    }

    @RequiresApi(api = Build.VERSION_CODES.O)
    private void startRXScreenCaptureService(Intent data) {
        final Intent intent = new Intent();
        intent.putExtra(RXScreenCaptureService.KEY_LARGE_ICON, getLargeIcon());
        intent.putExtra(RXScreenCaptureService.KEY_SMALL_ICON, getSmallIcon());
        intent.putExtra(RXScreenCaptureService.KEY_LAUNCH_ACTIVITY, getLaunchActivity().getCanonicalName());
        intent.putExtra(RXScreenCaptureService.KEY_CONTENT_TEXT, getContextText());
        intent.putExtra(RXScreenCaptureService.KEY_RESULT_DATA, data);
        startForegroundService(RXScreenCaptureService.getServiceIntent(this, RXScreenCaptureService.COMMAND_LAUNCH, intent));
    }

    /**
     * Android 10 及以上录屏通知使用
     *
     * @see RXScreenCaptureService#KEY_LARGE_ICON
     */
    public abstract int getLargeIcon();

    /**
     * Android 10 及以上录屏通知使用
     *
     * @see RXScreenCaptureService#KEY_SMALL_ICON
     */
    public abstract int getSmallIcon();

    /**
     * Android 10 及以上录屏通知使用
     *
     * @see RXScreenCaptureService#KEY_LAUNCH_ACTIVITY
     */
    public abstract Class<? extends Activity> getLaunchActivity();

    /**
     * Android 10 及以上录屏通知使用
     *
     * @see RXScreenCaptureService#KEY_CONTENT_TEXT
     */
    public abstract String getContextText();
}
