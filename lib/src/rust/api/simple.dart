// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

String greet({required String name, dynamic hint}) =>
    RustLib.instance.api.greet(name: name, hint: hint);

Future<void> initLib({required String path, dynamic hint}) =>
    RustLib.instance.api.initLib(path: path, hint: hint);

Stream<LogEntry> setupLogStream({dynamic hint}) =>
    RustLib.instance.api.setupLogStream(hint: hint);

enum Level {
  error,
  warn,
  info,
  debug,
  trace,
  ;
}

class LogEntry {
  final int timeMillis;
  final String msg;
  final Level logLevel;
  final String lbl;

  const LogEntry({
    required this.timeMillis,
    required this.msg,
    required this.logLevel,
    required this.lbl,
  });

  @override
  int get hashCode =>
      timeMillis.hashCode ^ msg.hashCode ^ logLevel.hashCode ^ lbl.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LogEntry &&
          runtimeType == other.runtimeType &&
          timeMillis == other.timeMillis &&
          msg == other.msg &&
          logLevel == other.logLevel &&
          lbl == other.lbl;
}
