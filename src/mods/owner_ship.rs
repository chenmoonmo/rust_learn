use std::ops::Add;

pub fn owner_ship() {
    let x = 5;
    let y = 5;
    // 对于基本类型或是储存在栈上的类型，赋值语句实际发生的是浅拷贝。但因为数据在栈上，比较简单，因此不会有所有权问题。
    let z = x; 

    let ss = "hello";
    let ss2 = ss;
    
    let s = String::from("hello world");
    // Rust 不会主动深克隆，只能主动调用 clone 方法。
    let s2 = s.clone();
    // 对于复杂类型来说，赋值语句会移动内存地址的所有权，因此 s 在赋值语句后失效。
    let s3= s;

    println!("{},{},{}",x,y,z);
    println!("{},{}",s2,s3);
    println!("{},{}",ss,ss2);
    // println!("{}",s);
    owner_ship_at_fn();
}

 fn string_concat_5 (s:String) -> String {
    s.add("5")
}

fn owner_ship_at_fn (){
    let mut s = String::from("5555");
    //  向函数传递参数会移交所有权，函数的返回值也有所有权
    s = string_concat_5(s);

    println!("{}",s);


}