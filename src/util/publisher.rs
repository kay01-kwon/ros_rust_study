extern crate rosrust;

pub fn run_publisher() {
    rosrust::init("talker");
    let chatter_pub = rosrust::publish("chatter", 100).unwrap();
    let rate = rosrust::rate(10.0);
    let mut count = 0;
    while rosrust::is_ok() {
        let msg = rosrust_msg::std_msgs::String {
            data: format!("Hello, world! {}", count),
        };
        chatter_pub.send(msg).unwrap();
        rate.sleep();
        count += 1;
    }
}