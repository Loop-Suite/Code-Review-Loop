# Public sync mapping (private -> public)

## 기준 레포

- Private canonical: `kimdzhekhon/Code-Review-Loop`
- Public mirror (sanitized): `kimdzhekhon/full-review-benchmark-public` (workflow benchmark repo)
- Private tooling mirror (automated sync via public workflow): `kimdzhekhon/claude-config`

## 동기화 대상

- 커밋/PR 메시지 규칙
- 변경범위와 영향도 공통 원칙
- 보안/권한 변경 여부 확인 루틴
- 공개에 적합한 릴리스/메트릭 보고 원칙
- `skills/full-review` 번들(원칙상 공개 판정/실행 정책 변경 시 반영 대상)

## 동기화 제외 항목

- 내부 운영 판단 근거 및 감사지표 원자료
- 특정 사용자/계정/조직 정보
- 내부 보안·비용 정책의 세부 수치

## 제안 주기

- 규칙 변경이 있을 때: 1회 PR 내부 + 1회 공개 PR
- 분기별: 규칙 실천률 체크 10분 스크립트 리뷰(필요시 자동화 추가)
- 분기 끝: 문서-실행 불일치 점검(템플릿/규칙 항목이 실제 PR에 반영되는지)

## 동기화 방식

- `~/.github/pull_request_template.md`와 `scripts/gov-sync-check.sh` 체크리스트로
  공개 동기화 필요 여부를 판단한다.
- `full-review` 스킬 번들은 공용 레포에서 `claude-config`로의 PR 자동 동기화를
  운영한다.
  - 자동 경로:
    - `full-review-benchmark-public/.github/workflows/sync-to-claude-config.yml`
    - `full-review-benchmark-public/scripts/sync-to-claude-config-pr.sh`
  - 수동 경로(로컬 디버그):
    - `full-review-benchmark-public/scripts/sync-to-claude-config.sh --target ../claude-config`
    - `--commit`로 로컬에서 임시 커밋 생성

## 운영 자동 점검 체크리스트

- `kimdzhekhon/Code-Review-Loop`에서 가버넌스 수정 시 `scripts/gov-sync-check.sh --base origin/main --head HEAD` 실행.
- 결과가 `공개판 동기화 필요 여부: 예`면 공개판 반영 계획을 PR 본문에 첨부.
- 공개판 반영이 완료되면 같은 체크리스트 항목을 `완료`로 갱신.
