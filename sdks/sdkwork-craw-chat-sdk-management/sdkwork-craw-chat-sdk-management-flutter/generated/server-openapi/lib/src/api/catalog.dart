import 'paths.dart';
import '../http/client.dart';

class CatalogApi {
  final HttpClient _client;

  CatalogApi(this._client);

  /// listChannelModels
  Future<dynamic> listChannelModels(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/channel-models'),
      params: params,
      headers: headers,
    );
  }

  /// saveChannelModel
  Future<dynamic> saveChannelModel(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/channel-models'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteChannelModel
  Future<dynamic> deleteChannelModel(
    Object channelId,
    Object modelId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/channel-models/${Uri.encodeComponent(String(channelId))}/models/${Uri.encodeComponent(String(modelId))}'),
      params: params,
      headers: headers,
    );
  }

  /// listChannels
  Future<dynamic> listChannels(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/channels'),
      params: params,
      headers: headers,
    );
  }

  /// saveChannel
  Future<dynamic> saveChannel(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/channels'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteChannel
  Future<dynamic> deleteChannel(
    Object channelId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/channels/${Uri.encodeComponent(String(channelId))}'),
      params: params,
      headers: headers,
    );
  }

  /// listCredentials
  Future<dynamic> listCredentials(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/credentials'),
      params: params,
      headers: headers,
    );
  }

  /// saveCredential
  Future<dynamic> saveCredential(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/credentials'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteCredential
  Future<dynamic> deleteCredential(
    Object tenantId,
    Object providerId,
    Object keyReference,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/credentials/${Uri.encodeComponent(String(tenantId))}/providers/${Uri.encodeComponent(String(providerId))}/keys/${Uri.encodeComponent(String(keyReference))}'),
      params: params,
      headers: headers,
    );
  }

  /// listModelPrices
  Future<dynamic> listModelPrices(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/model-prices'),
      params: params,
      headers: headers,
    );
  }

  /// saveModelPrice
  Future<dynamic> saveModelPrice(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/model-prices'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteModelPrice
  Future<dynamic> deleteModelPrice(
    Object channelId,
    Object modelId,
    Object proxyProviderId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/model-prices/${Uri.encodeComponent(String(channelId))}/models/${Uri.encodeComponent(String(modelId))}/providers/${Uri.encodeComponent(String(proxyProviderId))}'),
      params: params,
      headers: headers,
    );
  }

  /// listModels
  Future<dynamic> listModels(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/models'),
      params: params,
      headers: headers,
    );
  }

  /// saveModel
  Future<dynamic> saveModel(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/models'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteModel
  Future<dynamic> deleteModel(
    Object externalName,
    Object providerId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/models/${Uri.encodeComponent(String(externalName))}/providers/${Uri.encodeComponent(String(providerId))}'),
      params: params,
      headers: headers,
    );
  }

  /// listProviders
  Future<dynamic> listProviders(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/providers'),
      params: params,
      headers: headers,
    );
  }

  /// saveProvider
  Future<dynamic> saveProvider(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/providers'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteProvider
  Future<dynamic> deleteProvider(
    Object providerId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/providers/${Uri.encodeComponent(String(providerId))}'),
      params: params,
      headers: headers,
    );
  }
}
