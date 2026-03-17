#!/bin/bash

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

success() { echo -e "${GREEN}✓ $1${NC}"; }
error() { echo -e "${RED}✗ $1${NC}" >&2; exit 1; }

# 1. Validation
[ -d "src/active" ] && [ -d "src/solutions" ] || error "Not in a valid rust-advent-of-code project."

# Find active files (ignoring mod.rs)
active_files=($(ls src/active/*.rs 2>/dev/null | grep -v "mod.rs" || true))

if [ ${#active_files[@]} -eq 0 ]; then
    error "No active problem found in src/active/."
elif [ ${#active_files[@]} -gt 1 ]; then
    error "Expected exactly one active problem in src/active/, found ${#active_files[@]}."
fi

active_file="${active_files[0]}"
module_name=$(basename "$active_file" .rs)
success "Found active problem: $module_name"

# 2. Extract Year, Day, Part
# Try format: year2015_day1_part1 or 2015_01_1
if [[ "$module_name" =~ year([0-9]{4})_day([0-9]+)_part([12]) ]]; then
    year="${BASH_REMATCH[1]}"
    day="${BASH_REMATCH[2]}"
    part_num="${BASH_REMATCH[3]}"
elif [[ "$module_name" =~ ([0-9]{4})_([0-9]+)_([12]) ]]; then
    year="${BASH_REMATCH[1]}"
    day="${BASH_REMATCH[2]}"
    part_num="${BASH_REMATCH[3]}"
else
    error "Could not parse year, day, and part from '$module_name'. Expected 'year2015_day1_part1' or '2015_01_1'."
fi

# Normalize day (remove leading zeros)
day=$(echo "$day" | sed 's/^0*//')
year_mod="year$year"
day_mod="day$day"
part="part$part_num"

success "Target: $year Day $day $part"

# 3. Directories
solutions_year_dir="src/solutions/$year_mod"
solutions_day_dir="$solutions_year_dir/$day_mod"
mkdir -p "$solutions_day_dir"

# 4. Move File
target_file="$solutions_day_dir/$part.rs"
mv "$active_file" "$target_file"
success "Moved file to $target_file"

# 5. Update active/mod.rs
if [ -f "src/active/mod.rs" ]; then
    sed -i '' "/pub mod $module_name;/d" src/active/mod.rs
    success "Removed module from src/active/mod.rs"
fi

# 6. Update solutions/mod.rs
sol_mod="src/solutions/mod.rs"
if ! grep -q "pub mod $year_mod;" "$sol_mod"; then
    echo "pub mod $year_mod;" >> "$sol_mod"
    sort -o "$sol_mod" "$sol_mod"
    success "Added $year_mod to $sol_mod"
fi

# 7. Update year/mod.rs
year_mod_file="$solutions_year_dir/mod.rs"
[ ! -f "$year_mod_file" ] && touch "$year_mod_file"
if ! grep -q "pub mod $day_mod;" "$year_mod_file"; then
    echo "pub mod $day_mod;" >> "$year_mod_file"
    sort -o "$year_mod_file" "$year_mod_file"
    success "Added $day_mod to $year_mod_file"
fi

# 8. Update day/mod.rs
day_mod_file="$solutions_day_dir/mod.rs"
[ ! -f "$day_mod_file" ] && touch "$day_mod_file"
if ! grep -q "pub mod $part;" "$day_mod_file"; then
    echo "pub mod $part;" >> "$day_mod_file"
    sort -o "$day_mod_file" "$day_mod_file"
    success "Added $part to $day_mod_file"
fi

# 9. Update main.rs
if [ -f "src/main.rs" ]; then
    # In main.rs, the user may have: use rust_advent_of_code::active::$module_name::Solution;
    # We replace it with a commented-out version and a placeholder
    sed -i '' "s/use rust_advent_of_code::active::$module_name::Solution;/\/\/ use rust_advent_of_code::active::$module_name::Solution;\n\/\/ Import next active problem here/" src/main.rs
    # Also handle the run line if it's there
    sed -i '' "s/Solution::run();/\/\/ Solution::run();\n    \/\/ Run next active problem here/" src/main.rs
    success "Updated src/main.rs"
fi

# 10. Verification
echo -e "\nVerifying compilation..."
cargo check || error "Compilation check failed. Please review changes."
success "Compilation check passed!"

echo -e "\n${GREEN}Successfully completed!${NC}"
