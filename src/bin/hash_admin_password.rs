use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use uuid::Uuid;

fn main() {
    let password = std::env::args()
        .nth(1)
        .expect("usage: cargo run --bin hash_admin_password -- <password>");
    let salt = SaltString::encode_b64(Uuid::new_v4().as_bytes()).expect("salt should encode");
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("password hash should generate");
    println!("{hash}");
}
