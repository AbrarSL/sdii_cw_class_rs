use shop::Shop;
use text_interface::TextInterface;

mod customer;
mod customer_button;
mod food_queue;
mod gui_window;
mod shop;
mod text_interface;

fn main() {
    TextInterface::new(Shop::new(&[2, 3, 5])).run();
}
