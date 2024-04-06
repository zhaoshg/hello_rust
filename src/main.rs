fn main() {
string_from();
}

fn string_from(){
    let mut s1 = String::from("hello");
    s1.push_str(" world!");
    // let s2 = s1;
    let s2 = s1.clone();
    println!("{}", s1);
}

// fn test_one(x: i32) -> i32 {
//     println!("this is  function used to print {} ! ", x);
//     x + 1
// }
//
// fn test_loop1() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("the result is {}", result)
// }
//
// fn test_loop2() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{}!", number);
//         number = number - 1;
//     }
//     println!("LIFTOFF!!!");
// }
//
//
// fn test_loop3() {
//     for num in (1..4).rev() {
//         println!("{}!", num);
//     }
//     println!("LIFTOFF!!!");
// }
//
// fn test1() {
//     let mut x = 5;
//     let mut addr = &x as *const i32 as usize;
//     println!("The value of x = {}, addr = {:X} ", x, addr);
//     x += 1;
//     addr = &x as *const i32 as usize;
//     println!("The value of x = {}, addr = {:X} ", x, addr);
//     x += 1;
//     addr = &x as *const i32 as usize;
//     println!("The value of x = {}, addr = {:X} ", x, addr);
//
//
//     let y = x;
//     addr = &y as *const i32 as usize;
//     println!("The value of y = {}, addrY = {:X} ", y, addr);
//     let y = y + 1;
//     addr = &y as *const i32 as usize;
//     println!("The value of y = {}, addrY = {:X} ", y, addr);
//
//     const MAX_POINTS: u32 = 100000;
//     addr = &MAX_POINTS as *const u32 as usize;
//     println!("The value of MAX_POINTS = {}, addrY = {:X} ", MAX_POINTS, addr);
//
//
//     let tup = (500, 6.4, 1);
//     let (_x, _y, _z) = tup;
//
//     addr = &_x as *const i32 as usize;
//     println!("The value of _x = {}, addrX = {:X} ", _x, addr);
//     addr = &_z as *const i32 as usize;
//     println!("The value of _z = {}, addrY = {:X} ", _z, addr);
//
//     let one = tup.2;
//     println!("The value of one = {}, addrY = {:X} ", one, &one as *const i32 as usize);
//
//     let array_a = [1, 2, 3, 4, 5, ];
//     for it in array_a {
//         println!("{}", it)
//     }
//
//     let res = test_one(33);
//     println!("result is {}", res);
// }