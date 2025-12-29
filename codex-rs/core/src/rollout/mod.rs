//! Rollout module: persistence and discovery of session rollout files.

use std::path::Path;

use codex_protocol::protocol::InitialHistory;
use codex_protocol::protocol::RolloutItem;
use codex_protocol::protocol::SessionSource;

pub const SESSIONS_SUBDIR: &str = "sessions";
pub const ARCHIVED_SESSIONS_SUBDIR: &str = "archived_sessions";
pub const INTERACTIVE_SESSION_SOURCES: &[SessionSource] =
    &[SessionSource::Cli, SessionSource::VSCode];

pub(crate) mod error;
pub mod list;
pub(crate) mod policy;
pub mod recorder;

pub use codex_protocol::protocol::SessionMeta;
pub(crate) use error::map_session_init_error;
pub use list::find_conversation_path_by_id_str;
pub use recorder::RolloutRecorder;
pub use recorder::RolloutRecorderParams;

pub async fn fork_history_from_rollout(path: &Path) -> std::io::Result<InitialHistory> {
    let history = RolloutRecorder::get_rollout_history(path).await?;
    let items = history
        .get_rollout_items()
        .into_iter()
        .filter(|item| {
            matches!(
                item,
                RolloutItem::ResponseItem(_)
                    | RolloutItem::Compacted(_)
                    | RolloutItem::EventMsg(_)
            )
        })
        .collect::<Vec<_>>();
    if items.is_empty() {
        Ok(InitialHistory::New)
    } else {
        Ok(InitialHistory::Forked(items))
    }
}

#[cfg(test)]
pub mod tests;
