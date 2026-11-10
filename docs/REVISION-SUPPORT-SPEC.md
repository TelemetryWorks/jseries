# Revision-support specification

Status: public-source, user-supplied schema foundation

The product accepts explicitly selected, versioned schema packages and never silently substitutes another baseline or profile. Package baseline and qualification fields are author claims carried into output; they become trustworthy only through an external evidence and review process.

TelemetryWorks cannot obtain controlled MIL-STD-6016 revisions through an authorized channel. Consequently, the repository contains zero authoritative standard message definitions. D, E, F, F Change 1, G, and H may be cataloged as document identities, but public catalog metadata cannot substantiate a bit position, enumeration, processing rule, or revision delta.

Support must be reported independently for structure, semantics, transport representation, and stateful validation. Partial public-evidence support is acceptable when its exact scope, sources, conflicts, unsupported values, and confidence are visible. Coverage uses a declared public-evidence denominator, never the inaccessible complete standard.

A qualified package requires source identity, fact-level locators, lawful reuse review, independent expected results, exceptional and conditional cases, malformed/truncated behavior, source-to-output traceability, and known limitations. Stateful inference remains downstream and cannot be reported as data received in a message.

The current implementation supports exact 70-bit information-word validation; bounded initial/continuation/extension assembly; strict TOML package loading; immutable decode plans; raw extraction; specials; conditions; enumerations; two's complement; and exact rational scaling. It does not implement transport framing, parity handling, packed transport representations, full Link 16 assembly rules, actual J-message semantics, schema signatures, or conformance.
