use maelstrom_kafka::payload::KafkaPayload;
use maelstrom_protocol::messages::Message;

#[test]
fn test_kafka_node() -> color_eyre::Result<()> {
    let message: Message<KafkaPayload> = serde_json::from_str(
        r#"{"src": "c1", "dest": "n0", "body": { "type": "init", "msg_id": 1, "in_reply_to": null, "node_id": "n0", "node_ids": ["n0", "n1"]}}"#,
    )?;
    println!("{}", message);
    Ok(())
}
