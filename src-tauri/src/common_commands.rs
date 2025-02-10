#[taurpc::procedures(event_trigger = ApiEventTrigger, path = "common")]
trait Common {
    async fn setHomeserverUrl();
}

#[derive(Clone)]
pub struct CommonImpl;

#[taurpc::resolvers]
impl Common for CommonImpl {
    async fn setHomeserverUrl(self) {
        println!("Hello world");
    }
}