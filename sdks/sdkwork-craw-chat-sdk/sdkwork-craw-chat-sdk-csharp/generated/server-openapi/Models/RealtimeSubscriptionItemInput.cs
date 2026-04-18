using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class RealtimeSubscriptionItemInput
    {
        public string? ScopeType { get; set; }
        public string? ScopeId { get; set; }
        public List<string>? EventTypes { get; set; }
    }
}
