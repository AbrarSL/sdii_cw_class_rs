use shop::Shop;
use text_interface::TextInterface;

mod customer;
mod food_queue;
mod shop;
mod text_interface;

fn main() {
    let mut text_interface = TextInterface::new(Shop::new(&[2, 3, 5]));
    text_interface.run();
}
