use crate::query::QueryType;
use subvt_types::crypto::AccountId;
use subvt_types::onekv::OneKVCandidateSummary;
use subvt_types::subvt::{NetworkStatus, ValidatorDetails};
use subvt_types::telegram::TelegramChatValidator;

pub mod content;

pub enum MessageType {
    Intro,
    Ok,
    BadRequest,
    GenericError,
    Broadcast,
    BroadcastConfirm,
    UnknownCommand(String),
    InvalidAddress(String),
    InvalidAddressTryAgain(String),
    ValidatorNotFound {
        maybe_address: Option<String>,
    },
    AddValidatorNotFound(String),
    ValidatorExistsOnChat(String),
    TooManyValidatorsOnChat,
    NoValidatorsOnChat,
    ValidatorAdded,
    AddValidator,
    ValidatorList {
        validators: Vec<TelegramChatValidator>,
        query_type: QueryType,
    },
    ValidatorInfo {
        address: String,
        maybe_validator_details: Box<Option<ValidatorDetails>>,
        maybe_onekv_candidate_summary: Box<Option<OneKVCandidateSummary>>,
    },
    NominationSummary {
        chat_validator_id: u64,
        validator_details: ValidatorDetails,
    },
    NominationDetails {
        validator_details: ValidatorDetails,
        onekv_nominator_account_ids: Vec<AccountId>,
    },
    ValidatorRemoved(TelegramChatValidator),
    Settings,
    NetworkStatus(NetworkStatus),
}