This file is a merged representation of the entire codebase, combined into a single document by Repomix.

# File Summary

## Purpose
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.

## File Format
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Repository files (if enabled)
5. Multiple file entries, each consisting of:
  a. A header with the file path (## File: path/to/file)
  b. The full contents of the file in a code block

## Usage Guidelines
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.

## Notes
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)

# Directory Structure
```
.agents/skills/protobuf/assets/buf.gen.go-connect.yaml
.agents/skills/protobuf/assets/buf.gen.go.yaml
.agents/skills/protobuf/assets/buf.gen.java.yaml
.agents/skills/protobuf/assets/buf.gen.python.yaml
.agents/skills/protobuf/assets/buf.gen.ts.yaml
.agents/skills/protobuf/assets/buf.gen.yaml
.agents/skills/protobuf/assets/buf.lock
.agents/skills/protobuf/assets/buf.yaml
.agents/skills/protobuf/assets/Makefile.example
.agents/skills/protobuf/assets/proto/example/v1/book_service.proto
.agents/skills/protobuf/assets/proto/example/v1/book.proto
.agents/skills/protobuf/references/best_practices.md
.agents/skills/protobuf/references/buf_toolchain.md
.agents/skills/protobuf/references/examples.md
.agents/skills/protobuf/references/migration.md
.agents/skills/protobuf/references/protoc_toolchain.md
.agents/skills/protobuf/references/protovalidate.md
.agents/skills/protobuf/references/quick_reference.md
.agents/skills/protobuf/references/review_checklist.md
.agents/skills/protobuf/references/troubleshooting.md
.agents/skills/protobuf/SKILL.md
.agents/skills/rust-async-patterns/references/details.md
.agents/skills/rust-async-patterns/SKILL.md
.agents/skills/rust-best-practices/references/chapter_01.md
.agents/skills/rust-best-practices/references/chapter_02.md
.agents/skills/rust-best-practices/references/chapter_03.md
.agents/skills/rust-best-practices/references/chapter_04.md
.agents/skills/rust-best-practices/references/chapter_05.md
.agents/skills/rust-best-practices/references/chapter_06.md
.agents/skills/rust-best-practices/references/chapter_07.md
.agents/skills/rust-best-practices/references/chapter_08.md
.agents/skills/rust-best-practices/references/chapter_09.md
.agents/skills/rust-best-practices/SKILL.md
.gitignore
.opencode.json
AGENTS.md
docs/ai/setup.md
docs/ai/tools.md
docs/ai/troubleshooting.md
docs/ai/workflow.md
scripts/astfind.ps1
scripts/graph.ps1
scripts/repoctx.ps1
scripts/watchgraph.ps1
skills-lock.json
```

# Files

## File: .agents/skills/protobuf/assets/buf.gen.go-connect.yaml
````yaml
# buf.gen.go-connect.yaml - Go with Connect code generation
# Documentation: https://buf.build/docs/configuration/v2/buf-gen-yaml

version: v2
clean: true
managed:
  enabled: true
  override:
    - file_option: go_package_prefix
      value: github.com/yourorg/yourrepo/gen
  disable:
    - module: buf.build/bufbuild/protovalidate
      file_option: go_package_prefix
plugins:
  - remote: buf.build/protocolbuffers/go
    out: gen
    opt:
      - paths=source_relative
      - default_api_level=API_OPAQUE
  - remote: buf.build/connectrpc/go
    out: gen
    opt:
      - paths=source_relative
````

## File: .agents/skills/protobuf/assets/buf.gen.go.yaml
````yaml
# buf.gen.go.yaml - Go with gRPC code generation
# Documentation: https://buf.build/docs/configuration/v2/buf-gen-yaml

version: v2

managed:
  enabled: true
  override:
    - file_option: go_package_prefix
      value: github.com/yourorg/yourrepo/gen
  disable:
    - module: buf.build/bufbuild/protovalidate
      file_option: go_package_prefix
plugins:
  - remote: buf.build/protocolbuffers/go
    out: gen
    opt:
      - paths=source_relative

  - remote: buf.build/grpc/go
    out: gen
    opt:
      - paths=source_relative
````

## File: .agents/skills/protobuf/assets/buf.gen.java.yaml
````yaml
# buf.gen.java.yaml - Java with gRPC code generation
# Documentation: https://buf.build/docs/configuration/v2/buf-gen-yaml

version: v2

managed:
  enabled: true
  override:
    - file_option: java_package_prefix
      value: com.yourorg.yourrepo
    - file_option: java_multiple_files
      value: true

plugins:
  - remote: buf.build/protocolbuffers/java
    out: gen

  - remote: buf.build/grpc/java
    out: gen
````

## File: .agents/skills/protobuf/assets/buf.gen.python.yaml
````yaml
# buf.gen.python.yaml - Python with gRPC code generation
# Documentation: https://buf.build/docs/configuration/v2/buf-gen-yaml

version: v2

plugins:
  - remote: buf.build/protocolbuffers/python
    out: gen

  - remote: buf.build/protocolbuffers/pyi
    out: gen

  - remote: buf.build/grpc/python
    out: gen
````

## File: .agents/skills/protobuf/assets/buf.gen.ts.yaml
````yaml
# buf.gen.ts.yaml - TypeScript with Connect code generation
# Documentation: https://buf.build/docs/configuration/v2/buf-gen-yaml

version: v2

plugins:
  - remote: buf.build/bufbuild/es
    out: gen
    opt:
      - target=ts

  - remote: buf.build/connectrpc/es
    out: gen
    opt:
      - target=ts
````

## File: .agents/skills/protobuf/assets/buf.gen.yaml
````yaml
# buf.gen.yaml - Buf code generation configuration
# Documentation: https://buf.build/docs/configuration/v2/buf-gen-yaml

version: v2

# Managed mode: centralizes package naming, keeps .proto files language-agnostic
managed:
  enabled: true
  override:
    - file_option: go_package_prefix
      value: github.com/yourorg/yourrepo/gen
    - file_option: java_package_prefix
      value: com.yourorg.yourrepo

plugins:
  # Go
  - remote: buf.build/protocolbuffers/go
    out: gen
    opt:
      - paths=source_relative

  # Go gRPC
  - remote: buf.build/grpc/go
    out: gen
    opt:
      - paths=source_relative

  # TypeScript (uncomment if needed)
  # - remote: buf.build/connectrpc/es
  #   out: gen/ts
  #   opt:
  #     - target=ts

  # Python (uncomment if needed)
  # - remote: buf.build/protocolbuffers/python
  #   out: gen/python
````

## File: .agents/skills/protobuf/assets/buf.lock
````
# Generated by buf. DO NOT EDIT.
version: v2
deps:
  - name: buf.build/bufbuild/protovalidate
    commit: 2a1774d888024a9b93ce7eb4b59f6a83
    digest: b5:6b7f9bc919b65e5b79d7b726ffc03d6f815a412d6b792970fa6f065cae162107bd0a9d47272c8ab1a2c9514e87b13d3fbf71df614374d62d2183afb64be2d30a
````

## File: .agents/skills/protobuf/assets/buf.yaml
````yaml
# buf.yaml - Buf configuration file
# Documentation: https://buf.build/docs/configuration/v2/buf-yaml

version: v2
modules:
  - path: proto
deps:
  - buf.build/bufbuild/protovalidate
lint:
  use:
    - STANDARD
breaking:
  use:
    - FILE
````

## File: .agents/skills/protobuf/assets/Makefile.example
````
.PHONY: all lint format generate breaking deps

all: format lint generate

lint:
	buf lint

format:
	buf format -w

generate:
	buf generate

breaking:
	buf breaking --against '.git#branch=main'

deps:
	buf dep update
````

## File: .agents/skills/protobuf/assets/proto/example/v1/book_service.proto
````protobuf
// Book service demonstrating comprehensive CRUD patterns.
syntax = "proto3";

package example.v1;

import "buf/validate/validate.proto";
import "example/v1/book.proto";

// BookService manages books in the library catalog.
service BookService {
  // GetBook retrieves a single book by ID or ISBN.
  rpc GetBook(GetBookRequest) returns (GetBookResponse);

  // ListBooks retrieves a paginated list of books.
  rpc ListBooks(ListBooksRequest) returns (ListBooksResponse);

  // CreateBook adds a new book to the catalog.
  rpc CreateBook(CreateBookRequest) returns (CreateBookResponse);

  // UpdateBook modifies an existing book.
  rpc UpdateBook(UpdateBookRequest) returns (UpdateBookResponse);

  // DeleteBook removes a book from the catalog.
  rpc DeleteBook(DeleteBookRequest) returns (DeleteBookResponse);

  // BatchGetBooks retrieves multiple books by ID or ISBN.
  rpc BatchGetBooks(BatchGetBooksRequest) returns (BatchGetBooksResponse);

  // BatchCreateBooks adds multiple books in a single request.
  rpc BatchCreateBooks(BatchCreateBooksRequest) returns (BatchCreateBooksResponse);
}

message GetBookRequest {
  // The book to retrieve.
  BookRef ref = 1 [(buf.validate.field).required = true];
}

message GetBookResponse {
  // The requested book.
  Book book = 1;
}

message ListBooksRequest {
  // The list order.
  enum Order {
    ORDER_UNSPECIFIED = 0;
    // Order by create_time newest to oldest.
    ORDER_CREATE_TIME_DESC = 1;
    // Order by create_time oldest to newest.
    ORDER_CREATE_TIME_ASC = 2;
    // Order by update_time newest to oldest.
    ORDER_UPDATE_TIME_DESC = 3;
    // Order by update_time oldest to newest.
    ORDER_UPDATE_TIME_ASC = 4;
  }

  // The maximum number of items to return.
  //
  // The default value is 20.
  uint32 page_size = 1 [(buf.validate.field).uint32.lte = 100];

  // The page to start from.
  //
  // If empty, the first page is returned.
  string page_token = 2 [(buf.validate.field).string.max_len = 4096];

  // Filter by genre.
  //
  // If not specified, all genres are returned.
  Genre genre = 3 [(buf.validate.field).enum.defined_only = true];

  // The order to return results.
  //
  // If not specified, defaults to ORDER_CREATE_TIME_DESC.
  Order order = 4 [(buf.validate.field).enum.defined_only = true];
}

message ListBooksResponse {
  // The next page token.
  //
  // If empty, there are no more pages.
  string next_page_token = 1 [(buf.validate.field).string.max_len = 4096];

  // The books.
  repeated Book books = 2 [(buf.validate.field).repeated.max_items = 100];
}

message CreateBookRequest {
  // Title of the book.
  string title = 1 [(buf.validate.field).string.min_len = 1];

  // ISBN-13 identifier.
  string isbn = 2 [(buf.validate.field).string.pattern = "^(97[89])?\\d{9}(\\d|X)$"];

  // Reference to the author.
  string author_id = 3 [(buf.validate.field).string.uuid = true];

  // Publication year.
  int32 publication_year = 4 [(buf.validate.field).int32 = {
    gte: 1450
    lte: 2100
  }];

  // Genre classifications.
  repeated Genre genres = 5 [
    (buf.validate.field).repeated.items.enum.defined_only = true,
    (buf.validate.field).repeated.items.enum.not_in = 0
  ];
}

message CreateBookResponse {
  // The created book with server-generated fields populated.
  Book book = 1;
}

message UpdateBookRequest {
  // The book to update.
  BookRef ref = 1 [(buf.validate.field).required = true];

  // Title of the book.
  optional string title = 2 [(buf.validate.field).string.min_len = 1];

  // ISBN-13 identifier.
  optional string isbn = 3 [(buf.validate.field).string.pattern = "^(97[89])?\\d{9}(\\d|X)$"];

  // Reference to the author.
  optional string author_id = 4 [(buf.validate.field).string.uuid = true];

  // Publication year.
  optional int32 publication_year = 5 [(buf.validate.field).int32 = {
    gte: 1450
    lte: 2100
  }];

  // Genre classifications. Replaces existing genres when set.
  repeated Genre genres = 6 [
    (buf.validate.field).repeated.items.enum.defined_only = true,
    (buf.validate.field).repeated.items.enum.not_in = 0
  ];
}

message UpdateBookResponse {
  // The updated book.
  Book book = 1;
}

message DeleteBookRequest {
  // The book to delete.
  BookRef ref = 1 [(buf.validate.field).required = true];
}

message DeleteBookResponse {}

message BatchGetBooksRequest {
  // Books to retrieve. Max 100.
  repeated BookRef refs = 1 [
    (buf.validate.field).repeated.min_items = 1,
    (buf.validate.field).repeated.max_items = 100
  ];
}

message BatchGetBooksResponse {
  // Books in the same order as requested refs.
  // Missing books are omitted (check count matches request).
  repeated Book books = 1;
}

message BatchCreateBooksRequest {
  // Books to create. Max 100.
  repeated CreateBookRequest requests = 1 [
    (buf.validate.field).repeated.min_items = 1,
    (buf.validate.field).repeated.max_items = 100
  ];
}

message BatchCreateBooksResponse {
  // Created books in the same order as requests.
  repeated Book books = 1;
}
````

## File: .agents/skills/protobuf/assets/proto/example/v1/book.proto
````protobuf
// Book entity and related types.
//
// This file contains the Book entity and supporting types, separate from the
// service definition to allow reuse across multiple services.

syntax = "proto3";

package example.v1;

import "buf/validate/validate.proto";
import "google/protobuf/timestamp.proto";

// Book represents a book in the library catalog.
message Book {
  // Unique identifier for the book.
  string id = 1 [(buf.validate.field).string.uuid = true];

  // Title of the book.
  string title = 2 [(buf.validate.field).string.min_len = 1];

  // ISBN-13 identifier.
  string isbn = 3 [(buf.validate.field).string.pattern = "^(97[89])?\\d{9}(\\d|X)$"];

  // Reference to the author.
  string author_id = 4 [(buf.validate.field).string.uuid = true];

  // Publication year.
  int32 publication_year = 5 [(buf.validate.field).int32 = {
    gte: 1450
    lte: 2100
  }];

  // Genre classifications.
  repeated Genre genre = 6 [
    (buf.validate.field).repeated.items.enum.defined_only = true,
    (buf.validate.field).repeated.items.enum.not_in = 0
  ];

  // When the book was added to the catalog.
  google.protobuf.Timestamp create_time = 7 [(buf.validate.field).required = true];

  // When the book was last updated.
  google.protobuf.Timestamp update_time = 8;
}

// BookRef identifies a book by ID or ISBN.
message BookRef {
  oneof ref {
    option (buf.validate.oneof).required = true;

    // Unique identifier for the book.
    string id = 1 [(buf.validate.field).string.uuid = true];

    // ISBN-13 identifier.
    string isbn = 2 [(buf.validate.field).string.pattern = "^(97[89])?\\d{9}(\\d|X)$"];
  }
}

// Genre classification for books.
enum Genre {
  GENRE_UNSPECIFIED = 0;
  GENRE_FICTION = 1;
  GENRE_NON_FICTION = 2;
  GENRE_SCIENCE_FICTION = 3;
  GENRE_MYSTERY = 4;
  GENRE_BIOGRAPHY = 5;
}
````

## File: .agents/skills/protobuf/references/best_practices.md
````markdown
# Protocol Buffers Best Practices

Universal best practices for designing `.proto` files.
For buf CLI configuration, see [buf_toolchain.md](buf_toolchain.md).

**Validation:** Use [protovalidate](protovalidate.md) on every field—it makes the schema the single source of truth for both structure and constraints.

## Contents

