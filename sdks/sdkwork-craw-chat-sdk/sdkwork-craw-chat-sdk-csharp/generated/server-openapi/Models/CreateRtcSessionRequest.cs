using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CreateRtcSessionRequest
    {
        public string? RtcSessionId { get; set; }
        public string? ConversationId { get; set; }
        public string? RtcMode { get; set; }
    }
}
