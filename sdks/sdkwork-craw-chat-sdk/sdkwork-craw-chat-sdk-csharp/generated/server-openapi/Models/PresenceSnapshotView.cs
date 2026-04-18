using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class PresenceSnapshotView
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? CurrentDeviceId { get; set; }
        public List<DevicePresenceView>? Devices { get; set; }
    }
}
