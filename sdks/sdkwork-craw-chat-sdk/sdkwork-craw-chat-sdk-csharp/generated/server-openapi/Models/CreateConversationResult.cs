using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CreateConversationResult
    {
        public string? ConversationId { get; set; }
        public string? EventId { get; set; }
    }
}
