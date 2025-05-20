// node/tests/basic.rs

use node::{BitcoinNodeManager, NodeManager, TestConfig};
use tokio_test::block_on;

fn init_test_env() {
    let _ = tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .try_init();
}

fn get_available_port() -> u16 {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    listener.local_addr().unwrap().port()
}

#[test]
fn test_node_lifecycle() {
    init_test_env();
    let mut test_config = TestConfig::default();
    test_config.rpc_port = get_available_port();
    let mut node_manager = BitcoinNodeManager::new_with_config(&test_config).unwrap();

    // Test initial state
    let initial_state = block_on(node_manager.get_state()).unwrap();
    assert!(!initial_state.is_running);
    assert!(!initial_state.version.is_empty());

    // Test starting the node
    block_on(node_manager.start()).unwrap();
    let running_state = block_on(node_manager.get_state()).unwrap();
    assert!(running_state.is_running);

    // Test stopping the node
    block_on(node_manager.stop()).unwrap();
    let stopped_state = block_on(node_manager.get_state()).unwrap();
    assert!(!stopped_state.is_running);
}

#[test]
fn test_multiple_start_stop() {
    init_test_env();
    let mut test_config = TestConfig::default();
    test_config.rpc_port = get_available_port();
    let mut node_manager = BitcoinNodeManager::new_with_config(&test_config).unwrap();

    // Multiple starts should be idempotent
    block_on(node_manager.start()).unwrap();
    block_on(node_manager.start()).unwrap();
    let state = block_on(node_manager.get_state()).unwrap();
    assert!(state.is_running);

    // Multiple stops should be idempotent
    block_on(node_manager.stop()).unwrap();
    block_on(node_manager.stop()).unwrap();
    let state = block_on(node_manager.get_state()).unwrap();
    assert!(!state.is_running);
}

/// Regression test for automatic port selection functionality.
/// This test verifies that:
/// 1. When rpc_port is 0, a valid non-zero port is automatically selected
/// 2. The selected port is actually available for use
/// 3. The port selection mechanism works reliably
///
/// This test is important because:
/// - It prevents regression of the port selection feature
/// - It ensures bitcoind won't fail with "Invalid port" errors
/// - It verifies the port selection is working as expected
#[test]
fn test_auto_port_selection() {
    init_test_env();
    let test_config = TestConfig::default(); // This has rpc_port = 0
    let node_manager = BitcoinNodeManager::new_with_config(&test_config).unwrap();

    // Verify that a non-zero port was selected
    let port = node_manager.rpc_port();
    assert_ne!(port, 0, "Expected a non-zero port to be selected");

    // Verify that the port is actually available for use
    let listener = std::net::TcpListener::bind(("127.0.0.1", port));
    assert!(
        listener.is_ok(),
        "Selected port {} should be available for use",
        port
    );
}

#[test]
fn test_port_selection_behavior() {
    init_test_env();

    // Test explicit port selection
    let mut test_config = TestConfig::default();
    test_config.rpc_port = 8332;
    let node_manager = BitcoinNodeManager::new_with_config(&test_config).unwrap();
    assert_eq!(node_manager.rpc_port(), 8332);

    // Test port 0 behavior - should select a non-zero available port
    test_config.rpc_port = 0;
    let node_manager = BitcoinNodeManager::new_with_config(&test_config).unwrap();
    let auto_port = node_manager.rpc_port();
    assert_ne!(
        auto_port, 0,
        "Expected a non-zero port to be selected when port 0 is specified"
    );

    // Verify the selected port is available
    let listener = std::net::TcpListener::bind(("127.0.0.1", auto_port));
    assert!(
        listener.is_ok(),
        "Selected port {} should be available for use",
        auto_port
    );

    // Test port availability check with default config
    let test_config = TestConfig::default();
    let node_manager = BitcoinNodeManager::new_with_config(&test_config).unwrap();
    let port = node_manager.rpc_port();
    assert_ne!(port, 0, "Expected a non-zero port to be selected");
}
