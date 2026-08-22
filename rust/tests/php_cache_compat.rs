use serde_json::{Value, json};
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn php_to_rust_golden_is_compatible() {
    let output = Command::new("php")
        .args([
            "-r",
            "echo serialize(['access_token'=>'php-token','expires_in'=>7200,'nested'=>['ok'=>true]]);",
        ])
        .output()
        .expect("PHP is required for the bidirectional compatibility test");
    assert!(output.status.success());
    let decoded = taoyangli_tools::php_cache::decode(&output.stdout).unwrap();
    assert_eq!(
        decoded,
        json!({"access_token":"php-token","expires_in":7200,"nested":{"ok":true}})
    );
}

#[test]
fn rust_to_php_golden_is_compatible() {
    let original = json!({"access_token":"rust-token","expires_in":7200,"nested":{"ok":true}});
    let packed = taoyangli_tools::php_cache::encode(&original).unwrap();
    let mut child = Command::new("php")
        .args([
            "-r",
            "$value=unserialize(stream_get_contents(STDIN), ['allowed_classes'=>false]); echo json_encode($value, JSON_UNESCAPED_UNICODE|JSON_UNESCAPED_SLASHES);",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("PHP is required for the bidirectional compatibility test");
    child.stdin.as_mut().unwrap().write_all(&packed).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let decoded: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(decoded, original);
}
