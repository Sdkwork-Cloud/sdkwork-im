import 'paths.dart';
import '../http/client.dart';

class UsageApi {
  final HttpClient _client;

  UsageApi(this._client);

  /// listUsageRecords
  Future<dynamic> listUsageRecords(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/usage/records'),
      params: params,
      headers: headers,
    );
  }

  /// getUsageSummary
  Future<dynamic> getUsageSummary(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/usage/summary'),
      params: params,
      headers: headers,
    );
  }
}
