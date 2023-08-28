use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use std::cell::RefCell;

use crate::customer::Customer;

glib::wrapper! {
    pub struct CustomerButton(ObjectSubclass<imp::CustomerButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomerButton {
    pub fn new(customer: Option<Customer>) -> Self {
        let button: CustomerButton = glib::Object::builder().build();
        let imp = button.imp();

        button.set_label(if customer.is_some() { "O" } else { "X" });
        button.set_css_classes(if customer.is_some() {
            &["suggested-action"]
        } else {
            &["destructive-action"]
        });
        button.set_sensitive(customer.is_some());
        button.set_margin_start(6);
        button.set_margin_end(6);

        if let Some(customer) = customer {
            imp.customer.borrow_mut().replace(customer);
        }

        button
    }
}

mod imp {
    use super::*;

    #[derive(Default, Debug)]
    pub struct CustomerButton {
        pub customer: RefCell<Option<Customer>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomerButton {
        const NAME: &'static str = "CustomerButton";
        type Type = super::CustomerButton;
        type ParentType = gtk::Button;
    }

    impl ObjectImpl for CustomerButton {}

    impl WidgetImpl for CustomerButton {}

    impl ButtonImpl for CustomerButton {
        fn clicked(&self) {
            let customer = self.customer.borrow();

            if let Some(customer) = customer.as_ref() {
                let close_button = gtk::Button::builder()
                    .label("Close")
                    .css_classes(["destructive-action"])
                    .build();

                let dialog = adw::MessageDialog::builder()
                    .heading("Customer")
                    .body(format!(
                        "Name: {}\nItems: {}",
                        customer.full_name(),
                        customer.no_items()
                    ))
                    .extra_child(&close_button)
                    .transient_for(
                        self.obj()
                            .root()
                            .unwrap()
                            .downcast_ref::<gtk::ApplicationWindow>()
                            .unwrap(),
                    )
                    .build();

                close_button.connect_clicked(glib::clone!(@weak dialog => move |_| dialog.close()));

                dialog.present();
            }
        }
    }
}
