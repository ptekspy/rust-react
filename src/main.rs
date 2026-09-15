mod elements;
mod traits;

use elements::div::Div;
use elements::text::Text;
use traits::child::Child;

use crate::elements::a::A;

fn main() {
    let page = Div {
        child: Div {
            child: A {
                href: String::from("https://example.com"),
                child: Text {
                    child: String::from("Click me"),
                },
            },
        },
    };

    println!("{}", page.render());
}
