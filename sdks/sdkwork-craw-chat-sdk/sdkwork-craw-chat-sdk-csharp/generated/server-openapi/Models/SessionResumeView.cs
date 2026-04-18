using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class SessionResumeView
    {
        public string? TenantId { get; set; }
        public string? ActorId { get; set; }
        public string? ActorKind { get; set; }
        public string? SessionId { get; set; }
        public string? DeviceId { get; set; }
        public bool? ResumeRequired { get; set; }
        public int? ResumeFromSyncSeq { get; set; }
        public int? LatestSyncSeq { get; set; }
        public string? ResumedAt { get; set; }
        public PresenceSnapshotView? Presence { get; set; }
    }
}
