// Expired seconds of cache data.
pub const DEFAULT_CACHE_EXPIRED_SECS: u64 = 15;

// Query parameter `page` starts from `1`, default `per_page` is 20, and max value is 100
// (configured in .env).
pub const DEFAULT_PER_PAGE: u64 = 10;

// Identify if batch or chunk is invalid.
pub const INVALID_INDEX: i64 = -1;
