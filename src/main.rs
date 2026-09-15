mod elements;
mod traits;
mod values;

use elements::button::Button;
use elements::children::Children;
use elements::div::Div;
use values::text::Text;
use traits::child::Child;

fn main() {
    let children = Children::new()
        .push(Text {
            string: String::from("Hello "),
        })
        .push(Button {
            children: Children::new().push(Text {
                string: String::from("Click me"),
            }),
        });

    let page = Div { children };

    println!("{}", page.render());
}