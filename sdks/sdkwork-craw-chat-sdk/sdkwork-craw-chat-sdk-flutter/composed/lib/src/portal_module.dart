import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';

class CrawChatPortalModule {
  final CrawChatSdkContext context;

  CrawChatPortalModule(this.context);

  Future<Map<String, dynamic>?> getHome() {
    return context.backendClient.portal.getHome();
  }

  Future<Map<String, dynamic>?> getAuth() {
    return context.backendClient.portal.getAuth();
  }

  Future<PortalWorkspaceView?> getWorkspace() {
    return context.backendClient.portal.getWorkspace();
  }

  Future<Map<String, dynamic>?> getDashboard() {
    return context.backendClient.portal.getDashboard();
  }

  Future<Map<String, dynamic>?> getConversations() {
    return context.backendClient.portal.getConversations();
  }

  Future<Map<String, dynamic>?> getRealtime() {
    return context.backendClient.portal.getRealtime();
  }

  Future<Map<String, dynamic>?> getMedia() {
    return context.backendClient.portal.getMedia();
  }

  Future<Map<String, dynamic>?> getAutomation() {
    return context.backendClient.portal.getAutomation();
  }

  Future<Map<String, dynamic>?> getGovernance() {
    return context.backendClient.portal.getGovernance();
  }
}
