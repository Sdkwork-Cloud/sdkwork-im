using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ConversationReadCursorView
    {
        public string? TenantId { get; set; }
        public string? ConversationId { get; set; }
        public string? MemberId { get; set; }
        public string? PrincipalId { get; set; }
        public int? ReadSeq { get; set; }
        public string? LastReadMessageId { get; set; }
        public string? UpdatedAt { get; set; }
        public int? UnreadCount { get; set; }
    }
}
