mod kernel;
mod add;
//mod kernel;，rust 首先会尝试加载 src/kernel.rs 文件，如果文件不存在，还会尝试加载 src/kernel/mod.rs 文件
fn main() {
    kernel::recommend();
    println!("--------------------------------------------------------");
    kernel::recall::recall();
    kernel::filter::filter();
    kernel::sort::sort();
    //使用mod.rs
    println!("--------------------------------------------------------");
    println!("2+3={}",add::add(2, 3));
    println!("--------------------------------------------------------");
    //使用sub.rs构建模块的子模块
    let a=format!("{}-{}={}",3,2,add::sub::sub(3, 2));
    println!("{a}");
    println!("--------------------------------------------------------");
    //使用mod.rs构建模块的子模块
    println!("{}*{}={}",2,3,add::mul::mul(2, 3));
}
