---
name: start-problem
description: Scaffolds a new Advent of Code problem in src/active/ and prepares src/main.rs.
---

When a user wants to start a new Advent of Code problem (e.g., "Start 2015 day 8 part 1", "New problem 2015/8/1"):

1. **Parameters**: Extract Year, Day, and Part from the request.
2. **Scaffold**: Run the provided start script:
   ```bash
   bash .gemini/skills/start-problem/scripts/start_problem.sh $YEAR $DAY $PART
   ```
3. **Verify Results**: The script creates `src/active/year${YEAR}_day${DAY}_part${PART}.rs` with a template, adds it to `src/active/mod.rs`, and updates `src/main.rs` to run it. It also ensures the input directory exists.
4. **Instruction**: Inform the user that the problem is ready in `src/active/` and they can start coding.

## Template
The script uses a standard template implementing the `Runnable` trait and reading from `src/YYYY/dayD/partP/input.txt`.
