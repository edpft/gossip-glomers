use std::process::Command;

#[test]
fn test_echo() -> color_eyre::Result<()> {
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
fn test_generate() -> color_eyre::Result<()> {
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

#[test]
fn test_single_node_broadcast() -> color_eyre::Result<()> {
    let status = Command::new("./maelstrom/maelstrom")
        .args([
            "test",
            "-w",
            "broadcast",
            "--bin",
            "./target/debug/gossip-glomers",
            "--node-count",
            "1",
            "--time-limit",
            "20",
            "--rate",
            "10",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    Ok(())
}

#[test]
fn test_multi_node_broadcast() -> color_eyre::Result<()> {
    let status = Command::new("./maelstrom/maelstrom")
        .args([
            "test",
            "-w",
            "broadcast",
            "--bin",
            "./target/debug/gossip-glomers",
            "--node-count",
            "5",
            "--time-limit",
            "20",
            "--rate",
            "10",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    Ok(())
}

#[test]
fn test_fault_tolerant_broadcast() -> color_eyre::Result<()> {
    let status = Command::new("./maelstrom/maelstrom")
        .args([
            "test",
            "-w",
            "broadcast",
            "--bin",
            "./target/debug/gossip-glomers",
            "--node-count",
            "5",
            "--time-limit",
            "20",
            "--rate",
            "10",
            "--nemesis",
            "partition",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    Ok(())
}

#[test]
fn test_broadcast_efficiency_1() -> color_eyre::Result<()> {
    let status = Command::new("./maelstrom/maelstrom")
        .args([
            "test",
            "-w",
            "broadcast",
            "--bin",
            "./target/debug/gossip-glomers",
            "--node-count",
            "25",
            "--time-limit",
            "20",
            "--rate",
            "100",
            "--latency",
            "100",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    Ok(())
}
