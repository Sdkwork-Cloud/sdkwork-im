using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class RtcSession
    {
        public string? TenantId { get; set; }
        public string? RtcSessionId { get; set; }
        public string? ConversationId { get; set; }
        public string? ProviderPluginId { get; set; }
        public string? ProviderSessionId { get; set; }
        public string? RtcMode { get; set; }
        public string? State { get; set; }
        public string? CreatedAt { get; set; }
        public string? UpdatedAt { get; set; }
    }
}
