use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletException {
    #[error("Failed to create account")]
    FailedToCreateAccount,
}
