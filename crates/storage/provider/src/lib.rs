#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxzy/reth/issues/"
)]
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! This crate contains a collection of traits and trait implementations for common database
//! operations.
//!
//! ## Feature Flags
//!
//! - `test-utils`: Export utilities for testing

/// Various provider traits.
mod traits;
pub use traits::{
    AccountExtReader, AccountReader, BlockExecutionWriter, BlockExecutor, BlockExecutorStats,
    BlockHashReader, BlockIdReader, BlockNumReader, BlockReader, BlockReaderIdExt, BlockSource,
    BlockWriter, BlockchainTreePendingStateProvider, BundleStateDataProvider, CanonChainTracker,
    CanonStateNotification, CanonStateNotificationSender, CanonStateNotifications,
    CanonStateSubscriptions, ChainSpecProvider, ChangeSetReader, EvmEnvProvider, ExecutorFactory,
    HashingWriter, HeaderProvider, HistoryWriter, PrunableBlockExecutor, PruneCheckpointReader,
    PruneCheckpointWriter, ReceiptProvider, ReceiptProviderIdExt, StageCheckpointReader,
    StageCheckpointWriter, StateProvider, StateProviderBox, StateProviderFactory,
    StateRootProvider, StorageReader, TransactionsProvider, WithdrawalsProvider,
};

/// Provider trait implementations.
pub mod providers;
pub use providers::{
    DatabaseProvider, DatabaseProviderRO, DatabaseProviderRW, HistoricalStateProvider,
    HistoricalStateProviderRef, LatestStateProvider, LatestStateProviderRef, ProviderFactory,
};

#[cfg(any(test, feature = "test-utils"))]
/// Common test helpers for mocking the Provider.
pub mod test_utils;

/// Re-export provider error.
pub use reth_interfaces::provider::ProviderError;

pub mod chain;
pub use chain::{Chain, DisplayBlocksChain};

pub mod bundle_state;
pub use bundle_state::{BundleStateWithReceipts, OriginalValuesKnown, StateChanges, StateReverts};
