mod trait_object;

use trait_object::{Button, Select, Screen};
use chap17_oops::Post;

fn main() {

    let ui = Screen {
        components : vec![
            Box::new(
                Button {
                    width: 100,
                    height: 60,
                    label: String::from("Press it harder"),
                }
            ),
            Box::new(
                Select {
                    width: 200,
                    height: 50,
                    options: vec![]
                }
            )

        ]
    };

    ui.run();


    // --------------------

    let mut post = Post::new();

    post.add_text("yo just publish this");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("yo just publish this", post.content());
    post.request_review();
}
