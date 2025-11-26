use ordofp::monoid::{combine_all, Monoid};
use ordofp::semigroup::Semigroup;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
struct AppConfig {
    api_endpoint: String,
    retries: u32,
    features: HashSet<String>,
}

impl Semigroup for AppConfig {
    fn combine(&self, other: &Self) -> Self {
        // Simple logic: non-empty strings overwrite, numbers add up, sets merge
        let endpoint = if !other.api_endpoint.is_empty() {
            other.api_endpoint.clone()
        } else {
            self.api_endpoint.clone()
        };

        let mut features = self.features.clone();
        features.extend(other.features.clone());

        AppConfig {
            api_endpoint: endpoint,
            retries: self.retries + other.retries, // Additive for this example
            features,
        }
    }
}

// We implement Monoid to define how to merge configs
impl Monoid for AppConfig {
    fn empty() -> Self {
        AppConfig {
            api_endpoint: String::new(),
            retries: 0,
            features: HashSet::new(),
        }
    }
}

fn main() {
    println!("--- Example 05: Monoid Configuration Merging ---");

    let base_config = AppConfig {
        api_endpoint: "http://localhost:8080".to_string(),
        retries: 1,
        features: HashSet::from(["logging".to_string()]),
    };

    let user_config = AppConfig {
        api_endpoint: "http://production.com".to_string(), // Should overwrite
        retries: 2,                                        // Should add to base (total 3)
        features: HashSet::from(["metrics".to_string()]),  // Should merge
    };

    let env_config = AppConfig {
        api_endpoint: "".to_string(), // Keep previous
        retries: 0,
        features: HashSet::from(["tracing".to_string()]),
    };

    let configs = vec![base_config, user_config, env_config];

    // Combine all configs into one
    let final_config = combine_all(&configs);

    println!("Final Config: {:?}", final_config);

    assert_eq!(final_config.api_endpoint, "http://production.com");
    assert_eq!(final_config.retries, 3);
    assert!(final_config.features.contains("logging"));
    assert!(final_config.features.contains("metrics"));
    assert!(final_config.features.contains("tracing"));
}
