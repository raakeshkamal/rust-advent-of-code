#!/bin/bash

set -e

YEAR=$1
DAY=$2
PART=$3

if [ -z "$YEAR" ] || [ -z "$DAY" ] || [ -z "$PART" ]; then
    echo "Usage: $0 <year> <day> <part>"
    exit 1
fi

# 1. Validation
rs_files=(src/active/*.rs)
active_files=()
for f in "${rs_files[@]}"; do
    [ "$(basename "$f")" != "mod.rs" ] && [ -f "$f" ] && active_files+=("$f")
done

if [ ${#active_files[@]} -gt 0 ]; then
    echo "Error: An active problem already exists: ${active_files[0]}. Complete it first."
    exit 1
fi

module_name="year${YEAR}_day${DAY}_part${PART}"
active_file="src/active/${module_name}.rs"

# 2. Input Directory
input_dir="src/${YEAR}/day${DAY}/part${PART}"
mkdir -p "$input_dir"
[ ! -f "$input_dir/input.txt" ] && touch "$input_dir/input.txt"

# 3. Create File
cat > "$active_file" <<EOF
use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> i32 {
        // TODO: Solve the problem
        0
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("${input_dir}/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}
EOF

# 4. Update active/mod.rs
echo "pub mod ${module_name};" >> src/active/mod.rs

# 5. Update main.rs
sed -i '' "s/\/\/ Import next active problem here/\/\/ Import next active problem here\nuse rust_advent_of_code::active::${module_name}::Solution;/" src/main.rs
sed -i '' "s/\/\/ Run next active problem here/\/\/ Run next active problem here\n    Solution::run();/" src/main.rs

echo "Scaffolded ${module_name} successfully!"
