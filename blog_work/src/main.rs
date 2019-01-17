extern crate blog_work;
use blog_work::PostV1;
use blog_work::PostV2;

fn main() {
    version1();
    version2();
}

fn version1() {
    let mut post = PostV1::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn version2() {
    let mut post = PostV2::new();

    post.add_text("I ate burgers for lunch today!");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate burgers for lunch today!", post.content());
}
