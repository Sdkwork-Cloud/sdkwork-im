import '../logger/logger.dart';

/// Performance tracing utility for monitoring API calls and callbacks
///
/// Collects timing data and execution results for debugging and performance analysis
class Tracer {
  final List<TraceSpan> _spans = [];
  bool _enabled = false;

  /// Enable tracing data collection
  void enable() {
    _enabled = true;
  }

  /// Disable tracing data collection
  void disable() {
    _enabled = false;
  }

  /// Record an API call with parameters
  ///
  /// [data]: Metadata should contain:
  /// - source: Call origin (e.g. 'dart'/'native')
  /// - target: Call destination
  /// - serviceName: Service identifier
  /// - methodName: Called method name (optional)
  TraceSpan collectApiCall(Map<String, dynamic> data) {
    if (!_enabled) {
      return TraceSpan('disabled', {});
    }

    final span = TraceSpan('api-call', data);
    _spans.add(span);
    logger.debug('[tracer] API call:', [data]);
    return span;
  }

  /// Collect callback emit tracking
  ///
  /// [source]: Callback origin
  /// [target]: Callback destination
  /// [eventName]: Callback event name
  /// [eventData]: Callback event data
  TraceSpan collectCallbackEmit({
    required String source,
    required String target,
    required String eventName,
    required dynamic eventData,
  }) {
    if (!_enabled) {
      return TraceSpan('disabled', {});
    }

    final span = TraceSpan('callback-emit', {
      'source': source,
      'target': target,
      'eventName': eventName,
      'eventData': eventData,
    });
    _spans.add(span);
    logger.debug('[tracer] Callback emit:', [eventName, eventData]);
    return span;
  }

  /// Get all tracing data
  List<Map<String, dynamic>> getTraces() {
    return _spans.map((span) => span.toJson()).toList();
  }

  /// Clear tracing data
  void clear() {
    _spans.clear();
  }
}

/// Represents a single tracing span with timing information
class TraceSpan {
  final String id;
  final Map<String, dynamic> data;
  final DateTime startTime;
  DateTime? endTime;
  dynamic result;
  dynamic error;

  TraceSpan(this.id, this.data) : startTime = DateTime.now();

  /// Mark the span as successfully completed
  /// [res]: Result value of the traced operation
  void success(dynamic res) {
    endTime = DateTime.now();
    result = res;
  }

  /// Mark the span as failed with error
  /// [err]: Error object or message
  void fail(dynamic err) {
    endTime = DateTime.now();
    error = err;
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'data': data,
      'startTime': startTime.toIso8601String(),
      'endTime': endTime?.toIso8601String(),
      'duration': endTime?.difference(startTime).inMilliseconds,
      'result': result,
      'error': error?.toString(),
    };
  }
}

/// Global tracer instance
final tracer = Tracer();
