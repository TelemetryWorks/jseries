# Schema packages

This directory contains examples and public source/coverage metadata. It does not contain MIL-STD-6016 layouts or DFI/DUI catalogs. Users supply definitions they may lawfully use and identify each package's baseline, profile, source, and qualification.

The public DLA catalog entry is retained as source [S1]: https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=123964. It identifies revisions and distribution statements, not the semantic data needed to decode messages.

The `schemas/` directory itself is a complete, runnable, invented TOML starter package:

```text
schemas/
  package.toml
  messages/example.toml
  catalogs/*.toml
  vectors/examples.toml
```

Users add layouts under `messages/`, DFI/DUI or other value tables under `catalogs/`, and independently derived cases under `vectors/`. The manifest explicitly lists every file. `jseries schema validate <directory>` validates and resolves the package; `jseries decode --schema <directory> ...` loads it before decoding.

Each input word is an integer containing exactly the 70 information bits. Values with higher bits set are rejected. Transport framing, parity, padding, and packed byte representations must be handled outside this project before decoding.

There are no `examples/`, `public/`, `synthetic/`, or `authoritative/` buckets. The files at the schema root are an authoring template that users can copy and replace with their own lawful definitions. Qualification comes from evidence, provenance, review, and declared scope—not a directory name. Controlled or unauthorized source material must not be added.
