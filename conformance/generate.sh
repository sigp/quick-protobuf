#!/usr/bin/env bash
set -eux -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROTO_DIR="$SCRIPT_DIR/proto"
OUT_DIR="$SCRIPT_DIR/src/generated"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

# Generate main conformance protos (these have unique packages, no conflicts)
cargo run -p pb-rs -- \
    "$PROTO_DIR/conformance.proto" \
    "$PROTO_DIR/test_messages_proto2.proto" \
    "$PROTO_DIR/test_messages_proto3.proto" \
    -I "$PROTO_DIR" \
    -d "$OUT_DIR"

# Well-known types all share package google.protobuf, so pb-rs overwrites
# the output file for each one. Generate each to a temp dir and concatenate.
WKT_PROTOS=(any duration field_mask struct timestamp wrappers)
WKT_TMP=$(mktemp -d)
trap "rm -rf $WKT_TMP" EXIT

mkdir -p "$OUT_DIR/google"

# Generate each WKT separately
for wkt in "${WKT_PROTOS[@]}"; do
    wkt_out="$WKT_TMP/$wkt"
    mkdir -p "$wkt_out"
    cargo run -p pb-rs -- \
        "$PROTO_DIR/google/protobuf/$wkt.proto" \
        -I "$PROTO_DIR" \
        -d "$wkt_out"
done

# Build combined google/protobuf.rs:
# 1. Collect all unique header lines (imports, type aliases) across all WKT files
# 2. Then append all type definitions from each file
{
    # Write a unified header: collect all lines before the first type definition
    # across all files, deduplicate, maintaining order
    for wkt in "${WKT_PROTOS[@]}"; do
        sed -n '1,/^#\[/{ /^#\[/!p }' "$WKT_TMP/$wkt/google/protobuf.rs"
    done | awk '!seen[$0]++'

    # Append type definitions from each file (everything from first #[... onward)
    for wkt in "${WKT_PROTOS[@]}"; do
        sed -n '/^#\[/,$p' "$WKT_TMP/$wkt/google/protobuf.rs"
        echo ""
    done
} > "$OUT_DIR/google/protobuf.rs"

# Copy the google/mod.rs from any WKT generation (they're all identical)
cp "$WKT_TMP/${WKT_PROTOS[0]}/google/mod.rs" "$OUT_DIR/google/mod.rs"

# Add google module to the top-level mod.rs and sort for cargo fmt compliance
echo 'pub mod google;' >> "$OUT_DIR/mod.rs"
sort -o "$OUT_DIR/mod.rs" "$OUT_DIR/mod.rs"
