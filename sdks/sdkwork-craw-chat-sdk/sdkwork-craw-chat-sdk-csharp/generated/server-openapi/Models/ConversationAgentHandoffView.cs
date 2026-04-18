using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ConversationAgentHandoffView
    {
        public string? Status { get; set; }
        public ConversationActorView? Source { get; set; }
        public ConversationActorView? Target { get; set; }
        public string? HandoffSessionId { get; set; }
        public string? HandoffReason { get; set; }
        public string? AcceptedAt { get; set; }
        public ConversationActorView? AcceptedBy { get; set; }
        public string? ResolvedAt { get; set; }
        public ConversationActorView? ResolvedBy { get; set; }
        public string? ClosedAt { get; set; }
        public ConversationActorView? ClosedBy { get; set; }
    }
}
