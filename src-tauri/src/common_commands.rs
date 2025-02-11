// making the exported binding outside of src-tauri, otherwise tauri dev will make infinite loop
#[taurpc::procedures(event_trigger = ApiEventTrigger, path = "common", export_to = "../bindings/bindings.ts")]
pub trait Common {
    async fn set_homeserver_url();
}

#[derive(Clone)]
pub struct CommonImpl;

#[taurpc::resolvers]
impl Common for CommonImpl {
    async fn set_homeserver_url(self) {
        println!("Hello world");
    }
}