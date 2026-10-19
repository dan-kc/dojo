// cargo test --lib tokio::latest_config::

//! Wait for a configuration whose version is at least min_version, considering the current value immediately.
//! Updates may be coalesced: intermediate versions need not be observed. Return an owned snapshot, and return None
//! if all senders close before a suitable version is available. Do not hold a watch borrow across an await.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub version: u64,
    pub endpoint: String,
}
pub async fn latest_config(
    mut updates: tokio::sync::watch::Receiver<Config>,
    min_version: u64,
) -> Option<Config> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::watch;
    #[tokio::test]
    async fn current_and_closed() {
        let (tx, rx) = watch::channel(Config {
            version: 2,
            endpoint: "a".into(),
        });
        drop(tx);
        assert_eq!(latest_config(rx.clone(), 2).await.unwrap().endpoint, "a");
        assert_eq!(latest_config(rx, 3).await, None);
    }
    #[tokio::test]
    async fn sees_latest_update() {
        let (tx, rx) = watch::channel(Config {
            version: 0,
            endpoint: "old".into(),
        });
        tx.send(Config {
            version: 1,
            endpoint: "one".into(),
        })
        .unwrap();
        tx.send(Config {
            version: 3,
            endpoint: "new".into(),
        })
        .unwrap();
        drop(tx);
        assert_eq!(
            latest_config(rx, 2).await,
            Some(Config {
                version: 3,
                endpoint: "new".into()
            })
        );
    }
    #[tokio::test]
    async fn waits_for_future_version() {
        let (tx, rx) = watch::channel(Config {
            version: 0,
            endpoint: "old".into(),
        });
        let task = tokio::spawn(latest_config(rx, 2));
        tx.send(Config {
            version: 2,
            endpoint: "new".into(),
        })
        .unwrap();
        assert_eq!(task.await.unwrap().unwrap().version, 2);
    }
}
