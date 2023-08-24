mod imp;

use glib::Object;
use gtk::gio;
use gtk::glib;
use gtk::subclass::prelude::*;

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
