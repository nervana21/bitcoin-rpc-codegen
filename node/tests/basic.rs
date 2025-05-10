// node/tests/basic.rs

use node::{BitcoinNodeManager, NodeManager};
use tokio_test::block_on;

#[test]
fn test_node_lifecycle() {
    let node_manager = BitcoinNodeManager::new();

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
    let node_manager = BitcoinNodeManager::new();

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
