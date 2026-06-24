// OwlWatch PR-precision probe — THROWAWAY, do NOT merge.
// === secrets: REAL — MUST surface ===
pub const GENERIC_KEY: &str = "b7a71655da14702784cf9afaceac1bebde2d07f2d7c98852f52650c3fd863509";
pub const AWS_ACCESS_KEY_ID: &str = "AKIA2RT4NABCD7XYZ1Q9";
pub const GITHUB_PAT: &str = "ghp_16C7e42F292c6912E7710c838347Ae178B4a";
// === secrets: FAKE / placeholder — should HIDE ===
pub const API_KEY: &str = "your-api-key-here";
pub const PASSWORD: &str = "changeme";
pub const EXAMPLE_KEY: &str = "AKIAIOSFODNN7EXAMPLE";
// === looks-secret-but-isn't — should HIDE ===
pub const HOMEPAGE: &str = "https://github.com/example/example.git";
pub const REQUEST_ID: &str = "550e8400-e29b-41d4-a716-446655440000";
pub const DB_URL: &str = "postgres://localhost:5432/dev";

pub fn parse_id(input: &str) -> i64 {
    input.parse::<i64>().unwrap() // bug bait: unwrap on untrusted input
}

pub fn score(n: i64) -> i64 {     // complexity bait
    let mut t = 0;
    for i in 0..n {
        if i % 2 == 0 {
            if i % 3 == 0 { t += if i % 5 == 0 { i } else { -i }; }
            else if i % 7 == 0 { t *= 2; }
            else { t += 1; }
        } else if i % 11 == 0 { t -= 3; }
        else { t += i; }
    }
    t
}
