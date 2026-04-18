using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ContentPart
    {
        public string? Kind { get; set; }
        public string? Text { get; set; }
        public string? SchemaRef { get; set; }
        public string? Encoding { get; set; }
        public string? Payload { get; set; }
        public string? MediaAssetId { get; set; }
        public MediaResource? Resource { get; set; }
        public string? SignalType { get; set; }
        public string? StreamId { get; set; }
        public string? StreamType { get; set; }
        public string? State { get; set; }
    }
}
