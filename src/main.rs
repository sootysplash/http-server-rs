use httpconstants::HttpConstants;
use httpserver::HttpServer;

mod httpreader;
mod httpconstants;
mod httpserver;
mod httperror;

fn main() {
    println!("{:?}", HttpConstants::get_formatted_date());
    
    let mut http_server : HttpServer<_> = HttpServer::new(9000, "/", | mut reader | {
        let mut vec_response : Vec<u8> = Vec::new();
        vec_response.append(&mut "<h1>Hi from http</h1>this thing works".to_string().into_bytes());
        reader.write_response_headers(200, vec_response.len());
        reader.write_response_body(vec_response.as_slice()); 
    });
    http_server.start().unwrap();
}
