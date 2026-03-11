// Intentionally Vulnerable Rust/Actix-Web Application
// DO NOT USE IN PRODUCTION - FOR SECURITY TESTING ONLY

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::Command;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;
use regex::Regex;
use base64;

// VULNERABILITY: Hardcoded secrets (CWE-798)
const JWT_SECRET: &str = "super_secret_jwt_key_12345";
const ADMIN_PASSWORD: &str = "admin123";
const DB_PASSWORD: &str = "password123";
const API_KEY: &str = "AKIA_FAKE_RUST_KEY_FOR_TESTING_ONLY";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    role: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
    email: String,
    role: Option<String>,
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(r#"
        <html>
        <head><title>Vulnerable Rust App</title></head>
        <body>
            <h1>Intentionally Vulnerable Rust/Actix-Web Application</h1>
            <p>This application contains numerous security vulnerabilities for testing purposes.</p>
            <h2>Available Endpoints:</h2>
            <ul>
                <li>POST /api/login - SQL Injection</li>
                <li>GET /api/exec?cmd=ls - Command Injection</li>
                <li>GET /api/files?filename=test.txt - Path Traversal</li>
                <li>GET /api/search?query=test - XSS</li>
                <li>GET /api/proxy?url=http://example.com - SSRF</li>
                <li>POST /api/eval - Unsafe Code Execution</li>
                <li>DELETE /api/admin/users/{id} - Missing Authentication</li>
                <li>GET /api/users/{id} - IDOR</li>
                <li>POST /api/register - Mass Assignment</li>
                <li>GET /api/debug - Sensitive Data Exposure</li>
                <li>GET /api/redirect?url=http://example.com - Open Redirect</li>
                <li>POST /api/hash - Weak Cryptography</li>
                <li>GET /api/regex?input=test - ReDoS</li>
            </ul>
        </body>
        </html>
    "#))
}

// VULNERABILITY: SQL Injection (CWE-89)
async fn login(data: web::Json<LoginRequest>) -> Result<HttpResponse> {
    let username = &data.username;
    let password = &data.password;

    // Vulnerable: String formatting in SQL query (simulated)
    let query = format!("SELECT * FROM users WHERE username = '{}' AND password = '{}'", username, password);

    Ok(HttpResponse::Ok().json(json!({
        "query": query,
        "vulnerable": true
    })))
}

// VULNERABILITY: Command Injection (CWE-78)
async fn exec(req: HttpRequest) -> Result<HttpResponse> {
    let query_string = req.query_string();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let cmd = params.get("cmd").map(|s| s.as_str()).unwrap_or("");

    // Vulnerable: Direct execution of user input
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "output": stdout
            })))
        }
        Err(e) => {
            Ok(HttpResponse::Ok().json(json!({
                "error": e.to_string()
            })))
        }
    }
}

// VULNERABILITY: Path Traversal (CWE-22)
async fn files(req: HttpRequest) -> Result<HttpResponse> {
    let query_string = req.query_string();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let filename = params.get("filename").map(|s| s.as_str()).unwrap_or("");

    // Vulnerable: No sanitization of file path
    let path = format!("./uploads/{}", filename);

    match fs::read_to_string(&path) {
        Ok(content) => Ok(HttpResponse::Ok().json(json!({ "content": content }))),
        Err(e) => Ok(HttpResponse::Ok().json(json!({ "error": e.to_string() })))
    }
}

// VULNERABILITY: Cross-Site Scripting (XSS) (CWE-79)
async fn search(req: HttpRequest) -> Result<HttpResponse> {
    let query_string = req.query_string();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let query = params.get("query").map(|s| s.as_str()).unwrap_or("");

    // Vulnerable: Reflects user input without sanitization
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<h1>Search Results for: {}</h1>", query)))
}

