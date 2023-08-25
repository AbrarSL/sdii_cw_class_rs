use glib::Object;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use std::cell::RefCell;

use crate::customer::Customer;
use crate::shop::Shop;

glib::wrapper! {
    pub struct GuiWindow(ObjectSubclass<imp::GuiWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GuiWindow {
    pub fn new(shop: Shop) -> Self {
        let obj: Self = Object::builder().build();
        let imp = obj.imp();

        imp.shop.replace(shop);
        imp.initialize_queues();

        obj
    }
}

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(file = "resources/gui_window.blp")]
    pub struct GuiWindow {
        #[template_child]
        queue_container: gtk::TemplateChild<gtk::Box>,
        pub shop: RefCell<Shop>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GuiWindow {
        const NAME: &'static str = "GuiWindow";
        type Type = super::GuiWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl GuiWindow {
        pub fn initialize_queues(&self) {
            let shop = self.shop.borrow();

            for queue in shop.view_data() {
                let empty_spaces = queue.capacity() - queue.len();
                let queue_box = Self::construct_queue_box(queue.view_data(), empty_spaces);
                self.queue_container.append(&queue_box);
            }
        }

        fn construct_queue_box(customers: &[Customer], empty_spaces: usize) -> gtk::Box {
            let queue_box = gtk::Box::builder()
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .spacing(6)
                .orientation(gtk::Orientation::Vertical)
                .build();

            for customer in customers {
                queue_box.append(&Self::construct_customer_button(Some(customer)));
            }

            for _ in 0..empty_spaces {
                queue_box.append(&Self::construct_customer_button(None))
            }

            queue_box
        }

        fn construct_customer_button(customer: Option<&Customer>) -> gtk::Button {
            gtk::Button::builder()
                .label(if customer.is_some() { "O" } else { "X" })
                .css_classes(if customer.is_some() {
                    ["suggested-action"]
                } else {
                    ["destructive-action"]
                })
                .margin_start(6)
                .margin_end(6)
                .build()
        }
    }

    impl ObjectImpl for GuiWindow {}

    impl WidgetImpl for GuiWindow {}

    impl WindowImpl for GuiWindow {}

    impl ApplicationWindowImpl for GuiWindow {}
}
