using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class MediaDownloadUrlResponse
    {
        public string? MediaAssetId { get; set; }
        public string? StorageProvider { get; set; }
        public string? DownloadUrl { get; set; }
        public int? ExpiresInSeconds { get; set; }
    }
}
