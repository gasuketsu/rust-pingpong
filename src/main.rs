mod pingpong;

fn main() {
    let mut s = Box::new(pingpong::PingState) as Box<pingpong::State>;
    for _i in 0..10 {
        println!("{:?}", s.current_state());
        s = s.next_state();
    }
}
