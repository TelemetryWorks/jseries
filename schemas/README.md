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

Input format is always explicit. `logical70` is the normalized information value, `word75` adds five parity bits, and `simple80` is ten capture-order bytes containing 70 information, five parity, and five zero padding bits. The decoder does not infer an adapter from input length.

There are no `examples/`, `public/`, `synthetic/`, or `authoritative/` buckets. The files at the schema root are an authoring template that users can copy and replace with their own lawful definitions. Qualification comes from evidence, provenance, review, and declared scope—not a directory name. Controlled or unauthorized source material must not be added.
