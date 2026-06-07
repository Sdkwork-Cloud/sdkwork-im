import 'package:hybrid_runtime/hybrid_runtime.dart';

import '../bridge/message/interface.dart';
import 'class.dart';
import 'type.dart';

/// Base class for pack classes
abstract class PackClass implements PackClassInterface {
  dynamic _instance;

  PackClass([List<dynamic>? args]) {
    $init(args ?? []);
  }

  @override
  dynamic get $instance {
    return _instance;
  }

  /// Factory method for creating instances
  dynamic $createInstance(List<dynamic> args);

  @override
  void $destroy() {
    _instance?.destroy();
    _instance = null;
  }

  @override
  void $init(List<dynamic> args) {
    final instance = $createInstance(args);
    _instance = instance;
  }

  /**
   * 查找重载参数下标列表
   * @desc android 构造函数存在重载
   * 此方法通过实际传入参数与构造函数参数列表集合的比对，来获取当前实际的需要使用的参数列表
   *
   * 先以列表集合为主
   * 1. 列表集合为空，直接返回 [] 作为本次构造函数的参数下标列表；
   * 2. 列表集合只有一个列表，直接返回该列表；
   * 3. 从多往少遍历列表集合，对照入参，如果存在某个列表的入参全部非 null，返回该列表；
   *
   * 再以参数实际情况为主，查找入参中最后一个非空的参数下标
   * 1. 下标为 -1，直接返回列表集合中最少的列表；
   * 2. 下标不为 -1
   * 2.1 查找列表集合中以下标为结尾的列表返回；
   * 2.2 查找列表集合中包含下标的数量最少的列表返回；
   *
   * 兜底返回数量最少的列表
   *
   */
  List<int> findOverrideIndices(
    List<dynamic> args,
    List<List<int>> indicesList,
  ) {
    // 如果没有构造参数列表，直接返回 []
    if (indicesList.length == 0) {
      return [];
    }
    // 如果只有一个构造参数列表，不用匹配，直接返回第一项
    if (indicesList.length == 1) {
      return indicesList[0];
    }
    indicesList.sort((a, b) => b.length - a.length);
    // 匹配参数全在的情况
    for (final _indices in indicesList) {
      if (_indices.length == 0) {
        continue;
      }
      // 获取 _indices 对应的 args 值
      final values = _indices.map((i) => args[i]).toList();
      // 如果全部不为 null
      if (values.every((v) => v != null)) {
        return _indices;
      }
    }
    // 如果只是部分参数存在
    // 先找到最后一个非 null 的参数 index
    int lastIndex = args.lastIndexWhere((arg) => arg != null);
    // 如果参数列表都是 null 直接返回最少的
    if (lastIndex == -1) {
      return indicesList.last;
    }
    // 遍历 indicesList 找到 lastIndex 是哪个 indices 的最后一项
    // 如果能找到就直接返回该 indices
    // 当前 indicesList 已经默认为从多往少的设定
    for (int i = 0; i < indicesList.length; i++) {
      if (indicesList[i].length == 0) continue;
      if (indicesList[i].last == lastIndex) {
        return indicesList[i];
      }
    }

    // 遍历 indicesList 找到包含 lastIndex 且数量最少的 indices 返回
    for (int i = indicesList.length - 1; i >= 0; i--) {
      if (indicesList[i].length == 0) continue;
      if (indicesList[i].last == lastIndex) {
        return indicesList[i];
      }
    }
    // 兜底返回最少的那个
    return indicesList[indicesList.length - 1];
  }

