use bitflags::bitflags;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub mod database;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    _id: Option<ObjectId>,
    pub name: String,
    pub password: String
}

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
pub struct Repo {
    _id: Option<ObjectId>,
    name: String,
    owner: String,
    public_status: PublicStatus,
    modifiers: Vec<String>,
}

impl Repo {
    #[allow(dead_code)]
    fn name(&self) -> String {
        self.name.as_str().clone().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    _id: Option<ObjectId>,
    repo: String,
    proposer: String,
    authority: Authority,
    approvement: u8,

    itemtype: ItemType,
    name: String,
    description: String,
    description_word_vector: Vec<String>,
    word_vector: Vec<f64>,
    content: Option<Box<Item>>,
}

// {"mainkey": [0], "type":"房屋", "名称":"1203", "描述":"河北省廊坊市三河市燕郊开发区星河185小区，B3栋2单元1203", "词向量描述":["[<大房间>]+[<河边>]*0.3"], "vecs":[[1, 2, 3, 3]*50], "内容":[
//     {"mainkey": [1, 2], "type":"房间", "名称":"门口厕所", "描述":"进门右拐", "词向量描述":[
//         "[<厕所>]+[<小房间>]*0.3"]*2, "vecs":[[0, -1, 2, 2]*50, [0, -1, 2, 2]*50], "内容":[]},
//     {"mainkey": [3, 4, 5], "type":"房间", "名称":"客厅", "描述":"进门直走", "词向量描述":[
//         "[<客厅>]+[<餐厅>]*0.3"]*3, "vecs":[[4, 0, 0, 0]*50, [4, 0, 0, 0]*50, [4, 0, 0, 0]*50], "内容":[]},
// ]},
// {"mainkey": [6], "type":"物品", "名称":"服务器电脑", "描述":"服务器电脑", "词向量描述":[
//     "[<服务器>]+[<电脑主机>]*0.3"], "vecs":[[-0.8, -2, -2.2, 2]*50], "数量":1, "内容":[]}

#[derive(Debug, Serialize, Deserialize)]
enum PublicStatus {
    Public,
    Private,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum ItemType {
    Item,
    File,
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn create_item() {
        let repo = Repo {
            _id: 0 as u64,
            name: "rarara".to_string(),
            owner: String::from("ra"),
            public_status: PublicStatus::Private,
            modifiers: vec![String::from("ra"), String::from("ra"), String::from("ra")],
        };
        let j = serde_json::to_string(&repo).expect("msg");
        assert_eq!(
            j,
            r#"{"_id":0,"name":"rarara","owner":"ra","public_status":"Private","modifiers":["ra","ra","ra"]}"#
        );

        let item = Item {
            _id: 1 as u64,
            repo: repo.name(),
            proposer: String::from("ra"),
            authority: Authority::USER_READ
                | Authority::USER_WRITE
                | Authority::GROUP_READ
                | Authority::GROUP_WRITE
                | Authority::OTHER_READ
                | Authority::OTHER_READ,
            approvement: 0,
            itemtype: ItemType::Item,
            name: "Test".to_string(),
            description: "Test Item".to_string(),
            description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
            word_vector: vec![0.0, 0.0, 0.0],
            content: Some(Box::new(Item {
                _id: 2 as u64,
                repo: repo.name(),
                proposer: String::from("ra"),
                authority: Authority::USER_READ | Authority::OTHER_READ,
                approvement: 0,
                itemtype: ItemType::File,
                name: "Test sub".to_string(),
                description: "Test Sub Item".to_string(),
                description_word_vector: vec!["[<厕所>]+[<小房间>]*0.3".to_string()],
                word_vector: vec![1.0, 2.0, 3.0],
                content: None,
            })),
        };
        let j = serde_json::to_string(&item).expect("msg");
        assert_eq!(
            j,
            r#"{"_id":1,"repo":"rarara","proposer":"ra","authority":{"bits":31},"approvement":0,"itemtype":"Item","name":"Test","description":"Test Item","description_word_vector":["[<厕所>]+[<小房间>]*0.3"],"word_vector":[0.0,0.0,0.0],"content":{"_id":2,"repo":"rarara","proposer":"ra","authority":{"bits":17},"approvement":0,"itemtype":"File","name":"Test sub","description":"Test Sub Item","description_word_vector":["[<厕所>]+[<小房间>]*0.3"],"word_vector":[1.0,2.0,3.0],"content":null}}"#
        );
    }
}
