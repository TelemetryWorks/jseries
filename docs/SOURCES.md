# Public references

Checked September 21, 2026. These references support the limited external facts
used in the engineering documents, not any missing message layouts.

**[S1] DLA ASSIST Quick Search — MIL-STD-6016 document details**  
https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=123964

The catalog lists D (2008-12-12), E (2012-07-20), F (2017-01-31), F Change 1
incorporated (2017-08-31), G (2020-07-08), and H (2024-04-26). Each selected
entry is marked distribution statement C. The full documents were not accessed
for this package. Metadata alone establishes neither completeness of an acquired
copy nor the semantics of a particular message or field.

**[S2] The Cargo Book — cargo test**  
https://doc.rust-lang.org/cargo/commands/cargo-test.html

Documents testing and `--locked`, `--offline`, and `--frozen`. `--frozen` combines
locked and offline behavior. Cargo's offline mode does not provide missing
compiler, standard-library, linker, or dependency installations.

**[S3] The Cargo Book — cargo vendor**  
https://doc.rust-lang.org/cargo/commands/cargo-vendor.html

Documents vendoring crates.io and Git dependencies into local sources. This
increment has no such dependencies, so vendoring is not required for its current
workspace. Reassess when dependencies are introduced.

All architectural choices, requirement identifiers, toy formats, support-state
names, and roadmap gates in this package are project proposals, not quotations
or requirements attributed to MIL-STD-6016.
