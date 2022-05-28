use bitflags::bitflags;
use serde::{Deserialize, Serialize};

pub mod ra;

bitflags! {
    #[derive(Serialize, Deserialize)]
    struct Authority: u32 {
        const USER_READ = 0x1;
        const USER_WRITE = 0x2;
        const GROUP_READ = 0x4;
        const GROUP_WRITE = 0x8;
        const OTHER_READ = 0x10;
        const OTHER_WRITE = 0x11;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    owner: String,
    public_status: PublicStatus,
    modifiers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    repo: Repo,
    proposer: String,
    authority: Authority,
    approvement: u8,
}

#[derive(Debug, Serialize, Deserialize)]
enum PublicStatus {
    Public,
    Private,
}


#[test]
fn add() {
    let repo = Repo {
        owner: String::from("ra"),
        public_status: PublicStatus::Private,
        modifiers: vec![String::from("ra"), String::from("ra"), String::from("ra")],
    };
    let j = serde_json::to_string(&repo).expect("msg");
    assert_eq!(
        j,
        r#"{"owner":"ra","public_status":"Private","modifiers":["ra","ra","ra"]}"#
    );

    let item = Item {
        repo,
        proposer: String::from("ra"),
        authority: Authority::USER_READ
            | Authority::USER_WRITE
            | Authority::GROUP_READ
            | Authority::GROUP_WRITE
            | Authority::OTHER_READ
            | Authority::OTHER_READ,
        approvement: 0,
    };
    let j = serde_json::to_string(&item).expect("msg");
    assert_eq!(
        j,
        r#"{"repo":{"owner":"ra","public_status":"Private","modifiers":["ra","ra","ra"]},"proposer":"ra","authority":{"bits":31},"approvement":0}"#
    );
}
