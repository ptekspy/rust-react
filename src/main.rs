mod elements;
mod traits;
mod values;
mod globals;

use elements::attributes::{Attribute, Attributes};

fn main() {
    let attributes = Attributes::new()
        .push(Attribute::new("href", "https://example.com"))
        .push(Attribute::new("target", "_blank"));

    println!("{}", attributes.render());
}