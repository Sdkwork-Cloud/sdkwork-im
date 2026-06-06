import 'package:im_app_api_generated/im_app_api_generated.dart';

import 'context.dart';

class ImAppNotificationModule {
  final ImAppSdkContext context;

  ImAppNotificationModule(this.context);

  Future<NotificationListResponse?> list() {
    return context.transportClient.notification.notificationsList();
  }

  Future<NotificationRequestResponse?> request(RequestNotification body) {
    return context.transportClient.notification.notificationsRequestsCreate(
      body,
    );
  }

  Future<NotificationTask?> get(String notificationId) {
    return context.transportClient.notification.notificationsRetrieve(
      notificationId,
    );
  }
}
