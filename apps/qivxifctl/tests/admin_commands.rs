use serde_json::Value;
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn create_user_command_writes_member_profile() {
    let store = test_store("create-user");
    let admin = run_with_password(
        &[
            "admin",
            "bootstrap",
            "--store",
            store.to_str().unwrap(),
            "--name",
            "admin",
            "--password-stdin",
            "--json",
        ],
        "admin-pass",
    );
    assert_eq!(admin["roles"][0], "owner");

    let member = run_with_password(
        &[
            "admin",
            "create-user",
            "--store",
            store.to_str().unwrap(),
            "--name",
            "member",
            "--password-stdin",
            "--json",
        ],
        "member-pass",
    );
    assert_eq!(member["status"], "created");
    assert_eq!(member["name"], "member");
    assert_eq!(member["roles"][0], "member");
    assert!(
        member["profile_node_id"]
            .as_str()
            .unwrap()
            .starts_with("nod_")
    );
}

#[test]
fn issue_invite_and_key_commands_write_audit() {
    let store = test_store("keys");
    let _ = run_with_password(
        &[
            "admin",
            "bootstrap",
            "--store",
            store.to_str().unwrap(),
            "--name",
            "admin",
            "--password-stdin",
            "--json",
        ],
        "admin-pass",
    );
    let member = run_with_password(
        &[
            "admin",
            "create-user",
            "--store",
            store.to_str().unwrap(),
            "--name",
            "member",
            "--password-stdin",
            "--json",
        ],
        "member-pass",
    );
    let invite = run_json(&[
        "admin",
        "issue-invite",
        "--store",
        store.to_str().unwrap(),
        "--role",
        "member",
        "--json",
    ]);
    assert!(invite["secret"].as_str().unwrap().starts_with("qxi_inv_"));
    let key = run_json(&[
        "admin",
        "issue-key",
        "--store",
        store.to_str().unwrap(),
        "--user",
        "member",
        "--scope",
        "graph.read",
        "--json",
    ]);
    assert!(key["secret"].as_str().unwrap().starts_with("qxi_key_"));
    assert_eq!(key["key"]["user_id"], member["user_id"]);
    let audit = run_json(&[
        "admin",
        "audit-keys",
        "--store",
        store.to_str().unwrap(),
        "--json",
    ]);
    assert!(audit["audit"].as_array().unwrap().len() >= 2);
}

#[test]
fn create_user_command_rejects_public_role() {
    let store = test_store("reject-role");
    let _ = run_with_password(
        &[
            "admin",
            "bootstrap",
            "--store",
            store.to_str().unwrap(),
            "--name",
            "admin",
            "--password-stdin",
            "--json",
        ],
        "admin-pass",
    );
    let output = command()
        .args([
            "admin",
            "create-user",
            "--store",
            store.to_str().unwrap(),
            "--name",
            "guest",
            "--password-stdin",
            "--role",
            "public",
        ])
        .stdin(Stdio::piped())
        .output()
        .unwrap();
    assert!(!output.status.success());
}

fn run_with_password(args: &[&str], password: &str) -> Value {
    let mut child = command()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(password.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn run_json(args: &[&str]) -> Value {
    let output = command().args(args).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_qivxifctl"))
}

fn test_store(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("qivxifctl-{name}-{nanos}"));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("qivxif.redb")
}
