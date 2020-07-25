use super::SortOrder;
use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
    where F: Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
    }
}

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // do_sortを呼ぶ代わりに、sort_byを呼ぶようにする
    // is_power_of_twoはsort_byが呼ぶので削除
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;

        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);

        sub_sort(x, forward, comparator);        
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);

        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    // forward(bool値)をOrdering値に変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };

    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントがswap_conditionと等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

// #[cfg(test)]アトリュビートをつけたことで、cargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
    // 親モジュール(first)のsort関数を使用する
    use super::{sort, sort_by};
    use crate::SortOrder::*;
    use crate::utils::{new_u32_vec, is_sorted_ascending, is_sorted_descending};

    // deriveアトリュビートを使い、DebugトレイトとPartialEqトレイトの実装を自動導出する
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String, // String型
        last_name: String,  // String型
        age: u8,            // u8型(8ビット符号なし整数)
    }

    impl Student {
        // 構造体Studentを初期化して返す。
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            // Selfはimpl対象の型(ここでいうStudent)の別名
            Self {
                // to_stringメソッドで&str型の引数からString型の値を作る
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age
            }
        }
    }

    // #[test]のついた関数はcargo testした時に実行される
    #[test]
    fn sort_u32_ascending() {
        // テストデータとしてu32型のベクタを作成
        // xに型注釈Vec<u32>を付ける
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        
        // 戻り値がOk(())であることを確認
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        // xに型注釈Vec<u32>を付ける
        let mut x: Vec<u32>  = vec![10, 30, 11, 20, 4, 330, 21, 110];
        
        assert_eq!(sort(&mut x, &Descending), Ok(()));

        // xの要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_u32_large() {
        {
            // 乱数で65,536要素のデータ列を作る
            let mut x = new_u32_vec(65536);

            // 昇順にソートする
            assert_eq!(sort(&mut x, &Ascending), Ok(()));

            // ソート結果が正しいことを検証する
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);

            assert_eq!(sort(&mut x, &Descending), Ok(()));

            assert!(is_sorted_descending(&x));
        }
    }

    #[test]
    fn sort_string_ascending() {
        // テストデータとして文字列型のベクタを作成
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_string_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        
        assert_eq!(sort(&mut x, &Descending), Ok(()));

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

    #[test]
    // 年齢で昇順にソートする
    fn sort_student_by_age_ascending() {
        let taro    = Student::new("Taro", "Yamada", 16);
        let hanako  = Student::new("Hanako", "Yamada", 14);
        let kyoko   = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        
        // ソート後の期待値
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(
            // 第2引数はソート順を決めるクロージャ
            //   引数に2つのStudent構造体をとり、ageフィールドの値をcmpメソッドで比較することで大小を決定する
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, expected);
    }

    #[test]
    // 氏名で昇順にソートする
    fn sort_student_by_name_ascending() {
        let taro    = Student::new("Taro", "Yamada", 16);
        let hanako  = Student::new("Hanako", "Yamada", 14);
        let kyoko   = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(
                &mut x,
                // まずlast_nameを比較する
                &|a, b| a.last_name.cmp(&b.last_name)
                    // もしlast_nameが等しくない(LessまたはGreater)ならそれを返し、
                    // 等しいならfirst_nameを比較する
                    .then_with(|| a.first_name.cmp(&b.first_name))
            ),
            Ok(())
        );

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_to_fail() {
        // 2の冪乗になっていない
        let mut x = vec![10, 30, 11];
        
        assert!(sort(&mut x, &Ascending).is_err());
    }
}
