using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationInboxEntry
    {
        public string? TenantId { get; set; }
        public string? ConversationId { get; set; }
        public bool? AgentHandoff { get; set; }
        public string? ConversationType { get; set; }
        public string? LastActivityAt { get; set; }
        public string? LastMessageId { get; set; }
        public string? LastSenderId { get; set; }
        public int? MessageCount { get; set; }
        public int? LastMessageSeq { get; set; }
        public string? LastSummary { get; set; }
        public string? LastMessageAt { get; set; }
        public int? UnreadCount { get; set; }
    }
}
