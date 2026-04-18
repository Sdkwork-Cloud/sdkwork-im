using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class InboxResponse
    {
        public List<ConversationInboxEntry>? Items { get; set; }
    }
}
