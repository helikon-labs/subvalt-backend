use crate::app::notification::{NotificationPeriodType, NotificationTypeCode};
use lazy_static::lazy_static;

lazy_static! {
    /// Default notification rules.
    pub static ref DEFAULT_RULES: Vec<(NotificationTypeCode, NotificationPeriodType, u16)> = vec![
        (
            NotificationTypeCode::ChainValidateExtrinsic,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorActive,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorActiveNextSession,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorChilled,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorIdentityChanged,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorInactive,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorInactiveNextSession,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorOfflineOffence,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorSessionKeysChanged,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorSetController,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorUnclaimedPayout,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorStartedParaValidating,
            NotificationPeriodType::Off,
            0,
        ),
        (
            NotificationTypeCode::ChainValidatorStoppedParaValidating,
            NotificationPeriodType::Off,
            0,
        ),
        (
            NotificationTypeCode::OneKVValidatorLocationChange,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::OneKVValidatorOnlineStatusChange,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::OneKVValidatorRankChange,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::OneKVValidatorValidityChange,
            NotificationPeriodType::Immediate,
            0,
        ),
        // democracy
        (
            NotificationTypeCode::DemocracyCancelled,
            NotificationPeriodType::Off,
            0,
        ),
        (
            NotificationTypeCode::DemocracyDelegated,
            NotificationPeriodType::Off,
            0,
        ),
        (
            NotificationTypeCode::DemocracyNotPassed,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::DemocracyPassed,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::DemocracyProposed,
            NotificationPeriodType::Off,
            0,
        ),
        (
            NotificationTypeCode::DemocracySeconded,
            NotificationPeriodType::Off,
            0,
        ),
        (
            NotificationTypeCode::DemocracyStarted,
            NotificationPeriodType::Immediate,
            0,
        ),
        (
            NotificationTypeCode::DemocracyUndelegated,
            NotificationPeriodType::Off,
            0,
        ),
        (
            NotificationTypeCode::DemocracyVoted,
            NotificationPeriodType::Immediate,
            0,
        ),
    ];
}
