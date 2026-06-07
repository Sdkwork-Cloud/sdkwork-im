// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;
import android.content.Context;
import androidx.annotation.NonNull;
import com.volcengine.VolcApiEngine.runtime.*;

public class NativeVariableManager {

  static void init(@NonNull MessageClient msgClient, Context context) {
    com.volcengine.VolcApiEngine.runtime.NativeVariableManager m = msgClient.proto.variableManager;
    m.registerVar("ApplicationContext",
            (Object[] args) -> context.getApplicationContext());
  }
}
