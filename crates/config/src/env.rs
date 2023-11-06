use dotenvy::dotenv;
use std::env;

pub struct ThirdParties {
    pub send_grid_key: String,
    pub termii_key: String,
    pub mailchimp_key: String,
    pub mailgun_key: String,
    pub twilo_key: String,
    pub kafka_host: Vec<String>,
}


impl ThirdParties {
    pub fn get_values() -> Self {
       
        dotenv().ok();
        ThirdParties {
            send_grid_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            termii_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            mailchimp_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            mailgun_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            twilo_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            kafka_host: vec!["localhost:9092".to_string()],
        }
    }
}
