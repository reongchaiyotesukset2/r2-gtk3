mod window;
mod preferences;
mod providers;

pub use self::{
    window::Window,
    preferences::PreferencesWindow,
    //ProviderPage called with application.rs 
    //providers called with mod.rs go to file provider/page.rs
    providers::{ProviderPage},
    
};
