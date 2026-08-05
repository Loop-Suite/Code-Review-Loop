## Summary

### What changed

- [ ] 코드
- [ ] 문서
- [ ] 설정/도구

### Why

- 목적:
- 해결하는 문제:

### Validation

- 실행한 검증:
  - [ ] 테스트
  - [ ] 빌드
  - [ ] 수동 점검
  - 명령:

### Impact

- 영향을 받는 파일/영역:
- 예상 리스크:
- 롤백 방법:

### Governance check

- 커밋 메시지 형식 준수:
- PR 체크리스트 항목 완료:
- 공개판 동기화 필요 여부: [필요 / 불필요]
  - 필요 시 동기화 항목:
- 공개 미러(`full-review-benchmark-public`) 동기화 필요:
- 설정/도구 미러(`claude-config`) 동기화 필요:
  - 동기화 실행:
    - 자동 경로: `full-review-benchmark-public/.github/workflows/sync-to-claude-config.yml`
      (main push / workflow_dispatch)
    - 수동 실행: `full-review-benchmark-public/scripts/sync-to-claude-config-pr.sh`
      (PR 생성/업데이트)
- 가버넌스 스크립트 결과:
  - `./scripts/gov-sync-check.sh --base origin/main --head HEAD` 실행 결과:
  - `공개판 동기화 필요 여부`:
  - 확인 필요/완료 항목:

### Notes

- 관련 링크(이슈/토론/기존 PR):
- 추가 코멘트:
