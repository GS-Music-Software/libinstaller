mod detect;
mod elevated;

pub use detect::PackageManager;
pub use detect::detect_package_manager;
pub use detect::is_installed;
pub use elevated::run_elevated;
