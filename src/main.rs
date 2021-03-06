extern crate hyper;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;
use hyper::uri::RequestUri;

fn hello(req: Request, res: Response<Fresh>) {
    let mut res = res.start().unwrap();

    let path = match req.uri {
      RequestUri::AbsolutePath(str) => str,
      _ => "".to_string()
    };

    println!("path: {}", path);
    println!("remoteAddy: {}", req.remote_addr);
    println!("method: {}", req.method);
    
    let compound = format!("{} request for {} from {}", req.method, path, req.remote_addr);

    res.write_all(compound.as_bytes()).unwrap();
    res.end().unwrap();
}

fn main() {
    Server::http(hello).listen("127.0.0.1:3000").unwrap();
}
