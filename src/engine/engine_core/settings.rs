
pub struct Settings {
    pub app_name: String,
}

impl Settings{
    
   pub(crate) fn default() -> Self {
       Self{
           app_name: String::from("UwU"),
       }
   }
}
