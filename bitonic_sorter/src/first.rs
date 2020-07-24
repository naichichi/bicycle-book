// pubは他のモジュールからアクセスできることを示す
// 引数xの型について
//   &はポインタ経由で借用することを示す
//   mutは値が変更可能であることを示す
//   [u32]型はu32のスライス
pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);

        sub_sort(x, up)        
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);

        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], true);
        sub_sort(&mut x[mid_point..], false);
    }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}