using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class PortalLoginResponse
    {
        public string? AccessToken { get; set; }
        public string? RefreshToken { get; set; }
        public int? ExpiresAt { get; set; }
        public PortalUserView? User { get; set; }
        public PortalWorkspaceView? Workspace { get; set; }
    }
}
