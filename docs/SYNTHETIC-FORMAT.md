# Synthetic example

Every definition in the runnable `schemas/` starter package is invented and exists only to demonstrate package mechanics. It describes one 70-bit information word with word-format, mode, level, and conditionally applicable auxiliary fields. It is not a J-message, DFI/DUI definition, or revision difference.

The TOML package is the executable example. Validate and inspect it with:

```text
jseries schema validate schemas
jseries schema inspect schemas
```

Its vector demonstrates the supported 70-bit information-word representation. Rust integration tests load the package and independently assert a decoded enumeration.
