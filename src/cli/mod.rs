pub mod bulk;
pub mod check;
pub mod output;
pub mod pattern;
pub mod tld;
pub mod utils;

pub use bulk::bulk_check;
pub use check::check_domains;
pub use pattern::check_pattern;
pub use tld::check_tlds;
