use std::{thread, time::Duration};

use connectionthreadpool::ThreadPool;
use httpconstants::HttpConstants;
use httpserver::HttpServer;

pub mod connectionthreadpool;
pub mod httpreader;
pub mod httpconstants;
pub mod httpserver;
pub mod httperror;
pub mod executor;

fn main() {
    println!("{:?}", HttpConstants::get_current_formatted_date());
    
    let mut http_server : HttpServer<ThreadPool> = HttpServer::new(9000, 10000, ThreadPool::new(16));
    
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
