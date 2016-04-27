extern crate weblog;

fn main() {
    // Example line here
    let example = "127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] \"GET /apache_pb.gif HTTP/1.0\" 200 2326 \"http://www.example.com/start.html\" \"Mozilla/4.08 [en] (Win98; I ;Nav)\"";
    let v = weblog::parser::parse_weblog(example);
    println!("It's a weblog line! {}", v);
}
