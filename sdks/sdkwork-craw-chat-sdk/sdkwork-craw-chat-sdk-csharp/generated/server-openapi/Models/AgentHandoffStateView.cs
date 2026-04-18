using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class AgentHandoffStateView
    {
        public string? TenantId { get; set; }
        public string? ConversationId { get; set; }
        public string? Status { get; set; }
        public ChangeAgentHandoffStatusView? Source { get; set; }
        public ChangeAgentHandoffStatusView? Target { get; set; }
        public string? HandoffSessionId { get; set; }
        public string? HandoffReason { get; set; }
        public string? AcceptedAt { get; set; }
        public ChangeAgentHandoffStatusView? AcceptedBy { get; set; }
        public string? ResolvedAt { get; set; }
        public ChangeAgentHandoffStatusView? ResolvedBy { get; set; }
        public string? ClosedAt { get; set; }
        public ChangeAgentHandoffStatusView? ClosedBy { get; set; }
    }
}
