use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::{Request, Response};

#[derive(serde::Deserialize)]
struct SumRequest {
    a: i64,
    b: i64,
}

#[derive(serde::Serialize)]
struct SumResponse {
    result: i64,
}

#[rocket::post("/sum", data = "<req>")]
fn sum(req: Json<SumRequest>) -> Json<SumResponse> {
    Json(SumResponse {
        result: req.a + req.b,
    })
}

/// Health check used by useBackendReady.ts to wait for the backend to be ready.
#[rocket::get("/health")]
fn health() -> &'static str {
    "ok"
}

/// Handles preflight OPTIONS requests sent by the browser before CORS requests.
#[rocket::options("/sum")]
fn options_sum() -> &'static str {
    ""
}

/// Allowed origins for CORS. Includes the Tauri webview origins and the Nuxt dev server.
const ALLOWED_ORIGINS: &[&str] = &[
    "tauri://localhost",
    "https://tauri.localhost",
    "http://localhost:3000",
    "http://127.0.0.1:3000",
];

/// CORS fairing: mirrors the request Origin back if it is in the allow-list.
/// Using a wildcard (*) would expose the API to DNS rebinding attacks.
struct CorsFairing;

#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let origin = req.headers().get_one("Origin").unwrap_or_default();

        let allowed = if ALLOWED_ORIGINS.contains(&origin) {
            origin
        } else {
            // No Origin header (e.g. curl, direct call) — allow localhost explicitly.
            "http://localhost:3000"
        };

        res.set_header(Header::new(
            "Access-Control-Allow-Origin",
            allowed.to_string(),
        ));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

fn rocket_instance() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", rocket::routes![health, sum, options_sum])
        .attach(CorsFairing)
        .configure(rocket::Config {
            port: 8000,
            // Show errors in release builds; full logging in debug builds.
            log_level: if cfg!(debug_assertions) {
                rocket::config::LogLevel::Normal
            } else {
                rocket::config::LogLevel::Critical
            },
            ..Default::default()
        })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            std::thread::spawn(|| {
                rocket::execute(rocket_instance().launch())
                    .expect("Rocket failed to launch — port 8000 may already be in use");
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    fn client() -> Client {
        Client::tracked(rocket_instance()).expect("valid rocket instance")
    }

    #[test]
    fn test_sum_positive() {
        let c = client();
        let res = c
            .post("/sum")
            .header(ContentType::JSON)
            .body(r#"{"a":2,"b":3}"#)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert!(res.into_string().unwrap().contains("5"));
    }

    #[test]
    fn test_sum_negative() {
        let c = client();
        let res = c
            .post("/sum")
            .header(ContentType::JSON)
            .body(r#"{"a":-1,"b":-2}"#)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert!(res.into_string().unwrap().contains("-3"));
    }

    #[test]
    fn test_sum_zeros() {
        let c = client();
        let res = c
            .post("/sum")
            .header(ContentType::JSON)
            .body(r#"{"a":0,"b":0}"#)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert!(res.into_string().unwrap().contains("0"));
    }

    #[test]
    fn test_sum_large_numbers() {
        let c = client();
        let res = c
            .post("/sum")
            .header(ContentType::JSON)
            .body(r#"{"a":1000000,"b":999999}"#)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert!(res.into_string().unwrap().contains("1999999"));
    }

    #[test]
    fn test_options_preflight() {
        let c = client();
        let res = c.options("/sum").dispatch();
        assert_eq!(res.status(), Status::Ok);
    }
}
