#!/usr/bin/env python3
"""Sets the clock of new timed puzzles from their solutions.

Write a timed puzzle with `limit 99999`, write its solution, then run

    python3 crates/sim/tests/solutions/tune.py <pack>

from the repository root (pack as in packs/<pack>.gbx). It plays the pack's
solutions and gives every `limit 99999` puzzle a clock of twice the time its
solution took plus thirty seconds to read and think, rounded up to ten seconds.
"""

import math
import pathlib
import re
import subprocess
import sys

PLACEHOLDER = 99999
STEP = 600  # ten seconds of ticks

TESTS = {
    "base": "factory_puzzles", "tutorial": "tutorial_puzzles", "farm": "harvest_puzzles",
    "chips": "chip_fab_puzzles", "network": "network_puzzles", "outpost": "outpost_puzzles",
    "diner": "diner_puzzles", "toys": "toy_workshop_puzzles",
}


def main():
    pack = sys.argv[1]
    cargo = pathlib.Path.home() / ".cargo/bin/cargo"
    run = subprocess.run(
        [str(cargo), "test", "-q", "-p", "openfactor-sim", "--test", "puzzles", TESTS[pack], "--", "--exact", "--nocapture"],
        capture_output=True, text=True,
    )
    won = {m[1]: int(m[2]) for m in re.finditer(rf"^{pack} (\w+): won at (\d+) of {PLACEHOLDER}\b", run.stdout, re.M)}
    path = pathlib.Path("packs") / f"{pack}.gbx"
    text = path.read_text()
    changed = []

    def tune(block):
        name = block[1]
        if name not in won:
            return block[0]
        limit = math.ceil((won[name] * 2 + 1800) / STEP) * STEP
        changed.append(f"{name}: {won[name]} ticks -> limit {limit}")
        return block[0].replace(f"limit {PLACEHOLDER}", f"limit {limit}")

    text = re.sub(r"^puzzle (\w+)\n.*?^end$", tune, text, flags=re.M | re.S)
    path.write_text(text)
    print("\n".join(changed) or "nothing to tune")
    left = re.findall(rf"limit {PLACEHOLDER}", text)
    if left:
        print(f"{len(left)} placeholder clocks left: their solutions failed. The test says why:")
        print("\n".join(line for line in run.stdout.splitlines() + run.stderr.splitlines() if line.startswith("  ")))


main()
