#!/usr/bin/env bash

set -euo pipefail

BASE_REF="origin/main"
HEAD_REF="HEAD"
OUT_FILE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base)
      BASE_REF="$2"
      shift 2
      ;;
    --head)
      HEAD_REF="$2"
      shift 2
      ;;
    --out)
      OUT_FILE="$2"
      shift 2
      ;;
    -h|--help)
      cat <<EOF
Usage: $(basename "$0") [--base <ref>] [--head <ref>] [--out <file>]

  --base  base ref for diff (default: origin/main)
  --head  head ref for diff (default: HEAD)
  --out   write checklist to file in addition to stdout
EOF
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      exit 1
      ;;
  esac
done

if ! git rev-parse --verify "$BASE_REF" >/dev/null 2>&1; then
  echo "[WARN] base ref '$BASE_REF' not found locally. Trying remote fetch..." >&2
  git fetch origin "$BASE_REF" >/dev/null 2>&1 || true
fi

if ! git rev-parse --verify "$HEAD_REF" >/dev/null 2>&1; then
  echo "[WARN] head ref '$HEAD_REF' not found locally. Trying fetch..." >&2
  git fetch origin "$HEAD_REF" >/dev/null 2>&1 || true
fi

changed_files=()
while IFS= read -r file; do
  changed_files+=( "$file" )
done < <(
  git diff --name-only "$BASE_REF...$HEAD_REF" -- docs/organization .github/pull_request_template.md | sed '/^$/d'
)

if [[ ${#changed_files[@]} -eq 0 ]]; then
  output="No governance files changed between $BASE_REF and $HEAD_REF."
  echo "$output"
  if [[ -n "$OUT_FILE" ]]; then
    printf '%s\n' "$output" > "$OUT_FILE"
  fi
  exit 0
fi

needs_public_sync="아니오"
if printf '%s\n' "${changed_files[@]}" | grep -E -q '^docs/organization/|^\.github/pull_request_template\.md$'; then
  needs_public_sync="예"
fi

timestamp=$(date -u '+%Y-%m-%d %H:%M:%SZ')

output="# Governance sync checklist

## 생성일시: $timestamp

## 비교 구간
- base: <code>$BASE_REF</code>
- head: <code>$HEAD_REF</code>

## 변경된 가버넌스 파일
"
for f in "${changed_files[@]}"; do
  output+="- $f
"
done

output+="
## 공개판 동기화 필요 여부: $needs_public_sync
"

if [[ "$needs_public_sync" == "예" ]]; then
  output+="
### 공개판 동기화 액션리스트
"
  output+="- [ ] 공개 레포 PR 템플릿/기여 규칙의 변경 여부 재확인
"
  output+="- [ ] docs/COMMIT-RULES-PUBLIC.md에서 공개 가능한 항목만 반영
"
  output+="- [ ] 공개 레포 README 링크가 최신인지 확인
"
  output+="- [ ] 변경 이유/근거(민감정보 제외)를 PR 본문에 정리
"
  output+="- [ ] 공개 PR 번호/URL을 기록해 추적
"
  output+="
권장: private PR 본문의 \"공개판 동기화 필요 여부\"에 \"필요\"를 체크하고 대상 public 파일 목록을 기재하세요.
"
else
  output+="
### 액션
- [ ] 내부 규칙 변경 범위가 공개 동기화 대상에서 제외되는지 사유 확인
"
fi

if [[ -n "$OUT_FILE" ]]; then
  printf '%s\n' "$output" > "$OUT_FILE"
  echo "Wrote checklist to: $OUT_FILE"
fi

printf '%s\n' "$output"
