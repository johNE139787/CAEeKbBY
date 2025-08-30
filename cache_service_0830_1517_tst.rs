use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// CacheEntry represents a cached value with expiration time.
struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

/// CacheService is a simple in-memory cache service with expiration.
#[derive(Default)]
struct CacheService<T> {
    inner: Mutex<HashMap<String, CacheEntry<T>>>,
}

impl<T> CacheService<T> where T: Clone {
    /// Create a new CacheService instance.
    pub fn new() -> Self {
        CacheService {
            inner: Mutex::new(HashMap::new()),
        }
    }

    /// Set a value in the cache with a specified expiration time.
    pub fn set(&self, key: String, value: T, expiration: Duration) {
        let mut cache = self.inner.lock().unwrap();
        cache.insert(key, CacheEntry {
            value,
            expires_at: Instant::now() + expiration,
        });
    }

    /// Get a value from the cache if it is not expired.
    pub fn get(&self, key: &str) -> Option<T> {
        let mut cache = self.inner.lock().unwrap();
        if let Some(cache_entry) = cache.get_mut(key) {
            if Instant::now() < cache_entry.expires_at {
                return Some(cache_entry.value.clone());
            } else {
                cache.remove(key); // Remove expired entry
            }
        }
        None
    }
}

/// Fairing to add CacheService to Rocket's state.
pub struct CacheServiceFairing;

#[rocket::async_trait]
impl rocket::fairing::Fairing for CacheServiceFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "CacheService Fairing", kind: rocket::fairing::Kind::Transform,
        }
    }

    async fn on_attach(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket {
        rocket.manage(CacheService::<String>::new())
    }
}

/// Example route that uses the cache service.
#[get("/cache/<key>")]
async fn get_cache(key: String, cache: &State<CacheService<String>>) -> Option<String> {
    cache.get(&key)
}

/// Initialize the Rocket instance with the cache service fairing.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CacheServiceFairing)
        .mount("/", routes![get_cache])
}