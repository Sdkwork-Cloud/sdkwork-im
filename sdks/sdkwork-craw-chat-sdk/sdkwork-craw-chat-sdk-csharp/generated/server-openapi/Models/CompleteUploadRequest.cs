using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CompleteUploadRequest
    {
        public string? Bucket { get; set; }
        public string? ObjectKey { get; set; }
        public string? StorageProvider { get; set; }
        public string? Url { get; set; }
        public string? Checksum { get; set; }
    }
}
