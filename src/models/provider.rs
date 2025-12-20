use gtk::{
    gdk_pixbuf, gio,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};
mod imp {
    use std::cell::{Cell, RefCell};
    use super::*;

  // #[derive(glib::Properties)]
    //#[properties(wrapper_type = super::Provider)]
    pub struct Provider {
       //#[property(get, set, construct)]
     pub   id : Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Provider {
        const NAME: &'static str = "Provider";
        type Type = super::Provider;
        fn new()->Self{
            Self {
              id : Cell::default(),
            }
        }
    }

    impl ObjectImpl for Provider {
        
    }
}

glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>);
}

impl Provider{

    pub fn new(id :u8)->Provider{
       glib::Object::builder()
            .property("id", id)
            .build()
    }

}