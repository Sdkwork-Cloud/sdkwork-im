using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class MediaAsset
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? PrincipalKind { get; set; }
        public string? MediaAssetId { get; set; }
        public string? Bucket { get; set; }
        public string? ObjectKey { get; set; }
        public string? StorageProvider { get; set; }
        public string? Checksum { get; set; }
        public string? ProcessingState { get; set; }
        public MediaResource? Resource { get; set; }
        public string? CreatedAt { get; set; }
        public string? CompletedAt { get; set; }
    }
}
