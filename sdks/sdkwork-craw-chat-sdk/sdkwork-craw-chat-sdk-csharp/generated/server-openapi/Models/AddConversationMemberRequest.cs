using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class AddConversationMemberRequest
    {
        public string? PrincipalId { get; set; }
        public string? PrincipalKind { get; set; }
        public string? Role { get; set; }
    }
}
