# Requirements and traceability

The project requirements are separated by level:

- [L1.md](L1.md) contains product-level requirements.
- [L2.md](L2.md) decomposes the product requirements into system capabilities.
- [L3.md](L3.md) contains verifiable software requirements.

These requirements are project-authored and are not quotations from
MIL-STD-6016. Parent links form the hierarchy L1 -> L2 -> L3.

The machine-readable [requirements.json](requirements.json) register remains
the source used by automated traceability checks. `tools/check_traceability.py`
verifies unique IDs, valid parent levels, the existence of referenced test
functions, and the mapping of every Rust integration test. It validates the
mapping, not execution or completeness of the requirements.
