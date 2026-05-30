import type { HttpClient } from '../http/client';
import type { NotificationListResponse, NotificationRequestResponse, NotificationTask, RequestNotification } from '../types';
export declare class NotificationRequestsApi {
    private client;
    constructor(client: HttpClient);
    /** Request a notification task */
    create(body: RequestNotification): Promise<NotificationRequestResponse>;
}
export declare class NotificationApi {
    private client;
    readonly requests: NotificationRequestsApi;
    constructor(client: HttpClient);
    /** List notifications for the current principal */
    list(): Promise<NotificationListResponse>;
    /** Get a notification task */
    retrieve(notificationId: string): Promise<NotificationTask>;
}
export declare function createNotificationApi(client: HttpClient): NotificationApi;
//# sourceMappingURL=notification.d.ts.map