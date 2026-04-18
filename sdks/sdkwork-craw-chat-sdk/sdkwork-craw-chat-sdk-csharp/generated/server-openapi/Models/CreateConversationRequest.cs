using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CreateConversationRequest
    {
        public string? ConversationId { get; set; }
        public string? ConversationType { get; set; }
    }
}