  /**
   * 实例化参数处理
   * 将 pack 过后的 enum / class 转成 android / ios 平台侧的 enum / class
   */
  List<dynamic> transformToPlatformConstructorArgs(
    List<dynamic> args,
    List<int> indices,
    Map<String, dynamic> typeMap,
    Map<String, dynamic> enumMap,
    Map<String, dynamic> classMap,
    String platformVar,
  ) {
    if (indices.length == 0) {
      return [];
    }
    List<dynamic> params = [];
    for (int i = 0; i < indices.length; i++) {
      int argsIdx = indices[i];
      String type = typeMap['paramType-$argsIdx'];
      // collectionType genericKind typeName
      String collectionType = type.split(' ')[0];
      String genericKind = type.split(' ')[1];
      String typeName = type.split(' ')[2];
      dynamic arg = args[argsIdx];
      if (arg == null) {
        params.add(null);
        continue;
      }
      // 如果类别是 enum / class 且不是以平台侧 enum / class  传入的话
      // 需要转成 平台侧的 的 enum / class  传递给构造函数
      if (collectionType == '' &&
          genericKind == 'enum' &&
          !typeName.startsWith(platformVar)) {
        params.add(enumMap[typeName](arg));
      } else if (collectionType == '' &&
          genericKind == 'class' &&
          !typeName.startsWith(platformVar)) {
        params.add(classMap[typeName](arg));
      } else if (collectionType == 'List' &&
          genericKind == 'class' &&
          !typeName.startsWith(platformVar)) {
        List<dynamic> l = [];
        for (int j = 0; j < arg.length; j++) {
          l.add(classMap[typeName](arg[j]));
        }
        params.add(l);
      } else if (collectionType == 'List' &&
          genericKind == 'enum' &&
          !typeName.startsWith(platformVar)) {
        List<dynamic> l = [];
        for (int j = 0; j < arg.length; j++) {
          l.add(enumMap[typeName](arg[j]));
        }
        params.add(l);
      } else {
        params.add(arg);
      }
    }
    return params;
  }

  void updateInstance(dynamic instance) {
    if (instance is Map && this._instance is NativeClass) {
      final map = Map<String, dynamic>.from(instance);
      (this._instance as NativeClass).updateResource(
        NativeResource(
          instanceId: map['_instanceId'] ?? '',
          client: this._instance?.$resource.client,
        ),
      );
    } else {
      _instance = instance;
    }
  }

  /// 与 ts runtime 中的 fn2AndroidClass 功能一致
  /// 将 Dart 函数转换为 Android 回调类实例供 Android 侧使用
  ///
  /// [callback] - The Dart callback function
  /// [nativeClass] - The Android native class constructor
  /// [methodName] - The method name to assign the callback to
  ///
  /// 示例：fn2AndroidClass(callback, () => $p_a.VeLiveVideoEffectCallback(), "onResult")
  dynamic fn2AndroidClass(
    Function callback,
    dynamic Function() nativeClass,
    String methodName,
  ) {
    // 创建示例
    final instance = nativeClass();

    // 如果实例是 NativeObserverClass，注册回调
    if (instance is NativeObserverClass) {
      // 设置方法名和回调
      _setCallbackMethod(instance, methodName, callback);
    }

    return instance;
  }

  /// 辅助方法，在 native 类实例上设置回调方法
  void _setCallbackMethod(
    dynamic instance,
    String methodName,
    Function callback,
  ) {
    // 使用 noSuchMethod 动态设置回调
    // 允许我们在实例上设置任何方法名
    try {
      // 对于 NativeObserverClass 实例，我们可以使用 registerEvent 方法
      if (instance is NativeObserverClass) {
        instance.registerEvent(methodName, callback);
      } else {
        // 暂不支持其它类型实例
        throw UnsupportedError(
          'Cannot set callback on non-NativeObserverClass instance',
        );
      }
    } catch (e) {
      throw UnsupportedError('Failed to set callback method $methodName: $e');
    }
  }
}

/// Pack class interface
abstract class PackClassInterface {
  dynamic get $instance;
  void $destroy(); // Internal destroy method
  void $init(List<dynamic> args);
}

/// Base class for pack observer classes
abstract class PackObserverClass extends PackClass
    implements PackObserverInterface {
  @override
  final String $instanceId;

  @override
  final String? $namespace;

  @override
  final MessageClient? $messageClient;

  PackObserverClass({
    required this.$instanceId,
    this.$namespace,
    this.$messageClient,
  });

  /// Register event handler
  @override
  void registerEventHandler(String eventName, Function handler) {
    $messageClient?.sendInstanceEventAdd(
      $instanceId,
      $namespace ?? '',
      eventName,
    );
  }
}

/// Pack observer interface
abstract class PackObserverInterface {
  dynamic get $instance;
  String get $instanceId;
  MessageClient? get $messageClient;
  String? get $namespace;
  void registerEventHandler(String eventName, Function handler);
}
