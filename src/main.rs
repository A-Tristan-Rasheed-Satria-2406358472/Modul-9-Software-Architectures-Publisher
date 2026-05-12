use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use tokio_amqp::*;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672/%2f",
        ConnectionProperties::default().with_tokio(),
    )
    .await?;
    let channel = conn.create_channel().await?;

    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let messages = vec![
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406358472-Ah".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406358472-Bud".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406358472-Cia".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406358472-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406358472-Emi".to_owned(),
        },
    ];

    for msg in messages {
        let payload = msg.try_to_vec()?;
        channel
            .basic_publish(
                "",
                "user_created",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
    }

    Ok(())
}
