using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class RealtimeEvent
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? DeviceId { get; set; }
        public int? RealtimeSeq { get; set; }
        public string? ScopeType { get; set; }
        public string? ScopeId { get; set; }
        public string? EventType { get; set; }
        public string? DeliveryClass { get; set; }
        public string? Payload { get; set; }
        public string? OccurredAt { get; set; }
    }
}
