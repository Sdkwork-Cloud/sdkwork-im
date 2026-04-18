import 'paths.dart';
import '../http/client.dart';

class RoutingApi {
  final HttpClient _client;

  RoutingApi(this._client);

  /// listRoutingDecisionLogs
  Future<dynamic> listRoutingDecisionLogs(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/routing/decision-logs'),
      params: params,
      headers: headers,
    );
  }

  /// listProviderHealthSnapshots
  Future<dynamic> listProviderHealthSnapshots(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/routing/health-snapshots'),
      params: params,
      headers: headers,
    );
  }

  /// listRoutingProfiles
  Future<dynamic> listRoutingProfiles(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/routing/profiles'),
      params: params,
      headers: headers,
    );
  }

  /// createRoutingProfile
  Future<dynamic> createRoutingProfile(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/routing/profiles'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// listCompiledRoutingSnapshots
  Future<dynamic> listCompiledRoutingSnapshots(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/routing/snapshots'),
      params: params,
      headers: headers,
    );
  }
}
