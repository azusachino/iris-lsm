use crate::model::KVOps;

#[derive(Debug)]
pub struct IrisLsm {}

// impl KVOps<&str, &str> for IrisLsm {
//     fn put(&mut self, _k: &str, _v: &str) {
//         todo!()
//     }

//     fn delete(&mut self, _k: &str) {
//         todo!()
//     }

//     fn get(&self, _k: &str) -> anyhow::Result<&str> {
//         todo!()
//     }

//     fn scan(&self, _range: [&str; 2]) {
//         todo!()
//     }
// }

impl KVOps<String, String> for IrisLsm {
    fn put(&mut self, k: String, v: String) {
        todo!()
    }

    fn delete(&mut self, k: String) {
        todo!()
    }

    fn get(&self, k: String) -> anyhow::Result<String> {
        todo!()
    }

    fn scan(&self, range: [String; 2]) {
        todo!()
    }
}
