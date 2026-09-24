# Schema contract

## Current executable contract: synthetic v1

`schemas/synthetic/*.json` are the exact inputs to `tools/schema_compile.py`.
The validator is executable specification for this narrow format. It is not a
JSON Schema standard implementation and does not accept arbitrary real message
schemas.

A document has `format_version`, exact `baseline`, unique synthetic `schema_id`,
`schema_version`, `profile_id`, `kind=synthetic`, a project-authored source
locator, and message definitions. Unknown keys and duplicate object keys are
errors. The file-size limit is 1 MiB and reads are bounded. Printable ASCII is
required for toy text values to keep Rust literal generation unambiguous.

Each message declares an explicit synthetic identity, bit length, and fields.
Each field declares identifier, `SYNTH:` definition reference, internal `lsb`,
width, interpretation, special encodings, and optional condition. Bit zero is
the least-significant bit of a normalized integer. All overlaps are rejected;
conditional overlays are not supported. No actual bit-numbering convention is
inferred from a standard or capture.

Raw enumeration and special codes are canonical nonnegative decimal strings.
They are validated against the field width. Ordinary enumeration and special
sets must not overlap. `condition` is either null/absent or a raw-equality
predicate on another named field. A selector must have a valid normal value
before its condition can be evaluated. General expressions, cross-message
context, multi-branch selectors, and executable plug-ins are not implemented.

Supported rules are `unsigned`, `twos-complement`, `scaled-unsigned`, and
`enumeration`. Scaling contains an i64 numerator, nonzero u64 denominator, and
unit string. Its source field is limited to 63 bits. There is no floating-point
normalization, implicit unit conversion, additive offset, or hidden default.
These are prototype choices, not assertions about standard encodings.

The compiler embeds SHA-256 of the **exact source JSON bytes**. Whitespace
changes therefore change the source digest. `--check` reproduces generated Rust
and compares it with the checked-in file. This identifies the source and catches
stale generation; it does not verify the source's correctness or authenticate
its author. The complete archive inventory also covers compiler and Rust source
files. No runtime signature verification is implemented.

## Intended authoritative package contract — design only

The next schema design should have a manifest, source inventory, message
layouts, DFI/DUI definitions, rules, declared coverage, and verification evidence.
A conceptual unqualified manifest is:

```json
{
  "package_format": "qualified-schema-v1-PROPOSED",
  "baseline": "F-C1",
  "additional_changes": [],
  "implementation_profile": "TO_BE_ESTABLISHED",
  "schema_version": "0.1.0",
  "qualification": "UNVERIFIED",
  "source_inventory": null,
  "resolved_layout_digest": null,
  "dictionary_digest": null,
  "rule_digest": null,
  "schema_compiler_identity": null,
  "coverage_report": null,
  "evidence_bundle": null,
  "approval_record": null
}
```

This example is NOT loadable by the starter. No placeholder qualifies a schema.
Actual manifest schema and canonicalization rules require a reviewed ADR before
production implementation.

The effective snapshot must resolve every supported reference and approved
change before decoding. Precedence and conflicting modifications are explicit
errors, not last-write-wins behavior. A resolved snapshot is independently
versioned even when its authoring sources share definitions with other baselines.

Source evidence should reference exact document identity, approved local source
locator, section/table/figure or equivalent location, extracted definition
identity, reviewer, and verification artifacts. Minimize unnecessary source text
in generated files and follow the approved handling process for derived data.

Qualification is more than a mutable manifest flag. A release process must
validate a signed/approved evidence record, declared scope, source identities,
test results, and known exceptions before installing a real package. Package
integrity, publisher trust, source fidelity, and conformance coverage are
separate questions. This starter implements none of that production trust chain.

## Output contract

Current in-memory output contains a normalized word, field bit ranges and raw
values, interpretation status, baseline/schema/profile identity, and caller
provenance. The text CLI is for developer inspection and is not a stable data
exchange schema. A later serializer should use lossless integer representations
for u128/i128 values and preserve status instead of using a numeric sentinel.
A rejected capture must remain traceable even when no fields can be decoded.
