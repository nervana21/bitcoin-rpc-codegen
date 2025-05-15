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
