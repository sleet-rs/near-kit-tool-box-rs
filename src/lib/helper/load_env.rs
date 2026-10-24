// Loads the `.env` file from the current working directory into the
// process environment, so the bins can pick up credentials without the
// caller having to `source` or export anything.
//
// Existing process environment variables win: a var already set in the
// shell is not overwritten by the `.env` file.
//
// Safe to call more than once — the second call is a no-op for vars that
// are already set.
// =================================================
// =================================================
pub fn load_env() {
    // A missing `.env` is not an error: the vars may already be in the
    // environment, and the bins report a clear message when a required
    // var is missing.
    let _ = dotenvy::dotenv();
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
