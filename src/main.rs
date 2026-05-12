use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::CrosstownBus;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

fn main() {
    let mut p = CrosstownBus::new_queue_publisher(
        "amqp://guest:guest@localhost:5672".to_owned(),
    )
    .unwrap();

    let _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406358472-Ah".to_owned(),
        },
    );
    let _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406358472-Bud".to_owned(),
        },
    );
    let _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406358472-Cia".to_owned(),
        },
    );
    let _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406358472-Dira".to_owned(),
        },
    );
    let _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406358472-Emi".to_owned(),
        },
    );
}
