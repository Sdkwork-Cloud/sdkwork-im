/// Call type enumeration
/// Defines different types of API calls between native and Dart
enum CallType {
  /// Variable getter access
  varGetter('var_get'),

  /// Regular API method call
  plainApiCall('plain_api_call'),

  /// Create new instance
  newInstance('new_instance'),

  /// Instance method invocation
  instanceMethodInvoke('instance_method_invoke'),

  /// Instance property getter
  instancePropertyGet('instance_member_get'),

  /// Instance property setter
  instancePropertySet('instance_member_set'),

  /// Add instance event listener
  instanceEventListenerAdd('instance_event_add'),

  /// Remove instance event listener
  instanceEventListenerRemove('instance_event_remove'),

  /// Emit instance event
  instanceEventEmit('instance_event_emit'),

  /// Event result callback
  instanceEventResult('instance_event_result'),

  /// Emit callback
  callbackEmit('callback_emit'),

  /// Destroy instance
  destroyInstance('destroy_instance'),

  // Get class properties
  instancePropertiesGet('instance_properties_get');

  /// Native string value for the enum
  final String value;
  const CallType(this.value);
}

/// Return status enumeration
/// Represents the result status of an API call
enum ReturnStatus {
  success('success'),
  failed('failed'),
  notImplemented('notImplemented');

  final String value;
  const ReturnStatus(this.value);
}

/// Argument type enumeration
/// Defines different types of arguments that can be passed between native and Dart
enum ArgType {
  /// Instance reference
  instance('instance'),

  /// Callback function
  callback('callback'),

  /// Base64 encoded data
  base64('base64');

  final String value;
  const ArgType(this.value);
}

/// Call parameters for API invocation
class CallParams {
  /// Type of API call
  final CallType callType;

  /// Name of the service to call
  final String serviceName;

  /// Name of the method to invoke
  final String? methodName;

  /// Name of the member to access
  final String? memberName;

  /// Arguments for the call
  final List<dynamic>? args;

  /// Members should be settled.
  final Map<String, dynamic>? members;

  final String? _instanceId;

  /// Instance identifier
  String? get instanceId => _instanceId;

  final InstanceType? _instanceType;

  /// Type of instance creation
  InstanceType? get instanceType => _instanceType;

  final String? _traceId;

  /// Trace identifier for request tracking
  String? get traceId => _traceId;

  /// Whether to wait for the result
  final bool? waitResult;

  const CallParams({
    required this.callType,
    required this.serviceName,
    this.methodName,
    this.memberName,
    this.args,
    this.members,
    String? instanceId,
    InstanceType? instanceType,
    String? traceId,
    this.waitResult,
  }) : _instanceId = instanceId,
       _instanceType = instanceType,
       _traceId = traceId;

  /// Create CallParams from JSON data
  static CallParams fromJson(Map<dynamic, dynamic> args) {
    return CallParams(
      callType: CallType.values.firstWhere(
        (type) => type.value == args['callType'],
        orElse: () => CallType.varGetter,
      ),
      serviceName: args['serviceName'],
      methodName: args['methodName'],
      memberName: args['memberName'],
      args: args['args'],
      members: args['members'],
      instanceId: args['_instanceId'],
      instanceType: InstanceType.values.firstWhere(
        (type) => type.toString().split('.').last == args['_instanceType'],
        orElse: () => InstanceType.automatic,
      ),
      traceId: args['_traceId'],
      waitResult: args['waitResult'],
    );
  }

  /// Convert CallParams to JSON format
  Map<String, dynamic> toJson() {
    final Map<String, dynamic> json = {
      'callType': callType.value,
      'serviceName': serviceName,
    };

    if (methodName != null) json['methodName'] = methodName;
    if (memberName != null) json['memberName'] = memberName;
    if (args != null) json['args'] = args;
    if (members != null) json['members'] = members;
    if (_instanceId != null) json['_instanceId'] = _instanceId;
    if (_instanceType != null) json['_instanceType'] = _instanceType!.index;
    if (_traceId != null) json['_traceId'] = _traceId;
    if (waitResult != null) json['waitResult'] = waitResult;

    return json;
  }
}

/// Return parameters from API calls
class ReturnParams {
  /// Status of the API call
  final ReturnStatus status;

  /// Return message or data
  final dynamic msg;

  /// Trace identifier for response tracking
  final String? traceId;

  /// Decoded response data
  final dynamic decoded;

  const ReturnParams({
    required this.status,
    this.msg,
    this.traceId,
    this.decoded,
  });

  /// Create ReturnParams from JSON data
  static ReturnParams fromJson(Map<String, dynamic> json) {
    if (json.containsKey('status') && json.length == 1) {
      return ReturnParams(
        status: ReturnStatus.values.firstWhere(
          (e) => e.value == json['status'],
          orElse: () => ReturnStatus.failed,
        ),
        decoded: 0,
      );
    }

    return ReturnParams(
      status: ReturnStatus.values.firstWhere(
        (e) => e.value == json['status'],
        orElse: () => ReturnStatus.failed,
      ),
      msg: json['msg'],
      traceId: json['traceId'],
      decoded: json['decoded'],
    );
  }
}

/// Instance reference class
class Instance {
  /// Unique identifier for the instance
  final String? instanceId;

  /// Name of the service this instance belongs to
  final String? serviceName;

  const Instance({this.instanceId, this.serviceName});
}

/// Mixed data structure for instance data
class MixinData {
  final String? instanceId;
  final String? serviceName;
  final Map<String, dynamic>? data;

  MixinData({this.instanceId, this.serviceName, this.data});
}

/// Service type definition
typedef Service = dynamic;

/// Handler function type definition
typedef Handler = Function;

/// Argument type definition
typedef Arg = dynamic;

/// Instance creation type enumeration
enum InstanceType {
  /// Automatic instance creation
  automatic,

  /// Manual instance creation
  manual,
}
