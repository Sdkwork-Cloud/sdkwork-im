using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ResumeSessionRequest
    {
        public string? DeviceId { get; set; }
        public int? LastSeenSyncSeq { get; set; }
    }
}
