mod it;

fn main() {
    let router = it::Router { ip: "192.168.1.100".to_string() };

    router.ping("google.com");
    let trace = router.traceroute("google.com");
    println!("Trace route: {:?}", trace);
    let ip = router.nslookup("google.com");
    println!("DNS lookup: {}", ip);
}
