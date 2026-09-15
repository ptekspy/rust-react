mod elements;
mod traits;
mod values;

use elements::attributes::Attribute;

fn main() {
    let href = Attribute::new("href", "https://example.com");

    println!("{}", href.render());
}