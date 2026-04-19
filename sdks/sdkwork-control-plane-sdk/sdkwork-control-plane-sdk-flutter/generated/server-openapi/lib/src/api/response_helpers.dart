import '../models.dart';

JsonObject? sdkworkResponseAsMap(dynamic value) {
  if (value is Map<String, dynamic>) {
    return value;
  }
  if (value is Map) {
    return value.map((key, item) => MapEntry(key.toString(), item));
  }
  return null;
}

JsonObject sdkworkRequireJsonObject(
  dynamic value, {
  required String operationName,
}) {
  final jsonObject = sdkworkResponseAsMap(value);
  if (jsonObject != null) {
    return jsonObject;
  }
  if (value == null) {
    return <String, dynamic>{};
  }
  throw StateError(
    '$operationName expected a JSON object response but received ${value.runtimeType}.',
  );
}

String encodeIdentifier(Identifier value) {
  return Uri.encodeComponent(value.toString());
}
