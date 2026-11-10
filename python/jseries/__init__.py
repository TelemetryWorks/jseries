"""Python interface to the jseries Rust extension."""

# The native module has no Python AST; runtime tests and mypy.stubtest validate these exports.
# pylint: disable=import-error,no-name-in-module
from ._jseries import (
    DecodedField,
    DecodedRecord,
    DecodeError,
    Decoder,
    JSeriesError,
    SchemaError,
    __version__,
)

__all__ = [
    "DecodeError",
    "DecodedField",
    "DecodedRecord",
    "Decoder",
    "JSeriesError",
    "SchemaError",
    "__version__",
]
