#!/usr/bin/env python3
"""Read-only, fail-closed GitHub Actions triage and repair-intake planner."""
from __future__ import annotations

import hashlib
import json
import os
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

API = "https://api.github.com"
TOKEN = os.environ.get("GITHUB_TOKEN", "")
REPOSITORY = os.environ.get("GITHUB_REPOSITORY", "")
CURRENT_RUN_ID = os.environ.get("GITHUB_RUN_ID", "")
CURRENT_SHA = os.environ.get("GITHUB_SHA", "")
OUT = Path(os.environ.get("ATC_TRIAGE_OUTPUT", "artifacts/ai-agent-triage"))
MAX_REPAIR_ATTEMPTS = 3


class ApiError(RuntimeError):
    """The required GitHub evidence could not be read or validated."""


def api_get(path: str) -> dict:
    if not TOKEN or not REPOSITORY:
        raise ApiError("GITHUB_TOKEN or GITHUB_REPOSITORY is missing")
    request = urllib.request.Request(
        f"{API}{path}",
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {TOKEN}",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "ATC-AI-Agent-CI-Triage",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            data = json.loads(response.read().decode("utf-8"))
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as exc:
        raise ApiError(f"GitHub API request failed for {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise ApiError(f"GitHub API returned a non-object for {path}")
    return data


def classify_finding(workflow: str, job: str, step_names: list[str], conclusion: str | None) -> str:
    """Deterministic triage classification; never claims root cause is proven."""
    text = " ".join([workflow, job, *step_names, conclusion or ""]).lower()
    if any(term in text for term in ("merge conflict", "conflict marker", "cannot be merged")):
        return "MERGE_CONFLICT"
    if any(term in text for term in ("dependency review", "cargo audit", "audit", "vulnerability", "security scan")):
        return "DEPENDENCY_OR_SECURITY"
    if any(term in text for term in ("evidence", "provenance", "gate report", "sha mismatch", "verification")):
        return "EVIDENCE_OR_GATE"
    if any(term in text for term in ("test", "pytest", "unittest", "regression")):
        return "TEST_FAILURE"
    if any(term in text for term in ("fmt", "format", "clippy", "lint", "static analysis")):
        return "FORMAT_OR_STATIC_ANALYSIS"
    if any(term in text for term in ("checkout", "download", "timeout", "network", "rate limit", "runner", "setup", "install")):
        return "INFRASTRUCTURE_OR_SETUP"
    if conclusion in ("cancelled", "timed_out", "startup_failure", "action_required"):
        return "INFRASTRUCTURE_OR_RUN_CONTROL"
    return "UNCLASSIFIED"


def stable_finding_id(repository: str, workflow: str, head_sha: str, job_id: Any, step_names: list[str]) -> str:
    canonical = json.dumps(
        [repository, workflow, head_sha, job_id, step_names],
        ensure_ascii=True,
        separators=(",", ":"),
    )
    return "ATC-FINDING-" + hashlib.sha256(canonical.encode("utf-8")).hexdigest()[:20]


def make_blocked_report(generated: str, error: str) -> dict:
    return {
        "schema_version": "1.0.0",
        "repository": REPOSITORY or None,
        "triage_commit_sha": CURRENT_SHA or None,
        "status": "BLOCKED",
        "generated_at": generated,
        "error": error,
        "findings": [],
        "repair_policy": {
            "max_attempts_per_finding": MAX_REPAIR_ATTEMPTS,
            "attempts_used": None,
            "repair_execution_enabled": False,
            "reason": "Missing/unreadable evidence and no durable attempt ledger must fail closed.",
        },
        "note": "Missing or malformed evidence is not a clean result.",
    }


def collect_findings(runs: list[dict]) -> list[dict]:
    findings = []
    for run in runs:
        if not isinstance(run, dict):
            raise ApiError("workflow_runs contains a malformed item")
        run_id = run.get("id")
        if not isinstance(run_id, int) or str(run_id) == CURRENT_RUN_ID:
            continue
        conclusion = run.get("conclusion")
        run_status = run.get("status")
        if run_status == "completed" and conclusion in ("success", "skipped"):
            continue
        if run_status not in ("queued", "in_progress", "waiting", "requested", "completed"):
            continue
        run_sha = run.get("head_sha")
        if not isinstance(run_sha, str) or len(run_sha) != 40 or any(c not in "0123456789abcdef" for c in run_sha):
            raise ApiError(f"run {run_id} has missing or malformed head_sha")
        jobs_data = api_get(f"/repos/{REPOSITORY}/actions/runs/{run_id}/jobs?per_page=100")
        jobs = jobs_data.get("jobs")
        if not isinstance(jobs, list):
            raise ApiError(f"jobs list missing for run {run_id}")
        nonpassing_jobs = []
        for job in jobs:
            if not isinstance(job, dict):
                raise ApiError(f"run {run_id} contains a malformed job")
            job_conclusion = job.get("conclusion")
            job_status = job.get("status")
            if job_status == "completed" and job_conclusion == "success":
                continue
            raw_steps = job.get("steps") or []
            if not isinstance(raw_steps, list):
                raise ApiError(f"job {job.get('id')} contains malformed steps")
            steps = []
            for step in raw_steps:
                if not isinstance(step, dict):
                    raise ApiError(f"job {job.get('id')} contains a malformed step")
                step_conclusion = step.get("conclusion")
                step_status = step.get("status")
                if step_status == "completed" and step_conclusion == "success":
                    continue
                steps.append({
                    "name": step.get("name"),
                    "number": step.get("number"),
                    "status": step_status,
                    "conclusion": step_conclusion,
                    "exit_code": None,
                    "exit_code_note": "GitHub Actions Jobs API does not expose a reliable per-step process exit code",
                })
            step_names = [str(step.get("name") or "unnamed-step") for step in steps]
            category = classify_finding(
                str(run.get("name") or ""),
                str(job.get("name") or ""),
                step_names,
                str(job_conclusion or conclusion or ""),
            )
            finding_id = stable_finding_id(REPOSITORY, str(run.get("name") or ""), run_sha, job.get("id"), step_names)
            nonpassing_jobs.append({
                "job_id": job.get("id"),
                "name": job.get("name"),
                "status": job_status,
                "conclusion": job_conclusion,
                "html_url": job.get("html_url"),
                "classification": category,
                "finding_id": finding_id,
                "steps": steps,
                "repair_intake": {
                    "state": "NEEDS_DIAGNOSIS",
                    "max_attempts": MAX_REPAIR_ATTEMPTS,
                    "attempts_used": None,
                    "eligible_for_repair": False,
                    "reason": "A durable attempt ledger and a fresh diagnosis are required before repair execution.",
                },
            })
        findings.append({
            "workflow": run.get("name"),
            "run_id": run_id,
            "run_attempt": run.get("run_attempt"),
            "status": run_status,
            "conclusion": conclusion,
            "head_sha": run_sha,
            "sha_matches_current_triage_sha": run_sha == CURRENT_SHA,
            "head_branch": run.get("head_branch"),
            "event": run.get("event"),
            "created_at": run.get("created_at"),
            "updated_at": run.get("updated_at"),
            "html_url": run.get("html_url"),
            "jobs": nonpassing_jobs,
        })
    return findings


def write_evidence_record(triage_path: Path, report: dict, generated: str) -> None:
    """Adapt triage output to the repository's existing ATC-EVD-001 evidence schema.

    The adapter deliberately emits UNKNOWN, never PASS: run inventory alone cannot
    prove exit codes, complete pagination, or successful verification of every finding.
    """
    raw = triage_path.read_bytes()
    run_id = CURRENT_RUN_ID if CURRENT_RUN_ID.isdigit() else "0"
    sequence = str(int(run_id[-6:]) if run_id else 1)
    evidence = {
        "schema": "ATC-EVD-001",
        "schema_version": "1.0.0",
        "id": f"EVD-{datetime.now(timezone.utc).year}-{sequence}",
        "type": "AI_AGENT_CI_TRIAGE",
        "repository": REPOSITORY or "unknown",
        "commit": CURRENT_SHA if len(CURRENT_SHA) == 40 else "unknown",
        "producer": "scripts/ai_agent_ci_triage.py",
        "status": "UNKNOWN",
        "standards": ["ATC-AI-GOV-001", "ATC-STD-ENG-001"],
        "artifacts": ["triage.json", "summary.md"],
        "provenance": (
            f"generated_at={generated}; current_run_id={CURRENT_RUN_ID or 'unknown'}; "
            f"triage_status={report.get('status')}; finding_count={report.get('finding_count', 0)}; "
            "UNKNOWN is intentional: inventory is not complete runtime verification."
        ),
        "integrity": {"algorithm": "SHA-256", "digest": hashlib.sha256(raw).hexdigest()},
        "immutable": True,
    }
    (OUT / "evidence-record.json").write_text(
        json.dumps(evidence, indent=2) + "\n", encoding="utf-8"
    )


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    generated = datetime.now(timezone.utc).isoformat()
    try:
        payload = api_get(f"/repos/{REPOSITORY}/actions/runs?per_page=100")
        runs = payload.get("workflow_runs")
        if not isinstance(runs, list):
            raise ApiError("workflow_runs is missing or not an array")
        findings = collect_findings(runs)
    except (ApiError, AttributeError, TypeError, ValueError, KeyError) as exc:
        report = make_blocked_report(generated, str(exc))
        triage_path = OUT / "triage.json"
        triage_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        write_evidence_record(triage_path, report, generated)
        (OUT / "summary.md").write_text(
            "# AI agent CI triage\n\n**Status: BLOCKED**\n\n"
            f"- Repository: {REPOSITORY or 'unknown'}\n"
            f"- Triage SHA: {CURRENT_SHA or 'unknown'}\n"
            f"- Error: {exc}\n\nMissing evidence is not a clean result.\n",
            encoding="utf-8",
        )
        print(f"BLOCKED: {exc}", file=sys.stderr)
        return 2

    status = "FINDINGS" if findings else "NO_NONPASSING_RUNS_IN_WINDOW"
    report = {
        "schema_version": "1.0.0",
        "repository": REPOSITORY,
        "triage_commit_sha": CURRENT_SHA,
        "current_run_id": int(CURRENT_RUN_ID) if CURRENT_RUN_ID.isdigit() else None,
        "generated_at": generated,
        "status": status,
        "window": {"api": "GitHub Actions runs API", "limit": 100, "pagination": "first page only"},
        "finding_count": len(findings),
        "repair_policy": {
            "max_attempts_per_finding": MAX_REPAIR_ATTEMPTS,
            "attempts_used": None,
            "repair_execution_enabled": False,
            "reason": "Repair execution remains disabled until a durable attempt ledger and reviewed executor exist.",
        },
        "evidence_policy": {
            "source_mutations": False,
            "automatic_merge": False,
            "missing_api_evidence_is_pass": False,
            "step_exit_code_available_from_actions_api": False,
            "exit_code": None,
            "full_log_review_required_before_verified": True,
        },
        "findings": findings,
    }
    triage_path = OUT / "triage.json"
    triage_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    write_evidence_record(triage_path, report, generated)
    lines = [
        "# AI agent CI triage", "",
        f"- Status: **{status}**",
        f"- Repository: `{REPOSITORY}`",
        f"- Triage SHA: `{CURRENT_SHA}`",
        f"- Generated: {generated}",
        f"- Non-passing/in-flight runs: {len(findings)}",
        "- Source mutations: none",
        "- Automatic merge: disabled",
        "- Repair execution: disabled pending durable attempt ledger and reviewed executor",
        "- A run inventory is not a substitute for full log and exit-code verification.", "",
    ]
    for item in findings:
        lines.extend([
            f"## {item.get('workflow') or 'Unnamed workflow'} — run {item['run_id']}",
            f"- Status/conclusion: `{item.get('status')}` / `{item.get('conclusion')}`",
            f"- Head SHA: `{item.get('head_sha')}`",
            f"- Current triage SHA match: `{item.get('sha_matches_current_triage_sha')}`",
            f"- Run: {item.get('html_url')}",
        ])
        if not item["jobs"]:
            lines.append("- No non-passing job details were returned; inspect the run directly.")
        for job in item["jobs"]:
            lines.append(f"- Job {job.get('job_id')} — {job.get('name')}: {job.get('status')} / {job.get('conclusion')}; class={job.get('classification')}; finding={job.get('finding_id')} ({job.get('html_url')})")
            lines.append(f"  - Repair intake: {job['repair_intake']['state']}; eligible={job['repair_intake']['eligible_for_repair']}; attempts used=unknown")
            for step in job["steps"]:
                lines.append(f"  - Step {step.get('number')} — {step.get('name')}: {step.get('status')} / {step.get('conclusion')}; exit_code=null")
        lines.append("")
    (OUT / "summary.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"{status}: wrote {OUT / 'triage.json'} and {OUT / 'summary.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
