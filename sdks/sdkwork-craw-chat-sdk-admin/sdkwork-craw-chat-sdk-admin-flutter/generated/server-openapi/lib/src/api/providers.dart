import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class ProvidersApi {
  final AdminHttpClient _httpClient;

  ProvidersApi(this._httpClient);

  Future<JsonObject> getProviderBindings([QueryParams? params]) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/provider-bindings'),
      params: params,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'providers.getProviderBindings',
    );
  }

  Future<JsonObject> upsertProviderBindingPolicy(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/provider-bindings'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'providers.upsertProviderBindingPolicy',
    );
  }

  Future<JsonObject> getProviderPolicyHistory() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/provider-policies'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'providers.getProviderPolicyHistory',
    );
  }

  Future<JsonObject> getProviderPolicyDiff(QueryParams params) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/provider-policies/diff'),
      params: params,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'providers.getProviderPolicyDiff',
    );
  }

  Future<JsonObject> previewProviderPolicy(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/provider-policies/preview'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'providers.previewProviderPolicy',
    );
  }

  Future<JsonObject> rollbackProviderPolicy(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/provider-policies/rollback'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'providers.rollbackProviderPolicy',
    );
  }

  Future<JsonObject> getProviderRegistry() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/provider-registry'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'providers.getProviderRegistry',
    );
  }
}
