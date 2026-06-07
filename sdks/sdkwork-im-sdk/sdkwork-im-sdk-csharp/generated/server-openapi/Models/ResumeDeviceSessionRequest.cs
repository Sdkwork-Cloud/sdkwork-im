using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ResumeDeviceSessionRequest
    {
        public string? DeviceId { get; set; }
        public int? LastSeenSyncSeq { get; set; }
    }
}
