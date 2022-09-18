use crate::db::models::LayerMsg;
use crate::db::{table_name, DbPool};
use sqlx::{query_as, Result};
use std::fmt;

pub enum LayerMsgEvent {
    All,
    Deposit,
    Withdrawal,
}

impl fmt::Display for LayerMsgEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::All => "all",
            Self::Deposit => "deposit",
            Self::Withdrawal => "withdrawal",
        };

        write!(f, "{s}")
    }
}

impl From<Option<String>> for LayerMsgEvent {
    fn from(event: Option<String>) -> Self {
        match event.unwrap_or_default().to_lowercase().as_str() {
            "deposit" => Self::Deposit,
            "withdrawal" => Self::Withdrawal,
            _ => Self::All,
        }
    }
}

pub async fn fetch_all(
    db_pool: &DbPool,
    event: &LayerMsgEvent,
    sender: &str,
) -> Result<Vec<LayerMsg>> {
    match event {
        LayerMsgEvent::All => {
            let mut msgs =
                fetch_all_from_table(db_pool, table_name::LAYER1_MESSAGE, "deposit", sender)
                    .await?;
            let mut l2_msgs =
                fetch_all_from_table(db_pool, table_name::LAYER2_MESSAGE, "withdrawal", sender)
                    .await?;
            msgs.append(&mut l2_msgs);
            msgs.sort_by(|a, b| b.created_time.cmp(&a.created_time));

            Ok(msgs)
        }
        LayerMsgEvent::Deposit => {
            fetch_all_from_table(db_pool, table_name::LAYER1_MESSAGE, "deposit", sender).await
        }
        LayerMsgEvent::Withdrawal => {
            fetch_all_from_table(db_pool, table_name::LAYER2_MESSAGE, "withdrawal", sender).await
        }
    }
}

async fn fetch_all_from_table(
    db_pool: &DbPool,
    table_name: &str,
    event: &str,
    sender: &str,
) -> Result<Vec<LayerMsg>> {
    let stmt = format!(
        "select
            height, status, layer1_hash, layer2_hash, created_time, $1 \"event\"
        from {} where sender = $2 order by created_time desc",
        table_name,
    );
    query_as::<_, LayerMsg>(&stmt)
        .bind(event)
        .bind(sender)
        .fetch_all(db_pool)
        .await
}
