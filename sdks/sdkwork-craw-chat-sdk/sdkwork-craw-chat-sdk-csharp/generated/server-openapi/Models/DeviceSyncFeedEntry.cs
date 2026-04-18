using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class DeviceSyncFeedEntry
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? DeviceId { get; set; }
        public int? SyncSeq { get; set; }
        public string? OriginEventId { get; set; }
        public string? OriginEventType { get; set; }
        public string? ConversationId { get; set; }
        public string? MessageId { get; set; }
        public int? MessageSeq { get; set; }
        public string? MemberId { get; set; }
        public int? ReadSeq { get; set; }
        public string? LastReadMessageId { get; set; }
        public string? ActorId { get; set; }
        public string? ActorKind { get; set; }
        public string? ActorDeviceId { get; set; }
        public string? Summary { get; set; }
        public string? PayloadSchema { get; set; }
        public string? Payload { get; set; }
        public string? OccurredAt { get; set; }
    }
}
