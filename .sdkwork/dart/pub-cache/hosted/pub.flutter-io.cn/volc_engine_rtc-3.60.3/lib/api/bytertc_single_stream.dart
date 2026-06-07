/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;

/// @brief 单流推流参数
class PushSingleStreamParam extends $p.PushSingleStreamParam {
  PushSingleStreamParam({
    required super.roomId,
    required super.userId,
    String url = '',
    List<$p.DestInfo>? destInfos,
    $p.SingleStreamPushType pushType = $p.SingleStreamPushType.cdn,
  }) : super(
          isScreen: false,
          destInfos: destInfos ?? [],
          url: url,
          pushType: pushType,
        );

  PushSingleStreamParam.toCDN({
    required String roomId,
    required String userId,
    required String url,
  }) : this(
          roomId: roomId,
          userId: userId,
          url: url,
          pushType: $p.SingleStreamPushType.cdn,
        );

  PushSingleStreamParam.toRTCRoom({
    required String roomId,
    required String userId,
    required List<$p.DestInfo> destInfos,
  }) : this(
          roomId: roomId,
          userId: userId,
          destInfos: destInfos,
          pushType: $p.SingleStreamPushType.rtc,
        );
}
