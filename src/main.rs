#![allow(dead_code)]

mod math;

use math::evector::FVector4;

//union
struct TestStruct {
    a: i32,
    b: f32,
}
enum ETest {
    A,
    B,
    C,
    Write(String),
}
const TEST: ETest = ETest::A;
static TEST2: ETest = ETest::A;
fn main() {
    println!("Hello, world!");
    const PP: f64 = 2.0;
    let xx: f64 = 1.0;
    let yy: f64 = xx;
    let stringex: String = String::from("test");

    if true {
    } else {
        println!("false");
    }

    unsafe {
        println!("unsafe");
    }
}

//async function
async fn async_test() {
    println!("async");
}
//trait
trait Test {
    fn test(&self);
}
//lifetime func
fn lifetime_test<'a>(x: &'a i32) -> &'a i32 {
    x
}

//async await function
async fn async_await_test() {
    async_test().await;
}
// consuming example
fn consuming_test(x: String) {
    println!("{}", x);
}
