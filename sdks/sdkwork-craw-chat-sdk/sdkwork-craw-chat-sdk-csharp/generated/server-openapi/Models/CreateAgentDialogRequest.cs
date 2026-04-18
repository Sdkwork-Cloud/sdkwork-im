using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CreateAgentDialogRequest
    {
        public string? ConversationId { get; set; }
        public string? AgentId { get; set; }
    }
}
