pub mod state;
pub mod animation;
pub mod app;

// 重新导出主要类型和函数
pub use state::WatchState;
pub use animation::{AnimationState, AnimationController};
pub use app::WatchApp;