import 'paths.dart';
import '../http/client.dart';

class BillingApi {
  final HttpClient _client;

  BillingApi(this._client);

  /// listBillingEvents
  Future<dynamic> listBillingEvents(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/billing/events'),
      params: params,
      headers: headers,
    );
  }

  /// getBillingEventSummary
  Future<dynamic> getBillingEventSummary(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/billing/events/summary'),
      params: params,
      headers: headers,
    );
  }

  /// getBillingSummary
  Future<dynamic> getBillingSummary(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/billing/summary'),
      params: params,
      headers: headers,
    );
  }
}
