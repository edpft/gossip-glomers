use edn_rs::Edn;
use regex::Regex;
use std::{fs, process::Command, str::FromStr};

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
    let output = Command::new("./maelstrom/maelstrom")
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
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout)?;
    let results_line: String = stdout
        .lines()
        .filter(|line| line.contains("jepsen.store"))
        .collect();
    let re = Regex::new(r"jepsen.store Wrote (.*results.edn)$")?;
    let captures = re.captures(&results_line).unwrap();
    let path = captures.get(1).unwrap().as_str();
    let content = fs::read_to_string(path)?;
    let edn = Edn::from_str(&content)?;
    let messages_per_operation = edn[":net"][":all"][":msgs-per-op"]
        .to_float()
        .expect("Cast msgs-per-op to float");
    assert!(messages_per_operation < 30.0);
    let median_latency = edn[":workload"][":stable-latencies"]["0.5"]
        .to_float()
        .expect("Cast median latency to float");
    assert!(median_latency < 400.0);
    let maximum_latency = edn[":workload"][":stable-latencies"]["1"]
        .to_float()
        .expect("Cast maximum latency to float");
    assert!(maximum_latency < 600.0);
    Ok(())
}
