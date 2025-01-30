use std::{thread, time::Duration};

use httpconstants::HttpConstants;
use httpserver::HttpServer;

mod connectionthreadpool;
mod httpreader;
mod httpconstants;
mod httpserver;
mod httperror;

fn main() {
    println!("{:?}", HttpConstants::get_formatted_date());
    
    let mut http_server : HttpServer = HttpServer::new(9000, 4);
    
    http_server.add_endpoint("/", | mut reader | {
        let mut vec_response : Vec<u8> = Vec::new();
        vec_response.append(&mut "<h1>Hi from http</h1>this thing works".to_string().into_bytes());
        reader.write_response_headers(200, vec_response.len()).unwrap_or_default();
        reader.write_response_body(vec_response.as_slice()).unwrap_or_default(); 
    });
    
    http_server.add_endpoint("/sleep", | mut reader | {
        thread::sleep(Duration::from_secs(5));
        let mut vec_response : Vec<u8> = Vec::new();
        vec_response.append(&mut "<h1>Hi from http</h1>i woke up from my sleep".to_string().into_bytes());
        reader.write_response_headers(200, vec_response.len()).unwrap_or_default();
        reader.write_response_body(vec_response.as_slice()).unwrap_or_default(); 
    });
    
    http_server.start().unwrap();
    
}
