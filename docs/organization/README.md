# Commit governance and release flow (private base)

이 문서는 `Code-Review-Loop` 저장소를 기준으로 커밋/PR 운영 규칙을 정의합니다.
공개 저장소 동기화가 필요한 항목은 아래 `sync-policy`에 따라 정제본을 `public`
레포에 반영합니다.

## 1) 목적

- 코드 리뷰 품질 유지(명확한 변경 이력, 재현 가능한 병합 기록)
- 공개 저장소에서는 민감 정보를 노출하지 않되, 운영 기준은 일관되게 유지
- 토큰/시간 비용을 줄이기 위해 PR 단위 리뷰를 작고 확정 가능한 단위로 쪼개기

## 2) 커밋 규칙 (Conventional Commit)

### Format

`<type>(<scope>): <summary>`

예시:
- `feat(review): isolate deterministic manifest assembly`
- `fix(policy): reduce false negative in test-surface guard`
- `docs(governance): add commit rules and PR validation checklist`
- `chore(ci): pin benchmark runner dependencies`

### 권장 타입

- `feat`: 기능 추가
- `fix`: 버그/동작 수정
- `docs`: 문서
- `refactor`: 동작 불변 코드 구조 개선
- `perf`: 성능/효율 개선
- `test`: 테스트 보강
- `chore`: 빌드/도구/메타 변경
- `ci`: CI/CD 파이프라인 변경
- `revert`: 롤백

### Summary 규칙

- `summary`는 72자 내외, 동사 원형/명령형으로 작성
- `AND`/`또는` 연달아 나열하지 말고 핵심 효과 1줄
- 불필요한 접두어(`WIP`, `temp`, `fix maybe`) 금지

### Body(권장)

요약 1줄만 있는 커밋도 허용되지만, 영향도가 있는 작업은 아래 구조를 권장합니다.

```text
Motivation: 왜 필요한지 (현재 문제/요구)
Changes: 무엇을 바꿨는지 (핵심 파일/이유)
Validation: 어떻게 검증했는지 (테스트·벤치마크 명령)
Risk & rollback: 리스크와 되돌리기 방법
```

## 3) PR 체크리스트 (필수)

`[ ]` PR 제목에 커밋 타입/범위를 반영

`[ ]` 변경 근거가 README/설계 문서 또는 issue와 연결되는지

`[ ]` 영향 파일 수치와 테스트 범위를 간단히 기재

`[ ]` 성능/정밀도/토큰 비용 변경 시 측정 산출물 링크 첨부

`[ ]` 보안·권한·민감경로 관련 변경 시 점검 항목 명시

`[ ]` 공개판 동기화가 필요한 내용인지 확인

## 4) Branch & PR 정책

- 기본 브랜치: `main`
- 기능/문서 브랜치: `chore/*`, `feat/*`, `fix/*`
- 원칙적으로 한 PR당 변경범위를 제한:
  - `docs`만 바꿔야 할 때는 `docs-only` 라벨 또는 제목으로 표시
  - 동시 영향이 큰 변경은 1 PR 1 목적
- 리뷰 완료 전 머지는 지양, 자동 머지가 가능한 항목만 `merge` 라벨 사용

## 5) 공개판 동기화 (public sync)

비공개 규칙의 공개 가능 버전은 아래 정책으로 관리합니다.

- `Code-Review-Loop/docs/organization/` = **single source of truth**
- 공개판(`full-review-benchmark-public`)에는 아래 항목만 반영:
  1. 커밋 메시지 포맷
  2. PR 체크리스트
  3. 공통 분기/릴리스 원칙
- `claude-config` 반영 대상:
  1. `skills/full-review/SKILL.md`
  2. `skills/full-review/workflow.js`
  3. `skills/full-review/scripts/*.mjs`
  4. `skills/full-review/references/*.md`
- 공개판에는 제외:
  - 내부 토론 로그
  - 비공개 이슈/계정 정책
  - 내부 비용 정책(예: 특정 계정/라이선스 제한)

스킬 번들 동기화는 `full-review-benchmark-public`의 자동 워크플로를 통해 `claude-config`
로 PR 생성/업데이트 됩니다.

원하는 경우 로컬에서 수동 동기화를 검증할 수 있습니다.

- `full-review-benchmark-public/.github/workflows/sync-to-claude-config.yml`: GitHub Actions 자동 동기화
- `full-review-benchmark-public/scripts/sync-to-claude-config-pr.sh`: 수동 실행/검증용
- `full-review-benchmark-public/scripts/sync-to-claude-config.sh --target <path> --commit`: 임시 로컬 동기화

동기화가 필요한 규칙/문서 변경은 공개 PR, `claude-config` PR, 그리고 공개 레포
PR/문서 동기화 PR을 한 번에 묶어 운영하는 것을 권장합니다.

## 7) 거버넌스 변경 동기화 자동 점검

가버넌스 파일( `docs/organization/`, PR 템플릿 )이 바뀌면 공개판 반영 유무를 빠르게 판단하기 위해 아래 스크립트를 사용합니다.

```bash
./scripts/gov-sync-check.sh --base origin/main --head HEAD
```

선택 옵션:

- `--base <ref>`: 비교 기준 리비전(기본: `origin/main`)
- `--head <ref>`: 비교 대상 리비전(기본: `HEAD`)
- `--out <file>`: 체크리스트를 파일로 저장

권장 운영:

- `docs/organization/README.md` 또는 `public-sync-mapping.md` 수정 시
  체크리스트를 생성해서 PR 본문 `Governance check` 항목에 붙입니다.
- 공개판 동기화가 필요한 변경이면 같은 규칙 문서에서 공개판 버전을 갱신하고
  공개 PR을 즉시 생성합니다.

## 8) 분쟁/예외 처리

- 규칙 충돌 시, PR 템플릿의 `Decision` 섹션에 사유 기록
- 긴급 수정은 `chore/security` 또는 `chore/emergency`로 라벨링하고, 사후 회고에서
  규칙 위반 여부를 정리

## 9) 추가 근거/리서치 근거 문서

- [research-and-evidence-survey-2026-07-29.md](research-and-evidence-survey-2026-07-29.md): 코드 리뷰 정확도·오탐률·토큰 비용 관점에서 최신 벤치마크/논문, 구현 레퍼런스, 조정 우선순위를 정리한 내부 근거 문서입니다.

운영/문서 변경 시, 해당 근거 문서를 참고해 최신 근거가 반영되었는지 PR 본문에 1줄 추가하는 것을 권장합니다.
