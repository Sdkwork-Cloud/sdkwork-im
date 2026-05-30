using System;
using SDKwork.Common.Core;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;
using Sdkwork.Im.AppApi.Generated.Api;

namespace Sdkwork.Im.AppApi.Generated
{
    public class SdkworkAppClient
    {
        private readonly SdkHttpClient _httpClient;

        public PortalApi Portal { get; }
        public DeviceApi Device { get; }
        public PresenceApi Presence { get; }
        public RealtimeApi Realtime { get; }
        public SocialApi Social { get; }
        public ChatApi Chat { get; }
        public MediaApi Media { get; }
        public StreamApi Stream { get; }
        public RtcApi Rtc { get; }
        public NotificationApi Notification { get; }
        public AutomationApi Automation { get; }

        public SdkworkAppClient(string baseUrl)
        {
            _httpClient = new SdkHttpClient(baseUrl);
            Portal = new PortalApi(_httpClient);
            Device = new DeviceApi(_httpClient);
            Presence = new PresenceApi(_httpClient);
            Realtime = new RealtimeApi(_httpClient);
            Social = new SocialApi(_httpClient);
            Chat = new ChatApi(_httpClient);
            Media = new MediaApi(_httpClient);
            Stream = new StreamApi(_httpClient);
            Rtc = new RtcApi(_httpClient);
            Notification = new NotificationApi(_httpClient);
            Automation = new AutomationApi(_httpClient);
        }

        public SdkworkAppClient(SdkConfig config)
        {
            _httpClient = new SdkHttpClient(config);
            Portal = new PortalApi(_httpClient);
            Device = new DeviceApi(_httpClient);
            Presence = new PresenceApi(_httpClient);
            Realtime = new RealtimeApi(_httpClient);
            Social = new SocialApi(_httpClient);
            Chat = new ChatApi(_httpClient);
            Media = new MediaApi(_httpClient);
            Stream = new StreamApi(_httpClient);
            Rtc = new RtcApi(_httpClient);
            Notification = new NotificationApi(_httpClient);
            Automation = new AutomationApi(_httpClient);
        }


        public SdkworkAppClient SetAuthToken(string token)
        {
            _httpClient.SetAuthToken(token);
            return this;
        }

        public SdkworkAppClient SetAccessToken(string token)
        {
            _httpClient.SetAccessToken(token);
            return this;
        }

        public SdkworkAppClient SetHeader(string key, string value)
        {
            _httpClient.SetHeader(key, value);
            return this;
        }
    }
}
