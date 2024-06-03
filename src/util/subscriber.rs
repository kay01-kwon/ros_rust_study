extern crate rosrust;

pub fn run_subscriber() {
    rosrust::init("listener");
    let _subscriber_raii = rosrust::subscribe("chatter", 100, |v: rosrust_msg::std_msgs::String| {
        rosrust::ros_info!("I heard: {}", v.data);
    }).unwrap();
    rosrust::spin();
}