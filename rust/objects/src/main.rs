use objects::Post;

mod oops;

fn oop() {
    let mut post = oops::Post::new();
    post.add_text("hello alt post");
    post.add_text("\nwe do a little oops");
    println!("Content should be 'empty': {}", post.content());
    post.request_review();
    println!("Content should still be 'empty': {}", post.content());
    post.approve();
    println!("Content should now return: {}\n", post.content());

}

fn idiomatic() {
    let mut post = Post::new();

    post.add_text("hello post");

    post.add_text("\nhenlo");

    let post = post.request_review();

    let post = post.approve();

    println!("{}", post.content());
}

fn main() {
    oop();
    idiomatic();

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: name @ 3..=7
        } => println!("wowza: {}", name),// println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
