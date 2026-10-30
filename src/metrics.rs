//! Deterministic metadata-only metrics, derived from the already-inspected
//! optional WORK index. Never reads the target project or executes commands.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};

pub const RULE_VERSION: &str = "declared-metrics-v2";
pub const SOURCE: &str = ".project/WORK.json#/contracts";
pub const MAX_DEPENDENCY_DETAILS: usize = 200;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatusCounts {
    pub verified: usize,
    pub underway: usize,
    pub blocked: usize,
    pub planned: usize,
    pub cancelled: usize,
}
impl StatusCounts {
    fn empty() -> Self {
        Self {
            verified: 0,
            underway: 0,
            blocked: 0,
            planned: 0,
            cancelled: 0,
        }
    }
    fn update(&mut self, status: &str) -> bool {
        match status {
            "verified" => self.verified += 1,
            "in_progress" | "implemented" | "verifying" => self.underway += 1,
            "blocked" => self.blocked += 1,
            "candidate" | "ready" | "deferred" => self.planned += 1,
            "cancelled" => self.cancelled += 1,
            _ => return false,
        }
        true
    }
    pub fn total(&self) -> usize {
        self.verified + self.underway + self.blocked + self.planned + self.cancelled
    }
    pub fn applicable(&self) -> usize {
        self.total().saturating_sub(self.cancelled)
    }
    pub fn percent(&self) -> Option<u64> {
        let denominator = u64::try_from(self.applicable()).ok()?;
        let done = u64::try_from(self.verified).ok()?;
        done.checked_mul(100)?
            .checked_add(denominator / 2)?
            .checked_div(denominator)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workstream {
    pub name: String,
    pub statuses: StatusCounts,
    pub applicable: usize,
    pub percent: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delivery {
    pub state: String,
    pub completed: usize,
    pub applicable: usize,
    pub cancelled_excluded: usize,
    pub percent: Option<u64>,
    pub statuses: StatusCounts,
    pub workstreams: Vec<Workstream>,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReferenceCoverage {
    pub state: String,
    pub expected: usize,
    pub inspected: usize,
    pub present: usize,
    pub missing: usize,
    pub denied_or_unsupported: usize,
    pub percent: Option<u64>,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PendingDependency {
    pub contract_id: String,
    pub depends_on: String,
    pub target_recorded_status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependencies {
    pub state: String,
    pub declared_edges: usize,
    pub pending_by_recorded_status: usize,
    pub details: Vec<PendingDependency>,
    pub omitted_details: usize,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metrics {
    pub rule_version: String,
    pub delivery: Delivery,
    pub references: ReferenceCoverage,
    pub dependencies: Dependencies,
    pub independently_verified: String,
    pub historical_trend: String,
}

pub fn analyze(work: &Value) -> Metrics {
    let mut result = Metrics {
        rule_version: RULE_VERSION.into(),
        delivery: Delivery {
            state: "UNKNOWN".into(),
            completed: 0,
            applicable: 0,
            cancelled_excluded: 0,
            percent: None,
            statuses: StatusCounts::empty(),
            workstreams: Vec::new(),
            source: SOURCE.into(),
        },
        references: ReferenceCoverage {
            state: "NOT_ASSESSED".into(),
            expected: 0,
            inspected: 0,
            present: 0,
            missing: 0,
            denied_or_unsupported: 0,
            percent: None,
            source: ".project/WORK.json#/contracts/*/contract_file".into(),
        },
        dependencies: Dependencies {
            state: "UNKNOWN".into(),
            declared_edges: 0,
            pending_by_recorded_status: 0,
            details: Vec::new(),
            omitted_details: 0,
            source: ".project/WORK.json#/contracts/*/depends_on".into(),
        },
        independently_verified: "NOT_ASSESSED".into(),
        historical_trend: "NOT_ASSESSED".into(),
    };
    let work_status = work.get("status").and_then(Value::as_str);
    // A malformed dependency graph must not hide *observed* contract states.
    // Only duplicate/missing dependency errors permit an explicitly provisional
    // distribution; all other invalid/missing/unknown ledger types fail closed.
    let recoverable_dependencies = work_status == Some("INVALID")
        && work.get("findings").and_then(Value::as_array).is_some_and(|findings| {
            let errors: Vec<_> = findings
                .iter()
                .filter(|f| f.get("severity").and_then(Value::as_str) == Some("error"))
                .collect();
            !errors.is_empty()
                && errors.iter().all(|f| {
                    matches!(
                        f.get("code").and_then(Value::as_str),
                        Some("work.duplicate_dependency" | "work.missing_dependency")
                    )
                })
        });
    if work_status != Some("STRUCTURALLY_VALID") && !recoverable_dependencies {
        return result;
    }
    let Some(contracts) = work.get("contracts").and_then(Value::as_array) else {
        return result;
    };
    if work.get("contract_count").and_then(Value::as_u64) != Some(contracts.len() as u64) {
        return result;
    }
    let mut counts = StatusCounts::empty();
    let mut streams: BTreeMap<String, StatusCounts> = BTreeMap::new();
    let mut states: HashMap<&str, &str> = HashMap::new();
    for contract in contracts {
        let Some(status) = contract.get("status").and_then(Value::as_str) else {
            return result;
        };
        if !counts.update(status) {
            return result;
        }
        let Some(id) = contract.get("id").and_then(Value::as_str) else {
            return result;
        };
        if !id.starts_with("WORK-") || id.len() <= 5
            || !id[5..].chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
            || states.insert(id, status).is_some()
        {
            return result;
        }
        let stream = contract
            .get("workstream")
            .and_then(Value::as_str)
            .filter(|name| !name.trim().is_empty())
            .unwrap_or("Unassigned");
        streams
            .entry(stream.to_owned())
            .or_insert_with(StatusCounts::empty)
            .update(status);
    }
    if counts.total() != contracts.len() {
        return result;
    }
    result.delivery.state = if recoverable_dependencies {
        "PARTIAL"
    } else if counts.applicable() == 0 {
        "NOT_APPLICABLE"
    } else {
        "KNOWN"
    }
    .into();
    result.delivery.completed = counts.verified;
    result.delivery.applicable = counts.applicable();
    result.delivery.cancelled_excluded = counts.cancelled;
    result.delivery.percent = counts.percent();
    result.delivery.workstreams = streams
        .into_iter()
        .map(|(name, statuses)| Workstream {
            applicable: statuses.applicable(),
            percent: statuses.percent(),
            name,
            statuses,
        })
        .collect();
    result.delivery.statuses = counts;

    let expected = contracts
        .iter()
        .filter(|c| c.get("contract_file").and_then(Value::as_str).is_some())
        .count();
    result.references.expected = expected;
    let references = work.get("contract_files").and_then(Value::as_array);
    if expected == 0 {
        result.references.state = "NOT_APPLICABLE".into();
    } else if let Some(items) = references {
        result.references.inspected = items.len();
        for item in items {
            match item.get("status").and_then(Value::as_str) {
                Some("PRESENT") => result.references.present += 1,
                Some("MISSING") => result.references.missing += 1,
                _ => result.references.denied_or_unsupported += 1,
            }
        }
        if work
            .get("reference_inspection_limited")
            .and_then(Value::as_bool)
            != Some(true)
            && !recoverable_dependencies
            && items.len() == expected
        {
            result.references.state = "KNOWN".into();
            result.references.percent = u64::try_from(result.references.present)
                .ok()
                .and_then(|v| v.checked_mul(100))
                .and_then(|v| v.checked_add((expected / 2) as u64))
                .and_then(|v| v.checked_div(expected as u64));
        } else {
            result.references.state = "PARTIAL".into();
        }
    }

    result.dependencies.state = if recoverable_dependencies { "PARTIAL" } else { "KNOWN" }.into();
    let mut distinct_links = HashSet::new();
    for contract in contracts {
        let id = contract.get("id").and_then(Value::as_str).unwrap_or("");
        if let Some(edges) = contract.get("depends_on").and_then(Value::as_array) {
            for dep in edges {
                let Some(target) = dep.as_str() else {
                    result.dependencies.state = "PARTIAL".into();
                    continue;
                };
                // Invalid duplicate edges are reported by the parser; they
                // must not inflate the dependency graph or blocker counts.
                if !distinct_links.insert((id, target)) {
                    continue;
                }
                result.dependencies.declared_edges += 1;
                if let Some(target_state) = states.get(target) {
                    if *target_state != "verified" {
                        result.dependencies.pending_by_recorded_status += 1;
                        if result.dependencies.details.len() < MAX_DEPENDENCY_DETAILS {
                            result.dependencies.details.push(PendingDependency {
                                contract_id: id.into(),
                                depends_on: target.into(),
                                target_recorded_status: (*target_state).into(),
                            });
                        } else {
                            result.dependencies.omitted_details += 1;
                        }
                    }
                } else {
                    // A valid work ledger should already reject unresolved IDs,
                    // but keep fail-closed semantics if the upstream parser changes.
                    result.dependencies.state = "PARTIAL".into();
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn derives_recorded_status_and_sources_without_verifying() {
        let index = json!({"status":"STRUCTURALLY_VALID","contract_count":4,
            "contracts":[
                {"id":"WORK-A","status":"verified","workstream":"core","contract_file":"a.md","depends_on":[]},
                {"id":"WORK-B","status":"in_progress","workstream":"core","contract_file":"b.md","depends_on":["WORK-A"]},
                {"id":"WORK-C","status":"blocked","workstream":"api","depends_on":["WORK-B"]},
                {"id":"WORK-D","status":"cancelled","workstream":"api","depends_on":[]}
            ],
            "contract_files":[{"contract_id":"WORK-A","status":"PRESENT"},{"contract_id":"WORK-B","status":"MISSING"}],
            "reference_inspection_limited":false
        });
        let m = analyze(&index);
        assert_eq!(m.delivery.percent, Some(33));
        assert_eq!(m.delivery.applicable, 3);
        assert_eq!(m.delivery.workstreams.len(), 2);
        assert_eq!(m.references.percent, Some(50));
        assert_eq!(m.references.missing, 1);
        assert_eq!(m.dependencies.pending_by_recorded_status, 1);
        assert_eq!(m.dependencies.details[0].contract_id, "WORK-C");
        assert_eq!(m.independently_verified, "NOT_ASSESSED");
    }
    #[test]
    fn absent_or_invalid_inputs_do_not_fabricate_metrics() {
        for index in [
            json!({}),
            json!({"status":"INVALID","contract_count":1,
            "contracts":[{"id":"WORK-A","status":"verified"}]}),
        ] {
            let m = analyze(&index);
            assert_eq!(m.delivery.state, "UNKNOWN");
            assert_eq!(m.delivery.percent, None);
            assert_eq!(m.references.percent, None);
            assert_eq!(m.dependencies.state, "UNKNOWN");
        }
    }
    #[test]
    fn no_applicable_work_and_missing_refs_have_explicit_states() {
        let index = json!({"status":"STRUCTURALLY_VALID","contract_count":1,
            "contracts":[{"id":"WORK-X","status":"cancelled","depends_on":[]}],
            "contract_files":[],"reference_inspection_limited":false});
        let m = analyze(&index);
        assert_eq!(m.delivery.state, "NOT_APPLICABLE");
        assert_eq!(m.delivery.percent, None);
        assert_eq!(m.references.state, "NOT_APPLICABLE");
    }
    #[test]
    fn dependency_only_invalid_ledger_yields_explicit_provisional_statuses() {
        let index = json!({"status":"INVALID","contract_count":3,
            "contracts":[
                {"id":"WORK-A","status":"verified","workstream":"core","contract_file":"a.md","depends_on":[]},
                {"id":"WORK-B","status":"in_progress","workstream":"core","contract_file":"b.md",
                 "depends_on":["WORK-A","WORK-A","WORK-MISSING"]},
                {"id":"WORK-C","status":"candidate","workstream":"ux","depends_on":["WORK-B"]}
            ],
            "findings":[
                {"code":"work.duplicate_dependency","severity":"error","message":"duplicate"},
                {"code":"work.missing_dependency","severity":"error","message":"missing"}
            ],
            "contract_files":[{"status":"PRESENT"},{"status":"MISSING"}],
            "reference_inspection_limited":false
        });
        let metrics = analyze(&index);
        assert_eq!(metrics.delivery.state, "PARTIAL");
        assert_eq!(metrics.delivery.percent, Some(33));
        assert_eq!(metrics.delivery.statuses.total(), 3);
        assert_eq!(metrics.references.state, "PARTIAL");
        assert_eq!(metrics.references.percent, None);
        assert_eq!(metrics.dependencies.state, "PARTIAL");
        assert_eq!(metrics.dependencies.declared_edges, 3);
        assert_eq!(metrics.dependencies.pending_by_recorded_status, 1);
        assert_eq!(metrics.independently_verified, "NOT_ASSESSED");
    }

    #[test]
    fn unrelated_invalid_work_is_never_provisionally_scored() {
        for code in ["work.duplicate_id", "work.contract_status", "work.execution_field"] {
            let index = json!({"status":"INVALID","contract_count":1,
                "contracts":[{"id":"WORK-A","status":"verified","depends_on":[]}],
                "findings":[{"code":code,"severity":"error","message":"invalid"}]
            });
            let metrics = analyze(&index);
            assert_eq!(metrics.delivery.state, "UNKNOWN");
            assert_eq!(metrics.delivery.percent, None);
        }
    }

    #[test]
    fn reference_budget_cannot_be_reported_as_full_coverage() {
        let index = json!({"status":"STRUCTURALLY_VALID","contract_count":2,
            "contracts":[{"id":"WORK-A","status":"verified","contract_file":"a.md"},
                {"id":"WORK-B","status":"verified","contract_file":"b.md"}],
            "contract_files":[{"status":"PRESENT"}],"reference_inspection_limited":true});
        let m = analyze(&index);
        assert_eq!(m.references.state, "PARTIAL");
        assert_eq!(m.references.percent, None);
    }
}
