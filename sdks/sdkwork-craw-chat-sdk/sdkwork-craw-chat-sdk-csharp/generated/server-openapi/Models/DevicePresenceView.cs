using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class DevicePresenceView
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? DeviceId { get; set; }
        public string? Platform { get; set; }
        public string? SessionId { get; set; }
        public string? Status { get; set; }
        public int? LastSyncSeq { get; set; }
        public string? LastResumeAt { get; set; }
        public string? LastSeenAt { get; set; }
    }
}
