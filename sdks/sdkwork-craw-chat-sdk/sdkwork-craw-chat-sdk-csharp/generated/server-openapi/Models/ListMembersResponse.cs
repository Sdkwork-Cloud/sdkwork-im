using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ListMembersResponse
    {
        public List<ConversationMember>? Items { get; set; }
    }
}
