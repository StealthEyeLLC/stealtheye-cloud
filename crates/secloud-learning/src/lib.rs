//! Self-improving Skill contract validation for StealthEye Cloud S4.

pub const LEARNING_PACKET_SCHEMAS: &[&str] = &[
    "FeedbackSignalV0",
    "PatternCandidateV0",
    "SkillCandidateV0",
    "SkillPromotionDecisionV0",
    "SkillTemplateIndexV0",
    "TemplateToSkillCompilerV0",
];

pub const SKILL_PROMOTION_RULES: &[&str] = &[
    "evidence_required",
    "human_authority_preserved",
    "no_hidden_background_claims",
    "no_private_data_in_public_skill",
    "rollback_plan_required",
];

pub fn is_learning_schema(name: &str) -> bool {
    LEARNING_PACKET_SCHEMAS.contains(&name)
}

pub fn has_promotion_rule(rule: &str) -> bool {
    SKILL_PROMOTION_RULES.contains(&rule)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_skill_candidate_schema() {
        assert!(is_learning_schema("SkillCandidateV0"));
    }

    #[test]
    fn promotion_rules_preserve_authority() {
        assert!(has_promotion_rule("human_authority_preserved"));
    }
}
