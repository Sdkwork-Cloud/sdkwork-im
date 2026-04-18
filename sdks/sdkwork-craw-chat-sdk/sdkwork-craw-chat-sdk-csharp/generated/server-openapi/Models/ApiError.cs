using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class ApiError
    {
        public string? Code { get; set; }
        public string? Message { get; set; }
    }
}
