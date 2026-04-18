using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class RemoveConversationMemberRequest
    {
        public string? MemberId { get; set; }
    }
}
