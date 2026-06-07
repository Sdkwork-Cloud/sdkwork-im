using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class DeviceSyncFeedEntry
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? PrincipalKind { get; set; }
        public string? DeviceId { get; set; }
        public int? SyncSeq { get; set; }
        public string? EventId { get; set; }
        public string? OriginEventType { get; set; }
        public string? ActorId { get; set; }
        public string? ConversationId { get; set; }
        public string? MessageId { get; set; }
        public int? MessageSeq { get; set; }
        public string? Payload { get; set; }
        public int? ReadSeq { get; set; }
        public string? Summary { get; set; }
        public string? OccurredAt { get; set; }
    }
}
