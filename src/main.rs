mod atomic_write;
mod backend_factory;
mod cargo_audit;
mod cli;
mod core;
mod describe;
mod deterministic;
mod discourse;
mod evidence;
mod fixcheck;
mod humanvoice;
mod improve;
mod input;
mod lens;
mod llm;
mod manifest;
mod pipeline;
mod policy;
mod procutil;
mod promptctx;
mod quantify;
mod report;
mod requirements;
mod secretscan;
mod semgrep;
mod spec;
mod state;

use anyhow::Result;
use backend_factory::build_llm;
use clap::Parser;
use cli::{Cli, Cmd};
use pipeline::describe::run_describe;
use pipeline::improve::run_improve;
use pipeline::review::{run_review, ReviewArgs};

fn main() {
    if let Err(e) = real_main() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let cli = Cli::parse();

    // #125: build_llm() (which needs OPENROUTER_API_KEY set or a working `claude` CLI on PATH,
    // depending on --backend) used to run unconditionally before this match, coupling every
    // subcommand to having a working LLM backend even though only Review/Describe/Improve
    // actually need one today. Every current command does need it, so this is a no-op change in
    // practice — but it means a future command that doesn't (e.g. a spec-validation or
    // diff-stats-only command) won't be forced to demand an API key/CLI for no reason.
    match &cli.cmd {
        Cmd::Review {
            spec,
            diff,
            requirements,
            conventions,
            deterministic_results,
            lenses,
            out,
            concurrency,
            max_rounds,
            prior,
            human_voice,
            lang,
            deadline_minutes,
            fail_on,
        } => {
            let (llm, cheap_llm) = build_llm(&cli)?;
            let outcome = run_review(
                &llm,
                &cheap_llm,
                &ReviewArgs {
                    spec_path: spec,
                    diff_path: diff,
                    requirements_path: requirements,
                    conventions_path: conventions,
                    deterministic_results_path: deterministic_results,
                    lenses_arg: lenses,
                    out,
                    concurrency: *concurrency,
                    max_rounds: *max_rounds,
                    prior,
                    human_voice: *human_voice,
                    lang,
                    deadline_minutes: *deadline_minutes,
                    allow_sensitive_input: cli.allow_sensitive_input,
                },
            )?;
            // Checked ahead of (and independently from) fail_on.triggers below: with zero
            // findings, quantify::verdict() computes plain APPROVE regardless of *why* there are
            // zero findings — every selected lens failing (no defect-finding coverage at all)
            // produces the exact same verdict string as a genuinely clean diff. Real, reproduced
            // gap: a connection-refused custom endpoint with every lens erroring out used to
            // still exit 0 under `--fail-on request-changes`. `--fail-on never` is the one
            // explicit way to say "advisory only, don't fail even on a totally broken run."
            if outcome.completeness == quantify::ReviewCompleteness::Failed
                && *fail_on != cli::FailOn::Never
            {
                eprintln!(
                    "codereview: every selected lens failed — no defect-finding coverage at all, \
                     regardless of what verdict ({}) that computed to. Treating this as a \
                     --fail-on trigger; pass --fail-on never if a completely failed run should \
                     exit 0 anyway.",
                    outcome.verdict
                );
                std::process::exit(1);
            }
            if fail_on.triggers(&outcome.verdict)? {
                eprintln!(
                    "codereview: verdict {} triggers --fail-on {fail_on:?} — exiting non-zero",
                    outcome.verdict
                );
                std::process::exit(1);
            }
            Ok(())
        }
        Cmd::Describe {
            spec,
            diff,
            requirements,
            conventions,
            out,
            lang,
        } => {
            let (llm, _cheap_llm) = build_llm(&cli)?;
            run_describe(
                &llm,
                spec,
                diff,
                requirements,
                conventions,
                out,
                lang,
                cli.allow_sensitive_input,
            )
        }
        Cmd::Improve {
            spec,
            diff,
            requirements,
            conventions,
            out,
            lang,
        } => {
            let (llm, _cheap_llm) = build_llm(&cli)?;
            run_improve(
                &llm,
                spec,
                diff,
                requirements,
                conventions,
                out,
                lang,
                cli.allow_sensitive_input,
            )
        }
    }
}
