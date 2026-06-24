// Tests for the config module.
const FIXTURE_SERVICE_TOKEN: &str = "b7a71655da14702784cf9afaceac1bebde2d07f2d7c98852f52650c3fd863509";
const FIXTURE_API_KEY: &str = "your-api-key-here";
const FIXTURE_REPO: &str = "https://github.com/example/example.git";

#[test]
fn test_compute_runs() {
    assert!(super::compute(20) != i64::MIN);
}
