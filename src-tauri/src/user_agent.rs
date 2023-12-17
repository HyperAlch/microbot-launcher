use rand::seq::SliceRandom;
use std::vec;

pub fn generate_user_agent() -> String {
    let user_agents = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0", 
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36", 
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/105.0.0.0"
        ];

    user_agents
        .choose(&mut rand::thread_rng())
        .expect("user agent failed to generate")
        .to_string()
}
