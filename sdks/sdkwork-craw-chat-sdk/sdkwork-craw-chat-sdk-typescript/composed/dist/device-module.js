export class CrawChatDevicesModule {
    context;
    constructor(context) {
        this.context = context;
    }
    register(body) {
        return this.context.backendClient.device.register(body);
    }
    getSyncFeed(deviceId, params) {
        return this.context.backendClient.device.getDeviceSyncFeed(deviceId, params);
    }
}
