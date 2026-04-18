using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class RealtimeAckState
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? DeviceId { get; set; }
        public int? AckedThroughSeq { get; set; }
        public int? TrimmedThroughSeq { get; set; }
        public int? RetainedEventCount { get; set; }
        public string? AckedAt { get; set; }
    }
}
