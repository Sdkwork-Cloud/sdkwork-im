using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class AttachMediaRequest
    {
        public string? ConversationId { get; set; }
        public string? ClientMsgId { get; set; }
        public string? Summary { get; set; }
        public string? Text { get; set; }
        public Dictionary<string, string>? RenderHints { get; set; }
    }
}
