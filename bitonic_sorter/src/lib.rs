pub mod first;
pub mod second;

// 全てのモジュールから使用したいので、lib.rsに定義
pub enum SortOrder {
    Ascending,  // 昇順
    Descending, // 降順
}