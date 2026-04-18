using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class PortalApi
    {
        private readonly SdkHttpClient _client;

        public PortalApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Read the tenant portal home snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetHomeAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/home"));
        }

        /// <summary>
        /// Read the tenant portal sign-in snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetAuthAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/auth"));
        }

        /// <summary>
        /// Read the current tenant workspace snapshot
        /// </summary>
        public async Task<PortalWorkspaceView?> GetWorkspaceAsync()
        {
            return await _client.GetAsync<PortalWorkspaceView>(ApiPaths.BackendPath("/portal/workspace"));
        }

        /// <summary>
        /// Read the tenant dashboard snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetDashboardAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/dashboard"));
        }

        /// <summary>
        /// Read the tenant conversations snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetConversationsAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/conversations"));
        }

        /// <summary>
        /// Read the tenant realtime snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetRealtimeAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/realtime"));
        }

        /// <summary>
        /// Read the tenant media snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetMediaAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/media"));
        }

        /// <summary>
        /// Read the tenant automation snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetAutomationAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/automation"));
        }

        /// <summary>
        /// Read the tenant governance snapshot
        /// </summary>
        public async Task<Dictionary<string, object>?> GetGovernanceAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.BackendPath("/portal/governance"));
        }
    }
}
