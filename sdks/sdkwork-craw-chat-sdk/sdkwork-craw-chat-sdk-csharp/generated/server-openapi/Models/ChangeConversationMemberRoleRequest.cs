using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ChangeConversationMemberRoleRequest
    {
        public string? MemberId { get; set; }
        public string? Role { get; set; }
    }
}
