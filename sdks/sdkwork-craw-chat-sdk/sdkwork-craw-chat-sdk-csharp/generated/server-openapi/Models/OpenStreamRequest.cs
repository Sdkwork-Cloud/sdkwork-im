using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class OpenStreamRequest
    {
        public string? StreamId { get; set; }
        public string? StreamType { get; set; }
        public string? ScopeKind { get; set; }
        public string? ScopeId { get; set; }
        public string? DurabilityClass { get; set; }
        public string? SchemaRef { get; set; }
    }
}
