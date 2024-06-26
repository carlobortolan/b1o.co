use serde::Deserialize;

pub const DEFAULT_FRONTEND_URL: &str = "http://localhost:8000";
pub const FRONTEND_HOST_KEY: &str = "FRONTEND_HOST";
pub const FRONTEND_PORT_KEY: &str = "FRONTEND_PORT";
pub const FRONTEND_PROTOCOL_KEY: &str = "FRONTEND_PROTOCOL";

pub const DEFAULT_DB_URL: &str = "postgres://render_user:ZH7NNXVKhkloz2OiGhRkw3tNHjjw16kc@dpg-como66kf7o1s73f7uo3g-a.frankfurt-postgres.render.com/temp";
pub const DATABASE_URL_KEY: &str = "DATABASE_URL";
pub const DATABASE_PROTOCOL_KEY: &str = "DATABASE_PROTOCOL";
pub const DATABASE_PORT_KEY: &str = "DATABASE_PORT";
pub const DATABASE_USER_KEY: &str = "POSTGRES_USER";
pub const DATABASE_PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
pub const DATABASE_NAME_KEY: &str = "POSTGRES_DB";
pub const DATABASE_HOST_KEY: &str = "POSTGRES_HOST";

pub const DEFAULT_BACKEND_URL: &str = "http://localhost:8080";
pub const BACKEND_HOST_KEY: &str = "BACKEND_HOST";
pub const BACKEND_INTERNAL_HOST_KEY: &str = "BACKEND_HOST_INTERNAL";
pub const BACKEND_PORT_KEY: &str = "BACKEND_PORT";
pub const BACKEND_PROTOCOL_KEY: &str = "BACKEND_PROTOCOL";

pub const WATCHES_ROUTE: &str = "/watches";
pub const SINGLE_WATCH_ROUTE: &str = "/watches/{watch_id}";
pub const REVIEWS_BY_WATCH_ROUTE: &str = "/watches/{watch_id}/reviews";

pub const PLAYERS_ROUTE: &str = "/player";
pub const SINGLE_PLAYER_ROUTE: &str = "/players/{player_id}";
pub const NEXT_PLAYER_ROUTE: &str = "/next";
pub const START_ROUTE: &str = "/start";
pub const SCOREBOARD_ROUTE: &str = "/scoreboard";

pub enum AppComponent {
    Frontend,
    Backend,
}

#[derive(Deserialize)]
pub struct ApiQueryParams {
    pub expand: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct NextPlayerParams {
    pub winner: u32,
    pub loser: u32,
}

pub fn init_database_url() -> Url {
    let mut url = Url::parse(DEFAULT_DB_URL).expect("the default db url to be parseable");

    if let Ok(protocol) = std::env::var(DATABASE_PROTOCOL_KEY) {
        url.set_scheme(&protocol).unwrap();
    }

    if let Ok(protocol) = std::env::var(DATABASE_HOST_KEY) {
        url.set_host(Some(&protocol)).unwrap();
    }

    if let Ok(user) = std::env::var(DATABASE_USER_KEY) {
        url.set_username(&user).unwrap();
    }

    if let Ok(password) = std::env::var(DATABASE_PASSWORD_KEY) {
        url.set_password(Some(&password)).unwrap();
    }

    if let Ok(port) = std::env::var(DATABASE_PORT_KEY) {
        let port = port
            .parse::<u16>()
            .expect("the database port to parseable to u16");
        url.set_port(Some(port)).unwrap();
    };
    println!("{}", url);
    if let Ok(name) = std::env::var(DATABASE_NAME_KEY) {
        url.set_path(&name);
    }

    url
}

pub fn init_backend_url(component: AppComponent) -> Url {
    let mut url = Url::parse(DEFAULT_BACKEND_URL).expect("the default backend url to be parseable");

    if let Ok(protocol) = std::env::var(BACKEND_PROTOCOL_KEY) {
        url.set_scheme(&protocol).unwrap();
    }

    match component {
        AppComponent::Frontend => {
            if let Ok(host) = std::env::var(BACKEND_HOST_KEY) {
                url.set_host(Some(&host)).unwrap();
            }
        }
        AppComponent::Backend => {
            if let Ok(host) = std::env::var(BACKEND_INTERNAL_HOST_KEY) {
                url.set_host(Some(&host)).unwrap();
            } else if let Ok(host) = std::env::var(BACKEND_HOST_KEY) {
                url.set_host(Some(&host)).unwrap();
            }
        }
    }

    if let Ok(port) = std::env::var(BACKEND_PORT_KEY) {
        let port = port
            .parse::<u16>()
            .expect("the frontend port to parseable to u16");
        url.set_port(Some(port)).unwrap();
    };

    url
}

pub fn init_frontend_url() -> Url {
    let mut url =
        Url::parse(DEFAULT_FRONTEND_URL).expect("the default frontend url to be parseable");

    if let Ok(protocol) = std::env::var(FRONTEND_PROTOCOL_KEY) {
        url.set_scheme(&protocol).unwrap();
    }

    if let Ok(host) = std::env::var(FRONTEND_HOST_KEY) {
        url.set_host(Some(&host)).unwrap();
    }

    if let Ok(port) = std::env::var(FRONTEND_PORT_KEY) {
        let port = port
            .parse::<u16>()
            .expect("the frontend port to parseable to u16");
        url.set_port(Some(port)).unwrap();
    };

    url
}

pub use url::Url;
