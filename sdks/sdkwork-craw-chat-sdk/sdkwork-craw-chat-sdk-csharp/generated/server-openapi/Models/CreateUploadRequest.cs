using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CreateUploadRequest
    {
        public string? MediaAssetId { get; set; }
        public MediaResource? Resource { get; set; }
    }
}
