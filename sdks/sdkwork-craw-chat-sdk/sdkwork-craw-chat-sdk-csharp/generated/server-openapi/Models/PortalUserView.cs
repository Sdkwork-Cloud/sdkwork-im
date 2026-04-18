using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class PortalUserView
    {
        public string? Id { get; set; }
        public string? Login { get; set; }
        public string? Name { get; set; }
        public string? Role { get; set; }
        public string? Email { get; set; }
        public string? ActorKind { get; set; }
        public string? ClientKind { get; set; }
        public List<string>? Permissions { get; set; }
    }
}
