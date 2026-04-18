using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class RealtimeSubscriptionSnapshot
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? DeviceId { get; set; }
        public List<RealtimeSubscription>? Items { get; set; }
        public string? SyncedAt { get; set; }
    }
}
