use std::process::Command;

#[test]
fn test_handle_echo() -> color_eyre::Result<()> {
    let status = Command::new("./maelstrom/maelstrom")
        .args([
            "test",
            "-w",
            "echo",
            "--bin",
            "./target/debug/gossip-glomers",
            "--node-count",
            "1",
            "--time-limit",
            "10",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    Ok(())
}

#[test]
fn test_handle_generate() -> color_eyre::Result<()> {
    let status = Command::new("./maelstrom/maelstrom")
        .args([
            "test",
            "-w",
            "unique-ids",
            "--bin",
            "./target/debug/gossip-glomers",
            "--time-limit",
            "30",
            "--rate",
            "100",
            "--node-count",
            "3",
            "--availability",
            "total",
            "--nemesis",
            "partition",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    Ok(())
}
