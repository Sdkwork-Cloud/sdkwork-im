using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class TimelineViewEntry
    {
        public string? TenantId { get; set; }
        public string? ConversationId { get; set; }
        public string? MessageId { get; set; }
        public int? MessageSeq { get; set; }
        public string? Summary { get; set; }
    }
}
