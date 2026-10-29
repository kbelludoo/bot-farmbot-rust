use bot_farmbot_rust::game::types::*;
use serde_json::json;

#[test]
fn test_farmbot_lease_request() {
    let req = SessionLeaseRequest {
        holder_id: "test-holder-123".to_string(),
        force: true,
    };
    let val = serde_json::to_value(&req).unwrap();
    assert_eq!(val["holderId"], "test-holder-123");
    assert_eq!(val["force"], true);
}

#[test]
fn test_farmbot_bootstrap_deserialization() {
    let raw = json!({
        "account": {
            "id": "acc-999",
            "username": "HeroFarmer",
            "level": 42,
            "gold": 980000,
            "gems": 1500
        },
        "activeLevel": {
            "code": "act-2-stage-5",
            "name": "Lost Caverns"
        }
    });

    let boot: BootstrapResponse = serde_json::from_value(raw).unwrap();
    let acc = boot.account.unwrap();
    assert_eq!(acc.id, "acc-999");
    assert_eq!(acc.level, 42);
    assert_eq!(acc.gold, 980000);

    let lvl = boot.active_level.unwrap();
    assert_eq!(lvl.code, "act-2-stage-5");
}

#[test]
fn test_farmbot_run_complete() {
    let raw = json!({
        "success": true,
        "xp": 5400,
        "gold": 2100
    });

    let res: RunCompleteResponse = serde_json::from_value(raw).unwrap();
    assert!(res.success);
    assert_eq!(res.xp, 5400);
    assert_eq!(res.gold, 2100);
}
