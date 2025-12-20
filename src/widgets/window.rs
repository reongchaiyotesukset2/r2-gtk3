use gtk::{
    gio,
    glib::{self},   
    subclass::prelude::*,
    prelude::*,
};
  use gtk::subclass::prelude::DerivedObjectProperties;
  use std::cell::{Cell};

  use gtk::subclass::prelude::BinImpl;
 use crate::{
    widgets::{ProviderPage}
};

  use crate::models::{ProvidersModel};
  use crate::Application; 
  use crate::config;

mod imp {

  use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]    
    #[template(file = "../../ui/window.ui")]
    //#[template(resource = "/com/belmoussaoui/Authenticator/window.ui")]
    #[properties(wrapper_type = super::Window)]
 
  pub struct Window {
     #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
             #[template_child]
         pub page: TemplateChild<ProviderPage>,
    }
  
  
  #[glib::object_subclass]
   impl ObjectSubclass for Window {
   
         const NAME: &'static str = "Window";
         type Type = super::Window;
         type ParentType = gtk::ApplicationWindow;
          type Interfaces = (gio::Initable,);
         
          fn class_init(klass: &mut Self::Class) {
               Self::bind_template(klass);
            }
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }
            
   }

        impl WidgetImpl for Window {}
   
     #[glib::derived_properties]
    impl ObjectImpl for Window {
        fn constructed(&self) {
          
                     
        }
    
    }
   
 
        impl  WindowImpl for Window {}
        impl  ApplicationWindowImpl for Window {}
        impl  BinImpl for Window {}
        impl  ContainerImpl for Window {}
        impl BoxImpl for Window{}
        impl InitableImpl for Window {
             fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
               Ok(())
             }
        }
  
}


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::Initable, gio::ActionMap, gio::ActionGroup;
}


impl Window {
      pub fn new(model: &ProvidersModel, app: &Application) -> Self {
        gio::Initable::builder()
            .property("application", app)           
            .build(gio::Cancellable::NONE)
            .unwrap()
    }

 
    pub fn empty_status_page()
    {
    
    }
}
