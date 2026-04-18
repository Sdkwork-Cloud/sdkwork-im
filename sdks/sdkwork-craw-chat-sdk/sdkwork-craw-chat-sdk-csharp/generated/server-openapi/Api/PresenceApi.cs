using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class PresenceApi
    {
        private readonly SdkHttpClient _client;

        public PresenceApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Refresh device presence
        /// </summary>
        public async Task<PresenceSnapshotView?> HeartbeatAsync(PresenceDeviceRequest body)
        {
            return await _client.PostAsync<PresenceSnapshotView>(ApiPaths.BackendPath("/presence/heartbeat"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get current presence
        /// </summary>
        public async Task<PresenceSnapshotView?> GetPresenceMeAsync()
        {
            return await _client.GetAsync<PresenceSnapshotView>(ApiPaths.BackendPath("/presence/me"));
        }
    }
}
