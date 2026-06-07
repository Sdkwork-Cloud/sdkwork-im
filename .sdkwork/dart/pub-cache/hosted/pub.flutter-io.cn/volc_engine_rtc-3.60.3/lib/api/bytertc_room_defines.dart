/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;
export '../codegen/pack/index.dart'
    show
        SubscribeStateChangeReason,
        AVSyncEvent,
        PublishState,
        PublishStateChangeReason;

/// @brief 房间配置
class RoomConfig extends $p.RoomConfig {
  RoomConfig({
    $p.RoomProfile? profile,
    String? streamId,
    bool? isPublishAudio,
    bool? isPublishVideo,
    bool? isAutoSubscribeAudio,
    bool? isAutoSubscribeVideo,
  }) : super(
          profile: profile ?? $p.RoomProfile.communication,
          streamId: streamId ?? '',
          isPublishAudio: isPublishAudio ?? true,
          isPublishVideo: isPublishVideo ?? true,
          isAutoSubscribeAudio: isAutoSubscribeAudio ?? true,
          isAutoSubscribeVideo: isAutoSubscribeVideo ?? true,
        );
}
