using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class MediaResource
    {
        public int? Id { get; set; }
        public string? Uuid { get; set; }
        public string? Url { get; set; }
        public List<int>? Bytes { get; set; }
        public string? LocalFile { get; set; }
        public string? Base64 { get; set; }
        public string? Type { get; set; }
        public string? MimeType { get; set; }
        public int? Size { get; set; }
        public string? Name { get; set; }
        public string? Extension { get; set; }
        public Dictionary<string, string>? Tags { get; set; }
        public Dictionary<string, string>? Metadata { get; set; }
        public string? Prompt { get; set; }
    }
}
