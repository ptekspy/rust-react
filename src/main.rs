mod elements;
mod traits;

use elements::button::Button;
use elements::children::Children;
use elements::div::Div;
use elements::text::Text;
use traits::child::Child;

fn main() {
    let children = Children::new()
        .push(Text {
            child: String::from("Hello "),
        })
        .push(Button {
            child: Text {
                child: String::from("Click me"),
            },
        });

    let page = Div { children };

    println!("{}", page.render());
}