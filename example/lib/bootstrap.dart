import 'dart:io';

import 'package:flutter/services.dart';
import 'package:libsql_dart/libsql_dart.dart';
import 'package:path_provider/path_provider.dart';

Future<void> bootstrapDatabase(LibsqlClient client) async {
  await client.connect();
  await client.execute("drop table if exists tasks");
  await client.execute(
      "create table if not exists tasks (id integer primary key, title text, description text, completed integer)");
}

Future<void> runExtensionTest(LibsqlClient client) async {
  final extensionData = await rootBundle.load("assets/stats.dylib");
  final dir = await getApplicationCacheDirectory();
  List<int> bytes = extensionData.buffer
      .asUint8List(extensionData.offsetInBytes, extensionData.lengthInBytes);
  await File("${dir.path}/stats.dylib").writeAsBytes(bytes);

  await client.connect();
  await client.enableExtension();
  await client.loadExtension(path: "${dir.path}/stats.dylib");
  // ignore: avoid_print
  print(await client.query("select * from generate_series(1, 99);"));
  await client.disableExtension();
}
