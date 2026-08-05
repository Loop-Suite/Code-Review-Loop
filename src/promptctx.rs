use crate::input::Input;
use crate::spec::Spec;

/// 모든 LLM 호출이 공유하는 컨텍스트 블록(맥락·컨벤션·요구사항·diff).
pub fn shared_context(spec: &Spec, input: &Input) -> String {
    let mut c = String::new();
    c.push_str(&format!("## 프로젝트 맥락\n{}\n\n", spec.context));
    if let Some(conv) = &input.conventions {
        c.push_str(&format!("## repo 컨벤션(원문, 명시적 요구사항 다음으로 우선)\n{}\n\n", conv));
    }
    if let Some(req) = &input.requirements {
        c.push_str(&format!("## 요구사항\n{}\n\n", req));
    }
    c.push_str(&format!(
        "## 변경 파일 ({}개, +{}/-{})\n{}\n\n",
        input.changed_files.len(),
        input.added_lines,
        input.removed_lines,
        input.changed_files.join(", ")
    ));
    c.push_str(&format!("## diff\n```diff\n{}\n```\n\n", input.diff));
    c
}
