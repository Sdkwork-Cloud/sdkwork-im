using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ConversationInboxEntry
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? MemberId { get; set; }
        public string? ConversationId { get; set; }
        public string? ConversationType { get; set; }
        public int? MessageCount { get; set; }
        public string? LastMessageId { get; set; }
        public int? LastMessageSeq { get; set; }
        public string? LastSenderId { get; set; }
        public string? LastSenderKind { get; set; }
        public string? LastSummary { get; set; }
        public int? UnreadCount { get; set; }
        public string? LastActivityAt { get; set; }
        public ConversationAgentHandoffView? AgentHandoff { get; set; }
    }
}
