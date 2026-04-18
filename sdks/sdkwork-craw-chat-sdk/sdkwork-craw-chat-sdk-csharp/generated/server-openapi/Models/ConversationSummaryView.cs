using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ConversationSummaryView
    {
        public string? TenantId { get; set; }
        public string? ConversationId { get; set; }
        public int? MessageCount { get; set; }
        public string? LastMessageId { get; set; }
        public int? LastMessageSeq { get; set; }
        public string? LastSenderId { get; set; }
        public string? LastSenderKind { get; set; }
        public SummarySenderView? LastSender { get; set; }
        public string? LastSummary { get; set; }
        public string? LastMessageAt { get; set; }
        public ConversationAgentHandoffView? AgentHandoff { get; set; }
    }
}
