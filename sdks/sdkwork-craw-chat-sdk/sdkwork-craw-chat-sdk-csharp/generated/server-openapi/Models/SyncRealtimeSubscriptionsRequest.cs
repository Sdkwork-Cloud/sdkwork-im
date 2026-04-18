using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class SyncRealtimeSubscriptionsRequest
    {
        public string? DeviceId { get; set; }
        public List<RealtimeSubscriptionItemInput>? Items { get; set; }
    }
}
