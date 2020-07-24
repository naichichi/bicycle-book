use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGを初期化する、再現性をもたせるため毎回同じシード値を使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // n個の要素が格納できるようベクタを初期化
    let mut v = Vec::with_capacity(n);

    // 0からn-1までの合計n回繰り返し乱数を生成し、ベクタに追加する
    for _ in 0..n {
        v.push(rng.sample(&Standard));
    }

    v
}