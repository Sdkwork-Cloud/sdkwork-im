using System;
using SDKwork.Common.Core;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;
using Sdkwork.Im.AppApi.Generated.Api;

namespace Sdkwork.Im.AppApi.Generated
{
    public class SdkworkAppClient
    {
        private readonly SdkHttpClient _httpClient;

        public AutomationApi Automation { get; }
        public DeviceApi Device { get; }
        public NotificationApi Notification { get; }
        public PortalApi Portal { get; }
        public ProviderApi Provider { get; }
        public IotApi Iot { get; }
        public RtcApi Rtc { get; }

        public SdkworkAppClient(string baseUrl)
        {
            _httpClient = new SdkHttpClient(baseUrl);
            Automation = new AutomationApi(_httpClient);
            Device = new DeviceApi(_httpClient);
            Notification = new NotificationApi(_httpClient);
            Portal = new PortalApi(_httpClient);
            Provider = new ProviderApi(_httpClient);
            Iot = new IotApi(_httpClient);
            Rtc = new RtcApi(_httpClient);
        }

        public SdkworkAppClient(SdkConfig config)
        {
            _httpClient = new SdkHttpClient(config);
            Automation = new AutomationApi(_httpClient);
            Device = new DeviceApi(_httpClient);
            Notification = new NotificationApi(_httpClient);
            Portal = new PortalApi(_httpClient);
            Provider = new ProviderApi(_httpClient);
            Iot = new IotApi(_httpClient);
            Rtc = new RtcApi(_httpClient);
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
