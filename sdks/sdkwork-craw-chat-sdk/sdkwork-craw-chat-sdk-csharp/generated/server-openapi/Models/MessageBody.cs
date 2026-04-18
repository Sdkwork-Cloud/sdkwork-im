using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class MessageBody
    {
        public string? Summary { get; set; }
        public List<ContentPart>? Parts { get; set; }
        public Dictionary<string, string>? RenderHints { get; set; }
    }
}
