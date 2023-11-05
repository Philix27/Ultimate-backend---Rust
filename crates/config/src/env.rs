use dotenvy::dotenv;
use std::env;

pub struct ThirdParties {
    pub send_grid_key: String,
    pub termii_key: String,
    pub mailchimp_key: String,
    pub mailgun_key: String,
    pub twilo_key: String,
}

impl ThirdParties {
    pub fn getValues() -> Self {
        dotenv().ok();

        // Access environment variables

        ThirdParties {
            send_grid_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            termii_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            mailchimp_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            mailgun_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
            twilo_key: env::var("DATABASE_URL").expect("DATABASE_URL not set in .env"),
        }
    }
}
