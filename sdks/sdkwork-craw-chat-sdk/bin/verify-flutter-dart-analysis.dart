import 'dart:io';

import 'package:analyzer/dart/analysis/analysis_context_collection.dart';
import 'package:analyzer/dart/analysis/results.dart';

String normalizeTargetPath(String value) {
  final entityType = FileSystemEntity.typeSync(value);
  return switch (entityType) {
    FileSystemEntityType.directory => Directory(value).absolute.path,
    FileSystemEntityType.file => File(value).absolute.path,
    _ => Directory(value).absolute.path,
  };
}

String toRelativePath(String rootPath, String filePath) {
  final normalizedRoot = rootPath.replaceAll('\\', '/').replaceAll(RegExp('/+\$'), '');
  final normalizedFile = filePath.replaceAll('\\', '/');
  if (!normalizedFile.startsWith(normalizedRoot)) {
    return filePath;
  }

  return normalizedFile.substring(normalizedRoot.length).replaceFirst(RegExp(r'^/+'), '');
}

Future<bool> analyzeTarget(String targetPath) async {
  final collection = AnalysisContextCollection(includedPaths: [targetPath]);
  final visitedFiles = <String>{};
  var hasFailures = false;

  for (final context in collection.contexts) {
    final session = context.currentSession;
    for (final filePath in context.contextRoot.analyzedFiles()) {
      if (!filePath.endsWith('.dart') || !visitedFiles.add(filePath)) {
        continue;
      }

      final result = await session.getErrors(filePath);
      if (result is! ErrorsResult) {
        continue;
      }

      for (final error in result.errors) {
        final severityName = error.severity.name;
        if (severityName != 'ERROR' && severityName != 'WARNING') {
          continue;
        }

        final location = result.lineInfo.getLocation(error.offset);
        stderr.writeln(
          '${toRelativePath(targetPath, filePath)}:${location.lineNumber}:${location.columnNumber} '
          '[${severityName.toLowerCase()}] ${error.errorCode.name}: ${error.message}',
        );
        hasFailures = true;
      }
    }
  }

  if (!hasFailures) {
    stdout.writeln('[sdkwork-craw-chat-sdk] Flutter Dart analysis passed for $targetPath.');
  }

  return hasFailures;
}

Future<void> main(List<String> args) async {
  if (args.isEmpty) {
    stderr.writeln('[sdkwork-craw-chat-sdk] Expected at least one Dart analysis target path.');
    exit(64);
  }

  var hasFailures = false;
  for (final rawTarget in args) {
    final targetPath = normalizeTargetPath(rawTarget);
    if (await analyzeTarget(targetPath)) {
      hasFailures = true;
    }
  }

  if (hasFailures) {
    exit(1);
  }
}
