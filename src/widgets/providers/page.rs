use gtk::{
    gio,
    glib::{self},   
    subclass::prelude::*,
    prelude::*,
};
  use gtk::subclass::prelude::DerivedObjectProperties;
  use std::cell::{Cell};

  use gtk::subclass::prelude::BinImpl;



mod imp {

  use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]  
    #[template(file = "../../../ui/provider_page.ui")]

    #[properties(wrapper_type = super::ProviderPage)]
 
  pub struct ProviderPage {
     #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
     
    }
  
  
  #[glib::object_subclass]
   impl ObjectSubclass for ProviderPage {
   
         const NAME: &'static str = "ProviderPage";
         type Type = super::ProviderPage;
         type ParentType = gtk::ApplicationWindow;
          type Interfaces = (gio::Initable,);
         
          fn class_init(klass: &mut Self::Class) {
               Self::bind_template(klass);
            }
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }
            
   }

        impl WidgetImpl for ProviderPage {}
   
     #[glib::derived_properties]
    impl ObjectImpl for ProviderPage {
        fn constructed(&self) {
        }
    }
        impl  WindowImpl for ProviderPage {}
        impl  ApplicationWindowImpl for ProviderPage {}
        impl  BinImpl for ProviderPage {}
        impl  ContainerImpl for ProviderPage {}
        impl BoxImpl for ProviderPage{}
        impl InitableImpl for ProviderPage {
             fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
               Ok(())
             }
        }
  
}

glib::wrapper! {
    pub struct ProviderPage(ObjectSubclass<imp::ProviderPage>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::Initable,gtk::Buildable, gio::ActionGroup,gio::ActionMap;
}
impl ProviderPage {
 
    pub fn empty_status_page()
    {
    }
}

impl Default for ProviderPage {
    fn default() -> Self {
        glib::Object::new()
    }
}
