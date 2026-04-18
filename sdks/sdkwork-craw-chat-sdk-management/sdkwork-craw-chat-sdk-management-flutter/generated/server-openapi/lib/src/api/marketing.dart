import 'paths.dart';
import '../http/client.dart';

class MarketingApi {
  final HttpClient _client;

  MarketingApi(this._client);

  /// listMarketingCampaigns
  Future<dynamic> listMarketingCampaigns(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/marketing/campaigns'),
      params: params,
      headers: headers,
    );
  }

  /// saveMarketingCampaign
  Future<dynamic> saveMarketingCampaign(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/marketing/campaigns'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// updateMarketingCampaignStatus
  Future<dynamic> updateMarketingCampaignStatus(
    Object marketingCampaignId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/marketing/campaigns/${Uri.encodeComponent(String(marketingCampaignId))}/status'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }
}
