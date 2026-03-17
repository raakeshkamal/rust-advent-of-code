---
name: complete-problem
description: Automates moving a completed Advent of Code problem from src/active/ to src/solutions/ (nested by year/day) and updating module declarations.
---

When a user indicates they've completed an Advent of Code problem (e.g., "I'm done with [problem]", "Complete the current problem"):

1. **Verify State**: Ensure exactly one problem file exists in `src/active/` (ignoring `mod.rs`). The file should be named like `yearYYYY_dayD_partP.rs` or `YYYY_D_P.rs`.
2. **Execute Move**: Run the provided move script:
   ```bash
   bash .gemini/skills/complete-problem/scripts/move_solution.sh
   ```
3. **Verify Results**: The script handles moving the file to `src/solutions/yearYYYY/dayD/partP.rs`, creating/updating `mod.rs` files in the hierarchy (with alphabetical sorting), and updating `src/main.rs`. It also runs `cargo check`.
4. **Git Commit**: Ask if the user wants to commit:
   ```bash
   git add .
   git commit -m "feat: complete $year Day $day $part and move to solutions"
   ```

## Important Notes
- The script uses `sed -i ''` (macOS compatible) and `sort` for organization.
- Always confirm the `cargo check` output before proceeding with a commit.
- The input files are expected to be in `src/YYYY/dayD/partP/input.txt`.
