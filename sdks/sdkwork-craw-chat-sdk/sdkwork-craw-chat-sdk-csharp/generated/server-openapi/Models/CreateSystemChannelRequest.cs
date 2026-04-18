using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CreateSystemChannelRequest
    {
        public string? ConversationId { get; set; }
        public string? SubscriberId { get; set; }
    }
}
