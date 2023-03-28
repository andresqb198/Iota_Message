use iota_streams::app_channels::api::tangle::{Author, Subscriber};
use iota_streams::app::transport::tangle::PAYLOAD_BYTES;
use iota_streams::app::transport::tangle::client::Client;

fn main() {
    let node = "http://localhost:14265";
    let client = Client::new_from_url(node);

    let encoding = "utf-8";
    let multi_branching_flag = true;

    let mut author = Author::new("AUTHORSSEED", encoding, PAYLOAD_BYTES, multi_branching_flag, client);
    
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRETSTRING", encoding, PAYLOAD_BYTES, client);
}