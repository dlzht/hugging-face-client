// use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Pagination {
//   page_num: usize,
//   page_size: usize,
// }
//
// impl Default for Pagination {
//   fn default() -> Self {
//     Pagination {
//       page_num: 1,
//       page_size: 20,
//     }
//   }
// }
//
// impl Pagination {
//   pub fn page_size(mut self, page_size: usize) -> Self {
//     self.page_size = page_size;
//     self
//   }
//
//   pub fn page_num(mut self, page_num: usize) -> Self {
//     self.page_num = page_num;
//     self
//   }
// }
