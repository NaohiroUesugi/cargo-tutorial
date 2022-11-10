// let y = 8だとグローバルスコープで使えない
const GROBAL_POINTS: u32 = 200_000;

fn main() {
    sandbox_variables();
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
