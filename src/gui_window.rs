use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::gio;
use gtk::glib;

use std::cell::RefCell;

use crate::customer::Customer;
use crate::customer_button::CustomerButton;
use crate::shop::Shop;

glib::wrapper! {
    pub struct GuiWindow(ObjectSubclass<imp::GuiWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
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
        pub queue_container: gtk::TemplateChild<gtk::Box>,

        #[template_child]
        pub search_entry: gtk::TemplateChild<gtk::Entry>,

        pub shop: RefCell<Shop>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GuiWindow {
        const NAME: &'static str = "GuiWindow";
        type Type = super::GuiWindow;
        type ParentType = adw::ApplicationWindow;

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
                queue_box.append(&CustomerButton::new(Some(customer.clone())));
            }

            for _ in 0..empty_spaces {
                queue_box.append(&CustomerButton::new(None));
            }

            queue_box
        }
    }

    impl ObjectImpl for GuiWindow {}

    impl WidgetImpl for GuiWindow {}

    impl WindowImpl for GuiWindow {}

    impl ApplicationWindowImpl for GuiWindow {}

    impl AdwApplicationWindowImpl for GuiWindow {}
}
