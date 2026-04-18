using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class PortalMeResponse
    {
        public string? TenantId { get; set; }
        public PortalUserView? User { get; set; }
        public PortalWorkspaceView? Workspace { get; set; }
    }
}
