using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class Sender
    {
        public string? Id { get; set; }
        public string? Kind { get; set; }
        public string? MemberId { get; set; }
        public string? DeviceId { get; set; }
        public string? SessionId { get; set; }
        public Dictionary<string, string>? Metadata { get; set; }
    }
}
