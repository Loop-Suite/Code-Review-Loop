#!/usr/bin/env bash
# Wrapper invoked by the promptfoo exec provider. Reviews a single diff file (passed as an argument)
# with the actual codereview binary (--backend openrouter) and prints the entire report.md to stdout —
# assert-report.cjs then reads that text to judge the verdict/keywords.
#
# #176: also appends a MANIFEST_PROVIDER_CALLS marker line built from manifest.json's usage.calls,
# so assert-report.cjs can optionally assert a loose upper bound on total provider calls
# (metadata.expectMaxProviderCalls) — without this, a change that doubled the call count for a
# given case would still pass the suite cleanly as long as the verdict came out right.
set -euo pipefail

diff_file="$1"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bin="${CODEREVIEW_BIN:-$repo_root/target/release/codereview}"

if [[ ! -x "$bin" ]]; then
  echo "codereview binary not found: $bin — run 'cargo build --release' first, or set CODEREVIEW_BIN to its path" >&2
  exit 1
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$bin" review \
  --spec "$repo_root/specs/default.toml" \
  --diff "$diff_file" \
  --backend openrouter \
  --concurrency 1 \
  --max-rounds 1 \
  --out "$tmpdir" \
  >/dev/null

cat "$tmpdir/report.md"
echo ""
calls="$(grep -m1 '"calls"' "$tmpdir/manifest.json" 2>/dev/null | grep -o '[0-9]\+' | head -1)"
echo "<!-- MANIFEST_PROVIDER_CALLS: ${calls:-unknown} -->"
