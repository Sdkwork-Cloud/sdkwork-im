using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class DeviceSessionView
    {
        public string? TenantId { get; set; }
        public string? PrincipalId { get; set; }
        public string? PrincipalKind { get; set; }
        public string? DeviceId { get; set; }
        public string? ResumedAt { get; set; }
    }
}