- [File Structure](#file-structure)
- [Package Naming](#package-naming)
- [Message Design](#message-design)
- [Field Numbering](#field-numbering)
- [Scalar Type Selection](#scalar-type-selection)
- [Field Presence](#field-presence)
- [Enums](#enums)
- [Oneof](#oneof)
- [Maps](#maps)
- [Well-Known Types](#well-known-types)
- [Services and RPCs](#services-and-rpcs)
- [Schema Evolution](#schema-evolution)
- [Documentation](#documentation)
- [Common Patterns](#common-patterns)
- [API Design Principles](#api-design-principles)
- [Commit Generated Code](#commit-generated-code)

---

## File Structure

- Use `lower_snake_case.proto`
- Directory structure should match package: `acme/user/v1/user.proto` for `acme.user.v1`
- Keep files focused—large files with many unrelated messages cause dependency bloat
- Line length: soft limit 80 characters, hard limit 120 characters
- 2-space indentation
- Use `//` comments over `/* */`

### Element Ordering Within Files

Order elements consistently within each proto file:
1. License header (if applicable)
2. File-level documentation (if applicable)
3. Syntax declaration
4. Package declaration
5. Import statements
6. Options
7. Services (if any)
8. Messages (with related Ref messages immediately after their entity)
9. Enums (standalone, not nested in messages)

This ordering ensures services (the primary API contract) appear first, and keeps
reference messages close to their entities.

## Package Naming

- Use dot-delimited lowercase: `company.product.domain.version`
- Always include version suffix: `acme.user.v1`, `acme.billing.v2alpha1`
- Avoid language keywords in package names (e.g., `internal` blocks Go imports)
- Files in the same package must be in the same directory

```protobuf
syntax = "proto3";

package acme.user.v1;
```

## Message Design

### Naming

- Use `PascalCase`: `UserProfile`, `OrderItem`
- Use singular nouns for non-repeated entities
- Be specific: `CreateUserRequest` not `Request`
- Treat abbreviations as single words: `GetDnsRequest` not `GetDNSRequest`

### Field Naming

- Use `snake_case`: `user_id`, `created_at`
- Avoid abbreviations: `message` not `msg`
- Pluralize repeated fields: `repeated User users`
- Add [protovalidate](protovalidate.md) constraints to every field—format validators, length/range bounds, enum constraints, and required markers are all part of the field definition

### Nesting

Define messages and enums at the top level unless exclusively used by the parent.
Exception: request-scoped types like `ListBooksRequest.OrderBy` that have no external meaning.

## Field Numbering

### Wire Encoding Efficiency

- **1-15**: Use for frequently-set fields (1 byte tag encoding)
- **16-2047**: Standard fields (2 byte tag encoding)
- **19000-19999**: Reserved by Protocol Buffers—never use
- **536870912-max**: Reserved—never use

### Never Reuse Field Numbers

When a field is removed, reserve both the number and name:

```protobuf
message User {
  reserved 4, 8 to 10;
  reserved "legacy_field", "old_status";

  string id = 1;
  string email = 2;
  string name = 3;
  string display_name = 5;
}
```

Reusing field numbers causes data corruption when decoding messages serialized with old schemas.

## Scalar Type Selection

| Use Case | Type | Notes |
|----------|------|-------|
| Regular integers | `int32`, `int64` | Default choice |
| Likely negative values | `sint32`, `sint64` | More efficient encoding |
| Large positive values (>2^28) | `fixed32`, `fixed64` | Constant 4/8 bytes |
| Arbitrary bytes | `bytes` | Not for text |
| Text | `string` | Must be UTF-8 |
| Signed integers | Use signed types | Many languages lack unsigned support |

### Unsigned vs Signed Types

Prefer signed types (`int32`/`int64`) for public APIs since Java and JavaScript lack unsigned support.
For internal APIs, unsigned types are fine.
When using signed types for non-negative values, add validation: `(buf.validate.field).int32.gte = 0`

## Field Presence

Proto3 has three presence modes:

1. **Implicit presence** (default for scalars): Cannot distinguish unset from zero value
2. **Explicit presence** (`optional` keyword): Can detect if field was set
3. **Message fields**: Always have presence semantics

```protobuf
message Example {
  string name = 1;              // Implicit: "" means unset or empty
  optional string nickname = 2; // Explicit: can detect unset vs ""
  Address address = 3;          // Message: has presence (null vs empty)
}
```

Use `optional` when distinguishing "not provided" from "provided as empty/zero" matters.

## Enums

```protobuf
enum UserStatus {
  USER_STATUS_UNSPECIFIED = 0;
  USER_STATUS_ACTIVE = 1;
  USER_STATUS_INACTIVE = 2;
  USER_STATUS_SUSPENDED = 3;
}
```

### Rules

- First value must be zero and named `*_UNSPECIFIED` or `*_UNKNOWN`
- The zero value should have no semantic meaning—it indicates "not set"
- Prefix all values with the enum name in `UPPER_SNAKE_CASE` to avoid collisions
- Never remove enum values—reserve them instead
- Do not use `allow_alias`

### Deprecation

```protobuf
enum Status {
  STATUS_UNSPECIFIED = 0;
  STATUS_ACTIVE = 1;
  STATUS_LEGACY = 2 [deprecated = true];

  reserved 3;
  reserved "STATUS_REMOVED";
}
```

### Enum Value Documentation

Place comments above each enum value, not inline (inline comments often don't appear in generated docs).
Avoid grouping comments above multiple values; comments only attach to the first value.

### Required Enum Fields

For enum fields that must have a meaningful value (not UNSPECIFIED), always use **both** `not_in = 0` and `defined_only = true`:

```protobuf
Status status = 1 [
  (buf.validate.field).enum.not_in = 0,        // rejects UNSPECIFIED
  (buf.validate.field).enum.defined_only = true // rejects unknown values
];
```

Using only one leaves a gap: `defined_only` alone allows UNSPECIFIED; `not_in = 0` alone allows unknown values.
Optional enum fields where zero means "no preference" should use only `defined_only = true`.
See [protovalidate.md](protovalidate.md#enum-rules) for the full pattern table.

## Oneof

Use `oneof` when exactly one of several fields should be set:

```protobuf
message SearchQuery {
  oneof query {
    option (buf.validate.oneof).required = true;
    string text = 1;
    int64 id = 2;
    EmailFilter email = 3;
  }
}
```

**Validation:** Add `(buf.validate.oneof).required = true` for required choices. See [protovalidate.md](protovalidate.md#oneof-rules).

**Behavior:** Setting any member clears all others. Cannot distinguish "not set" from "set to removed field" across versions.

**Evolution:** Adding fields to existing oneof is safe. Moving existing fields into a oneof or removing fields is unsafe.

## Maps

```protobuf
message Project {
  map<string, string> labels = 1;
  map<string, User> members = 2;
}
```

### Map Constraints

- Keys: integral or string types only (no floats, bytes, enums, or messages)
- Values: any type except maps
- Iteration order is undefined

## Well-Known Types

Prefer standard types from `google/protobuf`:

| Type | Use For |
|------|---------|
| `google.protobuf.Timestamp` | Points in time |
| `google.protobuf.Duration` | Time spans |
| `google.protobuf.FieldMask` | Partial updates |
| `google.protobuf.Struct` | Dynamic JSON-like data |
| `google.protobuf.Any` | Arbitrary message types |

**Note:** Prefer custom empty messages over `google.protobuf.Empty` for extensibility.

## Services and RPCs

### Naming

- Service names: `PascalCase`, typically with `Service` suffix
- RPC names: `PascalCase` verbs: `GetUser`, `CreateOrder`, `ListProducts`
- Be specific: `ActivateUser` not `SetUserStatus`

### Request/Response Messages

- Name as `MethodNameRequest` and `MethodNameResponse`
- Each RPC should have unique request/response types (enables future evolution)
- Avoid reusing request types across RPCs
- Every request field should have protovalidate constraints—requests are the primary system boundary where validation matters most

```protobuf
service UserService {
  rpc GetUser(GetUserRequest) returns (GetUserResponse);
  rpc ListUsers(ListUsersRequest) returns (ListUsersResponse);
  rpc CreateUser(CreateUserRequest) returns (CreateUserResponse);
}
```

## Schema Evolution

### Adding Fields

Add new fields with the next available field number and appropriate protovalidate constraints:

```protobuf
message User {
  string id = 1 [(buf.validate.field).string.uuid = true];
  string email = 2 [(buf.validate.field).string.email = true];
  string name = 3 [(buf.validate.field).string.min_len = 1];
  string phone = 4;  // New field - add validation constraints
}
```

Use numbers 1-15 for common fields (1-byte encoding).
Avoid gaps in field numbers—gaps make it unclear whether a field was intentionally skipped or removed without reservation.

### Deprecating and Removing Fields

Mark fields as deprecated to warn consumers:

```protobuf
message User {
  string id = 1;
  // Deprecated: Use display_name instead.
  string name = 2 [deprecated = true];
  string display_name = 3;
}
```

Keep deprecated fields indefinitely for published APIs.
For internal APIs, you may eventually remove and reserve after migration:

```protobuf
message User {
  reserved 2;
  reserved "name";

  string id = 1;
  string display_name = 3;
}
```

### Replacing Fields with Different Types

Add a new field rather than modifying the existing one:

```protobuf
message User {
  string role = 3 [deprecated = true];  // Keep for compatibility
  UserRole user_role = 4;               // New typed field
}

enum UserRole {
  USER_ROLE_UNSPECIFIED = 0;
  USER_ROLE_ADMIN = 1;
  USER_ROLE_MEMBER = 2;
}
```

### Versioning Strategy

**Evolve within a version** for backwards-compatible changes (adding fields, enum values, methods; deprecating elements).

**Create new version** (`v1` -> `v2`) for breaking changes (type changes, semantic changes, restructuring).

### Testing Schema Evolution

Use `buf breaking --against '.git#branch=main'` to catch unintentional breaking changes.

## Documentation

Document all public APIs:

```protobuf
// UserService provides user management operations.
service UserService {
  // GetUser retrieves a user by their unique identifier.
  //
  // Returns NOT_FOUND if the user does not exist.
  rpc GetUser(GetUserRequest) returns (GetUserResponse);
}

// User represents a registered user account.
message User {
  // Unique identifier for the user. Format: UUID v4
  string id = 1;
}
```

- Use complete sentences with proper punctuation
- Document constraints, formats, and valid ranges
- Note error conditions on RPCs
- Skip comments on request/response messages (names are self-documenting); document fields within them

### Consistency

- Pick one form of a term and use it everywhere ("shortname" vs "short name" — pick one)
- Use "ID" not "id" or "Id" in comments
- Watch article/vowel mismatches: "an Environment" not "a Environment"

### Cross-Reference Consistency

When the same identifier appears in multiple messages, all validation constraints must be identical.
See [protovalidate.md](protovalidate.md#cross-reference-consistency) for examples.

## Common Patterns

### Pagination and List Requests

```protobuf
message ListUsersRequest {
  uint32 page_size = 1 [(buf.validate.field).uint32.lte = 250];
  string page_token = 2 [(buf.validate.field).string.max_len = 4096];
}

message ListUsersResponse {
  string next_page_token = 1 [(buf.validate.field).string.max_len = 4096];
  repeated User users = 2 [(buf.validate.field).repeated.max_items = 250];
}
```

See `assets/proto/example/v1/book_service.proto` for a complete example with ordering and filtering.

### Partial Updates with Field Masks

```protobuf
import "google/protobuf/field_mask.proto";

message UpdateUserRequest {
  User user = 1;
  google.protobuf.FieldMask update_mask = 2;
}
```

### Resource Metadata

```protobuf
message User {
  string id = 1;
  string email = 2;

  google.protobuf.Timestamp create_time = 10;
  google.protobuf.Timestamp update_time = 11;
}
```

### Soft Delete

```protobuf
message User {
  string id = 1;
  // Set when the user is soft-deleted. Null if active.
  google.protobuf.Timestamp delete_time = 20;
}
```

### Embedding vs Reference

Embed when data is small and always needed together (`Address shipping_address = 1`).
Reference by ID when data is large, optional, or separately managed (`string user_id = 1`).

### Reference Messages (Ref Pattern)

Use `*Ref` messages for flexible entity lookups.
Place immediately after the entity in the same file.
Each oneof field must be a unique identifier.

```protobuf
message UserRef {
  oneof value {
    option (buf.validate.oneof).required = true;
    string id = 1 [(buf.validate.field).string.uuid = true];
    string email = 2 [(buf.validate.field).string.email = true];
  }
}
```

### Create and Update Request Patterns

**Flat Fields (Recommended):** Request contains only user-modifiable fields directly.
Clear validation, but duplicates fields between entity and request.

```protobuf
message CreateUserRequest {
  string name = 1 [(buf.validate.field).required = true];
  string email = 2 [(buf.validate.field).required = true];
}

message UpdateUserRequest {
  string id = 1 [(buf.validate.field).string.uuid = true];
  optional string name = 2;
  optional string email = 3;
}
```

**Entity with FieldMask ([AIP-134](https://google.aip.dev/134)):** Reuses entity definition.
Skip validation on entity field: `User user = 1 [(buf.validate.field).ignore = IGNORE_ALWAYS];`

## API Design Principles

### Enums Over Booleans

Use enums instead of booleans when state might expand:

```protobuf
// Instead of: bool active = 1;
message User {
  UserStatus status = 1;
}

enum UserStatus {
  USER_STATUS_UNSPECIFIED = 0;
  USER_STATUS_ACTIVE = 1;
  USER_STATUS_INACTIVE = 2;
  USER_STATUS_SUSPENDED = 3;
}
```

### Oneofs for Mutually Exclusive State

Use `oneof` to model mutually exclusive states explicitly:

```protobuf
message PaymentMethod {
  oneof method {
    CreditCard credit_card = 1;
    BankAccount bank_account = 2;
    PayPalAccount paypal = 3;
  }
}
```

This makes the API self-documenting and prevents invalid states at the schema level.

### Format Validation Implies Presence

Many format validations reject empty strings, making `required = true` redundant.
Examples include `uuid`, `email`, `uri`, `hostname`, `ip`, and others.

```protobuf
// Sufficient - format validation handles presence
string id = 1 [(buf.validate.field).string.uuid = true];
string email = 2 [(buf.validate.field).string.email = true];
```

## Commit Generated Code

Commit generated proto code to version control.
This enables direct imports without running `buf generate`, reproducible builds, and immediate IDE support.
Do NOT add generated proto code to `.gitignore`.
````

## File: .agents/skills/protobuf/references/buf_toolchain.md
````markdown
# Buf Toolchain Reference

Reference for the buf CLI and configuration files.
For proto design patterns, see [best_practices.md](best_practices.md).

**Documentation:** [buf.build/docs](https://buf.build/docs)

## Contents

- [CLI Commands](#cli-commands)
- [buf.yaml Configuration](#bufyaml-configuration)
- [buf.gen.yaml Configuration](#bufgenyaml-configuration)
- [buf.lock](#buflock)
- [Project Structure](#project-structure)
- [BSR (Buf Schema Registry)](#bsr-buf-schema-registry)
- [Build Integration](#build-integration)

---

## CLI Commands

### Quick Reference

| Task | Command |
|------|---------|
| Lint | `buf lint` |
| Format (check) | `buf format --diff` |
| Format (apply) | `buf format -w` |
| Breaking check | `buf breaking --against '.git#branch=main'` |
| Generate | `buf generate` |
| Build | `buf build` |
| Push to BSR | `buf push` |
| Update deps | `buf dep update` |
| Prune deps | `buf dep prune` |
| Login | `buf registry login` |

**Check project Makefile/package.json first**—projects often wrap these commands with project-specific options.

### Breaking Change Detection

```bash
# Compare against git main branch
buf breaking --against '.git#branch=main'

# Compare against remote module
buf breaking --against 'buf.build/acme/api'

# Compare against specific git commit
buf breaking --against '.git#ref=abc123'

# Compare local directories
buf breaking --against ../previous-version
```

## buf.yaml Configuration

The `buf.yaml` file configures modules, linting, and breaking change detection.

### Basic Structure

```yaml
version: v2
modules:
  - path: proto
    name: buf.build/yourorg/yourmodule
lint:
  use:
    - STANDARD
breaking:
  use:
    - FILE
```

### Lint Configuration

The STANDARD rule set includes PROTOVALIDATE rules—no need to add both:

```yaml
# Correct
lint:
  use:
    - STANDARD

# Redundant
lint:
  use:
    - STANDARD
    - PROTOVALIDATE  # Already included in STANDARD
```

Additional options:

```yaml
lint:
  use:
    - STANDARD           # Recommended baseline
    - COMMENTS           # Require comments on all public elements
    - UNARY_RPC          # Disallow streaming RPCs

  except:
    - PACKAGE_VERSION_SUFFIX  # Allow packages without version

  ignore:
    - proto/internal          # Ignore paths

  ignore_only:
    ENUM_VALUE_PREFIX:
      - proto/legacy          # Ignore specific rule in specific paths

  # Prevent buf:lint:ignore comments
  disallow_comment_ignores: true
```

### Breaking Change Configuration

```yaml
breaking:
  use:
    - FILE        # Strictest: detects file-level breaks
    # - PACKAGE   # Package-level (allows moving between files)
    # - WIRE_JSON # Wire + JSON encoding breaks only
    # - WIRE      # Wire encoding breaks only

  except:
    - FIELD_SAME_JSON_NAME  # Allow JSON name changes

  ignore:
    - proto/internal        # Ignore paths
```

### Multi-Module Workspace

```yaml
version: v2
modules:
  - path: proto/public
    name: buf.build/yourorg/public-api
  - path: proto/internal
    name: buf.build/yourorg/internal-api
```

### Dependencies

```yaml
version: v2
modules:
  - path: proto
deps:
  - buf.build/googleapis/googleapis
  - buf.build/bufbuild/protovalidate
```

After adding dependencies, run `buf dep update` to generate/update `buf.lock`.

## buf.gen.yaml Configuration

The `buf.gen.yaml` file configures code generation.

### Basic Structure with Remote Plugins

Remote plugins are the recommended default—no local installation needed:

```yaml
version: v2
plugins:
  - remote: buf.build/protocolbuffers/go
    out: gen
    opt:
      - paths=source_relative
```

### Managed Mode

Managed mode centralizes package naming configuration, keeping `.proto` files language-agnostic.
This eliminates the need for `go_package`, `java_package`, etc. in proto files.

```yaml
version: v2
managed:
  enabled: true
  override:
    - file_option: go_package_prefix
      value: github.com/yourorg/yourrepo/gen
```

**Common managed mode options:**

| Option | Description |
|--------|-------------|
| `go_package_prefix` | Prefix for Go import paths |
| `java_package_prefix` | Prefix for Java packages (prepended to proto package) |
| `java_multiple_files` | Generate separate file per message (recommended: `true`) |
| `csharp_namespace_prefix` | Prefix for C# namespaces |
| `ruby_package_suffix` | Suffix for Ruby packages |
| `objc_class_prefix` | Prefix for Objective-C classes |

### Language-Specific Configurations

#### Go with Connect (Recommended)

```yaml
version: v2
managed:
  enabled: true
  override:
    - file_option: go_package_prefix
      value: github.com/yourorg/api/gen
plugins:
  - remote: buf.build/protocolbuffers/go
    out: gen
    opt:
      - paths=source_relative
  - remote: buf.build/connectrpc/go
    out: gen
    opt:
      - paths=source_relative
```

**Variants:**
- **gRPC:** Replace `buf.build/connectrpc/go` with `buf.build/grpc/go`
- **Opaque API (Go 1.21+):** Add `default_api_level=API_OPAQUE` to protocolbuffers/go options
- **With protovalidate:** Add `disable` block for `buf.build/bufbuild/protovalidate` to prevent managed mode from overwriting its go_package

#### TypeScript with Connect

```yaml
version: v2
plugins:
  - remote: buf.build/bufbuild/es
    out: gen
    opt:
      - target=ts
  - remote: buf.build/connectrpc/es
    out: gen
    opt:
      - target=ts
```

Options: `target=ts` (TypeScript), `target=js` (CommonJS), `target=js+dts` (JS + declarations)

#### Python with gRPC

```yaml
version: v2
plugins:
  - remote: buf.build/protocolbuffers/python
    out: gen
  - remote: buf.build/grpc/python
    out: gen
```

Add `buf.build/protocolbuffers/pyi` for type stubs.

#### Java with gRPC

```yaml
version: v2
managed:
  enabled: true
  override:
    - file_option: java_package_prefix
      value: com.yourorg.api
    - file_option: java_multiple_files
      value: true
plugins:
  - remote: buf.build/protocolbuffers/java
    out: gen
  - remote: buf.build/grpc/java
    out: gen
```

**Kotlin:** Use `buf.build/grpc/kotlin` instead of `buf.build/grpc/java`.

#### Other Languages

For C#, Swift, Rust, and other languages, see [buf.build/docs/generate/overview](https://buf.build/docs/generate/overview).

### Local Plugins

Use local plugins when remote plugins aren't available or for custom generators:

```yaml
plugins:
  - local: protoc-gen-custom
    out: gen
```

### Input Filtering

Control which protos are generated:

```yaml
version: v2
inputs:
  - directory: proto
    paths:
      - acme/user/v1      # Include only this package
    exclude_paths:
      - acme/internal     # Exclude this package

plugins:
  - remote: buf.build/protocolbuffers/go
    out: gen
```

### Clean Output Directory

Delete generated files before regenerating:

```yaml
version: v2
clean: true
plugins:
  - remote: buf.build/protocolbuffers/go
    out: gen
```

The `clean: true` option removes stale generated files before regenerating. This prevents
orphaned files when protos are renamed or deleted.

**Caution:** Only use `clean: true` if the output directory contains exclusively generated
code—it will delete all existing files in the output directory before regeneration.

## buf.lock

Auto-generated by `buf dep update`. Locks dependency versions for reproducible builds.

- Commit `buf.lock` to version control
- Run `buf dep update` when changing deps in `buf.yaml`
- Run `buf dep prune` to remove unused deps

## Project Structure

### Single Module

```
project/
├── buf.yaml
├── buf.gen.yaml
├── buf.lock
└── proto/
    └── acme/
        └── user/
            └── v1/
                ├── user.proto
                └── user_service.proto
```

### Multi-Module Workspace

```
project/
├── buf.yaml           # Workspace config with multiple modules
├── buf.gen.yaml
├── buf.lock
├── proto/
│   ├── public/        # Public API module
│   │   └── acme/
│   │       └── api/
│   │           └── v1/
│   └── internal/      # Internal module
│       └── acme/
│           └── internal/
│               └── v1/
```

## BSR (Buf Schema Registry)

### Module Naming

BSR modules follow the pattern: `buf.build/<owner>/<repository>`

- `buf.build/googleapis/googleapis` - Google APIs
- `buf.build/bufbuild/protovalidate` - Protovalidate
- `buf.build/yourorg/yourapi` - Your organization's API

### Consuming BSR Modules

Add to `buf.yaml` dependencies:

```yaml
deps:
  - buf.build/googleapis/googleapis
  - buf.build/bufbuild/protovalidate
```

Import in proto files:

```protobuf
import "google/api/annotations.proto";
import "buf/validate/validate.proto";
```

### Publishing to BSR

1. Configure module name in `buf.yaml`:
   ```yaml
   modules:
     - path: proto
       name: buf.build/yourorg/yourapi
   ```

2. Authenticate: `buf registry login`

3. Push: `buf push`

## Build Integration

**Check for existing commands first.** Many projects use Makefile, package.json scripts, or other build tools to wrap buf commands with project-specific options.

Common patterns:
- `make lint` / `make format` / `make generate`
- `npm run proto:lint` / `npm run proto:generate`
- Task runners, Bazel, etc.

Example Makefile if none exists: [Makefile.example](../assets/Makefile.example)
````

## File: .agents/skills/protobuf/references/examples.md
````markdown
# Protocol Buffers Examples

Complete service examples are in [assets/proto/example/v1/](../assets/proto/example/v1/).
Copy and adapt them as templates for your project.

## Service Template

### [book.proto](../assets/proto/example/v1/book.proto)

Entity definition demonstrating:
- Entity message with protovalidate constraints (uuid, patterns, ranges)
- BookRef oneof for flexible lookups (by ID or ISBN)
- Required oneof validation
- Enum with unspecified default
- Timestamp fields (create_time, update_time)

### [book_service.proto](../assets/proto/example/v1/book_service.proto)

A comprehensive CRUD service demonstrating:
- Standard and batch operations (Get, List, Create, Update, Delete, BatchGet, BatchCreate)
- BookRef pattern for lookups by ID or ISBN
- Protovalidate constraints (required, repeated bounds, enum.defined_only)
- Pagination with page_token and page_size
- Ordering with nested enum
- Field masks for partial updates
- Entity/service separation pattern

## Language Options

These examples omit language-specific file options (`go_package`, `java_package`, etc.).
Configure these via managed mode in `buf.gen.yaml`—see [buf_toolchain.md](buf_toolchain.md#managed-mode).
````

## File: .agents/skills/protobuf/references/migration.md
````markdown
# Migrating from Protoc to Buf

Guide for converting protoc-based projects to buf.
Based on the [official migration guide](https://buf.build/docs/migration-guides/migrate-from-protoc/).

## Contents

- [Key Concept: Include Paths to Modules](#key-concept-include-paths-to-modules)
- [Step 1: Create buf.yaml](#step-1-create-bufyaml)
- [Step 2: Verify Compilation](#step-2-verify-compilation)
- [Step 3: Create buf.gen.yaml](#step-3-create-bufgenyaml)
- [Step 4: Generate Code](#step-4-generate-code)
- [Step 5: Replace Vendored Dependencies with BSR](#step-5-replace-vendored-dependencies-with-bsr)
- [Step 6: Enable Managed Mode](#step-6-enable-managed-mode)
- [Step 7: Use Remote Plugins](#step-7-use-remote-plugins)
- [Step 8: Update Build Scripts](#step-8-update-build-scripts)
- [Migration Checklist](#migration-checklist)
- [Troubleshooting](#troubleshooting)

---

## Key Concept: Include Paths to Modules

The fundamental shift: protoc's `-I` include paths become Buf's module paths. With buf, there is no `-I` flag—each protoc `-I` path maps to a `path` field in buf.yaml.

## Step 1: Create buf.yaml

Place a `buf.yaml` at your workspace root. Convert each `-I` path to a module path:

**Before (protoc):**
```bash
protoc \
  -I proto \
  -I vendor/googleapis \
  -I vendor/protoc-gen-validate \
  --go_out=gen ...
```

**After (buf.yaml):**
```yaml
version: v2
modules:
  - path: proto
  - path: vendor/googleapis
  - path: vendor/protoc-gen-validate
lint:
  use:
    - STANDARD
breaking:
  use:
    - FILE
```

## Step 2: Verify Compilation

Test that your workspace compiles:

```bash
buf build
```

This discovers `.proto` files, compiles them in memory, and validates the build succeeds.

## Step 3: Create buf.gen.yaml

Replace protoc plugin invocations with buf.gen.yaml:

**Before (protoc):**
```bash
protoc \
  -I proto \
  -I vendor/googleapis \
  --go_out=gen \
  --go_opt=paths=source_relative \
  --go-grpc_out=gen \
  --go-grpc_opt=paths=source_relative \
  proto/**/*.proto
```

**After (buf.gen.yaml):**
```yaml
version: v2
plugins:
  - local: protoc-gen-go
    out: gen
    opt:
      - paths=source_relative
  - local: protoc-gen-go-grpc
    out: gen
    opt:
      - paths=source_relative
```

## Step 4: Generate Code

```bash
buf generate
```

## Step 5: Replace Vendored Dependencies with BSR

Once basic migration works, replace vendored third-party protos with BSR dependencies:

```yaml
# buf.yaml
version: v2
modules:
  - path: proto
deps:
  - buf.build/googleapis/googleapis
  - buf.build/bufbuild/protovalidate
```

Then run:
```bash
buf dep update
```

Delete the vendored directories after confirming generation still works.

## Step 6: Enable Managed Mode

Managed mode is the recommended approach—it lets you remove language-specific options from proto files:

```yaml
# buf.gen.yaml
version: v2
managed:
  enabled: true
  override:
    - file_option: go_package_prefix
      value: github.com/yourorg/api/gen
plugins:
  - remote: buf.build/protocolbuffers/go
    out: gen
    opt:
      - paths=source_relative
```

With managed mode enabled, you can remove `go_package` options from your proto files.

## Step 7: Use Remote Plugins

Remote plugins are the recommended default—no local installation needed:

```yaml
plugins:
  # Before: local plugin
  - local: protoc-gen-go
    out: gen

  # After: remote plugin (no installation needed)
  - remote: buf.build/protocolbuffers/go
    out: gen
```

## Step 8: Update Build Scripts

**Before:**
```makefile
generate:
	./scripts/generate.sh
```

**After:**
```makefile
.PHONY: all lint format generate breaking

all: format lint generate

lint:
	buf lint

format:
	buf format -w

generate:
	buf generate

breaking:
	buf breaking --against '.git#branch=main'
```

## Multiple Generation Templates

For projects with different API subsets:

```bash
buf generate proto/public --template buf.public.gen.yaml
buf generate proto/internal --template buf.internal.gen.yaml
```

## Migration Checklist

- [ ] Create `buf.yaml` with modules matching `-I` paths
- [ ] Run `buf build` to verify compilation
- [ ] Create `buf.gen.yaml` matching protoc plugin options
- [ ] Run `buf generate` and compare output
- [ ] Replace vendored deps with BSR deps (`buf dep update`)
- [ ] Enable managed mode and remove `go_package` options
- [ ] Switch to remote plugins
- [ ] Update Makefile/scripts
- [ ] Add `buf lint` and `buf breaking` to CI
- [ ] Delete vendored protos and old scripts
- [ ] Commit `buf.lock`

## Troubleshooting

### Import Not Found After Migration

Ensure all `-I` paths are represented as modules in buf.yaml. BSR dependencies use canonical paths:

```protobuf
// Vendored import
import "vendor/googleapis/google/api/annotations.proto";

// BSR import (after migration)
import "google/api/annotations.proto";
```

### Different Generated Output

Remote plugins may produce slightly different output than local versions. During migration, use local plugins for exact compatibility, then switch to remote plugins.

### go_package Still Required

If some files must keep `go_package` (e.g., files you don't control), disable managed mode for those paths:

```yaml
managed:
  enabled: true
  disable:
    - file_option: go_package
      path: vendor
```
````

## File: .agents/skills/protobuf/references/protoc_toolchain.md
````markdown
# Protoc Toolchain Reference

Reference for the `protoc` compiler and its ecosystem.
**The buf toolchain is recommended over protoc for new projects**—see [buf_toolchain.md](buf_toolchain.md).
Use this reference when working with existing protoc-based projects.

## When to Use Protoc vs Buf

**Use buf when:**
- Starting a new project
- Project already uses buf.yaml
- Need dependency management from BSR
- Need built-in linting and breaking change detection

**Use protoc when:**
- Project has existing Makefile/scripts using protoc
- Using protoc-specific plugins not available for buf
- Organization mandates protoc

**Detection signals for protoc projects:**
- Makefile with `protoc` commands
- No `buf.yaml` present
- `*.pb.go`, `*_pb2.py` files generated by protoc
- Include paths (`-I` flags) in build scripts
- `protoc-gen-*` binaries in project or PATH

## Installation

Download from [github.com/protocolbuffers/protobuf/releases](https://github.com/protocolbuffers/protobuf/releases)

Or via package manager:
- macOS: `brew install protobuf`
- Linux: `apt-get install -y protobuf-compiler`

### Common Plugins

```bash
# Go
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

# Python
pip install grpcio-tools

# TypeScript/JavaScript
npm install -g @bufbuild/protoc-gen-es
```

## Basic Usage

```bash
# Basic compilation
protoc --proto_path=proto --go_out=gen/go proto/acme/user/v1/user.proto

# Multiple outputs
protoc \
  --proto_path=proto \
  --go_out=gen/go \
  --go-grpc_out=gen/go \
  proto/acme/user/v1/*.proto

# With include paths for dependencies
protoc \
  -I proto \
  -I third_party/googleapis \
  --go_out=gen/go \
  proto/acme/user/v1/user.proto
```

### Key Flags

| Flag | Description |
|------|-------------|
| `--proto_path` / `-I` | Include path for imports (can specify multiple) |
| `--<lang>_out` | Output directory for generated code |
| `--<lang>_opt` | Options for the language plugin |
| `--descriptor_set_out` | Output binary descriptor set |

### Plugin Options

```bash
protoc \
  --proto_path=proto \
  --go_out=gen/go \
  --go_opt=paths=source_relative \
  --go-grpc_out=gen/go \
  --go-grpc_opt=paths=source_relative \
  proto/**/*.proto
```

## Include Paths

Protoc resolves imports relative to include paths (`-I` / `--proto_path`).

```bash
protoc \
  -I proto \                          # Your protos
  -I third_party \                    # Vendored dependencies
  -I /usr/local/include \             # System-installed WKTs
  --go_out=gen/go \
  proto/acme/user/v1/user.proto
```

### Well-Known Types

Google's well-known types (`google/protobuf/*.proto`) are typically installed with protoc.
If not found, check `/usr/local/include` or vendor them.

## Makefile Integration

```makefile
PROTO_DIR := proto
GEN_DIR := gen
PROTOC := protoc

PROTO_FILES := $(shell find $(PROTO_DIR) -name '*.proto')
INCLUDES := -I $(PROTO_DIR) -I third_party/googleapis -I third_party/protobuf/src

.PHONY: generate
generate: $(PROTO_FILES)
	$(PROTOC) $(INCLUDES) \
		--go_out=$(GEN_DIR)/go \
		--go_opt=paths=source_relative \
		--go-grpc_out=$(GEN_DIR)/go \
		--go-grpc_opt=paths=source_relative \
		$(PROTO_FILES)

.PHONY: clean
clean:
	rm -rf $(GEN_DIR)
```

## go_package Requirements

Protoc requires `go_package` option in `.proto` files for Go code generation:

```protobuf
syntax = "proto3";

package acme.user.v1;

option go_package = "github.com/acme/api/gen/go/acme/user/v1;userv1";
```

The format is `<import_path>;<package_name>` where:
- `import_path`: Full Go import path
- `package_name`: (Optional) Go package name, defaults to last path component

**Note:** Buf's managed mode eliminates this requirement—see [buf_toolchain.md](buf_toolchain.md).

## Language-Specific Outputs

### Go

```bash
protoc \
  --go_out=gen/go \
  --go_opt=paths=source_relative \
  --go-grpc_out=gen/go \
  --go-grpc_opt=paths=source_relative \
  proto/**/*.proto
```

**Options:**
- `paths=source_relative`: Output relative to proto file location
- `paths=import`: Output based on go_package (default)

### Python

```bash
python -m grpc_tools.protoc \
  -I proto \
  --python_out=gen/python \
  --grpc_python_out=gen/python \
  proto/**/*.proto
```

### TypeScript/JavaScript

```bash
protoc \
  -I proto \
  --es_out=gen/ts \
  --es_opt=target=ts \
  proto/**/*.proto
```

### Java

```bash
protoc \
  -I proto \
  --java_out=gen/java \
  --grpc-java_out=gen/java \
  proto/**/*.proto
```

## Descriptor Sets

Binary descriptor sets capture compiled proto schemas:

```bash
protoc \
  -I proto \
  --descriptor_set_out=schema.binpb \
  --include_imports \
  --include_source_info \
  proto/**/*.proto
```

Useful for runtime reflection, schema registries, and server reflection.

## Common Issues

See [troubleshooting.md](troubleshooting.md) for detailed solutions to:
- "Import not found" errors
- "go_package option required"
- Plugin not found
- Inconsistent generated code

---

## Migrating to Buf

For step-by-step migration instructions, see [migration.md](migration.md).
````

## File: .agents/skills/protobuf/references/protovalidate.md
````markdown
# Protovalidate Reference

Protovalidate declares validation constraints directly in `.proto` files, making the schema the single source of truth for both structure and validation.
Rules are field annotations enforced at runtime via generated code and CEL expressions.

**Documentation:** [protovalidate.com](https://protovalidate.com)

## Contents

- [Setup](#setup)
- [Core Concepts](#core-concepts)
- [String Rules](#string-rules)
- [Common String Patterns](#common-string-patterns)
- [Numeric Rules](#numeric-rules)
- [Bytes Rules](#bytes-rules)
- [Enum Rules](#enum-rules)
- [Repeated Field Rules](#repeated-field-rules)
- [Map Rules](#map-rules)
- [Oneof Rules](#oneof-rules)
- [Timestamp Rules](#timestamp-rules)
- [Duration Rules](#duration-rules)
- [Custom CEL Rules](#custom-cel-rules)
- [Common Patterns](#common-patterns)
- [Runtime Validation](#runtime-validation)

---

## Setup

Add protovalidate as a dependency:

```yaml
# buf.yaml
version: v2
modules:
  - path: proto
deps:
  - buf.build/bufbuild/protovalidate
```

Run `buf dep update`, then import in proto files:

```protobuf
import "buf/validate/validate.proto";
```

## Core Concepts

### Presence and Validation

- **Explicit presence** (proto3 `optional`, messages, oneofs): Unset fields skip validation unless marked `required`
- **Implicit presence** (proto3 scalars without `optional`): Fields always validate against their current value
- **Nested validation**: Child messages are automatically validated; parent fails if any child fails

### The `required` Rule

`required` means a field must be **set** (present), not that it must have a non-default value:

```protobuf
// With explicit presence: fails if field isn't set
optional string name = 1 [(buf.validate.field).required = true];

// Without explicit presence: fails if field equals default value
string name = 1 [(buf.validate.field).required = true];  // fails on ""
```

Empty strings and zero values pass `required` if the field is set/present.

## String Rules

```protobuf
message Example {
  // Length constraints (character count)
  string username = 1 [
    (buf.validate.field).string.min_len = 3,
    (buf.validate.field).string.max_len = 32
  ];

  // Byte length constraints
  string bio = 2 [
    (buf.validate.field).string.min_bytes = 1,
    (buf.validate.field).string.max_bytes = 1024
  ];

  // Exact length
  string country_code = 3 [(buf.validate.field).string.len = 2];

  // Pattern matching (RE2 regex)
  string slug = 4 [(buf.validate.field).string.pattern = "^[a-z][a-z0-9-]*$"];

  // Substring matching
  string path = 5 [(buf.validate.field).string.prefix = "/api/"];
  string filename = 6 [(buf.validate.field).string.suffix = ".proto"];
  string description = 7 [(buf.validate.field).string.contains = "important"];
  string safe_text = 8 [(buf.validate.field).string.not_contains = "<script>"];

  // Format validators
  string email = 9 [(buf.validate.field).string.email = true];
  string website = 10 [(buf.validate.field).string.uri = true];
  string id = 11 [(buf.validate.field).string.uuid = true];
  string compact_id = 12 [(buf.validate.field).string.tuuid = true];  // UUID without dashes

  // Network formats
  string host = 13 [(buf.validate.field).string.hostname = true];
  string ip = 14 [(buf.validate.field).string.ip = true];
  string address = 15 [(buf.validate.field).string.address = true];  // hostname or IP
  string endpoint = 16 [(buf.validate.field).string.host_and_port = true];

  // Value sets
  string status = 17 [(buf.validate.field).string = {in: ["active", "inactive"]}];
  string role = 18 [(buf.validate.field).string = {not_in: ["admin", "root"]}];
}
```

## Common String Patterns

Prefer well-known format validators (`uuid`, `email`, `uri`, `hostname`, `ip`, `address`, `host_and_port`) before reaching for `pattern`.
When no well-known constraint fits, use `pattern` for name/identifier fields:

| Field Type | Pattern |
|-----------|---------|
| Lowercase with hyphens | `^[a-z0-9][a-z0-9-]*[a-z0-9]$` |
| Versioned label | `^[a-z0-9]([a-z0-9._-]*[a-z0-9])?$` |
| Programming identifier | `^[_a-zA-Z][_a-zA-Z0-9]*$` |

Consider adding `min_len` and `max_len` constraints where appropriate.

## Numeric Rules

All numeric types (`int32`, `int64`, `uint32`, `uint64`, `sint32`, `sint64`, `fixed32`, `fixed64`, `sfixed32`, `sfixed64`, `float`, `double`) support:

```protobuf
message Pagination {
  // Comparison operators
  uint32 page_size = 1 [
    (buf.validate.field).uint32.gt = 0,
    (buf.validate.field).uint32.lte = 100
  ];

  int32 offset = 2 [
    (buf.validate.field).int32.gte = 0,
    (buf.validate.field).int32.lte = 10000
  ];

  // Value sets
  int32 version = 3 [(buf.validate.field).int32 = {in: [1, 2, 3]}];
  int32 port = 4 [(buf.validate.field).int32 = {not_in: [0, 22, 80, 443]}];

  // Exact value
  int32 api_version = 5 [(buf.validate.field).int32.const = 1];
}

message Coordinates {
  double latitude = 1 [
    (buf.validate.field).double.gte = -90,
    (buf.validate.field).double.lte = 90
  ];

  double longitude = 2 [
    (buf.validate.field).double.gte = -180,
    (buf.validate.field).double.lte = 180
  ];

  // Prevent infinity and NaN
  double distance = 3 [(buf.validate.field).double.finite = true];
}
```

## Bytes Rules

```protobuf
message Upload {
  bytes content = 1 [
    (buf.validate.field).bytes.min_len = 1,
    (buf.validate.field).bytes.max_len = 10485760  // 10MB
  ];

  // Magic bytes check
  bytes png_image = 2 [(buf.validate.field).bytes.prefix = "\x89PNG"];

  // IP address as bytes
  bytes ip_addr = 3 [(buf.validate.field).bytes.ip = true];
}
```

## Enum Rules

```protobuf
enum Status {
  STATUS_UNSPECIFIED = 0;
  STATUS_ACTIVE = 1;
  STATUS_INACTIVE = 2;
}

message Resource {
  // Must be a defined, non-UNSPECIFIED value
  Status status = 1 [
    (buf.validate.field).enum.not_in = 0,
    (buf.validate.field).enum.defined_only = true
  ];

  // Specific allowed values
  Status allowed = 2 [(buf.validate.field).enum = {in: [1, 2]}];

  // Excluded values
  Status restricted = 3 [(buf.validate.field).enum = {not_in: [0]}];

  // Exact value
  Status required_status = 4 [(buf.validate.field).enum.const = 1];
}
```

### Enum Validation Patterns

| Context | Constraints |
|---------|-------------|
| Required enum | `not_in = 0` + `defined_only = true` |
| Optional enum (zero = "not set") | `defined_only = true` only |
| Repeated enum items | `.repeated.items.enum.not_in = 0` + `.items.enum.defined_only = true` |

Required fields need both: `defined_only` alone allows `UNSPECIFIED` through, `not_in = 0` alone allows unknown values through.

## Repeated Field Rules

```protobuf
message BatchRequest {
  // Size constraints
  repeated string ids = 1 [
    (buf.validate.field).repeated.min_items = 1,
    (buf.validate.field).repeated.max_items = 250
  ];

  // Unique items (scalars and enums only)
  repeated string tags = 2 [(buf.validate.field).repeated.unique = true];

  // Validate each item
  repeated string emails = 3 [
    (buf.validate.field).repeated.items.string.email = true
  ];
}
```

## Map Rules

```protobuf
message Config {
  // Entry count constraints
  map<string, string> labels = 1 [
    (buf.validate.field).map.min_pairs = 1,
    (buf.validate.field).map.max_pairs = 10
  ];

  // Key constraints
  map<string, int32> scores = 2 [
    (buf.validate.field).map.keys.string = {min_len: 1, max_len: 64}
  ];

  // Value constraints
  map<string, int32> counts = 3 [
    (buf.validate.field).map.values.int32.gte = 0
  ];
}
```

## Oneof Rules

Use `(buf.validate.oneof).required = true` when the oneof represents a required choice (lookups, mutually exclusive options).
Omit `required` only when the oneof is intentionally optional.

```protobuf
message UserLookup {
  oneof value {
    option (buf.validate.oneof).required = true;
    string id = 1 [(buf.validate.field).string.uuid = true];
    string email = 2 [(buf.validate.field).string.email = true];
  }
}
```

## Timestamp Rules

```protobuf
import "google/protobuf/timestamp.proto";

message Event {
  google.protobuf.Timestamp created_at = 1 [(buf.validate.field).required = true];
  google.protobuf.Timestamp scheduled_at = 2 [(buf.validate.field).timestamp.gt_now = true];  // future
  google.protobuf.Timestamp occurred_at = 3 [(buf.validate.field).timestamp.lt_now = true];   // past
  google.protobuf.Timestamp expires_at = 4 [(buf.validate.field).timestamp.within = {seconds: 86400}];
}
```

## Duration Rules

```protobuf
import "google/protobuf/duration.proto";

message Config {
  google.protobuf.Duration timeout = 1 [
    (buf.validate.field).duration.gte = {seconds: 1},
    (buf.validate.field).duration.lte = {seconds: 300}
  ];
  google.protobuf.Duration interval = 2 [(buf.validate.field).duration.gt = {}];  // positive
}
```

## FieldMask Rules

```protobuf
import "google/protobuf/field_mask.proto";

message UpdateUserRequest {
  User user = 1;

  // Restrict allowed field paths
  google.protobuf.FieldMask update_mask = 2 [
    (buf.validate.field).field_mask = {in: ["name", "email", "bio"]}
  ];
}
```

Use `in` to specify the allowed field paths.

## Any Rules

```protobuf
import "google/protobuf/any.proto";

message Container {
  // Restrict to specific types
  google.protobuf.Any payload = 1 [
    (buf.validate.field).any = {
      in: ["type.googleapis.com/acme.User", "type.googleapis.com/acme.Group"]
    }
  ];
}
```

## Ignore Rules

```protobuf
message Request {
  // Skip all validation
  string internal = 1 [(buf.validate.field).ignore = IGNORE_ALWAYS];

  // Skip validation when field equals default value (useful for proto3 scalars)
  string optional_filter = 2 [(buf.validate.field).ignore = IGNORE_IF_ZERO_VALUE];
}
```

## Custom CEL Rules

CEL (Common Expression Language) enables complex validation logic. Within expressions, `this` refers to the field value (field rules) or entire message (message rules).

### Field-Level CEL

```protobuf
message Discount {
  int32 percentage = 1 [
    (buf.validate.field).cel = {
      id: "valid_percentage"
      message: "percentage must be between 0 and 100"
      expression: "this >= 0 && this <= 100"
    }
  ];

  // Multiple rules
  string promo_code = 2 [
    (buf.validate.field).cel = {
      id: "uppercase"
      message: "must be uppercase"
      expression: "this == this.upperAscii()"
    },
    (buf.validate.field).cel = {
      id: "alphanumeric"
      message: "must be alphanumeric"
      expression: "this.matches('^[A-Z0-9]+$')"
    }
  ];
}
```

### Message-Level CEL

```protobuf
message DateRange {
  option (buf.validate.message).cel = {
    id: "date_range_valid"
    message: "end_date must be after start_date"
    expression: "this.end_date > this.start_date"
  };

  google.protobuf.Timestamp start_date = 1;
  google.protobuf.Timestamp end_date = 2;
}

// Use has() to check field presence for conditional validation
message SearchFilters {
  option (buf.validate.message).cel = {
    id: "conditional_requirement"
    message: "max_price required when min_price is set"
    expression: "!has(this.min_price) || has(this.max_price)"
  };

  optional int32 min_price = 1;
  optional int32 max_price = 2;
}
```

## CEL Extension Functions

| Function | Description |
|----------|-------------|
| `isNan()`, `isInf()` | Test for NaN or infinity |
| `isEmail()`, `isHostname()`, `isUri()`, `isUriRef()` | Format validation |
| `isIp()`, `isIp(version)`, `isIpPrefix()` | IP/CIDR validation |
| `isHostAndPort()` | Host:port validation |
| `unique()` | Check list items are unique |
| `this` | Current value being validated |
| `now` | Current timestamp |

## Common Patterns

For entity and reference message patterns, see [best_practices.md](best_practices.md#common-patterns).

### Batch Request

```protobuf
message GetUsersRequest {
  repeated UserRef user_refs = 1 [
    (buf.validate.field).repeated.min_items = 1,
    (buf.validate.field).repeated.max_items = 250
  ];
}
```

### Pagination

```protobuf
message ListRequest {
  uint32 page_size = 1 [(buf.validate.field).uint32.lte = 250];
  string page_token = 2 [(buf.validate.field).string.max_len = 4096];
}
```

See `assets/proto/example/v1/book_service.proto` for a complete List example with response, ordering, and filtering.

### Cross-Reference Consistency

When the same identifier appears in multiple messages, all validation constraints must be identical.
See [best_practices.md](best_practices.md#cross-reference-consistency) for details and examples.

## Predefined Constraint Rules

When the same validation logic is repeated across many fields, define reusable predefined constraints.
Extension files must use proto2 or Editions syntax (not proto3).
Use field numbers 50000-99999 for private schemas.

```protobuf
// extensions.proto (proto2 or Editions)
extend buf.validate.StringRules {
  optional bool name = 50000 [(buf.validate.predefined).cel = {
    id: "string.name"
    message: "name must be 1-100 characters"
    expression: "this.size() >= 1 && this.size() <= 100"
  }];
}

// Usage in proto3 files
string first_name = 1 [(buf.validate.field).string.(acme.common.v1.name) = true];
```

For parameterized rules and more examples, see [protovalidate.com](https://protovalidate.com).

## Runtime Validation

Protovalidate annotations define constraints in the schema, but enforcement happens at runtime.
Add validation calls at your service boundaries—typically in RPC handlers or middleware.

**Go:**
```go
import protovalidate "github.com/bufbuild/protovalidate-go"

validator, err := protovalidate.New()
if err := validator.Validate(msg); err != nil {
    // Handle validation error
}
```

**TypeScript:**
```typescript
import { createValidator } from "@bufbuild/protovalidate";

const validator = createValidator();
const violations = validator.validate(msg);
```

For Python, Java, and other languages, see [protovalidate.com](https://protovalidate.com).
````

## File: .agents/skills/protobuf/references/quick_reference.md
````markdown
# Protocol Buffers Quick Reference

Fast lookup for common patterns and rules. For details, see [best_practices.md](best_practices.md).

## Field Numbering

| Range | Encoding | Use For |
|-------|----------|---------|
| 1-15 | 1 byte | Frequently-set fields |
| 16-2047 | 2 bytes | Standard fields |
| 2048-262143 | 3 bytes | Rarely-set fields |
| 19000-19999 | — | Reserved by protobuf (never use) |
| 536870912+ | — | Reserved (never use) |

**Rules:**
- Never reuse field numbers (reserve deleted ones)
- Avoid gaps in numbering
- Order in definition doesn't affect wire format

## Breaking Changes Checklist

### Safe Changes
- ✅ Add new fields
- ✅ Add new enum values
- ✅ Add new methods/services
- ✅ Add new messages
- ✅ Mark elements `deprecated`

### Wire-Safe but Breaks Generated Code
- ⚠️ Add `optional` to existing scalar fields
- ⚠️ Remove fields (reserve number + name)
- ⚠️ Remove enum values (reserve number + name)
- ⚠️ Move fields into a oneof
- ⚠️ Convert between compatible types (`int32` ↔ `int64`)

### Breaking (Requires New Version)
- ❌ Renumber fields
- ❌ Change field types incompatibly
- ❌ Rename fields (breaks JSON)
- ❌ Change RPC signatures
- ❌ Remove without reserving

## Scalar Type Selection

| Use Case | Type | Wire Encoding |
|----------|------|---------------|
| Regular integers | `int32`, `int64` | Varint (1-10 bytes) |
| Frequently negative | `sint32`, `sint64` | ZigZag varint (efficient for small absolute values) |
| Large values (>2²⁸) | `fixed32`, `fixed64` | Fixed 4/8 bytes |
| Arbitrary bytes | `bytes` | Length-prefixed |
| Text (UTF-8) | `string` | Length-prefixed |
| Boolean | `bool` | Varint (1 byte) |
| Decimals | `float`, `double` | Fixed 4/8 bytes |

**Tips:**
- Use signed types—many languages lack unsigned support
- Use `int32/int64` by default; switch to `sint*` if values are often negative
- Use `fixed*` only when values are consistently large

### Size Optimization

1. **Use field numbers 1-15** for frequently-set fields (1-byte tag)
2. **Use enums over strings** for fixed sets of values
3. **Consider `bytes` over `string`** if UTF-8 validation isn't needed
4. **Nested messages** add 2+ bytes overhead per instance

## Enum Template

```protobuf
enum Status {
  STATUS_UNSPECIFIED = 0;  // Required: zero = unset
  STATUS_ACTIVE = 1;
  STATUS_INACTIVE = 2;
}
```

**Rules:**
- First value must be `0` and `*_UNSPECIFIED`
- Prefix all values with enum name
- Never remove values—reserve them

## Message Template

```protobuf
message User {
  string id = 1;
  string email = 2;
  string display_name = 3;

  google.protobuf.Timestamp create_time = 10;
  google.protobuf.Timestamp update_time = 11;
}
```

## Service Template

```protobuf
service UserService {
  rpc GetUser(GetUserRequest) returns (User);
  rpc ListUsers(ListUsersRequest) returns (ListUsersResponse);
  rpc CreateUser(CreateUserRequest) returns (User);
  rpc UpdateUser(UpdateUserRequest) returns (User);
  rpc DeleteUser(DeleteUserRequest) returns (google.protobuf.Empty);
}
```

## Pagination Template

```protobuf
message ListUsersRequest {
  int32 page_size = 1;
  string page_token = 2;
}

message ListUsersResponse {
  repeated User users = 1;
  string next_page_token = 2;
}
```

## Field Mask Template

```protobuf
import "google/protobuf/field_mask.proto";

message UpdateUserRequest {
  User user = 1;
  google.protobuf.FieldMask update_mask = 2;
}
```

## Well-Known Types

| Import | Type | Use For |
|--------|------|---------|
| `google/protobuf/timestamp.proto` | `Timestamp` | Points in time |
| `google/protobuf/duration.proto` | `Duration` | Time spans |
| `google/protobuf/field_mask.proto` | `FieldMask` | Partial updates |
| `google/protobuf/empty.proto` | `Empty` | Empty responses |
| `google/protobuf/struct.proto` | `Struct` | Dynamic JSON |
| `google/protobuf/any.proto` | `Any` | Arbitrary messages |
| `google/protobuf/wrappers.proto` | `*Value` | Nullable scalars |

## Naming Conventions

| Element | Style | Example |
|---------|-------|---------|
| Files | `lower_snake_case.proto` | `user_service.proto` |
| Packages | `lower.dot.separated.v1` | `acme.user.v1` |
| Messages | `PascalCase` | `UserProfile` |
| Fields | `snake_case` | `display_name` |
| Services | `PascalCase` | `UserService` |
| RPCs | `PascalCase` | `GetUser` |
| Enums | `PascalCase` | `UserStatus` |
| Enum values | `UPPER_SNAKE_CASE` | `USER_STATUS_ACTIVE` |

## Common Buf Commands

```bash
buf lint                          # Lint protos
buf format -w                     # Format in place
buf breaking --against '.git#branch=main'  # Check breaking
buf generate                      # Generate code
buf dep update                    # Update dependencies
```

## Reserved Keywords

Avoid these in package names (cause issues in specific languages):

| Keyword | Language |
|---------|----------|
| `internal` | Go |
| `private` | Go |
| `class` | Python, Java, C# |
| `type` | TypeScript |
| `import` | Multiple |
| `package` | Multiple |

For Java, package names should avoid [JLS reserved keywords](https://docs.oracle.com/javase/specs/jls/se21/html/jls-3.html#jls-ReservedKeyword).
````

## File: .agents/skills/protobuf/references/review_checklist.md
````markdown
# Proto API Review Checklist

Use this checklist when reviewing `.proto` files before an API stabilizes.
It captures common issues found during real-world proto API reviews.

Every field in a production API should have [protovalidate](protovalidate.md) constraints.
A field without validation is a field that accepts anything.

## Contents

- [Enum Validation](#enum-validation)
- [Oneof Validation](#oneof-validation)
- [String Validation](#string-validation)
- [Cross-Reference Consistency](#cross-reference-consistency)
- [Pagination Validation](#pagination-validation)
- [List Requests](#list-requests)
- [Reserved Statements](#reserved-statements)
- [Documentation Quality](#documentation-quality)

---

## Enum Validation

Every enum field should have appropriate validation constraints.
The correct constraints depend on how the enum is used.

Proto3 enums should have a zero value with the `_UNSPECIFIED` suffix (e.g., `STATUS_UNSPECIFIED = 0`).
This value represents "not set" and should have no semantic meaning.
See [Enums](best_practices.md#enums) for naming conventions.

| Context | Constraints | Rationale |
|---------|-------------|-----------|
| Required enum field | `not_in = 0` + `defined_only = true` | Rejects the zero value and unknown values |
| Optional enum field (zero value = "not set") | `defined_only = true` only | Zero value means "not set" or "no preference" |
| Repeated enum items | `.repeated.items.enum.not_in = 0` + `.items.enum.defined_only = true` | Each item must be meaningful |

**Common mistake:** Using only `defined_only = true` on a required enum field.
This allows the zero value through, which is rarely the intent.

```protobuf
// WRONG: allows the zero value through
Status status = 1 [(buf.validate.field).enum.defined_only = true];

// RIGHT: rejects both the zero value and unknown values
Status status = 1 [
  (buf.validate.field).enum.not_in = 0,
  (buf.validate.field).enum.defined_only = true
];

// RIGHT: optional enum where zero means "no preference"
Status status_filter = 2 [(buf.validate.field).enum.defined_only = true];
```

## Oneof Validation

All oneofs representing a required choice should have `(buf.validate.oneof).required = true`.

**Check these oneof types:**

- **Lookup messages** (id-or-name lookups): always required
- **Mutually exclusive operations** (add/update/delete in diff messages): always required
- **Value types** (string/int/bool variants): required unless the oneof is intentionally optional

```protobuf
// Lookup by ID or email - always required
message UserLookup {
  oneof value {
    option (buf.validate.oneof).required = true;
    string id = 1 [(buf.validate.field).string.uuid = true];
    string email = 2 [(buf.validate.field).string.email = true];
  }
}
```

**Common mistake:** Omitting `required = true` on a lookup oneof.
This allows clients to send a lookup with no value set, which is never valid.

## String Validation

Prefer well-known standard constraints before reaching for `pattern`:

| Constraint | Use For |
|-----------|---------|
| `.string.uuid` | UUIDs |
| `.string.email` | Email addresses |
| `.string.uri` / `.string.uri_ref` | URIs |
| `.string.hostname` | Hostnames |
| `.string.ip` / `.string.ipv4` / `.string.ipv6` | IP addresses |
| `.string.address` | Hostname or IP |
| `.string.host_and_port` | Host:port pairs |

When no well-known constraint fits, use `pattern` for name/identifier fields:

| Field Type | Pattern |
|-----------|---------|
| Lowercase with hyphens | `^[a-z0-9][a-z0-9-]*[a-z0-9]$` |
| Versioned label | `^[a-z0-9]([a-z0-9._-]*[a-z0-9])?$` |
| Programming identifier | `^[_a-zA-Z][_a-zA-Z0-9]*$` |

**Review items:**

- [ ] String fields use well-known constraints where applicable before `pattern`
- [ ] Name fields have appropriate length bounds (`min_len`, `max_len`)
- [ ] Patterns are consistent with the character set the spec allows

## Cross-Reference Consistency

When the same identifier appears in multiple messages, all validation constraints must match.

**Example problem:** `User.name` has `max_len = 32` but `TeamMember.user_name` (which references the same user name) has `max_len = 64`.
A team member could reference a user name that is too long to be a valid user name.

**Review items:**

- [ ] Find all messages that reference the same identifier
- [ ] Verify `max_len`, `min_len`, and `pattern` are identical across all references
- [ ] Check companion messages for consistency

## Pagination Validation

Pagination fields should have validation constraints.

```protobuf
uint32 page_size = 1 [(buf.validate.field).uint32.lte = 250];
string page_token = 2 [(buf.validate.field).string.max_len = 4096];
```

**Review items:**

- [ ] `page_size` has an upper bound
- [ ] `page_token` has a `max_len`
- [ ] Response includes `next_page_token` with the same `max_len`
- [ ] Response repeated field has `max_items` matching the `page_size` upper bound

## List Requests

**Review items:**

- [ ] All List requests have pagination (`page_size` + `page_token`) with validation
- [ ] Consider whether ordering and filtering are appropriate for the use case
- [ ] Optional enum filter fields use `defined_only = true` only (zero value = "no preference")
- [ ] Default behavior is documented

## Reserved Statements

When field numbers are skipped, always add `reserved` statements to prevent accidental reuse.

**Review items:**

- [ ] No gaps in field numbering without a `reserved` statement
- [ ] Removed fields have both the field number and name reserved
- [ ] Oneofs that start at field 2+ have field 1 reserved (or documented why)

```protobuf
message Example {
  reserved 1;

  // Type of the value.
  ValueType type = 2;

  oneof value {
    string string_value = 3;
    int64 int_value = 4;
  }
}
```

## Documentation Quality

Documentation errors erode trust in the API and confuse implementers.

**Review items:**

- [ ] **Grammar: articles before vowels** — "an Environment" not "a Environment", "an Image" not "a Image"
- [ ] **Consistent terminology** — pick one form and use it everywhere (e.g., "shortname" not sometimes "short name" and sometimes "shortname")
- [ ] **No stale references** — check for references to old entity/concept names that have been renamed
- [ ] **Implicit behavior is documented** — if an entity is auto-created, has reserved names, or has special behavior, document it on the relevant field
- [ ] **Complete sentences** — comments should be full sentences with periods
- [ ] **No truncated comments** — watch for comments cut off mid-sentence (e.g., "typically by the" with no completion)
- [ ] **Typos** — common in large proto reviews: run spellcheck or read comments carefully
````

## File: .agents/skills/protobuf/references/troubleshooting.md
````markdown
# Troubleshooting

Common errors when working with Protocol Buffers, with solutions.

## Contents

- [Linting Errors](#linting-errors)
- [Breaking Change Errors](#breaking-change-errors)
- [Code Generation Errors](#code-generation-errors)
- [Import and Dependency Errors](#import-and-dependency-errors)
- [Field and Schema Evolution](#field-and-schema-evolution)
- [Runtime and Validation Errors](#runtime-and-validation-errors)

---

## Linting Errors

### Quick Reference

| Error | Problem | Fix |
|-------|---------|-----|
| ENUM_VALUE_PREFIX | `ACTIVE` | `STATUS_ACTIVE` (prefix with enum name) |
| ENUM_ZERO_VALUE_SUFFIX | `STATUS_UNKNOWN` | `STATUS_UNSPECIFIED` |
| PACKAGE_VERSION_SUFFIX | `acme.user` | `acme.user.v1` |
| FIELD_LOWER_SNAKE_CASE | `userName` | `user_name` |
| SERVICE_SUFFIX | `Users` | `UserService` |
| RPC_REQUEST_RESPONSE_UNIQUE | Shared request types | Unique `{Method}Request` per RPC |
| RPC_REQUEST_STANDARD_NAME | `UserRequest` | `GetUserRequest` |
| COMMENT_* | Missing comments | Add `//` comments on public elements |
| IMPORT_NO_WEAK/PUBLIC | `import public` | `import` (standard import) |

### Ignoring Lint Rules

For legitimate exceptions, use comments (unless `disallow_comment_ignores: true`):

```protobuf
// buf:lint:ignore ENUM_VALUE_PREFIX
enum LegacyStatus {
  UNKNOWN = 0;
  ACTIVE = 1;
}
```

Or configure in `buf.yaml`:

```yaml
lint:
  ignore_only:
    ENUM_VALUE_PREFIX:
      - proto/legacy
```

## Breaking Change Errors

Breaking changes are detected by `buf breaking --against <reference>`.
These errors indicate changes that would break existing clients.

### FIELD_NO_DELETE

```
// Before
message User {
  string id = 1;
  string email = 2;
  string name = 3;
}

// After - Error: field 3 deleted without reservation
message User {
  string id = 1;
  string email = 2;
}
```

**Fix:** Reserve deleted field numbers and names:

```protobuf
message User {
  reserved 3;
  reserved "name";

  string id = 1;
  string email = 2;
}
```

### FIELD_SAME_NUMBER

**Fix:** Never change field numbers. Create a new message version if restructuring is needed.

### FIELD_SAME_TYPE

**Fix:** Field types cannot change. Options:
1. Add a new field with the correct type, deprecate the old one
2. Create a new package version (v2)

```protobuf
message User {
  string user_id = 1 [deprecated = true];
  int64 user_id_v2 = 2;
}
```

### FIELD_SAME_JSON_NAME

**Fix:** Don't change JSON names. Use explicit `json_name` to preserve the original:

```protobuf
message User {
  string username = 1 [json_name = "userName"];  // Preserves JSON compatibility
}
```

### ENUM_VALUE_NO_DELETE

**Fix:** Reserve deleted enum values:

```protobuf
enum Status {
  reserved 2;
  reserved "STATUS_INACTIVE";

  STATUS_UNSPECIFIED = 0;
  STATUS_ACTIVE = 1;
}
```

### RPC_NO_DELETE

**Fix:** Don't remove RPCs. Deprecate instead:

```protobuf
// Deprecated: Use DeactivateUser instead.
rpc DeleteUser(DeleteUserRequest) returns (DeleteUserResponse) {
  option deprecated = true;
}
```

### MESSAGE_NO_DELETE / ENUM_NO_DELETE

**Fix:** Don't remove messages or enums that may be in use. Deprecate first:

```protobuf
option deprecated = true;
message LegacyUser { ... }
```

### FIELD_SAME_ONEOF

**Fix:** Never move existing fields into a oneof. Add new fields to the oneof instead:

```protobuf
message Request {
  string id = 1;  // Keep as-is
  oneof alternate_identifier {
    string name = 2;
    string email = 3;
  }
}
```

### Breaking Change Categories

Buf offers different strictness levels in `buf.yaml`:

```yaml
breaking:
  use:
    - FILE      # Strictest: file-level changes break
    - PACKAGE   # Package-level: allows moving between files
    - WIRE_JSON # Wire + JSON encoding only
    - WIRE      # Wire encoding only (most permissive)
```

For internal APIs, `WIRE_JSON` may be sufficient. For public APIs, use `FILE`.

## Code Generation Errors

### Buf: Plugin Not Found

**Fixes:**
1. Check plugin name spelling in `buf.gen.yaml`
2. For remote plugins, verify it exists on BSR: `buf.build/some/plugin`
3. For local plugins, ensure it's in PATH

### Buf: Remote Plugin Timeout

**Fix:** Retry the command. Transient network issues are common.

### Protoc: Plugin Not Found

**Fixes:**
1. Install the plugin
2. Ensure GOBIN is in PATH:
   ```bash
   export PATH="$PATH:$(go env GOPATH)/bin"
   ```

### Go: "go_package option required"

**Fixes:**

With buf (recommended): Enable managed mode in `buf.gen.yaml`:
```yaml
version: v2
managed:
  enabled: true
  override:
    - file_option: go_package_prefix
      value: github.com/yourorg/api/gen/go
```

With protoc: Add `go_package` to each proto file:
```protobuf
option go_package = "github.com/yourorg/api/gen/go/acme/user/v1;userv1";
```

### Go: Conflicting Package Names

**Fix:** Each proto package should map to a unique Go package.
Use buf managed mode to handle this automatically.

### Output Directory Issues

**Fixes:**
- Buf creates directories automatically—check write permissions
- Protoc requires directories to exist: `mkdir -p gen/go`

### Stale Generated Files

**Fix:** Clean and regenerate:
```bash
rm -rf gen/
buf generate
```

## Import and Dependency Errors

### Buf: Dependency Not Found

```
Error: import "buf/validate/validate.proto": not found
```

**Fix:** Add dependency to `buf.yaml` and update:

```yaml
deps:
  - buf.build/bufbuild/protovalidate
```

```bash
buf dep update
```

### Buf: Version Conflict

**Fix:** Run `buf dep update` to resolve.
If persists, check for conflicting version pins across dependencies.

### Protoc: Import Not Found

**Fixes:**

1. Add include path for well-known types:
   ```bash
   protoc -I /usr/local/include -I proto ...
   ```

2. Find where WKTs are installed:
   ```bash
   ls /usr/local/include/google/protobuf/
   ```

3. Vendor the dependency:
   ```bash
   git clone --depth 1 https://github.com/protocolbuffers/protobuf.git third_party/protobuf
   protoc -I third_party/protobuf/src -I proto ...
   ```

### Protoc: googleapis Import Not Found

**Fix:** Vendor googleapis or migrate to buf with BSR dependency:

```bash
git clone --depth 1 https://github.com/googleapis/googleapis.git third_party/googleapis
protoc -I third_party/googleapis -I proto ...
```

### Circular Import

**Fix:** Break the cycle by moving shared types to a common file:

```
# Before (circular)
a.proto imports b.proto
b.proto imports a.proto

# After (resolved)
common.proto (shared types)
a.proto imports common.proto
b.proto imports common.proto
```

## Field and Schema Evolution

### Reserved Field Conflicts

**Fix:** Use a different field number:

```protobuf
message User {
  reserved 3, 5;
  reserved "old_name";
  string name = 6;  // Not 3 or 5
}
```

### Field Number Reuse (Silent Corruption)

Reusing field numbers causes **silent data corruption**. Protobuf won't catch it:

```protobuf
// Version 1
message User {
  string email = 3;
}

// Version 2 - DANGEROUS: reused field number with different type
message User {
  int64 user_type = 3;  // Silently corrupts data!
}
```

**Prevention:**
1. Always reserve deleted field numbers
2. Run `buf breaking` in CI
3. Never reuse field numbers, even if types match

### Oneof Evolution

**Moving fields into oneof breaks wire compatibility:**

```protobuf
// Before
message Request {
  string id = 1;
}

// After - BREAKS clients
message Request {
  oneof ref {
    string id = 1;  // Same number, but now in oneof
    string name = 2;
  }
}
```

**Adding fields to existing oneof is safe.**

### Changing Field Optionality

Adding `optional` keyword changes generated code but is wire-compatible:

```protobuf
// Before - cannot distinguish unset from ""
string nickname = 2;

// After - can detect if set
optional string nickname = 2;
```

**With protovalidate:** Changing required validation is a behavioral breaking change.

### When to Create a New Version

Create a new package version (v1 → v2) when:
- Multiple breaking changes are needed
- The API shape is fundamentally changing
- Field type changes are required
- Deprecation period has ended

```protobuf
// acme/user/v1/user.proto - Keep for existing clients
package acme.user.v1;

// acme/user/v2/user.proto - New version with breaking changes
package acme.user.v2;
```

## Runtime and Validation Errors

### Protovalidate: Required Field Missing

**Fix:** Ensure required fields are populated before sending.

For optional fields that shouldn't fail validation when empty:

```protobuf
string url = 3 [
  (buf.validate.field).string.uri = true,
  (buf.validate.field).ignore = IGNORE_IF_DEFAULT_VALUE
];
```

### Protovalidate: Pattern Mismatch

**Fix:** Ensure input matches the regex. Check for:
- Uppercase letters when lowercase required
- Invalid characters
- Missing required prefix/suffix

### Unknown Fields Warning

**Causes:**
- Client using newer proto than server
- Proto files out of sync between services

**Fix:** Regenerate and redeploy to ensure consistent proto versions.

### Message Size Limits

**Fixes:**
1. Increase limit if appropriate:
   ```go
   grpc.MaxRecvMsgSize(16 * 1024 * 1024)
   ```
2. Use streaming for large payloads
3. Paginate responses
````

## File: .agents/skills/protobuf/SKILL.md
````markdown
---
name: protobuf
description: >-
  Use when working with Protocol Buffer (.proto) files, buf.yaml, buf.gen.yaml,
  or buf.lock. Covers proto design, buf CLI, gRPC/Connect services, protovalidate
  constraints, schema evolution, and troubleshooting lint/breaking errors.
filePatterns:
  - "**/*.proto"
  - "**/buf.yaml"
  - "**/buf.*.yaml"
  - "**/buf.gen.yaml"
  - "**/buf.gen.*.yaml"
  - "**/buf.lock"
---

# Protocol Buffers

## When You Need This Skill

- Creating or editing `.proto` files
- Setting up `buf.yaml` or `buf.gen.yaml`
- Designing gRPC or Connect services
- Adding protovalidate constraints
- Troubleshooting buf lint or breaking change errors

## Core Workflow

### 1. Match Project Style

Before writing proto code, review existing `.proto` files in the project.
Match conventions for naming, field ordering, structural patterns, validation, and documentation style.
If none exists, ask the user what style should be used or an existing library to emulate.

### 2. Write Proto Code

- Apply universal best practices from [best_practices.md](references/best_practices.md)
- Add [protovalidate](references/protovalidate.md) constraints to every field—this is not optional for production APIs
- For service templates, see [assets/](assets/)

### 3. Verify Changes

**Always run after making changes:**

```bash
buf format -w && buf lint
```

Check for a Makefile first—many projects use `make lint` or `make format`.

Fix all errors before considering the change complete.

## Quick Reference

| Task | Reference |
|------|-----------|
| Field types, enums, oneofs, maps | [quick_reference.md](references/quick_reference.md) |
| Schema evolution, breaking changes | [best_practices.md](references/best_practices.md) |
| Validation constraints | [protovalidate.md](references/protovalidate.md) |
| Complete service examples | [examples.md](references/examples.md), [assets/](assets/) |
| buf CLI, buf.yaml, buf.gen.yaml | [buf_toolchain.md](references/buf_toolchain.md) |
| Migrating from protoc | [migration.md](references/migration.md) |
| Lint errors, common issues | [troubleshooting.md](references/troubleshooting.md) |
| Proto API review checklist | [review_checklist.md](references/review_checklist.md) |

## Project Setup

### New Project

1. Create directory structure:
   ```
   proto/
   ├── buf.yaml
   ├── buf.gen.yaml
   └── company/
       └── domain/
           └── v1/
               └── service.proto
   ```

2. Use `assets/buf.yaml` as starting point
3. Add `buf.build/bufbuild/protovalidate` as a dependency in `buf.yaml` and run `buf dep update`
4. Use `assets/buf.gen.*.yaml` for code generation config

### Code Generation Templates

| Template | Use For |
|----------|---------|
| `buf.gen.go.yaml` | Go with gRPC |
| `buf.gen.go-connect.yaml` | Go with Connect |
| `buf.gen.ts.yaml` | TypeScript with Connect |
| `buf.gen.python.yaml` | Python with gRPC |
| `buf.gen.java.yaml` | Java with gRPC |

### Proto File Templates

Located in `assets/proto/example/v1/`:

| Template | Description |
|----------|-------------|
| `book.proto` | Entity message, BookRef oneof, enum |
| `book_service.proto` | Full CRUD with batch ops, pagination, ordering |

## Common Tasks

### Add a new field

1. Use next sequential field number
2. Add [protovalidate](references/protovalidate.md) constraints: every field should have validation appropriate to its type (format validators, length bounds, numeric ranges, enum constraints, etc.)
3. Document the field
4. Run `buf format -w && buf lint`

### Remove a field

1. Reserve the field number AND name:
   ```protobuf
   reserved 4;
   reserved "old_field_name";
   ```
2. Run `buf breaking --against '.git#branch=main'` to verify

### Add protovalidate constraints

Every field in a production API should have appropriate validation.
See [protovalidate.md](references/protovalidate.md) for the full reference.

Common constraints:
- String formats: `.string.uuid`, `.string.email`, `.string.uri`, `.string.pattern`
- String bounds: `.string.min_len`, `.string.max_len`
- Numeric bounds: `.int32.gte`, `.uint32.lte`
- Enum validation: `.enum.defined_only`, `.enum.not_in = 0`
- Repeated bounds: `.repeated.min_items`, `.repeated.max_items`
- Required fields: `(buf.validate.field).required = true`
- Oneof required: `(buf.validate.oneof).required = true`

## Verification Checklist

After making changes:
- [ ] Every field has appropriate protovalidate constraints
- [ ] `buf format -w` (apply formatting)
- [ ] `buf lint` (check style rules)
- [ ] `buf breaking --against '.git#branch=main'` (if modifying existing schemas)
````

## File: .agents/skills/rust-async-patterns/references/details.md
````markdown
# rust-async-patterns — detailed patterns and worked examples

## Patterns

### Pattern 1: Concurrent Task Execution

```rust
use tokio::task::JoinSet;
use anyhow::Result;

// Spawn multiple concurrent tasks
async fn fetch_all_concurrent(urls: Vec<String>) -> Result<Vec<String>> {
    let mut set = JoinSet::new();

    for url in urls {
        set.spawn(async move {
            fetch_data(&url).await
        });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(data)) => results.push(data),
            Ok(Err(e)) => tracing::error!("Task failed: {}", e),
            Err(e) => tracing::error!("Join error: {}", e),
        }
    }

    Ok(results)
}

// With concurrency limit
use futures::stream::{self, StreamExt};

async fn fetch_with_limit(urls: Vec<String>, limit: usize) -> Vec<Result<String>> {
    stream::iter(urls)
        .map(|url| async move { fetch_data(&url).await })
        .buffer_unordered(limit) // Max concurrent tasks
        .collect()
        .await
}

// Select first to complete
use tokio::select;

async fn race_requests(url1: &str, url2: &str) -> Result<String> {
    select! {
        result = fetch_data(url1) => result,
        result = fetch_data(url2) => result,
    }
}
```

### Pattern 2: Channels for Communication

```rust
use tokio::sync::{mpsc, broadcast, oneshot, watch};

// Multi-producer, single-consumer
async fn mpsc_example() {
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Spawn producer
    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx2.send("Hello".to_string()).await.unwrap();
    });

    // Consume
    while let Some(msg) = rx.recv().await {
        println!("Got: {}", msg);
    }
}

// Broadcast: multi-producer, multi-consumer
async fn broadcast_example() {
    let (tx, _) = broadcast::channel::<String>(100);

    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    tx.send("Event".to_string()).unwrap();

    // Both receivers get the message
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
}

// Oneshot: single value, single use
async fn oneshot_example() -> String {
    let (tx, rx) = oneshot::channel::<String>();

    tokio::spawn(async move {
        tx.send("Result".to_string()).unwrap();
    });

    rx.await.unwrap()
}

// Watch: single producer, multi-consumer, latest value
async fn watch_example() {
    let (tx, mut rx) = watch::channel("initial".to_string());

    tokio::spawn(async move {
        loop {
            // Wait for changes
            rx.changed().await.unwrap();
            println!("New value: {}", *rx.borrow());
        }
    });

    tx.send("updated".to_string()).unwrap();
}
```

### Pattern 3: Async Error Handling

```rust
use anyhow::{Context, Result, bail};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Timeout after {0:?}")]
    Timeout(std::time::Duration),
}

// Using anyhow for application errors
async fn process_request(id: &str) -> Result<Response> {
    let data = fetch_data(id)
        .await
        .context("Failed to fetch data")?;

    let parsed = parse_response(&data)
        .context("Failed to parse response")?;

    Ok(parsed)
}

// Using custom errors for library code
async fn get_user(id: &str) -> Result<User, ServiceError> {
    let result = db.query(id).await?;

    match result {
        Some(user) => Ok(user),
        None => Err(ServiceError::NotFound(id.to_string())),
    }
}

// Timeout wrapper
use tokio::time::timeout;

async fn with_timeout<T, F>(duration: Duration, future: F) -> Result<T, ServiceError>
where
    F: std::future::Future<Output = Result<T, ServiceError>>,
{
    timeout(duration, future)
        .await
        .map_err(|_| ServiceError::Timeout(duration))?
}
```

### Pattern 4: Graceful Shutdown

```rust
use tokio::signal;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

async fn run_server() -> Result<()> {
    // Method 1: CancellationToken
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // Spawn task that respects cancellation
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => {
                    tracing::info!("Task shutting down");
                    break;
                }
                _ = do_work() => {}
            }
        }
    });

    // Wait for shutdown signal
    signal::ctrl_c().await?;
    tracing::info!("Shutdown signal received");

    // Cancel all tasks
    token.cancel();

    // Give tasks time to cleanup
    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}

// Method 2: Broadcast channel for shutdown
async fn run_with_broadcast() -> Result<()> {
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    let mut rx = shutdown_tx.subscribe();
    tokio::spawn(async move {
        tokio::select! {
            _ = rx.recv() => {
                tracing::info!("Received shutdown");
            }
            _ = async { loop { do_work().await } } => {}
        }
    });

    signal::ctrl_c().await?;
    let _ = shutdown_tx.send(());

    Ok(())
}
```

### Pattern 5: Async Traits

```rust
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn get(&self, id: &str) -> Result<Entity>;
    async fn save(&self, entity: &Entity) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
}

pub struct PostgresRepository {
    pool: sqlx::PgPool,
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn get(&self, id: &str) -> Result<Entity> {
        sqlx::query_as!(Entity, "SELECT * FROM entities WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn save(&self, entity: &Entity) -> Result<()> {
        sqlx::query!(
            "INSERT INTO entities (id, data) VALUES ($1, $2)
             ON CONFLICT (id) DO UPDATE SET data = $2",
            entity.id,
            entity.data
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM entities WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// Trait object usage
async fn process(repo: &dyn Repository, id: &str) -> Result<()> {
    let entity = repo.get(id).await?;
    // Process...
    repo.save(&entity).await
}
```

### Pattern 6: Streams and Async Iteration

```rust
use futures::stream::{self, Stream, StreamExt};
use async_stream::stream;

// Create stream from async iterator
fn numbers_stream() -> impl Stream<Item = i32> {
    stream! {
        for i in 0..10 {
            tokio::time::sleep(Duration::from_millis(100)).await;
            yield i;
        }
    }
}

// Process stream
async fn process_stream() {
    let stream = numbers_stream();

    // Map and filter
    let processed: Vec<_> = stream
        .filter(|n| futures::future::ready(*n % 2 == 0))
        .map(|n| n * 2)
        .collect()
        .await;

    println!("{:?}", processed);
}

// Chunked processing
async fn process_in_chunks() {
    let stream = numbers_stream();

    let mut chunks = stream.chunks(3);

    while let Some(chunk) = chunks.next().await {
        println!("Processing chunk: {:?}", chunk);
    }
}

// Merge multiple streams
async fn merge_streams() {
    let stream1 = numbers_stream();
    let stream2 = numbers_stream();

    let merged = stream::select(stream1, stream2);

    merged
        .for_each(|n| async move {
            println!("Got: {}", n);
        })
        .await;
}
```

### Pattern 7: Resource Management

```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, Semaphore};

// Shared state with RwLock (prefer for read-heavy)
struct Cache {
    data: RwLock<HashMap<String, String>>,
}

impl Cache {
    async fn get(&self, key: &str) -> Option<String> {
        self.data.read().await.get(key).cloned()
    }

    async fn set(&self, key: String, value: String) {
        self.data.write().await.insert(key, value);
    }
}

// Connection pool with semaphore
struct Pool {
    semaphore: Semaphore,
    connections: Mutex<Vec<Connection>>,
}

impl Pool {
    fn new(size: usize) -> Self {
        Self {
            semaphore: Semaphore::new(size),
            connections: Mutex::new((0..size).map(|_| Connection::new()).collect()),
        }
    }

    async fn acquire(&self) -> PooledConnection<'_> {
        let permit = self.semaphore.acquire().await.unwrap();
        let conn = self.connections.lock().await.pop().unwrap();
        PooledConnection { pool: self, conn: Some(conn), _permit: permit }
    }
}

struct PooledConnection<'a> {
    pool: &'a Pool,
    conn: Option<Connection>,
    _permit: tokio::sync::SemaphorePermit<'a>,
}

impl Drop for PooledConnection<'_> {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let pool = self.pool;
            tokio::spawn(async move {
                pool.connections.lock().await.push(conn);
            });
        }
    }
}
```

## Debugging Tips

```rust
// Enable tokio-console for runtime debugging
// Cargo.toml: tokio = { features = ["tracing"] }
// Run: RUSTFLAGS="--cfg tokio_unstable" cargo run
// Then: tokio-console

// Instrument async functions
use tracing::instrument;

#[instrument(skip(pool))]
async fn fetch_user(pool: &PgPool, id: &str) -> Result<User> {
    tracing::debug!("Fetching user");
    // ...
}

// Track task spawning
let span = tracing::info_span!("worker", id = %worker_id);
tokio::spawn(async move {
    // Enters span when polled
}.instrument(span));
```
````

## File: .agents/skills/rust-async-patterns/SKILL.md
````markdown
---
name: rust-async-patterns
description: Master Rust async programming with Tokio, async traits, error handling, and concurrent patterns. Use when building async Rust applications, implementing concurrent systems, or debugging async code.
---

# Rust Async Patterns

Production patterns for async Rust programming with Tokio runtime, including tasks, channels, streams, and error handling.

## When to Use This Skill

- Building async Rust applications
- Implementing concurrent network services
- Using Tokio for async I/O
- Handling async errors properly
- Debugging async code issues
- Optimizing async performance

## Core Concepts

### 1. Async Execution Model

```
Future (lazy) → poll() → Ready(value) | Pending
                ↑           ↓
              Waker ← Runtime schedules
```

### 2. Key Abstractions

| Concept    | Purpose                                  |
| ---------- | ---------------------------------------- |
| `Future`   | Lazy computation that may complete later |
| `async fn` | Function returning impl Future           |
| `await`    | Suspend until future completes           |
| `Task`     | Spawned future running concurrently      |
| `Runtime`  | Executor that polls futures              |

## Quick Start

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
futures = "0.3"
async-trait = "0.1"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tokio::time::{sleep, Duration};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Async operations
    let result = fetch_data("https://api.example.com").await?;
    println!("Got: {}", result);

    Ok(())
}

async fn fetch_data(url: &str) -> Result<String> {
    // Simulated async operation
    sleep(Duration::from_millis(100)).await;
    Ok(format!("Data from {}", url))
}
```

## Detailed patterns and worked examples

Detailed pattern documentation lives in `references/details.md`. Read that file when the navigation tier above is insufficient.

## Best Practices

### Do's

- **Use `tokio::select!`** - For racing futures
- **Prefer channels** - Over shared state when possible
- **Use `JoinSet`** - For managing multiple tasks
- **Instrument with tracing** - For debugging async code
- **Handle cancellation** - Check `CancellationToken`

### Don'ts

- **Don't block** - Never use `std::thread::sleep` in async
- **Don't hold locks across awaits** - Causes deadlocks
- **Don't spawn unboundedly** - Use semaphores for limits
- **Don't ignore errors** - Propagate with `?` or log
- **Don't forget Send bounds** - For spawned futures
````

## File: .agents/skills/rust-best-practices/references/chapter_01.md
````markdown
# Chapter 1 - Coding Styles and Idioms

## 1.1 Borrowing Over Cloning

Rust's ownership system encourages **borrow** (`&T`) instead of **cloning** (`T.clone()`). 
> ❗ Performance recommendation

### ✅ When to `Clone`:

* You need to change the object AND preserve the original object (immutable snapshots).
* When you have `Arc` or `Rc` pointers.
* When data is shared across threads, usually `Arc`.
* Avoid massive refactoring of non performance critical code.
* When caching results (dummy example below):
```rust
fn get_config(&self) -> Config {
    self.cached_config.clone()
}
```
* When the underlying API expects Owned Data.

### 🚨 `Clone` traps to avoid:

* Auto-cloning inside loops `.map(|x| x.clone)`, prefer to call `.cloned()` or `.copied()` at the end of the iterator.
* Cloning large data structures like `Vec<T>` or `HashMap<K, V>`.
* Clone because of bad API design instead of adjusting lifetimes.
* Prefer `&[T]` instead of `Vec<T>` or `&Vec<T>`.
* Prefer `&str` or `&String` instead of `String`.
* Prefer `&T` instead of `T`.
* Clone a reference argument, if you need ownership, make it explicit in the arguments for the caller. Example:
```rust
fn take_a_borrow(thing: &Thing) {
    let thing_cloned = thing.clone(); // the caller should have passed ownership instead
}
```

### ✅ Prefer borrowing:
```rust
fn process(name: &str) {
    println!("Hello {name}");
}

let user = String::from("foo");
process(&user);
```

### ❌ Avoid redundant cloning:
```rust
fn process_string(name: String) {
    println!("Hello {name}");
}

let user = String::from("foo");
process(user.clone()); // Unnecessary clone
```

## 1.2 When to pass by value? (Copy trait)

Not all types should be passed by reference (`&T`). If a type is **small** and it is **cheap to copy**, it is often better to **pass it by value**. Rust makes it explicit via the `Copy` trait.

### ✅ When to pass by value, `Copy`:
* The type **implements** `Copy` (`u32`, `bool`, `f32`, small structs).
* The cost of moving the value is negligible.

```rust
fn increment(x: u32) -> u32 {
    x + 1
}

let num = 1;
let new_num = increment(num); // `num` still usable after this point
```

### ❓ Which structs should be `Copy`?
* When to consider declaring `Copy` on your own types:
* All fields are `Copy` themselves.
* The struct is `small`, up to 2 (maybe 3) words of memory or 24 bytes (each word is 64 bits/8bytes).
* The struct **represents a "plain data object"**, without resourcing to ownership (no heap allocations. Example: `Vec` and `Strings`).

❗**Rust Arrays are stack allocated.** Which means they can be copied if their underlying type is `Copy`, but this will be allocated in the program stack which can easily become a stack overflow. More on [Chapter 3 - Stack vs Heap](./chapter_03.md#33-stack-vs-heap-be-size-smart)

For reference, each primitive type size in bytes:

#### Integers:

| Type | Size |
|------------- |---------- |
| i8 u8 | 1 byte |
| i16 u16 | 2 bytes |
| i32 u32 | 4 bytes |
| i64 u64 | 8 bytes |
| isize usize | Arch |
| i128 u128 | 16 bytes |

#### Floating Point:

| Type | Size |
|---------- |---------- |
| f32 | 4 bytes |
| f64 | 8 bytes |


#### Other:

| Type | Size |
|---------- |---------- |
| bool | 1 byte |
| char | 4 bytes |


### ✅ Good struct to derive `Copy`:
```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32
}
```

### ❌ Bad struct to derive `Copy`:
```rust
#[derive(Debug, Clone)]
struct BadIdea {
    age: i32,
    name: String, // String is not `Copy`
}
```

### ❓Which Enums should be `Copy`?
* If your enum acts like tags and atoms.
* The enum payloads are all `Copy`.
* **❗Enums size are based on their largest element.**

### ✅ Good Enum to derive
```rust
#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
```

## 1.3 Handling `Option<T>` and `Result<T, E>`
Rust 1.65 introduced a better way to safely unpack Option and Result types with the `let Some(x) = … else { … }` or `let Ok(x) = … else { … }` when you have a default `return` value, `continue` or `break` default else case. It allows early returns when the missing case is **expected and normal**, not exceptional.

### ✅ Cases to use each pattern matching for Option and Return
* Use `match` when you want to pattern match against the inner types `T` and `E`
```rust
match self {
    Ok(Direction::South) => { … },
    Ok(Direction::North) => { … },
    Ok(Direction::East) => { … },
    Ok(Direction::West) => { … },
    Err(E::One) => { … },
    Err(E::Two) => { … },
}

match self {
    Some(3|5) => { … }
    Some(x) if x > 10 => { … }
    Some(x) => { … }
    None => { … }
}
```

* Use `match` when your type is transformed into something more complex Like `Result<T, E>` becoming `Result<Option<T>, E>`.
```rust
match self {
    Ok(t) => Ok(Some(t)),
    Err(E::Empty) => Ok(None),
    Err(err) => Err(err),
}
```

* Use `let PATTERN = EXPRESSION else { DIVERGING_CODE; }` when the divergent code doesn't need to know about the failed pattern matches or doesn't need extra computation:
```rust
let Some(&Direction::North) = self.direction.as_ref() else {
    return Err(DirectionNotAvailable(self.direction));
}
```

* Use `let PATTERN = EXPRESSION else { DIVERGING_CODE; }` when you want to break or continue a pattern match
```rust
for x in self {
    let Some(x) = x else {
        continue;
    }
}
```

* Use `if let PATTERN = EXPRESSION else { DIVERGING_CODE; }` when `DIVERGING_CODE` needs extra computation:
```rust
if let Some(x) = self.next() {
    // computation
} else {
    // computation when `None/Err` or not matched
}
```

❗**If you don't care about the value of the `Err` case, please use `?` to propagate the `Err` to the caller.**

### ❌ Bad Option/Return pattern matching:

* Conversion between Result and Option (prefer `.ok()`,`.ok_or()`, and `ok_or_else()`)
```rust
match self {
    Ok(t) => Some(t),
    Err(_) => None
}
```

* `if let PATTERN = EXPRESSION else { DIVERGING_CODE; }` when divergent code is a default or pre-computed value (prefer `let PATTERN = EXPRESSION else { DIVERGING_CODE; }`):
```rust
if let Some(values) = self.next() {
    // computation
    (Some(..), values)
} else {
    (None, Vec::new())
}
```

* Using `unwrap` or `expect` outside tests:
```rust
let port = config.port.unwrap();
```

## 1.4 Prevent Early Allocation

When dealing with functions like `or`, `map_or`, `unwrap_or`, `ok_or`, consider that they have special cases for when memory allocation is required, like creating a new string, creating a collection or even calling functions that manage some state, so they can be replaced with their `_else` counter-part:

### ✅ Good cases

```rust
let x = None;
assert_eq!(x.ok_or(ParseError::ValueAbsent), Err(ParseError::ValueAbsent));

let x = None;
assert_eq!(x.ok_or_else(|| ParseError::ValueAbsent(format!("this is a value {x}"))), Err(ParseError::ValueAbsent));


let x: Result<_, &str> = Ok("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);


let x : Result<_, String> = Ok("foo");
assert_eq!(x.map_or_else(|e|format!("Error: {e}"), |v| v.len()), 3);

let x = "1,2,3,4";
assert_eq!(x.parse_to_option_vec.unwrap_or_else(Vec::new), Ok(vec![1, 2, 3, 4]));
```

### ❌ Bad cases

```rust
let x : Result<_, String> = Ok("foo");
assert_eq!(x.map_or(format!("Error with uninformed content"), |v| v.len()), 3);

let x = "1,2,3,4";
assert_eq!(x.parse_to_option_vec.unwrap_or(Vec::new()), Ok(vec![1, 2, 3, 4])); // could be replaced with `.unwrap_or_default`

let x = None;
assert_eq!(x.ok_or(ParseError::ValueAbsent(format!("this is a value {x}"))), Err(ParseError::ValueAbsent));
```

### Mapping Err

When dealing with Result::Err, sometimes is necessary to log and transform the Err into a more abstract or more detailed error, this can be done with `inspect_err` and `map_err`:

```rust
let x = Err(ParseError::InvalidContent(...));

x
    .inspect_err(|err| tracing::error!("function_name: {err}"))
    .map_err(|err| GeneralError::from(("function_name", err)))?;
```

## 1.5 Iterator, `.iter` vs `for`

First we need to understand a basic loop with each one of them. Let's consider the following problem, we need to sum all even numbers between 0 and 10 incremented by 1:

* `for`:
```rust
let mut sum = 0;
for x in 0..=10 {
    if x % 2 == 0 {
        sum += x + 1;
    }
}
```

* `iter`:
```rust
let sum: i32 = (0..=10)
    .filter(|x| x % 2 == 0)
    .map(|x| x + 1)
    .sum();
```

> Both versions do the same thing and are correct and idiomatic, but each shines in different contexts.

### When to prefer `for` loops
* When you need **early exits** (`break`, `continue`, `return`).
* **Simple iteration** with side-effects (e.g., logging, IO)
    * logging can be done correctly in `Iterators` using `inspect` and `inspect_err` functions.
* When readability matters more than simplicity or chaining.

#### Example:
```rust
for value in &mut value {
    if *value == 0 {
        break;
    }
    *value += fancy_equation();
}
```

### When to prefer `iterators` loops (`.iter()` and `.into_iter()`)
* When you are `transforming collections` or `Option/Results`.
* You can **compose multiple steps** elegantly.
* No need for early exits.
* You need support for indexed values with `.enumerate`.
```rust
let values: Vec<_> = vec.into_iter()
    .enumerate()
    .filter(|(_index, value)| value % 2 == 0)
    .map(|(index, value)| value % index)
    .collect()
```
* You need to use collections functions like `.windows` or `chunks`.
* You need to combine data from multiple sources and don't want to allocate multiple collections.
* Iterators can be combined with `for` loops:
```rust
for value in vec.iter().enumerate()
    .filter(|(index, value)| value % index == 0) {
    // ...
}
```

> #### ❗REMEMBER: Iterators are Lazy
>
> * `.iter`, `.map`, `.filter` don't do anything until you call its consumer, e.g. `.collect`, `.sum`, `.for_each`.
> * **Lazy Evaluation** means that iterator chains are fused into one loop at compile time.

### 🚨 Anti-patterns to AVOID

* Don't chain without formatting. Prefer each chained function on its own line with the correct indentation (`rustfmt` should take care of this).
* Don't chain if it makes the code unreadable.
* Avoid needlessly collect/allocate of a collection (e.g. vector) just to throw it away later by some larger operation or by another iteration.
* Prefer `iter` over `into_iter` unless you don't need the ownership of the collection.
* Prefer `iter` over `into_iter` for collections that inner type implements `Copy`, e.g. `Vec<i32>`.
* For summing numbers prefer `.sum` over `.fold`. `.sum` is specialized for summing values, so the compiler knows it can make optimizations on that front, while fold has a blackbox closure that needs to be applied at every step. If you need to sum by an initial value, just added in the expression `let my_sum = [1, 2, 3].sum() + 3`.

## 1.6 Comments: Context, not Clutter

> "Context are for why, not what or how"

Well-written Rust code, with expressive types and good naming, often speaks for itself. Many high-quality codebases thrive on **few or no comments**. And that's a good thing.

Still, there are **moments where code alone isn't enough** - when there are performance quirks, external constraints, or non-obvious tradeoffs that require a nudge to the reader. In those cases, a concise comment can prevent hours of head-scratching or searching git history.

### ✅ Good comments 

* Safety concerns:
```rust
// SAFETY: We have checked that the pointer is valid and non-null. @Function xyz.
unsafe { std::ptr::copy_nonoverlapping(src, dst, len); }
```

* Performance quirks:
```rust
// This algorithm is a fast square root approximation
const THREE_HALVES: f32 = 1.5;
fn q_rsqrt(number: f32 ) -> f32 {
    let mut i: i32 = number.to_bits() as i32;
    i = 0x5F375A86_i32.wrapping_sub(i >> 1);
    let y = f32::from_bits(i as u32);
    y * (THREE_HALVES - (number * 0.5 * y * y))
}
```

* Clear code beats comments. However, when the why isn't obvious, say it plainly - or link to where:
```rust
// PERF: Generating the root store per subgraph caused high TLS startup latency on MacOS
// This works as a caching alternative. See: [ADR-123](link/to/adr-123)
let subgraph_tls_root_store: RootCertStore = configuration
    .tls
    .subgraph
    .all
    .create_certificate_store()
    .transpose()?
    .unwrap_or_else(crate::services::http::HttpClientService::native_roots_store);
```

### ❌ Bad comments

* Wall-of-text explanations: long comments and multiline comments
```rust
// Lorem Ipsum is simply dummy text of the printing and typesetting industry. 
// Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, 
// when an unknown printer took a galley
fn do_something_odd() {
    …
}
```
> Prefer `/// doc` comment if it's describing the function.

* Comments that could be better represented as functions or are plain obvious
```rust
fn computation() {
    // increment i by 1
    i += 1;
}
```

### ✅ Breaking up long functions over commenting them

If you find yourself writing a long comment explaining "what", "how" or "each step" in a function, it might be time to split it. So the suggestion is to refactor. This can be beneficial not only for readability, but testability:

#### ❌ Instead of:
```rust
fn process_request(request: T) {
    // We first need to validate request, because of corner case x, y, z
    // As the payload can only be decoded when they are valid
    // Then we can perform authorization on the payload
    // lastly with the authorized payload we can dispatch to handler
}
```

#### ✅ Prefer
```rust
fn process_request(request: T) -> Result<(), Error> {
    validate_request_headers(&request)?;
    let payload = decode_payload(&request);
    authorize(&payload)?;
    dispatch_to_handler(payload)
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_request_happy_path() { ... }

    #[test]
    fn validate_request_fails_on_x() { ... }

    #[test]
    fn validate_request_fails_on_y() { ... }

    #[test]
    fn decode_validated_request() { ... }

    #[test]
    fn authorize_payload_xyz() { ... }
}
```

Let **structure** and **naming** replace commentary, and enhance its documentation with **tests as living documentation**.

### 📝 TODOs are not comments - track them properly

Avoid leaving lingering `// TODO: Lorem Ipsum` comments in the code. Instead:
* Turn them into Jira or Github Issues.
* If needed, to avoid future confusion, reference the issue in the code and the code in the issue.

```rust
// See issue #123: support hyper 2.0
```

This helps keeping the code clean and making sure tasks are not forgotten.

### Comments as Living Documentation

There are a few gotchas when calling comments "living documentation":
* Code evolves.
* Context changes.
* Comments get stale.
* Many large comments make people avoid reading them.
* Team becomes fearful of delete irrelevant comments.

If you find a comment, **don't trust it blindly**. Read it in context. If it's wrong or outdated, fix or remove it. A misleading comment is worse than no comments at all. 

> Comments should bother you - they demand re-verification, just like stale tests.

When deeper justification is needed, prefer to:
* **Link to a Design Doc or an ADR**, business logic lives well in design docs while performance tradeoffs live well in ADRs.
* Move runtime example and usage docs into Rust Docs, `/// doc comment`, where they can be tested and kept up-to-date by tools like `cargo doc`.

> Doc-comments and Doc-testing, `///` and `//!` in [Chapter 8 - Comments vs Documentation](./chapter_08.md)

## 1.7 Use Declarations - "imports"

Different languages have different ways of sorting their imports, in the Rust ecosystem the [standard way](https://github.com/rust-lang/rustfmt/issues/4107) is:

- `std` (`core`, `alloc` would also fit here).
- External crates (what is in your Cargo.toml `[dependencies]`).
- Workspace crates (workspace member crates).
- This module `super::`.
- This module `crate::`.

```rust
// std
use std::sync::Arc;

// external crates
use chrono::Utc;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

// crate code lives in workspace
use broker::database::PooledConnection;

// super:: / crate::
use super::schema::{Context, Payload};
use super::update::convert_publish_payload;
use crate::models::Event;
```

Some enterprise solutions opt to include their core packages after `std`, so all external packages that start with enterprise name are located before the others:

```rust
// std
use std::sync::Arc;

// enterprise external crates
use enterprise_crate_name::some_module::SomeThing;

// external crates
use chrono::Utc;
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

// crate code lives in workspace
use broker::database::PooledConnection;

// super:: / crate::
use super::schema::{Context, Payload};
use super::update::convert_publish_payload;
use crate::models::Event;
```

One way of not having to manually control this is using the following arguments in your `rustfmt.toml`:

```toml
reorder_imports = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

> As of Rust version 1.88, it is necessary to execute rustfmt in nightly to correctly reorder code `cargo +nightly fmt`.
````

## File: .agents/skills/rust-best-practices/references/chapter_02.md
````markdown
# Chapter 2 - Clippy and Linting Discipline

Be sure to have `cargo clippy` installed with your rust compiler, run `cargo clippy -V` in your terminal for a rust project and you should get something like this `clippy 0.1.86 (05f9846f89 2025-03-31)`. If terminal fails to show a clippy version, please run the following code `rustup update && rustup component add clippy`.

Clippy documentation can be found [here](https://doc.rust-lang.org/clippy/usage.html).

## 2.1 Why care about linting?

Rust compiler is a powerful tool that catches many mistakes. However, some more in-depth analysis require extra tools, that is where `cargo clippy` clippy comes into to play. Clippy checks for:
* Performance pitfalls.
* Style issues.
* Redundant code.
* Potential bugs.
* Non-idiomatic Rust.

## 2.2 Always run `cargo clippy`

Add the following to your daily workflow:

```shell
$ cargo clippy --all-targets --all-features --locked -- -D warnings
```

* `--all-targets`: checks library, tests, benches and examples.
* `--all-features`: checks code for all features enabled, auto solves conflicting features.
* `--locked`: Requires `Cargo.lock` to be up-to-date, can be solved with `$ cargo update`.
* `-D warnings`: treats warnings as errors

Potential additions elements to add:

* `-- -W clippy::pedantic`: lints which are rather strict or have occasional false positives.
* `-- -W clippy::nursery`: Optionally can be added to check for new lints that are still under development.
* ❗ Add this to your Makefile, Justfile, xtask or CI Pipeline.

> Example at ApolloGraphQL
>
> In the `Router` project there is a `xtask` configured for linting that can be executed with `cargo xtask lint`. 

## 2.3 Important Clippy Lints to Respect

| Lint Name | Why | Link |
| --------- | ----| -----|
| `redundant_clone` | Detects unnecessary `clones`, has performance impact | [link (nursery + perf)](https://rust-lang.github.io/rust-clippy/master/#redundant_clone) |
| `needless_borrow` group | Removes redundant `&` borrowing | [link (style)](https://rust-lang.github.io/rust-clippy/master/#needless_borrow) |
| `map_unwrap_or` / `map_or` | Simplifies nested `Option/Result` handling | [`map_unwrap_or`](https://rust-lang.github.io/rust-clippy/master/#map_unwrap_or) [`unnecessary_map_or`](https://rust-lang.github.io/rust-clippy/master/#unnecessary_map_or) [`unnecessary_result_map_or_else`](https://rust-lang.github.io/rust-clippy/master/#unnecessary_result_map_or_else) |
| `manual_ok_or` | Suggest using `.ok_or_else` instead of `match` | [link (style)](https://rust-lang.github.io/rust-clippy/master/#manual_ok_or) |
| `large_enum_variant` | Warns if an enum has very large variant which is bad for memory. Suggests `Boxing` it | [link (perf)](https://rust-lang.github.io/rust-clippy/master/#large_enum_variant) |
| `unnecessary_wraps` | If your function always returns `Some` or `Ok`, you don't need `Option`/`Result` | [link (pedantic)](https://rust-lang.github.io/rust-clippy/master/#unnecessary_wraps) |
| `clone_on_copy` | Catches accidental `.clone()` on `Copy` types like `u32` and `bool` | [link (complexity)](https://rust-lang.github.io/rust-clippy/master/#clone_on_copy) |
| `needless_collect` | Prevents collecting and allocating an iterator, when allocation is not needed | [link (nursery)](https://rust-lang.github.io/rust-clippy/master/#needless_collect) |

## 2.4 Fix warnings, don't silence them!

**NEVER** just `#[allow(clippy::lint_something)]` unless:

* You **truly understand** why the warning happens and you have a reason why it is better that way.
* You **document** why it is being ignored.
* ❗ Don't use `allow`, but `expect`, it will give a warning in case the lint is not true anymore, `#[expect(clippy::lint_something)]`.

### Example:

```rust
// Faster matching is preferred over size efficiency
#[expect(clippy::large_enum_variant)]
enum Message {
    Code(u8),
    Content([u8; 1024]),
}
```

> The fix would be:
> 
> ```rust
> // Faster matching is preferred over size efficiency
> #[expect(clippy::large_enum_variant)]
> enum Message {
>     Code(u8),
>     Content(Box<[u8; 1024]>),
> }
> ```

### Handling false positives

Sometimes Clippy complains even when your code is correct, in those cases there are two solutions:
1. Try to refactor the code, so it improves the warning.
2. **Locally** override the lint with `#[expect(clippy::lint_name)]` and a comment with the reason.
3. Avoid global overrides, unless it is core crate issue, a good example of this is the Bevy Engine that has a set of lints that should be allowed by default.

## 2.5 Configure workspace/package lints

In your `Cargo.toml` file it is possible to determine which lints and their priorities over each other. In case of 2 or more conflicting lints, the higher priority one will be chosen. Example configuration for a package:

```toml
[lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[lints.clippy]
all = { level = "deny", priority = 10 }
redundant_clone = { level = "deny", priority = 9 }
manual_while_let_some = { level = "deny", priority = 4 }
pedantic = { level = "warn", priority = 3 }
```

And for a workspace:

```toml
[workspace.lints.rust]
future-incompatible = "warn"
nonstandard_style = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = 10 }
redundant_clone = { level = "deny", priority = 9 }
manual_while_let_some = { level = "deny", priority = 4 }
pedantic = { level = "warn", priority = 3 }
```
````

## File: .agents/skills/rust-best-practices/references/chapter_03.md
````markdown
# Chapter 3 - Performance Mindset

The **golden rule** of performance work:

> Don't guess, measure.

Rust code is often already pretty fast - don't "optimize" without evidence. Optimize only after finding bottlenecks.

### A good first steps
* Use `--release` flag on you builds (might sound dummy, but it is quite common to hear people complaining that their Rust code is slower than their X language code, and 99% of the time is because they didn't use the `--release` flag).
* `$ cargo clippy -- -D clippy::perf` gives you important tips on best practices for performance.
* [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html) is a cargo tool to create micro-benchmarks and test different code solutions. Write a test scenario and bench you solution against the original code, if your improvement is larger than 5%, might be a good performance improvement.
* [`cargo flamegraph`](https://github.com/flamegraph-rs/flamegraph) a powerful profiler for Rust code. For MacOS, [samply](https://github.com/mstange/samply) might be a better DX option.

> #### Further reading on Benchmarking:
> - [How to build a Custom Benchmarking Harness in Rust](https://bencher.dev/learn/benchmarking/rust/custom-harness/)


## 3.1 Flamegraph

Flamegraph helps you visualize how much time CPU spent on each task.

```shell
# Installing flamegraph
cargo install flamegraph

# cargo support provided through the cargo-flamegraph binary!
# defaults to profiling cargo run --release
cargo flamegraph

# by default, `--release` profile is used,
# but you can override this:
cargo flamegraph --dev

# if you'd like to profile a specific binary:
cargo flamegraph --bin=stress2

# Profile unit tests.
# Note that a separating `--` is necessary if `--unit-test` is the last flag.
cargo flamegraph --unit-test -- test::in::package::with::single::crate
cargo flamegraph --unit-test crate_name -- test::in::package::with::multiple:crate

# Profile integration tests.
cargo flamegraph --test test_name

# Run criterion benchmark
# Note that the last --bench is required for `criterion 0.3` to run in benchmark mode, instead of test mode.
cargo flamegraph --bench some_benchmark --features some_features -- --bench

# Run workspace example
cargo flamegraph --example some_example --features some_features
```

> ❗ Always run your profiles with `--release` enabled, the `--dev` flag isn't realistic as it doesn't have optimizations enabled.

The result will look like a flame graph where:

* The `y-axis` shows the **stack depth number**. When looking at a flamegraph, the main function of your program will be closer to the bottom, and the called functions will be stacked on top, with the functions that they call stacked on top of them.

* The `width of each box` shows the **total time that that function** is on the CPU or is part of the call stack. If a function's box is wider than others, that means that it consumes more CPU per execution than other functions, or that it is called more than other functions.

> ❗ The **color of each box** isn't significant, and **is chosen at random**.

### 🚨 Remember
* Thick stacks: heavy CPU usage
* Thin stacks: low intensity (cheap)

## 3.2 Avoid Redundant Cloning

> Cloning is cheap... **until it isn't**

In sections [Borrowing over Cloning](./chapter_01.md#11-borrowing-over-cloning) and [Important Clippy lints to respect](./chapter_02.md#23-important-clippy-lints-to-respect) we mentioned the impacts of cloning and the relevant clippy lint [`redundant_clone`](https://rust-lang.github.io/rust-clippy/master/#redundant_clone), so in this section we will explore a bit "when to pass ownership".

* 🚨 If you really need to clone, leave it to the last moment.

### When to pass ownership?

* Only `.clone()` if you truly need a new owned copy. A few examples:
    * Crate API Design requires owned data.
    * Have overloaded `std::ops` but still need ownership to the old data:
    ```rust
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
    ```
    * Need to do comparison snapshots or due to API you need multiple owned instances of the data.
    ```rust
    fn snapshot(a: &MyValue, b:&MyValue) -> MyValueDiff {
        a - b
    }

    impl Sub for MyValue {
        type Output = MyValueDiff;

        fn sub(self, other: Self) -> MyValue {
            ...
        }
    }

    fn main() {
        let mut a = MyValue::default();
        let b = a.clone();

        a.magical_update();
        println!("{:?}", snapshot(&a, &b));
    }
    ```
* You have reference counted pointers (`Arc, Rc`).
* You have small structs that are to big to `Copy` but as costly as `std::collections`. An example is HTTP client like `hyper_util::client::legacy::Client` that cloning allows you to share the connection pool.
* You have a chained struct modifier that needs owned mutation, some **builders** require owned mutation, but most custom builders can be done with `pub fn with_xyz(&mut self, value: Xyz) -> &mut Self`.
```rust
// Inline `HashMap` insertion extension

fn insert_owned(mut self, key: K, value: V) -> Self {
    self.insert(key, value);
    self
}
```
* Ownership can also be a good way to model business logic / state. For example:
```rust
let not_validated: String = ...;// some user source
let validated = Validate::try_from(not_validated)?;
// Technically that `try_from` maybe didn't need ownership, but taking it lets us model intent
```

### When **NOT** to pass ownership?

* Prefer API designs that take reference (`fn process(values: &[T])`), instead of ownership (`fn process(values: Vec<T>)`).
* If you only need read access to elements, prefer `.iter` or slices:
```rust
for item in &some_vec {
    ...
}
```
* You need to mutate data that is owned by another thread, use `&mut MyStruct`.

### Use `Cow` for `Maybe Owned` data

Sometimes you don't actually need owned data, but that is not clear from the API perspective, so using [`std::borrow::Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) is a way to efficiently address this case:

```rust
use std::borrow::Cow;

fn hello_greet(name: Cow<'_, str>) {
    println!("Hello {name}");
}

hello_greet(Cow::Borrowed("Julia"));
hello_greet(Cow::Owned("Naomi".to_string()));
```

## 3.3 Stack vs Heap: Be size-smart!

### ✅ Good Practices 

* Keep small types (`impl Copy`, `usize`, `bool`, etc) **on the stack**.
* Avoid passing huge types (`> 512 bytes`) by value or transferring ownership. Prefer pass by reference (e.g. `&T` and `&mut T`).
* Heap allocate recursive data structures:
```rust
enum OctreeNode<T> {
    Node(T),
    Children(Box<[Node<T>; 8]>),
}
```
* Return small types by value, types that implement `Copy` or a cheaply Cloned are efficient to return by value (e.g. `struct Vector2 {x: f32, y: f32}`).

### ❗ Be Mindful

* Only use `#[inline]` when benchmark proves beneficial, Rust is already pretty good at inlining **without** hints.
* Avoid massive stack allocations, box them. Example `let buffer: Box<[u8; 65536]> = Box::new(..)` would first allocate `[u8; 65536]` on the stack then box it, a non-const solution to this would be `let buffer: Box<[u8]> = vec![0; 65536].into_boxed_slice()`.
* For large `const` arrays, considering using [crate smallvec](https://docs.rs/smallvec/latest/smallvec/) as it behaves like an array, but is smart enough to allocate large arrays on the heap.

## 3.4 Iterators and Zero-Cost Abstractions

Rust iterators are lazy, but eventually compiled away into very efficient tight loops that are only called when consumed. Chaining `.filter()`, `.map()`, `.rev()`, `.skip()`, `.take()`, `.collect()` usually doesn't cost extra and the compiler can reason well enough how to optimize them.
* Prefer `iterators` over manual `for` loops when working with collections, the compiler can optimize them better than manually doing it.
* Calling `.iter()` only creates a **reference** to the original collection, this allows you to hold multiple iterators of the same collection.

#### ❗ Avoid creating intermediate collections unless it is really needed:

* Consider that `process` accepts an `iterator`.
* ❌ BAD - useless intermediate collection:
```rust
let doubled: Vec<_> = items.iter().map(|x| x * 2).collect();
process(doubled);
```
* ✅ GOOD - pass the iterator (`fn process(arg: impl Iterator<Item = T>)`):
```rust
let doubled_iter = items.iter().map(|x| x * 2);
process(doubled_iter);
```
````

## File: .agents/skills/rust-best-practices/references/chapter_04.md
````markdown
# Chapter 4 - Errors Handling

Rust enforces a strict error handling approach, but *how* you handle them defines where your code feels ergonomic, consistent and safe - as opposing cryptic and painful. This chapter dives into best practices for modeling and managing fallible operations across libraries and binaries.

> Even if you decide to crash you application with `unwrap` or `expect`, Rust forces you to declare that intentionally.

## 4.1 Prefer `Result`, avoid panic 🫨

Rust has a powerful type that wraps fallible data, [`Result<T, E>`](https://doc.rust-lang.org/std/result/), this allows us to handle Error cases according to our needs and manage the state of the application based on that.

* If your function can fail, prefer to return a `Result`:
```rust
fn divide(x: f64, y: f64) -> Result<f64, DivisionError> {
    if y == 0.0 {
        Err(DivisionError::DividedByZero)
    } else {
        Ok(x / y)
    }
}
```

* Use `panic!` only in unrecoverable conditions - typically tests, assertions, bugs or a need to crash the application for some explicit reason.
* There are 3 relevant macros that can replace `panic!` in appropriate conditions:
    * `todo!`, similar to panic, but alerts the compiler that you are aware that there is code missing.
    * `unreachable!`, you have reasoned about the code block and are sure that condition `xyz` is not possible and if ever becomes possible you want to be alerted.
    * `unimplemented!`, specially useful for alerting that a block is not yet implement with a reason.

## 4.2 Avoid `unwrap`/`expect` in Production

Although `expect` is preferred to `unwrap`, as it can have context, they should be avoided in production code as there are smarter alternatives to them. Considering that, they should be used in the following scenarios:
- In tests, assertions or test helper functions.
- When failure is impossible.
- When the smarter options can't handle the specific case.

### 🚨 Alternative ways of handling `unwrap`/`expect`:

* If your `Result` (or `Option`) can have a predefined early return value in case of `Result::Err`, that doesn't need to know the `Err` value, use `let Ok(..) = else { return ... }` pattern, as it helps with flatten functions:
```rust
let Ok(json) = serde_json::from_str(&input) else {
    return Err(MyError::InvalidJson);
}
```
* If your `Result` (or `Option`) needs error recovery in case of `Result::Err`, that doesn't need to know the `Err` value, use `if let Ok(..) else { ... }` pattern:
```rust
if let Ok(json) = serde_json::from_str(&input) else {
    ...
} else {
    Err(do_something_with_input(&input))
}
```
* Functions that can have to handle `Option::None` values are recommended to return `Result<T, E>`, where `E` is a crate or module level error, like the examples above.
* Lastly `unwrap_or`, `unwrap_or_else` or `unwrap_or_default`, these functions help you create alternative exits to unwrap that manage the uninitialized values.

## 4.3 `thiserror` for Crate level errors

Deriving Error manually is verbose and error prone, the rust ecosystem has a really good crate to help with this, `thiserror`. It allows you to create error types that easily implement `From` trait as well as easy error message (`Display`), improving developer experience while working seamlessly with `?` and integrating with `std::error::Error`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Network Timeout")]
    Timeout,
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid request information. Header: {headers}, Metadata: {metadata}")]
    InvalidRequest {
        headers: Headers,
        metadata: Metadata
    }
}
```

### Error Hierarchies and Wrapping

For layered systems the best practice is to use nested `enum/struct` errors with `#[from]`:

```rust
use crate::database::DbError;
use crate::external_services::ExternalHttpError;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Database handler error: {0}")]
    Db(#[from] DbError),
    #[error("External services error: {0}")]
    ExternalServices(#[from] ExternalHttpError)
}
```

## 4.4 Reserve `anyhow` for Binaries

`anyhow` is an amazing crate, and quite useful for projects that are beginning and need accelerated speed. However, there is a turning point where it just painfully propagates through your code, considering this, `anyhow` is recommended only for **binaries**, where ergonomic error handling is needed and there is no need for precise error types:

```rust
use anyhow::{Context, Result, anyhow};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("config.json")
        .context("Failed to read config file")?;
    Config::from_str(&content)
        .map_err(|err| anyhow!("Config parsing error: {err}"))
}
```

### 🚨 `Anyhow` Gotchas

* Keeping the `context` and `anyhow` strings up-to-date in all code base is harder than keeping `thiserror` messages as you don't have a single point of entry.
* `anyhow::Result` erases context that a caller might need, so avoid using it in a library.
* test helper functions can use `anyhow` with little to no issues.

## 4.5 Use `?` to Bubble Errors

Prefer using `?` over verbose alternatives like `match` chains:
```rust
fn handle_request(req: &Request) -> Result<ValidatedRequest, MyError> {
    validate_headers(req)?;
    validate_body_format(req)?;
    validate_credentials(req)?;
    let body = Body::try_from(req)?;

    Ok(ValidatedRequest::try_from((req, body))?)
}
```

> In case error recovery is needed, use `or_else`, `map_err`, `if let Ok(..) else`. To **inspect or log your error**, use `inspect_err`.

## 4.6 Unit Test should exercise errors

While many errors don't implement PartialEq and Eq, making it hard to do direct assertions between them, it is possible to check the error messages with `format!` or `to_string()`, making the errors meaningful and test validated:

```rust
#[test]
fn error_does_not_implement_partial_eq() {
    let err = divide(10., 0.0).unwrap_err();
    assert_eq!(err.to_string(), "division by zero");
}

#[test]
fn error_implements_partial_eq() {
    let err = process(my_value).unwrap_err();

    assert_eq!(
        err,
        MyError {
            ..
        }
    )
}
```

## 4.7 Important Topics

### Custom Error Structs

Sometimes you don't need an enum to handle your errors, as there is only one type of error that your module can have. This can be solved with `struct Errors`:

```rust
#[derive(Debug, thiserror::Error, PartialEq)]
#[error("Request failed with code `{code}`: {message}")]
struct HttpError {
    code: u16,
    message: String
}
```

### Async Errors

When using async runtimes, like Tokio, make sure that your errors implement `Send + Sync + 'static` where needed, specially in tasks or across `.await` boundaries:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ...
    Ok(())
}
```

> Avoid `Box<dyn std::error::Error>` in libraries unless it is really needed
````

## File: .agents/skills/rust-best-practices/references/chapter_05.md
````markdown
# Chapter 5 - Automated Testing

> Tests are not just for correctness. They are the first place people look to understand how your code works.

* Tests in rust are declared with the attribute macro `#[test]`. Most code editors can compile and run the functions declared under the macro individually or blocks of them.
* Test can have special compilation flags with `#[cfg(test)]`. Also executable in code editors if it contained `#[test]`, it is a good way to mock complicated functions or override traits.

## 5.1 Tests as Living Documentation

In Rust, as in many other languages, tests often show how the functions are meant to be used. If a test is clear and targeted, it's often more helpful than reading the function body, when combined with other tests, they serve as living documentation.

### Use descriptive names

> In the unit test name we should see the following:
> * `unit_of_work`: which *function* we are calling. The **action** that will be executed. This is often be the name of the the test `mod` where the function is being tested.
```rust
#[cfg(test)] 
mod test { 
    mod function_name { 
        #[test] 
        fn returns_y_when_x() { ... } 
    } 
}
```
> * `expected_behavior`: the set of **assertions** that we need to verify that the test works.
> * `state_that_the_test_will_check`: the general **arrangement**, or setup, of the specific test case.

#### ❌ Don't use a generic name for a test
```rust
#[test]
fn test_add_happy_path() {
    assert_eq!(add(2, 2), 4);
}
```
#### ✅ Use a name which reads like a sentence, describing the desired behavior
> Alternatively, if you function has too many tests, you can blob them together in a `mod`, it makes it easier to read and navigate.

```rust
// OPTION 1
#[test]
fn process_should_return_blob_when_larger_than_b() {
    let a = setup_a_to_be_xyz();
    let b = Some(2);
    let expected = MyExpectedStruct { ... };

    let result = process(a, b).unwrap();

    assert_eq!(result, expected);
}

// OPTION 2
mod process {
    #[test]
    fn should_return_blob_when_larger_than_b() {
        let a = setup_a_to_be_xyz();
        let b = Some(2);
        let expected = MyExpectedStruct { ... };

        let result = process(a, b).unwrap();

        assert_eq!(result, expected);
    }
}
```

> When executing `cargo test` the test output for each option will look like:
> Option 1: `process_should_return_blob_when_larger_than_b`.
> Option 2: `process::should_return_blob_when_larger_than_b`.

### Use modules for organization

Most IDEs can run a single module of tests all together.
The test name in the output will also contain the name of the module.
Together, that means you can use the module name to group related tests together:

```rust
#[cfg(test)]
mod test { // IDEs will provide a ▶️ button here

    mod process {
        #[test] // IDEs will provide a ▶️ button here
        fn returns_error_xyz_when_b_is_negative() {
            let a = setup_a_to_be_xyz();
            let b = Some(-5);
            let expected = MyError::Xyz;
            
            let result = process(a, b).unwrap_err();
            
            assert_eq!(result, expected);
        }

        #[test] // IDEs will provide a ▶️ button here
        fn returns_invalid_input_error_when_a_and_b_not_present() {
            let a = None;
            let b = None;
            let expected = MyError::InvalidInput;

            let result = process(a, b).unwrap_err();

            assert_eq!(result, expected);
        }
    }
}
```

### Only test one behavior per function

To keep tests clear, they should describe _one_ thing that the unit does.
This makes it easier to understand why a test is failing.

#### ❌ Don't test multiple things in the same test
```rust
fn test_thing_parser(...) {
    assert!(Thing::parse("abcd").is_ok());
    assert!(Thing::parse("ABCD").is_err());
}
```

#### ✅ Test one thing per test
```rust
#[cfg(test)]
mod test_thing_parser {
    #[test]
    fn lowercase_letters_are_valid() {
        assert!(
            Thing::parse("abcd").is_ok(),
            // Works like `eprintln`, `format` and `println` macros
            "Thing parse error: {:?}", 
            Thing::parse("abcd").unwrap_err()
        );
    }

    #[test]
    fn capital_letters_are_invalid() {
        assert!(Thing::parse("ABCD").is_err());
    }
}
```

> `Ok` scenarios should have an `eprintln` of the `Err` case.

### Use very few, ideally one, assertion per test

When there are multiple assertions per test, it's both harder to understand the intended behavior and 
often requires many iterations to fix a broken test, as you work through assertions one by one.

❌ Don't include many assertions in one test:

```rust
#[test]
fn test_valid_inputs() {
    assert!(the_function("a").is_ok());
    assert!(the_function("ab").is_ok());
    assert!(the_function("ba").is_ok());
    assert!(the_function("bab").is_ok());
}
```

If you are testing separate behaviors, make multiple tests each with descriptive names.
To avoid boilerplate, either use a shared setup function or [rstest](https://crates.io/crates/rstest) cases *with descriptive test names*:
```rust
#[rstest]
#[case::single("a")]
#[case::first_letter("ab")]
#[case::last_letter("ba")]
#[case::in_the_middle("bab")]
fn the_function_accepts_all_strings_with_a(#[case] input: &str) {
    assert!(the_function(input).is_ok());
}
```

> Considerations when using `rstest`
>
> * It's harder for both IDEs and humans to run/locate specific tests.
> * Expectation vs condition naming is now visually inverted (expectation first).

## 5.2 Add Test Examples to your Docs

We will deep dive into docs at a later stage, so in this section we will just briefly go over how to add tests to you docs. Rustdoc can turn examples into executable tests using `///` with a few advantages:

* These tests run with `cargo test` **BUT NOT** `cargo nextest run`. If using `nextest`, make sure to run `cargo t --doc` separately.
* They serve both as documentation and correctness checks, and are kept up to date by changes, due to the fact that the compiler checks them.
* No extra testing boilerplate. You can easily hide test sections by prefixing the line with `#`.
* ❗ There is no issue if you have test duplication between doc-tests and other non-public facing tests.

```rust
/// Helper function that adds any two numeric values together.
/// This function reasons about which would be the correct type to parse based on the type
/// and the size of the numeric value.
/// 
/// # Examples
/// 
/// ```rust
/// # use crate_name::generic_add;
/// use num::numeric;
/// 
/// # assert_eq!(
/// generic_add(5.2, 4) // => 9.2
/// # , 9.2)
/// 
/// # assert_eq!(
/// generic_add(2, 2.0) // => 4
/// # , 4)
/// ```
```

This documentation code would look like:
```rust
use num::numeric;

generic_add(5.2, 4) // => 9.2
generic_add(2, 2.0) // => 4
```

## 5.3 Unit Test vs Integration Tests vs Doc tests

As a general rule, without delving into *test pyramid naming*, rust has 3 sets of tests:

### Unit Test

Tests that go in the **same module** as the **tested unit** was declared, this allows the test runner to have visibility over private functions and parent `use` declarations. They can also consume `pub(crate)` functions from other modules if needed. Unit tests can be more focused on **implementation and edge-cases checks**.

* They should be as simple as possible, testing one state and one behavior of the unit. KISS.
* They should test for errors and edge cases.
* Different tests of the same unit can be combined under a single `#[cfg(test)] mod test_unit_of_work {...}`, allowing multiple submodules for different `units_of_work`.
* Try to keep external states/side effects to your API to minimum and focus those tests on the `mod.rs` files.
* Tests that are not yet fully implemented can be ignored with the `#[ignore = "optional message"]` attribute.
* Tests that intentionally panic should be annotated with the attribute `#[should_panic]`.

```rust
#[cfg(test)]
mod unit_of_work_tests {
    use super::*;

    #[test]
    fn unit_state_behavior() {
        let expected = ...;
        let result = ...;
        assert_eq!(result, expected, "Failed because {}", result - expected);
    }
}
```

### Integration Tests

Tests that go under the `tests/` directory, they are entirely external to your library and use the same code as any other code would use, not have access to private and crate level functions, which means they can **only test** functions on your **public API**. 

> Their purpose is to test whether many parts of the code work together correctly, units of code that work correctly on their own could have problems when integrated.

* Test for happy paths and common use cases.
* Allow external states and side effects, [testcontainers](https://rust.testcontainers.org/) might help.
* if testing binaries, try to break **executable** and **functions** into `src/main.rs` and `src/lib.rs`, respectively.

```
├── Cargo.lock 
├── Cargo.toml 
├── src 
│   └── lib.rs 
└── tests 
    ├── mod.rs 
    ├── common 
    │   └── mod.rs 
    └── integration_test.rs
```

### Doc Testing

As mentioned in section [5.2](#52-add-test-examples-to-your-docs), doc tests should have happy paths, general public API usage and more powerful attributes that improve documentation, like custom CSS for the code blocks.

### Attributes:

* `ignore`: tells rust to ignore the code, usually not recommended, if you want just a code formatted text, use `text`.
* `should_panic`: tells the rust compiler that this example block will panic.
* `no_run`: compiles but doesn't execute the code, similar to `cargo check`. Very useful when dealing with side-effects for documentation.
* `compile_fail`: Test rustdoc that this block should cause a compilation fail, important when you want to demonstrate wrong use cases.

## 5.4 How to `assert!`

Rust comes with 2 macros to make assertions:
* `assert!` for asserting boolean values like `assert!(value.is_ok(), "'value' is not Ok: {value:?}")`
* `assert_eq!` for checking equality between two different values, `assert_eq!(result, expected, "'result' differs from 'expected': {}", result.diff(expected))`.

### 🚨 `assert!` reminders
* Rust asserts support formatted strings, like the previous examples, those strings will be printed in case of failure, so it is a good practice to add what the actual state was and how it differs from the expected.
* If you don't care about the exact pattern matching value, using `matches!` combined with `assert!` might be a good alternative.
```rust
assert!(matches!(error, MyError::BadInput(_), "Expected `BadInput`, found {error}"));
```
* Use `#[should_panic]` wisely. It should only be used when panic is the desired behavior, prefer result instead of panic.
* There are some other that can enhance your testing experience like:
    * [`rstest`](https://crates.io/crates/rstest): fixture based test framework with procedural macros.
    * [`pretty_assertions`](https://crates.io/crates/pretty_assertions): overrides `assert_eq` and `assert_ne`, and creates colorful diffs between them.

## 5.5 Snapshot Testing with `cargo insta`

> When correctness is visual or structural, snapshots tell the story better than asserts.

1. Add to your dependencies:
```toml
insta = { version = "1.42.2", features = ["yaml"] }
```
> For most real world applications the recommendation is to use YAML snapshots of serializable values. This is because they look best under version control and the diff viewer and support redaction. To use this enable the yaml feature of insta.

2. For a better review experience, add the CLI `cargo install cargo-insta`.

3. Writing a simple test:
```rust
fn split_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

#[test]
fn test_split_words() {
    let words = split_words("hello from the other side");
    insta::assert_yaml_snapshot!(words);
}
```

4. Run `cargo insta test` to execute, and `cargo insta review` to review conflicts.

To learn more about `cargo insta` check out its [documentation](https://insta.rs/docs/quickstart/) as it is a very complete and well documented tool.

### What is snapshot testing?

Snapshot testing compares your output (text, Json, HTML, YAML, etc) against a saved "golden" version. On future runs, the test fails if the output changes, unless humanly approved. It is perfect for:
* Generate code.
* Serializing complex data.
* Rendered HTML.
* CLI output.

#### ❌ What not to test with snapshot
* Very stable, numeric-only or small structured data associated logic (prefer `assert_eq!`).
* Critical path logic (prefer precise unit tests).
* Flaky tests, randomly generated output, unless redacted.
* Snapshots of external resources, use mocks and stubs.

## 5.6 ✅ Snapshot Best Practices

* Named snapshots, it gives meaningful snapshot files names, e.g. `snapshots/this_is_a_named_snapshot.snap`
```rust
assert_snapshot!("this_is_a_named_snapshot", output);
```

* Keep snapshots small and clear. 
```rust
// ✅ Best case:
assert_snapshot!("app_config/http", whole_app_config.http);

// ❌ Worst case:
assert_snapshot!("app_config", whole_app_config); // Huge object
```

> #### 🚨 Avoid snapshotting huge objects 
> Huge objects become hard to review and reason about.

* Avoid snapshotting simple types (primitives, flat enums, small structs):
```rust
// ✅ Better:
assert_eq!(meaning_of_life, 42);

// ❌ OVERKILL:
assert_snapshot!("the_meaning_of_life", meaning_of_life); // meaning_of_life == 42
```

* Use [redactions](https://insta.rs/docs/redactions/) for unstable fields (randomly generated, timestamps, uuid, etc):
```rust
use insta::assert_json_snapshot;

#[test]
fn endpoint_get_user_data() {
    let data = http::client.get_user_data();
    assert_json_snapshot!(
        "endpoints/subroute/get_user_data",
        data,
        ".created_at" => "[timestamp]",
        ".id" => "[uuid]"
    );
}
```
* Commit your snapshots into git. They will be stored in `snapshots/` alongside your tests.
* Review changes carefully before accepting.
````

## File: .agents/skills/rust-best-practices/references/chapter_06.md
````markdown
# Chapter 6 - Generics, Dynamic Dispatch and Static Dispatch

> Static where you can, dynamic where you must

Rust allows you to handle polymorphic code in two ways:
* **Generics / Static Dispatch**: compile-time, monomorphized per use.
* **Trait Objects / Dynamic Dispatch**: runtime vtable, single implementation.

Understanding the trade-offs lets you write faster, smaller and more flexible code.

## 6.1 [Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties. We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code. 

We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types. Let's first look at how to define functions, structs, enums, and methods using generics. Generics can also be used to implement Type State Pattern and constrain a struct functionality to certain expected types, more on type state on [Chapter 7](./chapter_07.md).

[Generics by Examples](https://doc.rust-lang.org/rust-by-example/generics.html).

### Generics Performance

You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won't make your program run any slower than it would with concrete types. Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. The compiler checks for all occurrences of the generic parameter and generates code for the concrete types the generic code is called with.

## 6.2 Static Dispatch: `impl Trait` or `<T: Trait>`

A static dispatch is basically a constrained version of a generics, a trait bounded generic, at compile-time it is able to check if your generic satisfies the declared traits.

### ✅ Best when:
* You want **zero runtime cost**, by paying the compile time cost.
* You need **tight loops or performance**.
* Your types are **known at compile time**.
* Your are working with **single-use implementations** (monomorphized).

### 🏎️ Example: High-performance function with generic
```rust
fn specialized_sum<T: MyTrait, U: Iterator<Item = T>>(iter: U) -> T {
    iter.map(|x| x.random_mapping()).sum()
}

// or, equivalent, more modern
fn specialized_sum<T: MyTrait>(iter: impl Iterator<Item = T>) -> T {
    iter.map(|x| x.random_mapping()).sum()
}
```

This is compiled into **specialized machine code** for each usage, fast and inlined.

## 6.3 Dynamic Dispatch: `dyn Trait`

Usually dynamic dispatch is used with some kind of pointer or a reference, like `Box<dyn Trait>`, `Arc<dyn Trait>` or `&dyn trait`.

### ✅ Best when:
* You absolutely need runtime polymorphism.
* You need to **store different implementations** in one collection.
* You want to **abstract internals behind a stable interface**.
* You are writing a **plugin-style architecture**.

> ❗ Closer to what you would get in an object oriented language and can have some heavy costs associated to it. Can avoid generic entirely and let you mix types that implement the same traits.

### 🚚 Example: Heterogeneous collection

```rust
trait Animal {
    fn greet(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn greet(&self) -> String {
        "woof".to_string()
    }
}

struct Cat;
impl Animal for Cat {
    fn greet(&self) -> String {
        "meow".to_string()
    }
}

fn all_animals_greeting(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{}", animal.greet())
    }
}
```

## 6.4 Trade-off summary

| | Static Dispatch (impl Trait) | Dynamic Dispatch (dyn Trait) |
|------------------- |------------------------------ |---------------------------------- |
| Performance | ✅ Faster, inlined | ❌ Slower: vtable indirection |
| Compile time | ❌ Slower: monomorphization | ✅ Faster: shared code |
| Binary size | ❌ Larger: per-type codegen | ✅ Smaller |
| Flexibility | ❌ Rigid, one type at a time | ✅ Can mix types in collections |
| Use in trait fn() | ❌ Traits must be object-safe | ✅ Works with trait objects |
| Errors | ✅ Clearer | ❌ Erased types can confuse errors |

* Prefer generics/static dispatch when you control the call site and want performance.
* Use dynamic dispatch when you need abstraction, plugins or mixed types. 🚨 Runtime cost.
* If you are not sure, start with generics, trait bound them - then use `Box<dyn Trait>` when flexibility outweighs speed.

> Favor static dispatch until your trait needs to live behind a pointer.

## 6.5 Best Practices for Dynamic Dispatch

Dynamic dispatch `Ptr<dyn Trait>` is a powerful tool, but it also has significant performance trade-offs. You should only reach for it when **type erasure or runtime polymorphism** are essential. It is important to know when you need Trait Objects:

### ✅ Use Dynamic Dispatch When:

* You need heterogeneous types in a collection:
```rust
fn all_animals_greeting(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{}", animal.greet())
    }
}
```

* You want runtime plugins or hot-swappable components.
* You want to abstract internals from the caller (library design).


### ❌ Avoid Dynamic Dispatch When:

* You control the concrete types.
* You are writing code in performance critical paths.
* You can express the same logic in other ways while keeping simplicity, e.g. generics.

## 6.6 🚨 Trait Objects Ergonomics

* Prefer `&dyn Trait` over `Box<dyn Trait>` when you don't need ownership.
* Use `Arc<dyn Trait>` for shared access across threads.
* Don't use `dyn Trait` if the trait has methods that return `Self`.
* **Avoid boxing too early**. Don't box inside structs unless you are sure it'll be beneficial or is required (recursive).
```rust
// ✅ Use generics when possible
struct Renderer<B: Backend> {
    backend: B
}

// ❌ Premature Boxing
struct Renderer {
    backend: Box<dyn Backend> // Boxing too early
}
```
* If you must expose a `dyn trait` in a public API, `Box` at the boundary, not internally.
* **Object Safety**: You can only create `dyn Traits` from object-safe traits:
    * It has **no generic methods**.
    * It doesn't require `Self: Sized`.
    * All method signatures use `&self`, `&mut self` or `self`.
    ```rust
    // ✅ Object Safe
    trait Runnable {
        fn run(&self);
    }

    // ❌ Not Object Safe
    trait Factory {
        fn create<T>() -> T; // generic methods are not allowed
    }
    ```
````

## File: .agents/skills/rust-best-practices/references/chapter_07.md
````markdown
# Chapter 7 - Type State Pattern

Models state at compile time, preventing bugs by making illegal states unrepresentable. It takes advantage of the Rust generics and type system to create sub-types that can only be reached if a certain condition is achieved, making some operations illegal at compile time. 

> Recently it became the standard design pattern of Rust programming. However, it is not exclusive to Rust, as it is achievable and has inspired other languages to do the same [swift](https://swiftology.io/articles/typestate/) and [typescript](https://catchts.com/type-state).

## 7.1 What is Type State Pattern?

**Type State Pattern** is a design pattern where you encode different **states** of the system as **types**, not as runtime flags or enums. This allows the compiler to enforce state transitions and prevent illegal actions at compile time. It also improves the developer experience, as developers only have access to certain functions based on the state of the type.

> Invalid states become compile errors instead of runtime bugs.

## 7.2 Why use it?

* Avoids runtime checks for state validity. If you reach certain states, you can make certain assumptions of the data you have.
* Models state transitions as type transitions. This is similar to a state machine, but in compile time.
* Prevents data misuse, e.g. using uninitialized objects.
* Improves API safety and correctness.
* The phantom data field is removed after compilation so no extra memory is allocated.

## 7.3 Simple Example: File State

[Github Example](https://github.com/apollographql/rust-best-practices/tree/main/examples/simple-type-state)
```rust
use std::{io, path::{Path, PathBuf}};

struct FileNotOpened;
struct FileOpened;

#[derive(Debug)]
struct File<State> {
    /// Path to the opened file
    path: PathBuf,
    /// Open `File` handler
    handle: Option<std::fs::File>,
    /// Type state manager
    _state: std::marker::PhantomData<State>
}

impl File<FileNotOpened> {
    /// `open` is the only entry point for this struct.
    /// * When called with a valid path, it will return a `File<FileOpened>` with a valid `handler` and `path`
    /// * `open` serves as an alternative to `new` and `defaults` methods (usable when your struct needs valid data to exist).
    fn open(path: &Path) -> io::Result<File<FileOpened>> {
        // If file is invalid, it will return `std::io::Error`
        let file = std::fs::File::open(path)?;
        Ok(
            File {
                path: path.to_path_buf(),
                // Always valid
                handle: Some(file),
                _state: std::marker::PhantomData::<FileOpened>
            }
        )
    }
}

impl File<FileOpened> {
    /// Reads the content of the `File` as a `String`.
    /// `read` can only be called by state `File<FileOpened>`
    fn read(&mut self) -> io::Result<String> {
        use io::Read;

        let mut content = String::new();
        let Some(handle)= self.handle.as_mut() else {
            unreachable!("Safe to unwrap as state can only be reached when file is open");
        };
        handle.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Returns the valid path buffer.
    fn path(&self) -> &PathBuf {
        &self.path
    }
}
```

## 7.4 Real-World Examples

### Builder Pattern with Compile-Time Guarantees

> Forces the user to **set required fields** before calling `.build()`.

[Github Example](https://github.com/apollographql/rust-best-practices/tree/main/examples/type-state-builder)

A type-state pattern can have more than one associated states:

```rust
use std::marker::PhantomData;

struct MissingName;
struct NameSet;
struct MissingAge;
struct AgeSet;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: Option<String>,
}

struct Builder<NameState, AgeState> {
    name: Option<String>,
    age: u8,
    email: Option<String>,
    _name_marker: PhantomData<NameState>,
    _age_marker: PhantomData<AgeState>,
}

impl Builder<MissingName, MissingAge> {
    fn new() -> Self {
        Builder { name: None, age: 0, _name_marker: PhantomData, _age_marker: PhantomData, email: None }
    }

    fn name(self, name: String) -> Builder<NameSet, MissingAge> {
        Builder { name: Some(name), _name_marker: PhantomData::<NameSet>, age: self.age, _age_marker: PhantomData, email: None }
    }

    fn age(self, age: u8) -> Builder<MissingName, AgeSet> {
        Builder { age, _age_marker: PhantomData::<AgeSet>, name: None, _name_marker: PhantomData, email: None }
    }
}

impl Builder<NameSet, MissingAge> {
    fn age(self, age: u8) -> Builder<NameSet, AgeSet> {
        Builder { age, _age_marker: PhantomData::<AgeSet>, name: self.name, _name_marker: PhantomData::<NameSet>, email: None }
    }
}

impl Builder<MissingName, AgeSet> {
    fn email(self, email: String) -> Self {
        Self { name: self.name , age: self.age , email: Some(email) , _name_marker: self._name_marker , _age_marker: self._age_marker }
    }

    fn name(self, name: String) -> Builder<NameSet, AgeSet> {
        Builder { name: Some(name), _name_marker: PhantomData::<NameSet>, age: self.age, _age_marker: PhantomData::<AgeSet>, email: self.email }
    }
}

impl Builder<NameSet, AgeSet> {
    fn build(self) -> Person {
        Person { 
            name: self.name.unwrap_or_else(|| unreachable!("Name is guarantee to be set")), 
            age: self.age,
            email: self.email,
        }
    }
}
```

Although a bit more verbose than a usual builder, this guarantees that all necessary fields are present (note that e-mail is optional field only present in the final builder).

#### Usage:
```rust
// ✅ Valid cases
let person: Person = Builder::new().name("name".to_string()).age(30).build();
let person: Person = Builder::new().age(30).name("name".to_string()).build();
let person: Person = Builder::new().age(30).name("name".to_string()).email("myself@email.com".to_string()).build();

// ❌ Invalid cases
let person: Person = Builder::new().name("name".to_string()).build(); // ❌ Compile error: Age required to `build`
let person: Person = Builder::new().age(30).build(); // ❌ Compile error: Name required to `build`
let person: Person = Builder::new().age(30).email("myself@email.com".to_string()).build(); // ❌ Compile error: Name required to `build`
let person: Person = Builder::new().build();// ❌ Compile error: Name and Age required to `build`
```

### Network Protocol State Machine

Illegal transitions like sending a message before connecting **simply don't compile**:

```rust
// Mock example
struct Disconnected;
struct Connected;

struct Client<State> {
    stream: Option<std::net::TcpStream>,
    _state: std::marker::PhantomData<State>
}

impl Client<Disconnected> {
    fn connect(addr: &str) -> std::io::Result<Client<Connected>> {
        let stream = std::net::TcpStream::connect(addr)?;
        Ok(Client {
            stream: Some(stream),
            _state: std::marker::PhantomData::<Connected>
        })
    }
}

impl Client<Connected> {
    fn send(&mut self, msg: &str) {
        use std::io::Write;
        let Some(stream) = self.stream.as_mut() else {
            unreachable!("Stream is guarantee to be set");
        };
        stream.write_all(msg.as_bytes())
    }
}
```

## 7.5 Pros and Cons

### ✅ Use Type-State Pattern When:
* Your want **compile-time state safety**.
* You need to enforce **API constraints**.
* You are writing a library/crate that is heavy dependent on variants.
* Your want to replace runtime booleans or enums with **type-safe code paths**.
* You need compile time correctness.

### ❌ Avoid it when:
* Writing trivial states like enums.
* Don't need type-safety.
* When it leads to overcomplicated generics.
* When runtime flexibility is required.

### 🚨 Downsides and Cautions
* Can lead to more **verbose solutions**.
* Can lead to **complex type signatures**.
* May require **unsafe** to return **variant outputs** based on different states.
* May required a bunch of duplication (e.g. same struct field reused).
* PhantomData is not intuitive for beginners and can feel a bit hacky.

> Use this pattern when it **saves bugs, increases safety or simplifies logic**, not just for cleverness.
````

## File: .agents/skills/rust-best-practices/references/chapter_08.md
````markdown
# Chapter 8 - Comments vs Documentation

> Clear code beats clear comments. However, when the why isn't obvious, comment it plainly - or link to where you can read more context.

## 8.1 Comments vs Documentation: Know the Difference

| Purpose | Use `// comment` | Use `/// doc` or `//! crate doc` |
|-------------- |------------------------------------------- |---------------------------------------------------------------- |
| Describe Why | ✅ Yes - explains tricky reasoning | ❌ Not for documentation |
| Describe API | ❌ Not useful | ✅ Yes - public interfaces, usage, details, errors, panics |
| Maintainable | 🚨 Often becomes obsolete and hard to reason | ✅ Tied to code, appears in generated docs and can run test cases |
| Visibility | Local development only | Exported to users and tools like `cargo doc` |

## 8.2 When to use comments

Use `//` comments (double slashed) when something can't be expressed clearly in code, like:
* **Safety Guarantees**, some of which can be better expressed with code conditionals.
* Workarounds or **Optimizations**.
* Legacy or **platform-specific** behaviors. Some of them can be expressed with `#[cfg(..)]`.
* Links to **Design Docs** or **ADRs**.
* Assumptions or **gotchas** that aren't obvious.

> Name your comments! For example, a comment regarding a safety guarantee should start with `// SAFETY: ...`.

### ✅ Good comment:
```rust
// SAFETY: `ptr` is guaranteed to be non-null and aligned by caller
unsafe { std::ptr::copy_nonoverlapping(src, dst, len); }
```

### ✅ Design context comment:
```rust
// CONTEXT: Reuse root cert store across subgraphs to avoid duplicate OS calls:
// [ADR-12](link/to/adr-12): TLS Performance on MacOS
```

## 8.3 When comments get in the way

Avoid comments that:
* Restate obvious things (`// increment i by 1 for the next loop`).
* Can grow stale over time.
* `TODO`s without actions (links to some versioned issue).
* Could be replaced by better naming or smaller functions.

### ❌ Bad comment:
```rust
fn compute(counter: &mut usize) {
    // increment by 1
    *counter += 1;
}
```

### ❌ Too long or outdated
```rust
// Originally written in 2028 for some now-defunct platform
```

## 8.4 Don't Write Living Documentation (living comments)

Comments as a "living documentation" is a **dangerous myth**, as comments are **not free**:
* They **rot** - nobody compiles comments.
* They **mislead** - readers usually assume they are true with no critique, e.g. "the other developer knows this code better than I do".
* They **go stale** - unless maintained with the code, they become irrelevant.
* They are **noisy** - comments can clutter your code with multiple unnecessary lines.

If something deserves to live beyond a PR, put it in:
* An **ADR** (Architectural Design Record).
* A Design Document.
* Document it **in code** by using types, doc comments, examples, renaming code blocks into cleaner functions.
* Add tests to cover and explain the change.

> ### 🚨 If you find a comment, **read it in context**. Does it still make sense? If not, remove or update it, or ask for help. Comments should bother you.

## 8.5 Replace Comments with Code

Instead of long commented blocks, break logic into named helper functions:

#### ❌ Commented code block:
```rust
fn save_user(&self) -> Result<(), MyError> {
    // check if the user is authenticated
    if self.is_authenticated() {
        // serialize user data
        let data = serde_json::to_string(self)?;
        // write to file
        std::fs::write(self.path(), data)?;
    }
}
```
**✅ Extract for clarity**:

```rust
fn save_auth_user(&self) -> Result<PathBuf, MyError> {
    if self.is_authenticated() {
        let path = self.path();
        let serialized_user = serde_json::to_string(self)?;
        std::fs::write(path, serialized_user)?;
        Ok(path)
    } else {
        Err(MyError::UserNotAuthenticated)
    }
}
```

## 8.6 `TODO` should become issues

Don't leave `// TODO:` scattered around the codebase with no owner. Instead:
1. File Github Issue or Jira Ticket. (Prefer github issues on public repositories).
2. Reference the issue in the code:

```rust
// TODO(issue #42): Remove workaround after bugfix
```

This makes `TODO`s trackable, actionable and visible to everyone.

## 8.7 When to use doc comments

Use `///` doc comments to document:
* All **public functions, structs, traits, enums**.
* Their purpose, their usage and their behaviors.
* Anything developers need to understand how to use it correctly.
* Add context that related to `Errors` and `Panics`.
* Plenty of examples.

### ✅ Good doc comment:

```rust
/// Loads [`User`] profile from disk
/// 
/// # Error
/// - Returns [`MyError`] if the file is missing [`MyError::FileNotFound`].
/// - Returns [`MyError`] if the content is an invalid Json, [`MyError::InvalidJson`].
fn load_user(path: &Path) -> Result<User, MyError> {...}
```

**Doc comments can also include examples, links and even tests:**

```rust
/// Returns the square of the integer part of any number.
/// Square is limited to `u128`.
/// 
/// # Examples
/// 
/// ```rust
/// assert_eq!(square(4.3), 16)
/// ```
fn square(x: impl ToInt) -> u128 { ... }
```

## 8.8 Documentation in Rust: How, When and Why

Rust provides **first-class documentation tooling** via rustdoc, which makes documenting your code a key part of writing idiomatic and maintainable rust. There are doc specific lints to help with documentation, like:

| Lint | Description |
|-------------- |------------------------------------------- |
| [missing_docs](https://doc.rust-lang.org/rustdoc/lints.html#missing_docs) | Warns that a public functions, struct, const, enum has missing documentation |
| [broken_intra_doc_links](https://doc.rust-lang.org/rustdoc/lints.html#broken_intra_doc_links) | Detects if an internal documentation link is broken. Specially useful when things are renamed. |
| [empty_docs](https://rust-lang.github.io/rust-clippy/master/#empty_docs) | Disallow empty docs - preventing bypass of `missing_docs` |
| [missing_panics_doc](https://rust-lang.github.io/rust-clippy/master/#missing_panics_doc) | Warns that documentation should have a `# Panics` section if function can panic |
| [missing_errors_doc](https://rust-lang.github.io/rust-clippy/master/#missing_errors_doc) | Warns that documentation should have a `# Errors` section if function returns a `Result` explaining `Err` conditions |
| [missing_safety_doc](https://rust-lang.github.io/rust-clippy/master/#missing_safety_doc) | Warns that documentation should have a `# Safety` section if public facing functions have visible unsafe blocks |


### Difference between `///` and `//!`

| Style | Used for | Scope |Example |
|---------- |------------------------------ |------------------------------------------- |---------------------------------------------------------------- |
| `///` | Line doc comment | Public items like struct, fn, enum, consts | Documenting, giving context and usage to `fn`, `struct`, `enum`, etc |
| `//!` | Module level doc comment | Modules or entire crates | Explaining crate/module purpose with common use cases and quickstart |

### `///` Item level documentation

Use `///` for functions, structs, traits, enums, const, etc:

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
* ✅ Write clear and descriptive **What it does** and **how to use it**.
* ✅ Use `# Examples` section to better explain **how to use it**.
* ✅ Prefer writing examples that can be tested via `cargo test`, even if you have to hide their output with starting `#`:
```rust
/// ```
/// let result = my_crate::add(2, 3);
/// # assert_eq!(result, 5);
/// ```
```
* ✅ Use `# Panics`, `# Errors` and `# Safety` sections when relevant.
* Add relevant context to the type.

### `//!` Module/Crate level Documentation

Use `//!` when you want to document the **purpose of a module or a crate**. It is places at the top of a `lib.rs` or `mod.rs` file, for example `engine/mod.rs`:
```rust
//! This module implements a custom chess engine.
//! 
//! It handles board state, move generation and check detection.
//! 
//! # Example
//! ```
//! let board = chess::engine::Board::default();
//! assert!(board.is_valid());
//! ```
```

## 8.9 Checklist for Documentation coverage

📦 Crate-Level (lib.rs)
- [ ] `//!` doc at top explains **what the crate does**, and **what problems it solves**.
- [ ] Includes crate-level `# Examples` or pointers to modules.

📁 Modules (mod.rs or inline)
- [ ] `//!` doc explains **what this module is for**, its **exports**, and **invariants**.
- [ ] Avoid repeating doc comments on re-exported items unless clarification is needed.

🧱 Structs, Enums, Traits
- `///` doc explains:
    - [ ] The role this type plays.
    - [ ] Invariants or expectations.
    - [ ] Example construction or usage.
- [ ] Consider using [`#[non_exhaustive]`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute) if external users may match on it.

🔧 Functions and Methods
- `///` doc covers:
    - [ ] What it does.
    - [ ] Parameters and their meaning.
    - [ ] Return value behavior.
    - [ ] Edge cases (`# Panics`, `# Errors`).
    - [ ] Usage example, `# Examples`.

📑 Traits
- [ ] Explain the **purpose** of the trait (marker? dynamic dispatch?).
- [ ] Doc for each method — include **when/why** to implement it.
- [ ] Document clearly default implemented methods and when to override.

📦 Public Constants
- [ ] Document what they configure and when you'd want to use them.

### 📌 Best Practices
* ✅ Use examples generously — they double as test cases.
* ✅ Prefer clarity over formality — it's for humans, not machines.
* ✅ Prefer doc comments to explain usage, and leave implementation details to code comments if needed.
* ✅ Use `cargo doc --open` to check your output often.
* ✅ Add `#![deny(missing_docs)]` and other relevant doc lints in top-level modules if you want to enforce full doc coverage.
````

## File: .agents/skills/rust-best-practices/references/chapter_09.md
````markdown
# Chapter 9 - Understanding Pointers

Many higher level languages hide memory management, typically **passing by value** (copy data) or **passing by reference** (reference to shared data) without worrying about allocation, heap, stack, ownership and lifetimes, it is all delegated to the garbage collector or VM. Here is a comparison on this topic between a few languages:

### 📌 Language Comparison 

| Language | Value Types | Reference/Pointer Types | Async Model & Types | Manual Memory |
|------------ |------------------------------------- |----------------------------------------------------------- |---------------------------------------------------------------------------- |------------------------------ |
| Python | None | Everything is a reference | async def, await, Task, coroutines and asyncio.Future | ❌ Not Allowed |
| Javascript | Primitives | Objects | `async/await`, `Promise`, `setTimeout`. single threaded event loop | ❌ Not Allowed |
| Java | Primitives | Objects | `Future<T>`, threads, Loom (green threads) | ❌ Almost none & not recommended |
| Go | Values are copied unless using `&T` | Pointers (`*T`, `&T`), escape analysis | goroutines, `channels`, `sync.Mutex`, `context.Context` | ⚠️ Limited |
| C | Primitives and structs supported | Raw pointers `T*` and `*void` | Threads, event loops (`libuv`, `libevent`) | ✅ Fully |
| C++ | Primitives and references | Raw `T*` and smart pointers `shared_ptr` and `unique_ptr` | threads, `std::future`, `std::async`, (since c++ 20 `co_await/coroutines`) | ✅ Mostly |
| Rust | Primitives, Arrays, `impl Copy` | `&T`, `&mut T`, `Box<T>`, `Arc<T>` | `async/await`, `tokio`, `Future`, `JoinHandle`, `Send + Sync` | ✅🔒 Safe and Explicit |

## 9.1 Thread Safety

Rust tracks pointers using `Send` and `Sync` traits:
- `Send` means data can move across threads.
- `Sync` means data can be referenced from multiple threads.

> A pointer is thread-safe only if the data behind it is.

| Pointer Type | Short Description | Send + Sync? | Main Use |
|---------------- |--------------------------------------------------------------------------- |-------------------------------------- |------------ |
| `&T` | Shared reference | Yes | Shared access |
| `&mut T` | Exclusive mutable reference | No, not Send | Exclusive mutation |
| `Box<T>` | Heap-allocated owning pointer | Yes, if T: Send + Sync | Heap allocation |
| `Rc<T>` | Single-threaded ref counted pointer | No, neither | Multiple owners (single-thread) |
| `Arc<T>` | Atomic ref counter pointer | Yes | Multiple owners (multi-thread) |
| `Cell<T>` | Interior mutability for copy types | No, not Sync | Shared mutable, non-threaded |
| `RefCell<T>` | Interior mutability (dynamic borrow checker) | No, not Sync | Shared mutable, non-threaded |
| `Mutex<T>` | Thread-safe interior mutability with exclusive access | Yes | Shared mutable, threaded |
| `RwLock<T>` | Thread-safe shared readonly access OR exclusive mutable access | Yes | Shared mutable, threaded |
| `OnceCell<T>` | Single-thread one-time initialization container (interior mutability ONCE) | No, not Sync | Simple lazy value initialization |
| `LazyCell<T>` | A lazy version of `OnceCell<T>` that calls function closure to initialize | No, not Sync | Complex lazy value initialization |
| `OnceLock<T>` | Thread-safe version of `OnceCell<T>` | Yes | Multi-thread single init |
| `LazyLock<T>` | Thread-safe version of `LazyCell<T>` | Yes | Multi-thread complex init |
| `*const T/*mut T` | Raw Pointers | No, user must ensure safety manually | Raw memory / FFI |

## 9.2 When to use pointers:

### `&T` - Shared Borrow:

Probably the most common type in a Rust code base, it is **Safe, with no mutation** and allows **multiple readers**.

```rust
let data: String = String::from_str("this a string").unwrap();

print_len(&data);
print_capacity(&data);
print_bytes(&data);

fn print_len(s: &str) {
    println!("{}", s.len())
}

fn print_capacity(s: &String) {
    println!("{}", s.capacity())
}

fn print_bytes(s: &String) {
    println!("{:?}", s.as_bytes())
}
```
### `&mut T` - Exclusive Borrow:

Probably the most common *mutable* type in a Rust code base, it is **Safe, but only allows one mutable borrow at a time**.

```rust
let mut data: String = String::from_str("this a string").unwrap();
mark_update(&mut data);

fn mark_update(s: &mut String) {
    s.push_str("_update");
}
```

### [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html) - Heap Allocated

Single-owner heap-allocated data, great for recursive types and large structs.

```rust
pub enum MySubBoxedEnum<T> {
    Single(T),
    Double(Box<MySubBoxedEnum<T>>, Box<MySubBoxedEnum<T>>),
    Multi(Vec<T>), // Note that Vec is already a boxed value
}
```

### [`Rc<T>`](https://doc.rust-lang.org/std/rc/struct.Rc.html) - Reference Counter (single-thread)

You need multiple references to data in a single thread. Most common example is linked-list implementation.

### [`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html) - Atomic Reference Counter (multi-thread)

You need multiple references to data in multiple threads. Most common use cases is sharing readonly Vec across thread with `Arc<[T]>` and wrapping a `Mutex` so it can be easily shared across threads, `Arc<Mutex<T>>`.

### [`RefCell<T>`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) - Runtime checked interior mutability

Used when you need shared access and the ability to mutate date, borrow rules are enforced at runtime. **It may panic!**.

```rust
use std::cell::RefCell;
let x = RefCell::new(42);
*x.borrow_mut() += 1;

assert_eq!(&*x.borrow(), 42, "Not meaning of life");
```

Panic example:
```rust
use std::cell::RefCell;
let x = RefCell::new(42);

let borrow = x.borrow();

let mutable = x.borrow_mut();
```

### [`Cell<T>`](https://doc.rust-lang.org/std/cell/struct.Cell.html) - Copy-only interior mutability

Somewhat the fast and safe version of `RefCell`, but it is limited to types that implement the `Copy` trait:

```rust
use std::cell::Cell;

struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}

let my_struct = SomeStruct {
    regular_field: 0,
    special_field: Cell::new(1),
};

let new_value = 100;

// ERROR: `my_struct` is immutable
// my_struct.regular_field = new_value;

// WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
// which can always be mutated with copy values
my_struct.special_field.set(new_value);
assert_eq!(my_struct.special_field.get(), new_value);
```

### [`Mutex<T>`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) - Thread-safe mutability

An exclusive access pointer that allows a thread to read/write the data contained inside. It is usually wrapped in an `Arc` to allow shared access to the Mutex.

### [`RwLock<T>`](https://doc.rust-lang.org/std/sync/struct.RwLock.html) - Thread-safe mutability

Similar to a `Mutex`, but it allows multiple threads to read it OR a single thread to write. It is usually wrapped in an `Arc` to allow shared access to the RwLock.


### [`*const T/*mut T`](https://doc.rust-lang.org/std/primitive.pointer.html) - Raw pointers

Inherently **unsafe** and necessary for FFI. Rust makes their usage explicit to avoid accidental misuse and unwilling manual memory management.

```rust
let x = 5;
let ptr = &x as *const i32;
unsafe {
    println!("PTR is {}", *ptr)
}
```

### [`OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html) - Single-thread single initialization container

Most useful when you need to share a configuration between multiple data structures.

```rust
use std::{cell::OnceCell, rc::Rc};

#[derive(Debug, Default)]
struct MyStruct {
    distance: usize,
    root: Option<Rc<OnceCell<MyStruct>>>, 
}

fn main() {
    let root = MyStruct::default();
    let root_cell = Rc::new(OnceCell::new());
    if let Err(previous) = root_cell.set(root) {
        eprintln!("Previous Root {previous:?}");
    }
    let child_1 = MyStruct{
        distance: 1,
        root: Some(root_cell.clone())
    };

    let child_2 = MyStruct{
        distance: 2,
        root: Some(root_cell.clone())
    };


    println!("Child 1: {child_1:?}");
    println!("Child 2: {child_2:?}");
}
```

### [`LazyCell`](https://doc.rust-lang.org/std/cell/struct.LazyCell.html) - Lazy initialization of `OnceCell`

Useful when the initialized data can be delayed to when it is actually being called.

### [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html) - thread-safe `OnceCell`

Useful when you need a `static` value.

```rust
use std::sync::OnceLock;

static CELL: OnceLock<u32> = OnceLock::new();

// `OnceLock` has not been written to yet.
assert!(CELL.get().is_none());

// Spawn a thread and write to `OnceLock`.
std::thread::spawn(|| {
    let value = CELL.get_or_init(|| 12345);
    assert_eq!(value, &12345);
})
.join()
.unwrap();

// `OnceLock` now contains the value.
assert_eq!(
    CELL.get(),
    Some(&12345),
);
```

### [`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html) - thread-safe `LazyCell`

Similar to `OnceLock`, but the static value is a bit more complex to initialize.

```rust
use std::sync::LazyLock;

static CONFIG: LazyLock<HashMap<&str, T>> = LazyLock::new(|| {
    let data = read_config();
    let mut config: HashMap<&str, T> = data.into();
    config.insert("special_case", T::default());
    config
});

let _ = &*CONFIG;
```

## References
- [Mara Bos - Rust Atomics and Locks](https://marabos.nl/atomics/)
- [Semicolon video on pointers](https://www.youtube.com/watch?v=Ag_6Q44PBNs)
````

## File: .agents/skills/rust-best-practices/SKILL.md
````markdown
---
name: rust-best-practices
description: >
  Guide for writing idiomatic Rust code based on Apollo GraphQL's best practices handbook. Use this skill when:
  (1) writing new Rust code or functions,
  (2) reviewing or refactoring existing Rust code,
  (3) deciding between borrowing vs cloning or ownership patterns,
  (4) implementing error handling with Result types,
  (5) optimizing Rust code for performance,
  (6) writing tests or documentation for Rust projects.
license: MIT
compatibility: Rust 1.70+, Cargo
metadata:
  author: apollographql
  version: "1.1.0"
allowed-tools: Bash(cargo:*) Bash(rustc:*) Bash(rustfmt:*) Bash(clippy:*) Read Write Edit Glob Grep
---

# Rust Best Practices

Apply these guidelines when writing or reviewing Rust code. Based on Apollo GraphQL's [Rust Best Practices Handbook](https://github.com/apollographql/rust-best-practices).

## Best Practices Reference

Before reviewing, familiarize yourself with Apollo's Rust best practices. Read ALL relevant chapters in the same turn in parallel. Reference these files when providing feedback:

- [Chapter 1 - Coding Styles and Idioms](references/chapter_01.md): Borrowing vs cloning, Copy trait, Option/Result handling, iterators, comments
- [Chapter 2 - Clippy and Linting](references/chapter_02.md): Clippy configuration, important lints, workspace lint setup
- [Chapter 3 - Performance Mindset](references/chapter_03.md): Profiling, avoiding redundant clones, stack vs heap, zero-cost abstractions
- [Chapter 4 - Error Handling](references/chapter_04.md): Result vs panic, thiserror vs anyhow, error hierarchies
- [Chapter 5 - Automated Testing](references/chapter_05.md): Test naming, one assertion per test, snapshot testing
- [Chapter 6 - Generics and Dispatch](references/chapter_06.md): Static vs dynamic dispatch, trait objects
- [Chapter 7 - Type State Pattern](references/chapter_07.md): Compile-time state safety, when to use it
- [Chapter 8 - Comments vs Documentation](references/chapter_08.md): When to comment, doc comments, rustdoc
- [Chapter 9 - Understanding Pointers](references/chapter_09.md): Thread safety, Send/Sync, pointer types

## Quick Reference

### Borrowing & Ownership
- Prefer `&T` over `.clone()` unless ownership transfer is required
- Use `&str` over `String`, `&[T]` over `Vec<T>` in function parameters
- Small `Copy` types (≤24 bytes) can be passed by value
- Use `Cow<'_, T>` when ownership is ambiguous

### Error Handling
- Return `Result<T, E>` for fallible operations; avoid `panic!` in production
- Never use `unwrap()`/`expect()` outside tests
- Use `thiserror` for library errors, `anyhow` for binaries only
- Prefer `?` operator over match chains for error propagation

### Performance
- Always benchmark with `--release` flag
- Run `cargo clippy -- -D clippy::perf` for performance hints
- Avoid cloning in loops; use `.iter()` instead of `.into_iter()` for Copy types
- Prefer iterators over manual loops; avoid intermediate `.collect()` calls

### Linting
Run regularly: `cargo clippy --all-targets --all-features --locked -- -D warnings`

Key lints to watch:
- `redundant_clone` - unnecessary cloning
- `large_enum_variant` - oversized variants (consider boxing)
- `needless_collect` - premature collection

Use `#[expect(clippy::lint)]` over `#[allow(...)]` with justification comment.

### Testing
- Name tests descriptively: `process_should_return_error_when_input_empty()`
- One assertion per test when possible
- Use doc tests (`///`) for public API examples
- Consider `cargo insta` for snapshot testing generated output

### Generics & Dispatch
- Prefer generics (static dispatch) for performance-critical code
- Use `dyn Trait` only when heterogeneous collections are needed
- Box at API boundaries, not internally

### Type State Pattern
Encode valid states in the type system to catch invalid operations at compile time:
```rust
struct Connection<State> { /* ... */ _state: PhantomData<State> }
struct Disconnected;
struct Connected;

impl Connection<Connected> {
    fn send(&self, data: &[u8]) { /* only connected can send */ }
}
```

### Documentation
- `//` comments explain *why* (safety, workarounds, design rationale)
- `///` doc comments explain *what* and *how* for public APIs
- Every `TODO` needs a linked issue: `// TODO(#42): ...`
- Enable `#![deny(missing_docs)]` for libraries
````

## File: .gitignore
````
# Added by code-review-graph
.code-review-graph/
````

## File: .opencode.json
````json
{
  "mcpServers": {
    "code-review-graph": {
      "command": "uvx",
      "args": [
        "code-review-graph",
        "serve"
      ],
      "cwd": "P:\\vs-code\\projects\\Warden",
      "type": "stdio",
      "env": []
    },
    "ast-grep": {
      "command": "ast-grep-mcp-server",
      "args": [],
      "type": "stdio",
      "env": {}
    }
  }
}
````

## File: AGENTS.md
````markdown
# Warden — Peer-to-Peer Decentralized Chat over SSH

## Project Overview

A decentralized chat application where messages travel directly between peers over SSH-secured channels, with no central server storing messages or identities. Peer discovery and NAT traversal are handled by a DHT-based overlay network (Kademlia via libp2p); SSH is used purely as the authenticated transport layer once two peers have found each other.

**Identity = Ed25519 SSH keypair.** No usernames, no passwords, no central registry.

## Architecture

```
+-------------------+     +--------------------+     +-------------------+
|   Peer A          |     |  Discovery Overlay  |     |   Peer B          |
|  SSH Keypair      |<--->|  Kademlia DHT       |<--->|  SSH Keypair      |
|  DHT Node         |     |  Rendezvous Nodes   |     |  DHT Node         |
|  SSH Client/Svr   |     |  Relay Nodes        |     |  SSH Client/Svr   |
|  Encrypted Store  |     +--------------------+     |  Encrypted Store  |
+-------------------+                                  +-------------------+
        |                      SSH tunnel                      |
        +------------------------------------------------------+
                         (pubkey auth, subsystem "chat")
```

### Layer Stack

1. **Identity** — Ed25519 SSH keypair, PeerID = base58(SHA-256(pubkey))
2. **Discovery** — Kademlia DHT (libp2p) maps PeerID → multiaddr
3. **NAT Traversal** — Hole-punching + relay fallback (libp2p circuit relay)
4. **Transport** — SSH subsystem `chat` (embedded SSH server per peer)
5. **Application** — Protobuf-framed ChatFrame messages over SSH channel
6. **Storage** — Encrypted local SQLite database

## Tech Stack

| Layer | Choice | Skill |
|-------|--------|-------|
| Language | Rust | `rust-async-patterns`, `rust-best-practices` |
| Discovery/NAT | libp2p (`rust-libp2p`) | — |
| SSH Transport | `thrussh` / `russh` | — |
| Message Framing | Protocol Buffers | `protobuf` |
| Local Storage | SQLCipher or age-encrypted SQLite | — |
| Client | CLI first, desktop later | — |

## Core Components

- **Identity & Key Manager** — Generate/store Ed25519 keypair, derive PeerID
- **Discovery Module** — libp2p Kademlia DHT client
- **NAT Traversal** — Hole punching + circuit relay fallback
- **SSH Transport** — Embedded SSH server (subsystem-only, hardened)
- **Chat Protocol Handler** — Framed ChatFrame (protobuf) over SSH channel
- **Local Storage** — Encrypted message/contact store
- **Outbox Queue** — Store-and-forward for offline delivery

## Security Model

- SSH public key auth for peer authentication (trust-list / `authorized_keys` model)
- Sign-then-encrypt message frames (portable verification beyond SSH channel)
- Embedded SSH server restricted to `chat` subsystem only — no shell, exec, or port forwarding
- Encrypted local storage at rest
- Allowlist-only spam protection in v1

## Milestones

| Phase | Scope |
|-------|-------|
| M0 | SSH subsystem chat between 2 hardcoded IPs |
| M1 | Kademlia DHT peer discovery |
| M2 | NAT traversal (hole punch + relay) |
| M3 | MVP: contacts, encrypted history, basic UI |
| M4 | Offline store-and-forward delivery |
| M5 | Group chat (multi-peer fanout) |

## Key Design Decisions

- **SSH Subsystem pattern (Option A):** Each peer runs an embedded SSH server; peers connect and request the `chat` subsystem. Reuses well-audited SSH transport/auth.
- **Direct connection first:** Try hole punch, fall back to relay. Relay sees only encrypted SSH bytes.
- **No offline delivery in v1:** Sender queues locally, retries when DHT shows peer online.
- **DHT is public:** PeerID ↔ address mappings are discoverable (same tradeoff as BitTorrent/IPFS).

## Installed Skills

- `rust-async-patterns` — Rust async/await, Tokio, async patterns
- `rust-best-practices` — Rust conventions, idioms, and code quality
- `protobuf` — Protocol Buffer design, buf CLI, schema evolution

## Agent Guidelines

### Tool Hierarchy (use in this order)
1. **code-review-graph MCP tools** — structural queries, callers/callees, impact radius
2. **ast-grep MCP** — AST-aware structural search (before regex)
3. **LSP references** — go-to-definition, find-all-references via rust-analyzer
4. **ripgrep** — only when graph and AST don't cover the need
5. **Read** — read only specific functions/modules, never entire files

### Code Change Rules
- **Preserve project architecture** — match existing patterns for modules, error handling, imports
- **Minimal edits** — change only what's necessary; no incidental reformatting
- **No `any` in Rust** — keep type-safe; avoid `unsafe` unless absolutely required
- **Preserve formatting** — do not reformat existing code; match surrounding style
- **Read before write** — understand the file's imports and conventions before editing
- **Avoid unnecessary edits** — if a change isn't requested, don't make it

### Context Minimization
- Never read an entire file when a function/block suffices
- Use `get_minimal_context` (~100 tokens) as your first entry point
- Use `query_graph` to discover relationships before reading files
- Batch reads in parallel when you need multiple files

<!-- code-review-graph MCP tools -->
## MCP Tools: code-review-graph

**IMPORTANT: This project has a knowledge graph. ALWAYS use the
code-review-graph MCP tools BEFORE using Grep/Glob/Read to explore
the codebase.** The graph is faster, cheaper (fewer tokens), and gives
you structural context (callers, dependents, test coverage) that file
scanning cannot.

### When to use graph tools FIRST

- **Exploring code**: `semantic_search_nodes` or `query_graph` instead of Grep
- **Understanding impact**: `get_impact_radius` instead of manually tracing imports
- **Code review**: `detect_changes` + `get_review_context` instead of reading entire files
- **Finding relationships**: `query_graph` with callers_of/callees_of/imports_of/tests_for
- **Architecture questions**: `get_architecture_overview` + `list_communities`

Fall back to Grep/Glob/Read **only** when the graph doesn't cover what you need.

### Key Tools

| Tool | Use when |
| ------ | ---------- |
| `detect_changes` | Reviewing code changes — gives risk-scored analysis |
| `get_review_context` | Need source snippets for review — token-efficient |
| `get_impact_radius` | Understanding blast radius of a change |
| `get_affected_flows` | Finding which execution paths are impacted |
| `query_graph` | Tracing callers, callees, imports, tests, dependencies |
| `semantic_search_nodes` | Finding functions/classes by name or keyword |
| `get_architecture_overview` | Understanding high-level codebase structure |
| `refactor_tool` | Planning renames, finding dead code |

### Workflow

1. The graph auto-updates on file changes (via hooks).
2. Use `detect_changes` for code review.
3. Use `get_affected_flows` to understand impact.
4. Use `query_graph` pattern="tests_for" to check coverage.
````

## File: docs/ai/setup.md
````markdown
# AI Environment Setup

## Overview

This document describes the AI-augmented development environment configured for this repository. The setup integrates local code intelligence tools with OpenCode to minimize token usage while maximizing code understanding.

## Prerequisites

- **Windows PowerShell 5.1+** (the shell this environment targets)
- **Node.js 24+** (for OpenCode and npm-installed tools)
- **Rust toolchain 1.92+** (for cargo-installed tools)
- **Python 3.10+** (for code-review-graph dependencies via uv)

## Installed Tools

| Tool | Version | Installer | Purpose |
|------|---------|-----------|---------|
| OpenCode | 1.17.13 | npm | AI coding agent |
| code-review-graph | 2.3.6 | uv | Knowledge graph for code |
| ast-grep | 0.44.1 | npm | AST pattern search |
| ast-grep-mcp-server | 0.1.3 | cargo | MCP server for ast-grep |
| ripgrep | 15.1.0 | cargo | Fast file search |
| fd | 10.4.2 | cargo | Fast file find |
| Repomix | 1.16.0 | npm | Repository summarization |
| GitHub CLI | 2.87.3 | system | GitHub integration |
| uv / uvx | 0.11.26 | installer | Python package manager |
| pipx | 1.15.0 | uv | Python app isolation |
| rust-analyzer | 1.92.0 | rustup | Rust LSP server |
| typescript-language-server | 5.3.0 | npm | TypeScript LSP server |

## Quick Start

```powershell
# Build the knowledge graph
code-review-graph build

# Watch for changes (in background terminal)
code-review-graph watch

# Generate repo summary
repomix

# AST search
ast-grep run --pattern "unwrap()"

# Fast file search
rg "fn main" --type rust

# Fast file find
fd "mod.rs"
```

## Configuration Files

- `.opencode.json` — OpenCode MCP server configuration
- `AGENTS.md` — Agent instructions with graph-aware workflow
- `scripts/*.ps1` — Repository intelligence scripts
- `docs/ai/*.md` — This documentation suite
````

## File: docs/ai/tools.md
````markdown
# Tools Reference

## code-review-graph

**Purpose:** Builds a persistent, incrementally-updated structural knowledge graph of the codebase using Tree-sitter AST parsing.

**Installation:** `uv tool install code-review-graph`

**Key Commands:**

| Command | Description |
|---------|-------------|
| `code-review-graph build` | Full graph build from scratch |
| `code-review-graph update` | Incremental update (git-based) |
| `code-review-graph watch` | File watcher for auto-updates |
| `code-review-graph status` | Graph health and statistics |
| `code-review-graph serve` | Start MCP server for AI tools |
| `code-review-graph detect-changes` | Risk-scored change analysis |
| `code-review-graph visualize` | Generate HTML graph visualization |
| `code-review-graph install` | Auto-configure for supported AI platforms |

**MCP Tools (available via OpenCode):**
- `build_or_update_graph_tool` — Build or incrementally update
- `get_minimal_context_tool` — Ultra-compact context (~100 tokens)
- `get_impact_radius_tool` — Blast radius of changed files
- `get_review_context_tool` — Token-optimized review context
- `query_graph_tool` — Callers, callees, tests, imports, inheritance
- `semantic_search_nodes_tool` — Search by name or meaning
- `detect_changes_tool` — Risk-scored change impact
- `refactor_tool` — Rename preview, dead code detection

---

## ast-grep

**Purpose:** Structural code search using AST patterns rather than regex.

**Installation:** `npm install -g @ast-grep/cli`

**Key Commands:**

| Command | Description |
|---------|-------------|
| `ast-grep run --pattern "..."` | Search by AST pattern |
| `ast-grep run --pattern "..." --rewrite "..."` | Search and rewrite |
| `ast-grep run --pattern "..." --lang rust` | Language-specific search |
| `ast-grep lsp` | Start LSP server |
| `ast-grep scan` | Scan with rule configuration |
| `ast-grep test` | Test ast-grep rules |

**Examples:**
```bash
# Find all unwrap() calls in Rust
ast-grep run --pattern "unwrap()" --lang rust

# Find all async function definitions
ast-grep run --pattern "async fn $$($$$)" --lang rust

# Find uses of a specific struct
ast-grep run --pattern "SomeStruct" --lang rust
```

**MCP Server:** `ast-grep-mcp-server` (installed via cargo) provides AST search capabilities through MCP.

---

## ripgrep (rg)

**Purpose:** Ultra-fast recursive file content search with regex support.

**Installation:** `cargo install ripgrep`

**Key Usage:**
```bash
rg "pattern"                      # Basic search
rg "fn main" --type rust          # Search only Rust files
rg "impl" -g "*.rs"               # Glob filter
rg "TODO|FIXME" --type rust       # Multi-pattern
rg "pattern" --context 3          # Show surrounding context
rg "pattern" -l                   # List files only
```

---

## fd

**Purpose:** Fast, user-friendly file finder.

**Installation:** `cargo install fd-find`

**Key Usage:**
```bash
fd "main.rs"                      # Find file
fd "mod" --type d                 # Find directories
fd "*.rs" -x rg "fn main" {}     # Find and exec
fd --ext rs "server"              # By extension
```

---

## Repomix

**Purpose:** Pack entire repository into AI-friendly context files.

**Installation:** `npm install -g repomix`

**Key Usage:**
```bash
repomix                           # Generate repo summary (stdout)
repomix --output repo-context.txt # Write to file
repomix --style markdown          # Markdown output
```

---

## GitHub CLI (gh)

**Purpose:** GitHub operations from the command line.

**Version:** 2.87.3

**Key Usage:**
```bash
gh pr status                      # Check PR status
gh pr create --title "..."        # Create PR
gh pr review 123                  # Review PR
gh issue list                     # List issues
```

---

## uv / uvx

**Purpose:** Fast Python package installer and resolver.

**Installation:** Installed via `irm https://astral.sh/uv/install.ps1 | iex`

**Key Usage:**
```bash
uv tool install <package>         # Install Python tools (like pipx)
uvx <package>                     # Run Python tools without installing
uv venv                           # Create virtual environment
uv pip install <package>          # Install from requirements
```

---

## rust-analyzer

**Purpose:** Rust LSP server providing code completion, navigation, and refactoring.

**Installation:** `rustup component add rust-analyzer`

**Version:** 1.92.0

Provides IDE features including:
- Go to definition / implementation
- Find all references
- Type hints and hover information
- Code actions and refactoring
- Workspace symbol search
````

## File: docs/ai/troubleshooting.md
````markdown
# Troubleshooting

## Tools

### code-review-graph fails with "uvx not found"

```powershell
# Add uv to PATH
$env:Path = "$env:USERPROFILE\.local\bin;$env:Path"
# Or add permanently to PowerShell profile
```

### code-review-graph build fails with no nodes

This is expected for empty or new repositories. The graph will populate once source files exist. Supported languages include Rust, TypeScript, Go, Python, and 15+ others via Tree-sitter.

### ast-grep not found

```powershell
npm install -g @ast-grep/cli
```

### ripgrep (rg) not found

```powershell
cargo install ripgrep
```

### fd not found

```powershell
cargo install fd-find
```

### ast-grep-mcp-server not found

```powershell
cargo install ast-grep-mcp
```

### rust-analyzer not found

```powershell
rustup component add rust-analyzer
```

## OpenCode

### OpenCode cannot connect to MCP servers

1. Verify `.opencode.json` exists and has correct paths
2. Check that `uvx` / `ast-grep-mcp-server` are in PATH
3. Restart OpenCode after modifying `.opencode.json`

### OpenCode plugin not loaded

The code-review-graph plugin is at:
```
$env:USERPROFILE\.config\opencode\plugins\crg-plugin.ts
```

If it's missing, re-run:
```powershell
code-review-graph install --platform opencode -y
```

## Graph Issues

### Graph is stale

```powershell
code-review-graph update
```

### Graph needs full rebuild

```powershell
code-review-graph build
```

### Graph has wrong working directory

Ensure `.opencode.json` has the correct `cwd` field for the code-review-graph MCP server.

## MCP Configuration

The `ast-grep` MCP server uses the Rust binary (`ast-grep-mcp-server`). If it fails to start, check:

1. Binary is in PATH: `which ast-grep-mcp-server`
2. Config file exists if `--config` is specified in args
3. No port conflicts (uses stdio, not network)
````

## File: docs/ai/workflow.md
````markdown
# AI-Assisted Workflow

## Core Principle

**Use the knowledge graph first, then AST search, then LSP, then grep.**

This hierarchy minimizes tokens and latency while maximizing context quality.

## Standard Workflow

### Exploring Code

1. **Query the graph** — `query_graph` for callers, callees, imports
2. **Semantic search** — `semantic_search_nodes` to find by name/purpose
3. **AST search** — `ast-grep run --pattern "..."` for structural matches
4. **Grep** — `rg "keyword"` only when graph and AST don't cover it

### Making Changes

1. **Check impact** — `get_impact_radius` before modifying any function
2. **Get context** — `get_review_context` for the files you need to change
3. **Get affected flows** — `get_affected_flows` for execution path impact
4. **Read minimum** — Read only specific functions, not entire files
5. **Check coverage** — `query_graph pattern="tests_for"` for test files

### Code Review

1. **Detect changes** — `detect_changes` for risk-scored analysis
2. **Review context** — `get_review_context` for token-optimized snippets
3. **Check architecture** — `get_architecture_overview` for structural impact

### Refactoring

1. **Plan renames** — `refactor_tool` for rename preview and dead code
2. **Check blast radius** — `get_impact_radius` for all affected locations
3. **Verify flows** — `get_affected_flows` to ensure no breakage

## Commands Reference

```powershell
# Graph operations
graph              # code-review-graph build
watchgraph         # code-review-graph watch
repoctx            # repomix generate summary

# Search operations
astfind -p "..."   # ast-grep AST search
rg "pattern"       # ripgrep regex search
fd "filename"      # fd file search

# Graph queries
code-review-graph status                          # graph health
code-review-graph detect-changes                  # change analysis
code-review-graph get-minimal-context             # compact context
code-review-graph visualize                       # HTML graph viz
```

## LSP Integration

Before manual searching, prefer LSP-based navigation:

- **Go to definition** — Use LSP references instead of grep
- **Find all references** — Use LSP symbol lookup
- **Type inspection** — Use `rust-analyzer` / `typescript-language-server`

rust-analyzer is configured via rustup component. typescript-language-server is installed globally via npm.
````

## File: scripts/astfind.ps1
````powershell
#!/usr/bin/env pwsh
<#
.SYNOPSIS
    AST-aware code search using ast-grep.
.DESCRIPTION
    Search code by AST pattern rather than regex.
    Supports pattern matching, structural search, and rewriting.
.EXAMPLE
    ./scripts/astfind.ps1 -Pattern "SomeStruct::new"
    ./scripts/astfind.ps1 -Pattern "unwrap()" -Rewrite "context()?"
#>
param(
    [Parameter(Mandatory)]
    [string]$Pattern,
    [string]$Rewrite,
    [string]$Language,
    [switch]$Help
)
if ($Help) { Get-Help $PSCommandPath; exit 0 }
$args = @("run", "--pattern", $Pattern)
if ($Rewrite) { $args += @("--rewrite", $Rewrite) }
if ($Language) { $args += @("--lang", $Language) }
ast-grep @args
````

## File: scripts/graph.ps1
````powershell
#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Build or update the code-review-graph knowledge graph.
.DESCRIPTION
    Builds the structural code graph for the current repository.
    Run this when you clone a new repo or need a full rebuild.
.EXAMPLE
    ./scripts/graph.ps1
    ./scripts/graph.ps1 --skip-flows
#>
param(
    [switch]$SkipFlows,
    [switch]$Help
)
if ($Help) {
    Get-Help $PSCommandPath
    exit 0
}
$args = @()
if ($SkipFlows) { $args += "--skip-flows" }
code-review-graph build @args
````

## File: scripts/repoctx.ps1
````powershell
#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Generate a repository summary / context.
.DESCRIPTION
    Uses repomix to produce a consolidated context of the repository.
    Pipe output to a file or clipboard for sharing with AI tools.
.EXAMPLE
    ./scripts/repoctx.ps1
    ./scripts/repoctx.ps1 | Set-Clipboard
#>
param(
    [string]$Output,
    [switch]$Help
)
if ($Help) { Get-Help $PSCommandPath; exit 0 }
if ($Output) {
    repomix --output $Output
} else {
    repomix
}
````

## File: scripts/watchgraph.ps1
````powershell
#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Watch the repository and auto-update the code graph.
.DESCRIPTION
    Starts file watcher that incrementally updates the knowledge graph
    whenever a file changes. Run in a background terminal.
.EXAMPLE
    ./scripts/watchgraph.ps1
#>
param([switch]$Help)
if ($Help) { Get-Help $PSCommandPath; exit 0 }
code-review-graph watch
````

## File: skills-lock.json
````json
{
  "version": 1,
  "skills": {
    "protobuf": {
      "source": "bufbuild/claude-plugins",
      "sourceType": "github",
      "skillPath": "plugins/protobuf/skills/protobuf/SKILL.md",
      "computedHash": "3d9770c72e7521e0c5327a4ff280ab4b23640f84e24e5010c9e520fa6fb45707"
    },
    "rust-async-patterns": {
      "source": "wshobson/agents",
      "sourceType": "github",
      "skillPath": "plugins/systems-programming/skills/rust-async-patterns/SKILL.md",
      "computedHash": "9203b9abc8238c578887994730421879778720184b027859fecda3f76d31ab05"
    },
    "rust-best-practices": {
      "source": "apollographql/skills",
      "sourceType": "github",
      "skillPath": "skills/rust-best-practices/SKILL.md",
      "computedHash": "3f89cd417842ca60bffefea01ee88fc28116b75a8e69329c4d6db446fbfcad02"
    }
  }
}
````
