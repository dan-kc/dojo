# tokio/latest_config

Reference implementation. Read after attempting the exercise.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub version: u64,
    pub endpoint: String,
}
pub async fn latest_config(
    mut updates: tokio::sync::watch::Receiver<Config>,
    min_version: u64,
) -> Option<Config> {
    loop {
        {
            let current = updates.borrow_and_update();
            if current.version >= min_version {
                return Some(current.clone());
            }
        }
        if updates.changed().await.is_err() {
            return None;
        }
    }
}
```
