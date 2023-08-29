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

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(file = "resources/queue_box.blp")]
    pub struct QueueBox;

    #[glib::object_subclass]
    impl ObjectSubclass for QueueBox {
        const NAME: &'static str = "QueueBox";
        type Type = super::QueueBox;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for QueueBox {}

    impl WidgetImpl for QueueBox {}

    impl BoxImpl for QueueBox {}
}
