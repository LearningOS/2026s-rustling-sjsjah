// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res = 42;
    let option = Some(12);
    //for x in option {
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
/*
右边 (option)：是数据源（可能是有值的 Some，也可能是空的 None）。
左边 (Some(x))：是要求/期望 
*/