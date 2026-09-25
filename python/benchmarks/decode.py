#!/usr/bin/env python3
"""Measure end-to-end Python-to-Rust logical70 decoding throughput."""

import argparse
from collections.abc import Callable
from pathlib import Path
import statistics
import time
from typing import TypeVar

import jseries

ROOT = Path(__file__).resolve().parents[2]
Result = TypeVar("Result")


def measure(callable_: Callable[[], Result], iterations: int) -> tuple[float, Result]:
    samples: list[float] = []
    result = callable_()
    for _ in range(iterations):
        started = time.perf_counter_ns()
        result = callable_()
        samples.append((time.perf_counter_ns() - started) / 1_000_000_000)
    return statistics.median(samples), result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rows", type=int, default=10_000)
    parser.add_argument("--iterations", type=int, default=7)
    parser.add_argument("--include-single", action="store_true")
    arguments = parser.parse_args()
    if arguments.rows < 1 or arguments.iterations < 1:
        parser.error("--rows and --iterations must be positive")

    decoder = jseries.Decoder(ROOT / "schemas")
    rows = [[0x1C94] for _ in range(arguments.rows)]

    elapsed, records = measure(
        lambda: decoder.decode_many_logical70("EXAMPLE-70", rows),
        arguments.iterations,
    )
    if len(records) != arguments.rows:
        raise RuntimeError("batch benchmark returned the wrong number of records")
    print(
        f"batch rows={arguments.rows} median={elapsed:.6f}s "
        f"messages_per_second={arguments.rows / elapsed:,.0f}"
    )

    if arguments.include_single:
        elapsed, records = measure(
            lambda: [decoder.decode_logical70("EXAMPLE-70", row) for row in rows],
            arguments.iterations,
        )
        if len(records) != arguments.rows:
            raise RuntimeError("single-call benchmark returned the wrong number of records")
        print(
            f"single-call loop rows={arguments.rows} median={elapsed:.6f}s "
            f"messages_per_second={arguments.rows / elapsed:,.0f}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
