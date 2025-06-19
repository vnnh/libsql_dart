# libsql_dart

LibSQL Dart client library to interact with LibSQL/Turso database instance.

## Supported Features

- Local, Remote, and Embedded replica
- Running execute and query SQL statement with named or positional params

## Getting Started

### Add it to your `pubspec.yaml`.

```
libsql_dart:
```

### Create the client

- In memory

```dart
final client = LibsqlClient.memory();
```

- Local

```dart
final dir = await getApplicationCacheDirectory();
final path = '${dir.path}/local.db';
final client = LibsqlClient.local(path);
```

- Remote

```dart
final client = LibsqlClient.remote('<TURSO_OR_LIBSQL_URL>',
    authToken: '<TOKEN>');
```

- Embedded replica

```dart
final dir = await getApplicationCacheDirectory();
final path = '${dir.path}/local.db';
final client = LibsqlClient.replica(path,
    syncUrl: '<TURSO_OR_LIBSQL_URL>',
    authToken: '<TOKEN>',
	syncIntervalSeconds: 5,
    readYourWrites: true);
```

- Offline writes

```dart
final dir = await getApplicationCacheDirectory();
final path = '${dir.path}/local.db';
final client = LibsqlClient.offline(path,
    syncUrl: '<TURSO_OR_LIBSQL_URL>',
    authToken: '<TOKEN>');
```

### Connect

```dart
await client.connect();
```

### Call `sync` to sync remote changes to local replica when using embedded replica or offline writes

```dart
await client.sync();
```

### Run SQL statements

- Create table

```dart
await client.execute("create table if not exists customers (id integer primary key, name text);");
```

- Insert query

```dart
await client.query("insert into customers(name) values ('John Doe')");
```

- Select query

```dart
print(await client.query("select * from customers"));
```

- Batch transaction

```dart
await client.batch("""insert into customers (name) values ('Jane Doe'); insert into customers (name) values ('Jake Doe');""");
```

- Prepared statement

```dart
final statement = await client
	.prepare("select * from customers where id = ?");
await statement.query(positional: [1])
```

- Transaction

```dart
final tx = await client.transaction();
await tx
	.execute("update customers set name = 'John Noe' where id = 1");
await tx
	.execute("update customers set name = 'Jane Noe' where id = 2");
print(await tx
	.query("select * from customers where id = ?", positional: [1]));
await tx.commit();
```

- Read the locally replicated db using `sqflite` when using embedded replica

```dart
final db = await openDatabase(path, readOnly: true);
final result = await db.rawQuery('select * from customers');
print(result);
```

**Note** Code snippets above also use `path_provider` and `sqflite` packages. When using other sqlite libraries to read the file, you need to make sure that it is done in read only mode, because the replication process assumes exclusive write lock over the file.

## Demo

![Demo](https://raw.githubusercontent.com/dikatok/libsql_dart/main/assets/demo.gif)

## Pre-requisites

- `libsql` for obvious reason needs internet connection to perform most of it's functionalities, thus make sure required permission is handled/provided for each deployment target (android, ios, etc)
- starting from version `0.7.0`, having `rustup` installed is no longer necessary for building your flutter app, it will be used to build the binary if available and will download the pre-built binaries instead otherwise.
