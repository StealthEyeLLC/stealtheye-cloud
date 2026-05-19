//! Hypothesis racing contract validation for StealthEye Cloud S4.

pub const HYPOTHESIS_PACKET_SCHEMAS: &[&str] = &[
    "HypothesisRaceV0",
    "CandidateBranchV0",
    "CandidateReducerV0",
    "RaceDecisionV0",
];

pub const RACE_STOP_CONDITIONS: &[&str] = &[
    "proof_passed",
    "all_candidates_failed",
    "budget_boundary",
    "authority_boundary",
    "private_data_risk",
];

pub fn is_hypothesis_schema(name: &str) -> bool {
    HYPOTHESIS_PACKET_SCHEMAS.contains(&name)
}

pub fn is_stop_condition(name: &str) -> bool {
    RACE_STOP_CONDITIONS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_hypothesis_race_schema() {
        assert!(is_hypothesis_schema("HypothesisRaceV0"));
    }

    #[test]
    fn includes_authority_boundary_stop_condition() {
        assert!(is_stop_condition("authority_boundary"));
    }
}
