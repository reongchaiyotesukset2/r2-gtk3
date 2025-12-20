use crate::models::provider;
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
//#[cfg(test)]
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
        pub selected_provider: RefCell<Option<Provider>>,
        pub selected_image: RefCell<Option<gio::File>>,
        //#[template_child]
        //pub button3 : TemplateChild<gtk::Button>,
        //#[template_child]
        //pub page: TemplateChild<ProviderPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProviderPage {
        const NAME: &'static str = "ProviderPage";
        type Type = super::ProviderPage;
        // testing from gtk::Window run sussess
        // config gtk initiative to gtk::Window from gtkApplicationWindow
        type ParentType = gtk::ApplicationWindow;

        fn new() -> Self {
            
            Self {
                actions: gio::SimpleActionGroup::new(),
                is_locked : Cell::default(),
                selected_provider: RefCell::default(),
                selected_image: RefCell::default(),
                //button3 : TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
             Self::bind_template(klass);          
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    
   #[glib::derived_properties]
    impl ObjectImpl for ProviderPage {
    /*
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
*/
        fn constructed(&self) {
  
           self.parent_constructed();
             let app = self.obj();
         
                    let button3_action = gio::ActionEntry::builder("button3")
                   .activate(|app : &Self::Type, _, _| {  
                      app.save();
                    }).build();
                    
                       let button4_action = gio::ActionEntry::builder("page.button4")
                   .activate(|app : &Self::Type, _, _| {  
                      
                    }).build();
              
                  app.add_action_entries([
                      button3_action,
	                  button4_action
                    ]);  
            
        }
        
    }
   
    impl  WindowImpl for ProviderPage {} //add new
    impl  WidgetImpl for ProviderPage {}
    impl  ApplicationWindowImpl for ProviderPage {}  //add new
    impl  BinImpl for ProviderPage {} //add new
    impl  ContainerImpl for ProviderPage {} //add new
    
}

glib::wrapper! {
    pub struct ProviderPage(ObjectSubclass<imp::ProviderPage>)
    
    /*
      @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::Initable, gio::ActionMap, gio::ActionGroup;
    */
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::Initable,gtk::Buildable, gio::ActionGroup,gio::ActionMap;
}
 
impl ProviderPage {
   pub fn set_provider(&self, provider: Option<Provider>) {
      let imp = self.imp();
         imp.selected_provider.replace(Some(provider).expect("error!! replace provider"));
   }
   
    pub  fn delete_provider(&self)  {
          println!("provider fn");
        let imp = self.imp();
        if let Some(provider) = imp.selected_provider.borrow().as_ref() {
      
               //self.emit_by_name::<()>("deleted", &[provider]);
               println!("if11");
        } else {
           println!("else222");
        }
    }
    
      pub fn save(&self) {
      println!("save");
      }
    
}

impl Default for ProviderPage {
    fn default() -> Self {
        glib::Object::new()
    }
}
