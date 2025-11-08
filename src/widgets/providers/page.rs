use gtk::{
    gio,
    glib::{self,clone},   
    subclass::prelude::*,
    glib::Properties,
    prelude::*,
};
use crate::{
    models::{Provider},
    
};

mod imp {
    use std::{cell::Cell,cell::RefCell, sync::LazyLock};

    use glib::subclass::Signal;

    use super::*;


    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]    
    #[template(file = "../../../ui/provider_page.ui")]
    #[properties(wrapper_type = super::ProviderPage)]
 
    pub struct ProviderPage {
        pub actions: gio::SimpleActionGroup,
        
    
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>, 
        #[template_child]
        pub delete_button: TemplateChild<gtk::Button>,
        pub selected_provider: RefCell<Option<Provider>>,
        pub selected_image: RefCell<Option<gio::File>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProviderPage {
        const NAME: &'static str = "ProviderPage";
        type Type = super::ProviderPage;
        type ParentType = gtk::ApplicationWindow;

        fn new() -> Self {
            
            Self {
                actions: gio::SimpleActionGroup::new(),
            
               is_locked : Cell::default(),
                delete_button: TemplateChild::default(),
     
                selected_provider: RefCell::default(),
                selected_image: RefCell::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
             Self::bind_template(klass);          
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProviderPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
                vec![
               
                    Signal::builder("deleted")
                        .param_types([Provider::static_type()])
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn constructed(&self) {
            let imp = self.obj();
            imp.delete_provider();
        }
    }
   
    impl  WindowImpl for ProviderPage {} //add new
    impl WidgetImpl for ProviderPage {}
    impl ApplicationWindowImpl for ProviderPage {}  //add new
    impl  BinImpl for ProviderPage {} //add new
    impl  ContainerImpl for ProviderPage {} //add new
    
}

glib::wrapper! {
    pub struct ProviderPage(ObjectSubclass<imp::ProviderPage>)
        @extends gtk::Widget,gtk::Window,
        @implements gtk::Buildable;
}


impl ProviderPage {


    
     fn delete_provider(&self)  {
        let imp = self.imp();
        if let Some(provider) = imp.selected_provider.borrow().as_ref() {
            println!("if");
        } else {
           println!("else");
        }
       
    }

}

impl Default for ProviderPage {
    fn default() -> Self {
        glib::Object::new()
    }
}
