// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import android.util.Base64;

import org.json.JSONArray;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.List;

public class BasicTransformer {
    public static String getString(JSONObject source, String key, String defaultValue) {
        if (source == null) return defaultValue;
        return source.optString(key, defaultValue);
    }

    public static int getInt(JSONObject source, String key, int defaultValue) {
        if (source == null) return defaultValue;
        return source.optInt(key, defaultValue);
    }

    public static double getDouble(JSONObject source, String key, double defaultValue) {
        if (source == null) return defaultValue;
        return source.optDouble(key, defaultValue);
    }

    public static boolean getBoolean(JSONObject source, String key, boolean defaultValue) {
        if (source == null) return defaultValue;
        return source.optBoolean(key, defaultValue);
    }

    public static float getFloat(JSONObject source, String key, float defaultValue) {
        if (source == null) return defaultValue;
        return (float) source.optDouble(key, defaultValue);
    }

    public static byte[] getBytes(JSONObject source, String key, byte[] defaultValue) {
        if (source == null) return defaultValue;
        String base64String = source.optString(key);
        if (base64String.isEmpty()) {
            return null;
        }
        return Base64.decode(base64String, Base64.DEFAULT);
    }

    public static <T extends Enum<T>> T getEnumByIndex(JSONObject source, Class<T> clazz, String key) {
        try {
            int index = getInt(source, key, -1);
            T[] constants = clazz.getEnumConstants();
            if (constants != null && index >= 0 && index < constants.length) {
                return constants[index];
            }
        } catch (Exception error)  {
            return null;
        }
        return null;
    }

    public static <T> List<T> getList(JSONObject source, String key, List<T> defaultValue) {
        // This is tricky with generics and JSONObject. 
        // We might need to return JSONArray and let caller handle it, 
        // or iterate if we know the type.
        // For now, let's return a List if the underlying object is a JSONArray
        try {
            JSONArray jsonArray = source.optJSONArray(key);
            if (jsonArray != null) {
                List<T> list = new ArrayList<>();
                for (int i = 0; i < jsonArray.length(); i++) {
                    list.add((T) jsonArray.get(i));
                }
                return list;
            }
        } catch (Exception ignored) {
        }
        return defaultValue;
    }
    
    public static JSONArray getJSONArray(JSONObject source, String key) {
        if (source == null) return null;
        return source.optJSONArray(key);
    }
    
    public static JSONObject getJSONObject(JSONObject source, String key) {
        if (source == null) return null;
        return source.optJSONObject(key);
    }
}
