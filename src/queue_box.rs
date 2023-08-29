use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::customer_button::CustomerButton;
use crate::food_queue::FoodQueue;

glib::wrapper! {
    pub struct QueueBox(ObjectSubclass<imp::QueueBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl QueueBox {
    pub fn new(queue: &FoodQueue) -> Self {
        let queue_box: QueueBox = glib::Object::builder().build();

        queue_box.set_margin_top(12);
        queue_box.set_margin_bottom(12);
        queue_box.set_margin_start(12);
        queue_box.set_margin_end(12);
        queue_box.set_spacing(6);
        queue_box.set_orientation(gtk::Orientation::Vertical);

        let empty_spaces = queue.capacity() - queue.len();

        for customer in queue.view_data() {
            queue_box.append(&CustomerButton::new(Some(customer.clone())));
        }

        for _ in 0..empty_spaces {
            queue_box.append(&CustomerButton::new(None));
        }

        queue_box
    }
}

mod imp {
    use super::*;

    #[derive(Default, Debug)]
    pub struct QueueBox;

    #[glib::object_subclass]
    impl ObjectSubclass for QueueBox {
        const NAME: &'static str = "QueueBox";
        type Type = super::QueueBox;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for QueueBox {}

    impl WidgetImpl for QueueBox {}

    impl BoxImpl for QueueBox {}
}
