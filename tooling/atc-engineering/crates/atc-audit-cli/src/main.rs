use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use atc_atclang::{create_source, inspect, replace_source, CompilerRequest};
use atc_core::DerivedState;
use atc_gates::{EvidenceCheck, EvidenceStatus, GateReport, ReadinessInput, SeparationOfDuties};
use atc_maintenance::{
    audit_repository, engineer_until_clean, repair_until_stable, EngineeringPolicy, FindingKind,
};
use atc_requirements::{analyze, detect_signals};

fn env_value(name: &str, fallback: &str) -> String {
    env::var(name).unwrap_or_else(|_| fallback.to_string())
}

fn env_bool(name: &str) -> bool {
    matches!(
        env::var(name).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

fn option_value(options: &[String], name: &str) -> Option<String> {
    options
        .windows(2)
        .find_map(|pair| (pair[0] == name).then(|| pair[1].clone()))
}

fn main() {
    let mut args = env::args_os().skip(1);
    let root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let options: Vec<String> = args.map(|arg| arg.to_string_lossy().into_owned()).collect();
    let gate_requested = options.iter().any(|arg| arg == "--gate");
    let requirements_requested = options.iter().any(|arg| arg == "--requirements");
    let repair_requested = options.iter().any(|arg| arg == "--repair");
    let engineer_requested = options.iter().any(|arg| arg == "--engineer");
    let repair_iterations = option_value(&options, "--max-repair-iterations")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(8);
    let atclang_inspect = options.iter().any(|arg| arg == "--atclang-inspect");
    let atclang_create = options.iter().any(|arg| arg == "--atclang-create");
    let atclang_edit = options.iter().any(|arg| arg == "--atclang-edit");
    let atclang_compile = options.iter().any(|arg| arg == "--atclang-compile");
    let requested_state = env_value("ATC_REQUESTED_STATE", "TESTNET_READY");

    if atclang_inspect || atclang_create || atclang_edit || atclang_compile {
        let input = option_value(&options, "--input");
        let output = option_value(&options, "--output");

        if atclang_inspect {
            let path = input
                .as_deref()
                .map(PathBuf::from)
                .unwrap_or_else(|| root.clone());
            match inspect(&path) {
                Ok(info) => println!(
                    "ATCLANG_INSPECT kind={:?} path={} bytes={}",
                    info.kind,
                    info.path.display(),
                    info.bytes
                ),
                Err(error) => {
                    eprintln!("ATCLANG ERROR: {error}");
                    std::process::exit(2);
                }
            }
        }
        if atclang_create {
            let path = output
                .clone()
                .or(input.clone())
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("main.atc"));
            let source = env::var("ATC_SOURCE").unwrap_or_else(|_| "contract Main {}\n".into());
            if let Err(error) = create_source(&path, &source) {
                eprintln!("ATCLANG CREATE ERROR: {error}");
                std::process::exit(2);
            }
            println!("ATCLANG CREATE PASS: {}", path.display());
        }
        if atclang_edit {
            let path = input
                .as_deref()
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("main.atc"));
            let source = env::var("ATC_SOURCE").unwrap_or_else(|_| "contract Main {}\n".into());
            if let Err(error) = replace_source(&path, &source) {
                eprintln!("ATCLANG EDIT ERROR: {error}");
                std::process::exit(2);
            }
            println!("ATCLANG EDIT PASS: {}", path.display());
        }
        if atclang_compile {
            let input = input
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("main.atc"));
            let output = output
                .map(PathBuf::from)
                .unwrap_or_else(|| root.join("main.atvm"));
            let executable = env_value("ATCLANG_COMPILER", "atclang");
            let request = CompilerRequest::new(executable, input, output);
            if let Err(error) = request.run() {
                eprintln!("ATCLANG COMPILE ERROR: {error}");
                std::process::exit(2);
            }
            println!(
                "ATCLANG COMPILE PASS: {} -> {}",
                request.input.display(),
                request.output.display()
            );
        }
    }

    if requirements_requested {
        let signals = match detect_signals(&root) {
            Ok(signals) => signals,
            Err(error) => {
                eprintln!("REQUIREMENTS ERROR: {error}");
                std::process::exit(2);
            }
        };
        let profile = analyze(signals);
        println!(
            "REQUIREMENTS technologies={:?} complete={}",
            profile.technologies,
            profile.is_complete()
        );
        for requirement in &profile.requirements {
            println!(
                "REQUIREMENT {} {:?} {:?} {} — {}",
                requirement.id,
                requirement.kind,
                requirement.status,
                requirement.name,
                requirement.reason
            );
        }
        if !profile.is_complete() {
            eprintln!("REQUIREMENTS BLOCK: target software is not complete");
            std::process::exit(1);
        }
    }

    let report = if engineer_requested {
        let policy = EngineeringPolicy {
            max_iterations: repair_iterations,
            write_evidence: true,
        };
        let result = match engineer_until_clean(&root, &policy) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("ENGINEERING ERROR: {error}");
                std::process::exit(2);
            }
        };
        println!(
            "ENGINEERING phase={:?} iterations={} repairs={} ready={}",
            result.phase,
            result.iterations,
            result.repair_events.len(),
            result.ready
        );
        for event in &result.repair_events {
            println!(
                "ENGINEERING_REPAIR iteration={} action={:?} path={}",
                event.iteration,
                event.action,
                event.path.display()
            );
        }
        result.final_report
    } else if repair_requested {
        let run = match repair_until_stable(&root, repair_iterations) {
            Ok(run) => run,
            Err(error) => {
                eprintln!("REPAIR ERROR: {error}");
                std::process::exit(2);
            }
        };
        for event in &run.events {
            println!(
                "REPAIR iteration={} action={:?} path={}",
                event.iteration,
                event.action,
                event.path.display()
            );
        }
        println!(
            "REPAIR SUMMARY iterations={} applied={} blocked={}",
            run.iterations,
            run.events.len(),
            run.blocked.len()
        );
        run.final_report
    } else {
        match audit_repository(&root) {
            Ok(report) => report,
            Err(error) => {
                eprintln!("AUDIT ERROR: {error}");
                std::process::exit(2);
            }
        }
    };

    for finding in &report.findings {
        let path = finding
            .path
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<repository>".to_string());
        println!(
            "{:?} {} {}: {}",
            finding.kind, finding.code, path, finding.message
        );
    }

    if gate_requested {
        let state = match DerivedState::from_str(&requested_state) {
            Ok(state) => state,
            Err(error) => {
                eprintln!("GATE ERROR: {error}");
                std::process::exit(2);
            }
        };
        let evidence = if report.findings.is_empty() {
            vec![EvidenceCheck {
                control_id: "AUDIT-REPOSITORY".into(),
                status: EvidenceStatus::Pass,
            }]
        } else {
            report
                .findings
                .iter()
                .map(|finding| EvidenceCheck {
                    control_id: finding.code.to_string(),
                    status: match finding.kind {
                        FindingKind::Security | FindingKind::Error => EvidenceStatus::Fail,
                        FindingKind::Consistency | FindingKind::Connectivity => {
                            EvidenceStatus::Unknown
                        }
                    },
                })
                .collect()
        };
        let sod = SeparationOfDuties {
            coder: env_value("ATC_CODER", ""),
            validator: env_value("ATC_VALIDATOR", ""),
            auditor: env_value("ATC_AUDITOR", ""),
            release_authority: env_value("ATC_RELEASE_AUTHORITY", ""),
            human_approval: env_bool("ATC_HUMAN_APPROVAL"),
        };
        let gate = GateReport::from_input(&ReadinessInput {
            evidence,
            sod,
            requested_state: state,
        });
        println!("GATE_REPORT {}", gate.to_json());
        if !gate.is_allowed() {
            std::process::exit(1);
        }
    }

    if report.is_clean() {
        println!("AUDIT PASS: no findings");
        return;
    }

    let security = report.has_kind(FindingKind::Security);
    let connectivity = report.has_kind(FindingKind::Connectivity);
    let consistency = report.has_kind(FindingKind::Consistency);
    let errors = report.has_kind(FindingKind::Error);
    eprintln!("AUDIT FAIL: errors={errors} security={security} consistency={consistency} connectivity={connectivity} findings={}", report.findings.len());
    std::process::exit(1);
}
