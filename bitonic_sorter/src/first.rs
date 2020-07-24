// pubは他のモジュールからアクセスできることを示す
// 引数xの型について
//   &はポインタ経由で借用することを示す
//   mutは値が変更可能であることを示す
//   [u32]型はu32のスライス
pub fn sort(x: &mut [u32], up: bool) {
    // 未実装の意味。コンパイルは通るが、実行するとpanicする
    unimplemented!();
}

fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!();
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}