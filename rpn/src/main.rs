fn main() {
    // 6.1 + 5.2 * 4.3 - 3.4 / 2.5 * 1.6 と等しい 
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = rpn(exp);

    // デバッグビルド時のみ答えが正しいかチェックする
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    println!("{} = {:.4}", exp, ans);
}

// RPN形式の文字列expを受け取り、f64型の計算結果を返す
fn rpn(exp: &str) -> f64 {
    let mut stack = Vec::new();

    // expの要素をスペースで分割し、順に束縛する
    for token in exp.split_whitespace() {
        // parseメソッドをつかってtokenがf64型に変更できるか試す。
        // 成功するとOk(6.1)が返される
        if let Ok(num) = token.parse::<f64>() {
            // tokenがf64型ならスタックにpushする
            stack.push(num);
        } else {
            // |x, y| x + y はクロージャ
            // 引数x, yを取り、x + yを計算して答えを返す
            match token {
                // &はstackが束縛されたベクタへの参照を表し、mutによりapply2関数の中でベクタを変更することを許可している。
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                // tokenが演算子ではない場合、エラーを起こして終了する
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }

    // スタックから数値を1つ取り出す。失敗したらエラーを起こして終了
    // セミコロンを省略した場合、関数の戻り値として呼び出し元へ返せる
    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F型のトレイト境界
where F: Fn(f64, f64) -> f64,
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        // クロージャーfun(|x, y| x + y)で計算
        let z = fun(x, y);
        stack.push(z);
    } else {
        panic!("Stack underflow");
    }
}    