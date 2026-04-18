using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ConversationMember
    {
        public string? TenantId { get; set; }
        public string? ConversationId { get; set; }
        public string? MemberId { get; set; }
        public string? PrincipalId { get; set; }
        public string? PrincipalKind { get; set; }
        public string? Role { get; set; }
        public string? State { get; set; }
        public string? InvitedBy { get; set; }
        public string? JoinedAt { get; set; }
        public string? RemovedAt { get; set; }
        public Dictionary<string, string>? Attributes { get; set; }
    }
}
