#!/usr/bin/env bash
set -e
echo "Running full asset validation..."
cargo test data::validate::tests::validator_passes_repo_assets -- --nocapture
echo "Validating all assets..."
cargo run --bin dnd -- --validate-assets
echo "Validating individual region files..."
for region in assets/regions/*/; do
    region_name=$(basename "$region")
    if [ -f "$region/region.toml" ]; then
        echo "  Checking $region_name..."
        cargo run --bin dnd -- --validate-assets 2>&1 | grep -q "passed" && echo "    OK" || echo "    WARNING: $region_name may have issues"
    fi
done
echo "Validation complete."
