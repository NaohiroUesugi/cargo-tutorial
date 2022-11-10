// let y = 8だとグローバルスコープで使えない
const GROBAL_POINTS: u32 = 200_000;

fn main() {
    sandbox_variables();
    sandbox_data_tyles();
}

fn sandbox_variables() {
    // 変数
    let mut x = 5; // `let x = 5`だと不変なためコンパイルエラー
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定数
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    println!("The value of GROBAL_POINTS is: {}", GROBAL_POINTS);

    // シャドーイング
    let shadow = 5;
    let shadow = shadow + 1;
    {
        let shadow = shadow * 2;
        println!("The value of shadow in the inner scope is: {}", shadow);
    }

    println!("The value of shadow is: {}", shadow);
}

fn sandbox_data_tyles() {
    // 不動小数点
    let _x = 2.0; // f64が基準

    let _y: f32 = 3.0; // f32

    // 数値演算
    let sum = 5 + 10; // 足k算
    let difference = 95.5 - 4.3; // 引き算
    let product = 4 * 30; // 掛け算
    let quotient = 56.7 / 32.2; // 割り算
    let floored = 2 / 3; // 結果は0
    let remainder = 43 % 5; // 余り
    println!("sum is: {}", sum);
    println!("difference is: {}", difference);
    println!("product is: {}", product);
    println!("quotient is: {}", quotient);
    println!("floored is: {}", floored);
    println!("remainder is: {}", remainder);

    // 論理値型
    let _t = true;

    let _f: bool = false; // 明示的型注釈�きで
}
