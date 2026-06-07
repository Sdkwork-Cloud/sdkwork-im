// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import androidx.annotation.Nullable;
import androidx.annotation.RestrictTo;

import org.json.JSONException;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;

@RestrictTo(RestrictTo.Scope.LIBRARY)
public class ByteRTCTypes {

    private static final String TAG = "ByteRTCTypes";

    public final Object arguments;

    public final String logEx;

    public ByteRTCTypes(Object arguments) throws JSONException {
        this(arguments, null);
    }

    private Map<String, Object> jsonObjectToMap(org.json.JSONObject jsonObject) throws JSONException {
        Map<String, Object> map = new HashMap<>();
        Iterator<String> keys = jsonObject.keys();
        while (keys.hasNext()) {
            String key = keys.next();
            Object value = jsonObject.get(key);
            if (value instanceof org.json.JSONObject) {
                value = jsonObjectToMap((org.json.JSONObject) value);
            }
            if (value instanceof org.json.JSONArray) {
                value = jsonArrayToList((org.json.JSONArray) value);
            }
            map.put(key, value);
        }
        return map;
    }

    public ByteRTCTypes(Object arguments, String logEx) throws JSONException {
        if (arguments instanceof org.json.JSONObject) {
            this.arguments = jsonObjectToMap((org.json.JSONObject) arguments);
        } else if (arguments instanceof org.json.JSONArray) {
            this.arguments = jsonArrayToList((org.json.JSONArray) arguments);
        } else {
            this.arguments = arguments;
        }
        this.logEx = logEx;
    }

    public <T> T opt(String key, T defaultValue, Class<T> clazz) {
        if (arguments == null) {
            return defaultValue;
        }

        final Object value = ((Map<?, ?>) arguments).get(key);
        if (value == null) {
            return defaultValue;
        }

        if (clazz.isInstance(value)) {
            return clazz.cast(value);
        } else if (clazz == Float.class) { // Dart 中没有 Float 类型，使用 Double 适配
            if (value instanceof Double) {
                return clazz.cast(((Double) value).floatValue());
            } else if (value instanceof java.math.BigDecimal) {
                return clazz.cast(((java.math.BigDecimal) value).floatValue());
            }
        } else if (clazz == Long.class) { // int, if 32 bits not enough
            if (value instanceof Integer) {
                return clazz.cast(((Integer) value).longValue());
            } else if (value instanceof java.math.BigDecimal) {
                return clazz.cast(((java.math.BigDecimal) value).longValue());
            }
        } else if (clazz == Integer.class) { // int, if 32 bits not enough
            if (value instanceof Long) {
                return clazz.cast(((Long) value).intValue());
            } else if (value instanceof java.math.BigDecimal) {
                return clazz.cast(((java.math.BigDecimal) value).intValue());
            }
        } else if (clazz == Double.class) {
            if (value instanceof java.math.BigDecimal) {
                return clazz.cast(((java.math.BigDecimal) value).doubleValue());
            } else if (value instanceof Number) {
                return clazz.cast(((Number) value).doubleValue());
            }
        }

        throw new ClassCastException("Argument (" + key + "): Cannot cast " + value.getClass() + " to " + clazz);
    }

    public int optInt(String key) {
        return optInt(key, 0);
    }

    public int optInt(String key, int defaultValue) {
        return opt(key, defaultValue, Integer.class);
    }

    public double optDouble(String key) {
        return opt(key, 0.0, Double.class);
    }

    public String optString(String key, String defaultValue) {
        return opt(key, defaultValue, String.class);
    }

    public String optString(String key) {
        return optString(key, "");
    }

    public boolean optBoolean(String key) {
        return opt(key, Boolean.FALSE, Boolean.class);
    }

    public boolean optBoolean(String key, boolean defValue) {
        return opt(key, defValue, Boolean.class);
    }

    public ByteRTCTypes optBox(String key) throws JSONException {
        if (arguments == null) {
            return new ByteRTCTypes(null, logEx);
        }
        return new ByteRTCTypes(opt(key, null, Map.class), logEx);
    }

    @SuppressWarnings("unchecked")
    public <T> List<T> getList(String key) {
        return opt(key, Collections.emptyList(), List.class);
    }

    public JSONObject optJSONObject(String key) {
        return new JSONObject(opt(key, Collections.emptyMap(), Map.class));
    }

    public List<Object> jsonArrayToList(org.json.JSONArray array) throws JSONException {
        List<Object> list = new ArrayList<>();
        for (int i = 0; i < array.length(); i++) {
            Object value = array.get(i);
            if (value instanceof org.json.JSONObject) {
                value = jsonObjectToMap((org.json.JSONObject) value); // 递归转 Map
            } else if (value instanceof org.json.JSONArray) {
                value = jsonArrayToList((org.json.JSONArray) value); // 递归转 List
            }
            list.add(value);
        }
        return list;
    }

    @Nullable
    public JSONObject optNullJSONObject(String key) {
        Map<?, ?> opt = opt(key, null, Map.class);
        if (opt == null) {
            return null;
        }
        return new JSONObject(opt);
    }

    public byte[] optBytes(String key) {
        return opt(key, new byte[0], byte[].class);
    }

    public byte[] optBytes(String key, byte[] defaultValue) {
        return opt(key, defaultValue, byte[].class);
    }

    public long optLong(String key) {
        return opt(key, 0L, Long.class);
    }

    /**
     * 中没有 Float 类型，所以使用 Double 承接数据，转换为 Float
     */
    public float optFloat(String key) {
        return opt(key, 0F, Float.class);
    }
}
