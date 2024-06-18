pub mod common {
    use dotenvy;
    use std::{env, path::PathBuf};

    pub const INTEGRATION_TESTS_ARG: &str = "integration_tests";

    fn get_dotenv() -> Option<PathBuf> {
        let args: Vec<String> = env::args().collect();
        if args.contains(&INTEGRATION_TESTS_ARG.to_string()) {
            return dotenvy::from_filename("integration_tests.env").ok();
        }
        return dotenvy::from_filename(".env").ok();
    }

    pub fn get_env_variable(variable: &str) -> String {
        get_dotenv();
        match env::var(variable) {
            Ok(var) => return var,
            Err(e) => {
                eprintln!("{} must be set in environment variables : {}", variable, e);
                return "".to_string();
            }
        }
    }
}