// VULNERABILITY: Server-Side Request Forgery (SSRF) (CWE-918)
async fn proxy(req: HttpRequest) -> Result<HttpResponse> {
    let query_string = req.query_string();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let url = params.get("url").map(|s| s.as_str()).unwrap_or("");

    // Vulnerable: No URL validation
    let response = reqwest::get(url);

    match response {
        Ok(resp) => {
            let body = resp.text().unwrap_or_default();
            Ok(HttpResponse::Ok().json(json!({ "data": body })))
        }
        Err(e) => Ok(HttpResponse::Ok().json(json!({ "error": e.to_string() })))
    }
}

// VULNERABILITY: Unsafe Code / Potential Memory Issues
async fn eval(body: String) -> Result<HttpResponse> {
    // Vulnerable: This demonstrates unsafe patterns
    // In real attacks, this could involve unsafe Rust code
    Ok(HttpResponse::Ok().json(json!({
        "message": "Code evaluation endpoint (vulnerable pattern)",
        "input": body
    })))
}

// VULNERABILITY: Mass Assignment (CWE-915)
async fn register(data: web::Json<RegisterRequest>, app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let mut users = app_state.users.lock().unwrap();

    // Vulnerable: Allows setting 'role' field directly
    let new_user = User {
        id: (users.len() + 1) as i32,
        username: data.username.clone(),
        password: data.password.clone(),
        email: data.email.clone(),
        role: data.role.clone().unwrap_or_else(|| "user".to_string()), // Attacker can set role=admin
    };

    users.push(new_user.clone());

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "user": new_user
    })))
}

// VULNERABILITY: Insecure Direct Object Reference (IDOR) (CWE-639)
async fn get_user(path: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let users = app_state.users.lock().unwrap();

    // Vulnerable: No authorization check
    let user = users.iter().find(|u| u.id == user_id);

    Ok(HttpResponse::Ok().json(json!({
        "user": user
    })))
}

// VULNERABILITY: Missing Authentication (CWE-306)
async fn delete_user(path: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let mut users = app_state.users.lock().unwrap();

    // Vulnerable: No authentication or authorization required
    users.retain(|u| u.id != user_id);

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "deleted": user_id
    })))
}

// VULNERABILITY: Sensitive Data Exposure (CWE-200)
async fn debug() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "jwt_secret": JWT_SECRET,
        "admin_password": ADMIN_PASSWORD,
        "db_password": DB_PASSWORD,
        "api_key": API_KEY,
        "environment": std::env::vars().collect::<HashMap<String, String>>()
    })))
}

// VULNERABILITY: Open Redirect (CWE-601)
async fn redirect(req: HttpRequest) -> Result<HttpResponse> {
    let query_string = req.query_string();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let url = params.get("url").map(|s| s.as_str()).unwrap_or("https://example.com");

    // Vulnerable: No validation of redirect URL
    Ok(HttpResponse::Found()
        .header("Location", url)
        .finish())
}

// VULNERABILITY: Weak Cryptography (CWE-327)
#[derive(Deserialize)]
struct HashRequest {
    password: String,
}

async fn hash(data: web::Json<HashRequest>) -> Result<HttpResponse> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Vulnerable: Using weak hash (for demonstration)
    let mut hasher = DefaultHasher::new();
    data.password.hash(&mut hasher);
    let hash = hasher.finish();

    Ok(HttpResponse::Ok().json(json!({
        "hash": hash.to_string(),
        "algorithm": "DefaultHasher (weak)"
    })))
}

// VULNERABILITY: Regular Expression Denial of Service (ReDoS) (CWE-1333)
async fn regex_test(req: HttpRequest) -> Result<HttpResponse> {
    let query_string = req.query_string();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let input = params.get("input").map(|s| s.as_str()).unwrap_or("");

    // Vulnerable: Catastrophic backtracking regex
    let re = Regex::new(r"^(a+)+$").unwrap();
    let is_match = re.is_match(input);

    Ok(HttpResponse::Ok().json(json!({
        "matched": is_match
    })))
}

// VULNERABILITY: Insecure Randomness (CWE-330)
async fn generate_token() -> Result<HttpResponse> {
    use rand::Rng;

    // Vulnerable: Using weak random
    let mut rng = rand::thread_rng();
    let token: u32 = rng.gen();

    Ok(HttpResponse::Ok().json(json!({
        "token": token.to_string(),
        "algorithm": "rand::thread_rng() (predictable)"
    })))
}

