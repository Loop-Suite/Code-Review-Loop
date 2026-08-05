# 추가 리서치 보강 (2026-07-29)

## 1) 연구 근거 요약 (코드 리뷰 정확도·오탐률·비용 관점)

- **SWE-PRBench (arXiv:2603.26130)**  
  350개 PR 기반 벤치마크에서 LLM 8개가 config_A(2000 token diff+summary)에서도 사람 라벨된 이슈 검출율이 대체로 **15~31%**에 머문다는 점을 보고.  
  타입별 분석에서 **Type2(문맥 기반) 이슈가 큰 폭으로 약화**된다고 함.  
  → 리뷰는 “많이 말하기”보다 **검증 가능한 발견 후보 집중 + 문맥 과적재 완화**가 더 중요.

- **ContextCRBench / FSE 2026 (2511.07017)**  
  기존 벤치마크의 문맥 제한(이슈 설명·파일 단위의 거친 문맥)을 보완해 PR-텍스트/텍스트 기반 문맥이 성능에 유의미한 영향을 준다고 보고.  
  → 우리 파이프라인에서 `discourse` 이전/이후로 넘어가는 문맥을 “텍스트 핵심·심볼 단위”로 분해해 넘기는 근거가 됨.

- **SWR-Bench (FSE 2026 연구)**  
  1000개 검증 PR, full-project context를 쓰고 구조적으로 라벨링된 ground truth를 사용하는 설정.  
  보고된 결과는 단순 고정 프롬프트의 단점이 크고, 다중 리뷰(또는 단계별 집계) 시 오탐 억제가 중요함을 시사.  
  → 다각도 토론에서 다수결이 아닌 **증거 기반 상위-우선 축소 전략**이 더 합리적.

- **Claw-SWE-Bench (arXiv:2606.12344)**  
  동일 백본에서 minimal adapter는 Pass@1 **19.1%**, full adapter는 **73.4%**로 성능 차가 큼(동일 모델).  
  → LLM의 성능 자체보다 **어댑터/워크플로 자체를 정교하게 설계**하는 것이 더 큰 성능 차이를 만든다는 근거.

- **AACR-Bench (arXiv:2601.19494)**  
  PR 라벨 기반 노이즈/누락을 보완하기 위해 expert-verified pipeline을 사용해 잠재 결함 검출률을 **285% 증가**했다고 보고.  
  → 검증 데이터 구축 단계에서 “사람 검증 + 회수 편향 보정” 절차가 성능 체감에 직접적.

- **SWE-Cycle (arXiv:2605.13139)**  
  환경 복원 → 구현 → 테스트 생성 같은 단계별 탑재 및 end-to-end FullCycle이 단일 task보다 실질적 어려움을 더 잘 반영한다고 함.  
  → review-only 판정이 최종 품질과 동일하지 않으므로, 코멘트 후보를 **정적 점수+실행 기반 검증(선택적/격리 환경)**으로 분리.

- **Evaluating AGENTS.md (arXiv:2602.11988)**  
  저장소 컨텍스트 파일이 항상 이득이 아니며, 경우에 따라 task success를 낮추고 추론 토큰을 **20% 이상 증가**시킬 수 있다고 보고.  
  → 우리가 `reviewer memory`/`요약 규칙`을 설계할 때 **과잉 정제된 상시 규칙 대신 실행 증거 기반 정책**을 선호해야 함.

- **Survey: 99 papers (arXiv:2602.13377)**  
  코드 리뷰 평가가 99개 논문을 통합 분석했으며, 정밀 태스크 다층 분해와 동적 실행 검증, 다국어·도메인 확장 필요성을 강조.  
  → 현재 `Code-Review-Loop`의 score/effort/vet 분리 설계와 일치하며, 후속 런에서는 카테고리별 slice가 필요.

## 2) 구현 레퍼런스 (추가 검토 후보 레포)

- [OpenReview](https://github.com/vercel-labs/openreview): `@openreview` on-demand 리뷰 + 샌드박스 실행 + inline suggestion + durable workflow (GitHub 이벤트 기반).  
  현재 루틴에서 “검토 → 제안 적용 → 반응 기반 재실행” 구조를 확인할 수 있는 공개 구현.

- [Gito](https://gito.bot/): 멀티 벤더(Anthropic/OpenAI/호환 LLM) + 로컬/CI 통합 + 통계와 학습된 정책 분리 구조를 노출하는 접근.

- [Mira](https://docs.miracode.ai/): self-host 가능한 코드 리뷰 봇. 인덱싱 기반 룰, 취약점 탐지(OSV), PR/권한별 룰 학습(react/feedback) 기능이 강조됨.

- [Open Code Review (Alibaba)](https://github.com/alibaba/open-code-review): deterministic + agent hybrid 구조, rule 기반(예: NPE/SQLi/XSS)와 라인 단위 리뷰를 함께 다루는 CLI 중심 접근.

- [Code Review Bench (withmartian)](https://www.codereviewbenchmark.com/): 실서비스형 PR 추적 벤치마크를 제공하며, 실제 툴별 점수 비교를 위해 운영 메트릭을 구조화.

## 3) 정확도 개선을 위한 즉시 반영 항목(우리 저장소 맵핑)

1. **한 단계 더:** 단일 리뷰 후 끝내지 않고, P0/P1 중심 1회 이상 `generate-review-revise` 경로를 워크플로로 봉인(샘플 토글)
2. **문맥 게이트 강화:** 렌즈별 프롬프트에는 필요한 파일/심볼/테스트 정보만 전달하고, 과도한 `context token` 확장은 `severity <= P2`로만 제한.
3. **정확도/오탐 분리 메트릭:** 후보 집계(score)와 확정 집계를 분리해 UI에 항상 “확정률 vs 후보률”로 동시에 노출.
4. **적응형 비용 정책:** 긴 변경에서는 discourse/재리뷰 라운드를 감축하고, 작은 P0/P1 클러스터에서만 추가 합의 라운드.
5. **리포트 기반 규칙 감사:** `docs/organization/public-sync-mapping.md`에 맞춰 공개판 동기화 정책까지 동일하게 반영해 재현성 보장.

## 4) 다음 실험 제안(토큰·정확도 가설 검증용)

- A/B 스위치 3개로 검증:
  - `문맥 크기`: 최소/중간/상세 컨텍스트
  - `discourse`: off / limited / full
  - `재리뷰`: off / enabled(P0/P1 only)
- 각 셋에서 다음 지표를 동시에 저장:
  - Precision@P0,P1, confirmed_fdp, duplicate overlap, hallucination profile,
  - 토큰 소비(입력/출력), 실행 단계 수, 처리 시간.
- 기존 벤치/테스트에서 수치 추세만 보고 조기 결론 내리지 말고, 3회 이상 반복(run-to-run) 편차 보고.
