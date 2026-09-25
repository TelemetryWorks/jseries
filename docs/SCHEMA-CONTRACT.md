# TOML schema-package contract

A package is a directory rooted at `package.toml`. The manifest identifies package format version, identity, claimed baseline/profile, qualification, source statement, and explicit relative paths to message, catalog, and vector TOML files. Paths must remain relative, contain only normal components, and end in `.toml`.

The current loader rejects unknown keys, duplicate manifest file references, duplicate catalog/message/field identities, oversized files/packages, unsafe paths, invalid or overlapping 70-bit ranges, unresolved definition/catalog/selector references, condition cycles, invalid scale rules, and conflicting codes. One file is limited to 1 MiB and a package to 16 MiB.

Raw patterns and codes are quoted canonical strings: decimal (`"63"`) or lowercase hexadecimal (`"0x3f"`). Strings avoid TOML's signed 64-bit integer ceiling and preserve the ability to express every 70-bit value. Leading `+`, noncanonical leading zeroes, uppercase hexadecimal, and negative values are rejected.

```toml
# package.toml
format_version = 1
id = "example"
version = "0.1.0"
baseline = "SYNTHETIC"
profile = "example"
qualification = "project-authored-example"
source = "project-authored example; not standards data"
messages = ["messages/example.toml"]
catalogs = ["catalogs/mode.toml"]
vectors = ["vectors/examples.toml"]
```

Messages declare `id`, `word_count`, and `fields`. Each field declares `id`, `definition_ref`, zero-based `word`, least-significant-bit `lsb`, `width`, and an inline interpretation. Supported interpretations are `unsigned`, `twos-complement`, `scaled-unsigned`, and `catalog`. Optional `specials` precede normal interpretation; an optional equality `condition` controls applicability.

Catalog entries map a raw string to a label. Vector documents contain named cases with a message ID, explicit input format, and word strings. Vector expected-output assertions are a planned extension.

Package metadata is a claim supplied by the package author, not a cryptographic proof. A directory name such as `authoritative` cannot confer authority. Public-source packages must identify fact-level sources and uncertainty before this project can make scoped claims about them.
