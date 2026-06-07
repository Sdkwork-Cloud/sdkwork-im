// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import androidx.annotation.NonNull;

import com.ss.bytertc.engine.VideoEncoderConfig;
import com.ss.bytertc.engine.data.HumanOrientation;
import com.ss.bytertc.engine.data.Orientation;
import com.ss.bytertc.engine.data.Position;
import com.ss.bytertc.engine.live.MixedStreamConfig;
import com.ss.bytertc.engine.video.ByteWatermark;
import com.ss.bytertc.engine.video.RTCWatermarkConfig;
import com.ss.bytertc.engine.video.VideoCaptureConfig;

import org.json.JSONException;

import java.util.List;

public class ByteRTCTypeHelper {

    public static HumanOrientation toHumanOrientation(ByteRTCTypes value) throws JSONException {
        return new HumanOrientation(
                toByteOrientation(value.optBox("forward")),
                toByteOrientation(value.optBox("right")),
                toByteOrientation(value.optBox("up"))
        );
    }

    public static Orientation toByteOrientation(ByteRTCTypes value) {
        return new Orientation(
                value.optFloat("x"),
                value.optFloat("y"),
                value.optFloat("z")
        );
    }

    public static Position toBytePosition(ByteRTCTypes value) {
        return new Position(
                value.optFloat("x"),
                value.optFloat("y"),
                value.optFloat("z")
        );
    }

    public static RTCWatermarkConfig toRTCWatermarkConfig(ByteRTCTypes value) throws JSONException {
        return new RTCWatermarkConfig(
                value.optBoolean("visibleInPreview"),
                toByteWatermark(value.optBox("positionInLandscapeMode")),
                toByteWatermark(value.optBox("positionInPortraitMode"))

        );
    }

    private static ByteWatermark toByteWatermark(ByteRTCTypes value) {
        return new ByteWatermark(
                value.optFloat("x"),
                value.optFloat("y"),
                value.optFloat("width"),
                value.optFloat("height")
        );
    }

    public static VideoCaptureConfig toVideoCaptureConfig(ByteRTCTypes obj) {
        VideoCaptureConfig frameRate = new VideoCaptureConfig(
                obj.optInt("width"),
                obj.optInt("height"),
                obj.optInt("frameRate")
        );
        frameRate.capturePreference = VideoCaptureConfig.CapturePreference
                .convertFromInt(obj.optInt("capturePreference"));
        return frameRate;
    }

    @NonNull
    public static VideoEncoderConfig.EncoderPreference toEncoderPreference(int value) {
        for (VideoEncoderConfig.EncoderPreference preference : VideoEncoderConfig.EncoderPreference.values()) {
            if (preference.getValue() == value) {
                return preference;
            }
        }

        throw new IllegalArgumentException("Unknown VideoEncoderConfig.EncoderPreference value: " + value);
    }

    public static VideoEncoderConfig toVideoEncoderConfig(ByteRTCTypes value) {
        VideoEncoderConfig config = new VideoEncoderConfig(
                value.optInt("width"),
                value.optInt("height"),
                value.optInt("frameRate"),
                value.optInt("maxBitrate"),
                value.optInt("minBitrate"));
        int i = value.optInt("encoderPreference");
//        config.encodePreference = toEncoderPreference(value.optInt("encoderPreference"));
        config.encodePreference = toEncoderPreference(i);
        return config;
    }

    public static VideoEncoderConfig[] toVideoEncoderConfigArray(List<?> values) throws JSONException {
        if (values == null) {
            return new VideoEncoderConfig[0];
        }
        VideoEncoderConfig[] retValue = new VideoEncoderConfig[values.size()];

        for (int i = 0; i < values.size(); i++) {
            retValue[i] = toVideoEncoderConfig(new ByteRTCTypes(values.get(i)));
        }

        return retValue;
    }
}
