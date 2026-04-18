using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class PortalLoginRequest
    {
        public string? TenantId { get; set; }
        public string? Login { get; set; }
        public string? Password { get; set; }
        public string? DeviceId { get; set; }
        public string? SessionId { get; set; }
        public string? ClientKind { get; set; }
    }
}
