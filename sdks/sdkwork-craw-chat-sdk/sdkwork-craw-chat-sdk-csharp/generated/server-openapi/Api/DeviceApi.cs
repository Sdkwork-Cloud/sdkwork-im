using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class DeviceApi
    {
        private readonly SdkHttpClient _client;

        public DeviceApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Register the current device
        /// </summary>
        public async Task<RegisteredDeviceView?> RegisterAsync(RegisterDeviceRequest body)
        {
            return await _client.PostAsync<RegisteredDeviceView>(ApiPaths.BackendPath("/devices/register"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get device sync feed entries
        /// </summary>
        public async Task<DeviceSyncFeedResponse?> GetDeviceSyncFeedAsync(string deviceId, Dictionary<string, object>? query = null)
        {
            return await _client.GetAsync<DeviceSyncFeedResponse>(ApiPaths.BackendPath($"/devices/{deviceId}/sync-feed"), query);
        }
    }
}
