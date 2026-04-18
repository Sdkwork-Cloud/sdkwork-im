using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class SessionApi
    {
        private readonly SdkHttpClient _client;

        public SessionApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Resume the current app session
        /// </summary>
        public async Task<SessionResumeView?> ResumeAsync(ResumeSessionRequest body)
        {
            return await _client.PostAsync<SessionResumeView>(ApiPaths.BackendPath("/sessions/resume"), body, null, null, "application/json");
        }

        /// <summary>
        /// Disconnect the current app session device route
        /// </summary>
        public async Task<PresenceSnapshotView?> DisconnectAsync(PresenceDeviceRequest body)
        {
            return await _client.PostAsync<PresenceSnapshotView>(ApiPaths.BackendPath("/sessions/disconnect"), body, null, null, "application/json");
        }
    }
}
