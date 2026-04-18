using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class RtcSignalEvent
    {
        public string? TenantId { get; set; }
        public string? RtcSessionId { get; set; }
        public string? ConversationId { get; set; }
        public string? RtcMode { get; set; }
        public string? SignalType { get; set; }
        public string? SchemaRef { get; set; }
        public string? Payload { get; set; }
        public Sender? Sender { get; set; }
        public string? SignalingStreamId { get; set; }
        public string? OccurredAt { get; set; }
    }
}
