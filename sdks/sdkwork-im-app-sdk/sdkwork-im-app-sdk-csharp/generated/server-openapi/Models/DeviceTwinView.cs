using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class DeviceTwinView
    {
        public string? TenantId { get; set; }
        public string? DeviceId { get; set; }
        public string? DesiredStateJson { get; set; }
        public string? ReportedStateJson { get; set; }
        public string? UpdatedAt { get; set; }
    }
}