// VULNERABILITY: Hardcoded Credentials (CWE-798)
#[derive(Deserialize)]
struct AdminLoginRequest {
    password: String,
}

async fn admin_login(data: web::Json<AdminLoginRequest>) -> Result<HttpResponse> {
    // Vulnerable: Hardcoded admin password
    if data.password == ADMIN_PASSWORD {
        Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "role": "admin"
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "success": false
        })))
    }
}

// VULNERABILITY: Information Exposure Through Error Messages (CWE-209)
async fn database_connect() -> Result<HttpResponse> {
    // Vulnerable: Detailed error messages exposed
    let error_msg = format!("Connection failed: Access denied for user 'root'@'localhost' using password '{}'", DB_PASSWORD);

    Ok(HttpResponse::Ok().json(json!({
        "error": error_msg,
        "stack_trace": "Simulated stack trace with sensitive info"
    })))
}

// VULNERABILITY: Missing Rate Limiting (CWE-770)
#[derive(Deserialize)]
struct BruteForceRequest {
    password: String,
}

async fn brute_force_target(data: web::Json<BruteForceRequest>) -> Result<HttpResponse> {
    // Vulnerable: No rate limiting, allows brute force
    if data.password == "correct_password" {
        Ok(HttpResponse::Ok().json(json!({ "success": true })))
    } else {
        Ok(HttpResponse::Ok().json(json!({ "success": false })))
    }
}

// VULNERABILITY: Cleartext Transmission of Sensitive Information (CWE-319)
#[derive(Deserialize)]
struct CredentialsRequest {
    username: String,
    password: String,
}

async fn send_credentials(data: web::Json<CredentialsRequest>) -> Result<HttpResponse> {
    // Vulnerable: Credentials sent without HTTPS enforcement
    Ok(HttpResponse::Ok().json(json!({
        "received": true,
        "username": data.username
    })))
}

// VULNERABILITY: Use of GET Request Method With Sensitive Query Strings (CWE-598)
async fn reset_password(req: HttpRequest) -> Result<HttpResponse> {
    let query_string = req.query_string();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap_or_default();
    let token = params.get("token").map(|s| s.as_str()).unwrap_or("");
    let password = params.get("password").map(|s| s.as_str()).unwrap_or("");

    // Vulnerable: Sensitive data in GET parameters (appears in logs)
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "token": token,
        "password": password
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Starting Vulnerable Rust/Actix-Web Application...");
    println!("WARNING: This application is intentionally vulnerable!");
    println!("Access at: http://localhost:8080");

    // Initialize app state with sample users
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![
            User {
                id: 1,
                username: "admin".to_string(),
                password: "hashed_password".to_string(),
                email: "admin@example.com".to_string(),
                role: "admin".to_string(),
            },
            User {
                id: 2,
                username: "user".to_string(),
                password: "hashed_password".to_string(),
                email: "user@example.com".to_string(),
                role: "user".to_string(),
            },
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/api/login", web::post().to(login))
            .route("/api/exec", web::get().to(exec))
            .route("/api/files", web::get().to(files))
            .route("/api/search", web::get().to(search))
            .route("/api/proxy", web::get().to(proxy))
            .route("/api/eval", web::post().to(eval))
            .route("/api/register", web::post().to(register))
            .route("/api/users/{id}", web::get().to(get_user))
            .route("/api/admin/users/{id}", web::delete().to(delete_user))
            .route("/api/debug", web::get().to(debug))
            .route("/api/redirect", web::get().to(redirect))
            .route("/api/hash", web::post().to(hash))
            .route("/api/regex", web::get().to(regex_test))
            .route("/api/generate-token", web::get().to(generate_token))
            .route("/api/admin-login", web::post().to(admin_login))
            .route("/api/database-connect", web::get().to(database_connect))
            .route("/api/brute-force-target", web::post().to(brute_force_target))
            .route("/api/send-credentials", web::post().to(send_credentials))
            .route("/api/reset-password", web::get().to(reset_password))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
