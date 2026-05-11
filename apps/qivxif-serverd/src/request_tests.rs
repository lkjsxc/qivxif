use super::*;
use qivxif_sim::RegionHandle;
use qivxif_storage::WorldStore;
use std::sync::{Arc, atomic::AtomicU64};

fn test_state() -> (tempfile::TempDir, AppState) {
    let root = tempfile::tempdir().unwrap();
    let store = Arc::new(WorldStore::open(root.path(), 5).unwrap());
    let world_id = store.meta().world_id.clone();
    let state = AppState {
        build_contract: "test".to_string(),
        protocol_contract: qivxif_protocol::CURRENT_PROTOCOL_CONTRACT.to_string(),
        world_id,
        next_session: AtomicU64::new(1),
        region: RegionHandle::spawn(5, store),
    };
    (root, state)
}

#[tokio::test]
async fn join_before_hello_is_rejected() {
    let (_root, state) = test_state();
    let mut session = Session::new(1);
    let msg = respond(
        ClientMsg::JoinWorld {
            player: "probe".to_string(),
        },
        &state,
        &mut session,
    )
    .await;
    assert!(matches!(
        msg,
        ServerMsg::Error {
            code: ErrorCode::HelloRequired,
            ..
        }
    ));
}

#[tokio::test]
async fn protocol_contract_mismatch_is_rejected() {
    let (_root, state) = test_state();
    let mut session = Session::new(1);
    let msg = respond(
        ClientMsg::Hello {
            build_contract: "test".to_string(),
            protocol_contract: "mismatch".to_string(),
        },
        &state,
        &mut session,
    )
    .await;
    assert!(matches!(
        msg,
        ServerMsg::Error {
            code: ErrorCode::ProtocolContractMismatch,
            ..
        }
    ));
}

#[tokio::test]
async fn hello_reports_local_compose_capabilities() {
    let (_root, state) = test_state();
    let mut session = Session::new(1);
    let msg = respond(
        ClientMsg::Hello {
            build_contract: "test".to_string(),
            protocol_contract: qivxif_protocol::CURRENT_PROTOCOL_CONTRACT.to_string(),
        },
        &state,
        &mut session,
    )
    .await;
    assert!(matches!(
        msg,
        ServerMsg::HelloOk {
            caps: LOCAL_COMPOSE_CAPS,
            ..
        }
    ));
}
