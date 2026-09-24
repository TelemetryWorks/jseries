# Requirements and traceability

The JSON register is authoritative for this project's proposed requirements; these are not quotations from the military standard. Parent links form L1 -> L2 -> L3. Rust tests are supplied but unexecuted. Python evidence does not change that status.

| ID | Parent | Requirement | Implementation status |
|---|---|---|---|
| L1-001 | — | Provide explicitly scoped support for requested D-through-H baselines. | planned |
| L1-002 | — | Preserve field and interpretation fidelity without silent coercion. | planned |
| L1-003 | — | Retain evidence sufficient to identify decoding inputs and definitions. | planned |
| L1-004 | — | Support robust operation and verification in approved offline environments. | planned |
| L1-005 | — | Separate capture handling, decoding, output, and state reconstruction. | planned |
| L2-BASE | L1-001 | Select and isolate a precise effective baseline. | planned |
| L2-SOURCE | L1-001 | Qualify definitions against registered authoritative sources. | planned |
| L2-COVER | L1-001 | Report actual scope and evidence without invented denominators. | planned |
| L2-FIELD | L1-002 | Extract and interpret supported encodings with explicit status. | planned |
| L2-SCHEMA | L1-002 | Reject malformed or unresolved schema definitions. | planned |
| L2-CONTEXT | L1-002 | Resolve conditional meaning without guessing unavailable context. | planned |
| L2-PROV | L1-003 | Retain raw data references and reproducible definition identities. | planned |
| L2-SEC | L1-004 | Bound parsing and prevent unqualified use of definitions. | planned |
| L2-OPS | L1-004 | Provide offline, cross-platform build and test evidence. | planned |
| L2-INGEST | L1-005 | Maintain independently verifiable component boundaries. | planned |
| L3-REV-001 | L2-BASE | Accept only exact requested baseline identifiers; no implicit newest baseline. | rust_supplied_uncompiled |
| L3-REV-002 | L2-BASE | Treat F and F-C1 as distinct document baseline identities. | rust_supplied_uncompiled |
| L3-REV-003 | L2-BASE | Reject mismatched bundles and unsupported messages without cross-revision fallback. | rust_supplied_uncompiled |
| L3-REV-004 | L2-BASE | Exercise independently specified synthetic differences and equivalences across all six identities. | rust_supplied_uncompiled |
| L3-REV-005 | L2-BASE | Keep independent decoder instances isolated from each other's baseline selection. | rust_supplied_uncompiled |
| L3-AUTH-001 | L2-SEC | Prevent synthetic bundles from entering standards-decode mode. | rust_supplied_uncompiled |
| L3-AUTH-002 | L2-COVER | Do not claim that real J-message definitions are included. | rust_supplied_uncompiled |
| L3-PROV-001 | L2-PROV | Require bounded nonempty selection evidence and source identifiers. | rust_supplied_uncompiled |
| L3-PROV-002 | L2-PROV | Retain logical input, source offset, field ranges, raw fields, and selected schema identity. | rust_supplied_uncompiled |
| L3-PROV-003 | L2-PROV | Give bundled synthetic source documents distinct exact-byte content fingerprints. | rust_supplied_uncompiled |
| L3-BIT-001 | L2-FIELD | Reject invalid logical word lengths and high bits rather than truncate. | rust_supplied_uncompiled |
| L3-BIT-002 | L2-FIELD | Extract every supported bit position including full 128-bit boundary cases. | rust_supplied_uncompiled |
| L3-BIT-003 | L2-FIELD | Reject zero-width or out-of-bounds field ranges. | rust_supplied_uncompiled |
| L3-BIT-004 | L2-FIELD | Require input logical word length to match the selected layout. | rust_supplied_uncompiled |
| L3-SEM-001 | L2-FIELD | Recognize applicable special codes before numeric conversion. | rust_supplied_uncompiled |
| L3-SEM-002 | L2-FIELD | Return invalid status for undefined enumeration codes. | rust_supplied_uncompiled |
| L3-SEM-003 | L2-CONTEXT | Distinguish false conditions from invalid or special selector context while retaining raw bits. | rust_supplied_uncompiled |
| L3-SEM-004 | L2-CONTEXT | Evaluate fields in dependency order independently of their source listing order. | rust_supplied_uncompiled |
| L3-SEM-005 | L2-FIELD | Interpret explicit two's-complement widths including signed boundaries. | rust_supplied_uncompiled |
| L3-SEM-006 | L2-FIELD | Preserve supported scaling as exact rationals rather than forced floating-point values. | rust_supplied_uncompiled |
| L3-SCH-001 | L2-SCHEMA | Reject unsupported overlap and out-of-bounds schema fields. | rust_supplied_uncompiled |
| L3-SCH-002 | L2-SCHEMA | Reject duplicate field identifiers. | rust_supplied_uncompiled |
| L3-SCH-003 | L2-SCHEMA | Reject missing selectors and conditional dependency cycles. | rust_supplied_uncompiled |
| L3-SCH-004 | L2-SCHEMA | Reject invalid scales or widths outside the implemented arithmetic subset. | rust_supplied_uncompiled |
| L3-SCH-005 | L2-SCHEMA | Reject duplicate, out-of-width, or conflicting normal/special codes. | rust_supplied_uncompiled |
| L3-CLI-001 | L2-BASE | Require explicit baseline and evidence for the synthetic demo. | rust_supplied_uncompiled |
| L3-CLI-002 | L2-SEC | Mark all demo output synthetic and report its schema identity. | rust_supplied_uncompiled |
| L3-CLI-003 | L2-SEC | Refuse standards mode while no qualified schemas are installed. | rust_supplied_uncompiled |
| L3-CLI-004 | L2-SEC | Reject duplicate and unknown CLI options. | rust_supplied_uncompiled |
| L3-CLI-005 | L2-FIELD | Reject malformed CLI words without silently discarding high bits. | rust_supplied_uncompiled |
| L3-CLI-006 | L2-BASE | Reject unknown messages at the CLI without looking in another baseline. | rust_supplied_uncompiled |
| L3-CLI-007 | L2-COVER | Report all six baseline identities with zero verified standard messages. | rust_supplied_uncompiled |
| L3-SRC-001 | L2-SOURCE | Register all six source baselines; leave unavailable document hashes and approved local locators unset. | public_catalog_register_supplied |
| L3-COV-001 | L2-COVER | Leave inventory denominators null until a reviewed authoritative inventory exists. | coverage_register_supplied |
| L3-GEN-001 | L2-SCHEMA | Reproduce generated Rust from exact synthetic JSON sources and detect stale generated files. | python_implemented |
| L3-GEN-002 | L2-SCHEMA | Reject unknown/duplicate JSON keys, invalid numeric types/codes, and oversized schema files. | python_implemented |
| L3-OPS-001 | L2-OPS | Build and test without public package downloads on pre-provisioned offline runners. | ci_configuration_supplied_unexecuted |
| L3-OPS-002 | L2-OPS | Record actual Windows and RHEL compile/test results and exact toolchain versions. | planned |
| L3-ING-001 | L2-INGEST | Normalize actual capture packing using an identified interface specification and preserve source bytes. | planned |
| L3-ING-002 | L2-INGEST | Assemble real messages using verified rules, source keys, completeness checks, and bounded resources. | planned |
| L3-REL-001 | L2-SOURCE | Install only real schema packages qualified for declared scope by a reviewed evidence and trust chain. | planned |
| L3-PROV-004 | L2-PROV | Identify real capture, adapter, schema compiler, decoder build, and output contract for reproducible replay. | planned |
| L3-VAL-001 | L2-INGEST | Keep stateful validation and carried-forward track state separate from received decoded events. | planned |
| L3-DIFF-001 | L2-SOURCE | Review adjacent authoritative baseline changes and record verified equivalence before sharing definitions. | planned |

`tools/check_traceability.py` verifies unique IDs, valid parent levels, existence of referenced test functions, and mapping of every Rust integration test. It validates the mapping, not execution or completeness of the requirements.
