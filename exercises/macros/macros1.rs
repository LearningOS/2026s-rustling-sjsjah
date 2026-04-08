// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.
//宏的参数可以使用 ()、[] 以及 {}:
//从根本上来说，宏是通过一种代码来生成另一种代码
/*优点:
    可变参数
     宏可以被展开成其他代码, 而且该过程是在编译器对代码解释之前 ,所以宏可以为指定类型实现特征
     先展开成实现特征的代码后再编译 
      
     相较于函数:需要在运行时才能编译(特征需要在编译器实现)
     : 运行要快一些(零开销),而且可以提前报错. 笑
     
  缺点:
    难读难理解难维护
*/




macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
