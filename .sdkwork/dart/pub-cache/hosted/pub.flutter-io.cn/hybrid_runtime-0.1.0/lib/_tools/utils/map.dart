/// Converts a list of key-value pairs to a Map
Map<K, V> fromPairs<K extends String, V>(List<MapEntry<K, V>> entries) {
  return Map.fromEntries(entries);
}

/// Converts a Map to a list of key-value pairs
List<MapEntry<K, V>> toPairs<K extends String, V>(Map<K, V> map) {
  return map.entries.toList();
}

/// Transforms values in a Map
///
/// [map] Source map
/// [transform] Value transformation function
Map<K, V2> mapValues<K, V1, V2>(
  Map<K, V1> map,
  V2 Function(V1 value) transform,
) {
  return Map.fromEntries(
    map.entries.map(
      (entry) => MapEntry(entry.key, transform(entry.value)),
    ),
  );
}

/// Transforms keys in a Map
///
/// [map] Source map
/// [transform] Key transformation function
Map<K2, V> mapKeys<K1, K2, V>(
  Map<K1, V> map,
  K2 Function(K1 key) transform,
) {
  return Map.fromEntries(
    map.entries.map(
      (entry) => MapEntry(transform(entry.key), entry.value),
    ),
  );
}

/// Filters key-value pairs in a Map
///
/// [map] Source map
/// [predicate] Filter condition function
Map<K, V> filterMap<K, V>(
  Map<K, V> map,
  bool Function(K key, V value) predicate,
) {
  return Map.fromEntries(
    map.entries.where(
      (entry) => predicate(entry.key, entry.value),
    ),
  );
}
