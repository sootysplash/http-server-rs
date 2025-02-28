use std::{collections::BTreeMap, io::{Error, Read, Write}, net::{SocketAddr, TcpStream}, time::Duration};
use crate::httperror::*;

use crate::{httpconstants::HttpConstants, httperror::HttpErrorWrapper};

pub struct HttpReader {
    req_method : String,
    req_path : String,
    req_protocol : String,
    req_headers : BTreeMap<String, String>,
    req_lines : BTreeMap<i32, String>,
    req_body : BTreeMap<i32, u8>,
    response_headers : BTreeMap<String, String>,
    sent_headers : bool,
    tcpstream : TcpStream,
    max_client_timeout_ms : u64,
}

#[allow(dead_code)]
impl HttpReader {
    pub fn new(input : TcpStream, max_client_timeout_ms : u64) -> Result<HttpReader, HttpErrorWrapper> {
        let mut this = HttpReader {
            req_method: String::from(""),
            req_path : String::from(""),
            req_protocol : String::from(""),
            req_headers : BTreeMap::new(),
            req_lines : BTreeMap::new(),
            req_body : BTreeMap::new(),
            response_headers : BTreeMap::new(),
            sent_headers : false,
            tcpstream : input,
            max_client_timeout_ms,
        };
        let result = this.read_lines();
        if result.is_some() {
            return Result::Err(result.unwrap());
        } else {
            return Result::Ok(this);   
        }

    }

    fn read_lines(&mut self) -> Option<HttpErrorWrapper> {
        
        let mut had_first_line = false;
        let mut current_str : Vec<u8> = Vec::new();
        let mut index = 0;
        let mut buf = [0u8];
        let mut stream = &self.tcpstream;
        stream.set_read_timeout(Some(Duration::from_millis(self.max_client_timeout_ms))).unwrap();
        loop {

            let result = stream.read(&mut buf);
            if result.is_err() {
                return Option::Some(INCOMPLETE_REQUEST.to_wrapped_respond(stream));
            } else if result.unwrap() == 0 {
                return Option::Some(CLIENT_TIMEOUT.to_wrapped_respond(stream));
            }

            let character = buf[0];
            current_str.push(character);
            if character as char != '\n' {
                continue;
            }

            if current_str.len() == 2 {// empty line after http headers or before first line
                current_str.clear();
                if !had_first_line {
                    continue;
                } else {
                    break;
                }
            } else {
                let str = String::from_utf8_lossy(current_str.clone().as_slice()).to_string();
                self.req_lines.insert(index, str);
                index = index + 1;
                current_str.clear();
                had_first_line = true;
            }
            
        }
        
        let mut start_line_iterator = self.req_lines.get(&0).unwrap().split_whitespace();
        let mut had_request_error = false;
        let method = start_line_iterator.next();
        if method.is_none() {
            had_request_error = true;
        } else {
            self.req_method = method.unwrap().to_string();
        }
        let path = start_line_iterator.next();
        if path.is_none() {
            had_request_error = true;
        } else {
            self.req_path = path.unwrap().to_string();
        }
        let protocol = start_line_iterator.next();
        if protocol.is_none() {
            had_request_error = true;
        } else {
            self.req_protocol = protocol.unwrap().to_string();
        }
        if had_request_error {
            return Option::Some(BAD_REQUEST_LINE.to_wrapped_respond(stream));
        }

        let mut i = 1;
        let len = self.req_lines.len();
        while i < len {
            let unsafe_header = self.req_lines.get(&(i as i32))
                .unwrap()
                .split_once(": ");
            if unsafe_header.is_none() {
                return Option::Some(INCORRECT_HEADER_FORMAT.to_wrapped_respond(stream));
            }
            let header = unsafe_header
                .unwrap();
            
            let key = header.0;
            
            let value = header.1;
            let value = value.split_at(value.len() - 2).0; // remove newline
            
            self.req_headers.insert(key.to_string(), value.to_string());
            i += 1;
        }        
        
        let content_length = self.req_headers.get("Content-Length");
        if content_length.is_some() {
            let unwrapped_length = content_length.unwrap();
            let parsed = usize::from_str_radix(unwrapped_length, 10);
            if parsed.is_err() { // this checks for negative values as well
                return Option::Some(INCORRECT_CONTENT_LENGTH.to_wrapped_respond(stream));
            }
            let amount_to_read = parsed.unwrap();
            let mut buf: Vec<u8> = Vec::new();
            buf.resize(amount_to_read, 0);
            let buf = buf.as_mut_slice();
            
            let result = stream.read(buf);
            
            if result.is_err() || result.unwrap() != amount_to_read {
                return Option::Some(INCOMPLETE_BODY.to_wrapped_respond(stream));
            }
            
            for i in 0..amount_to_read  {
                self.req_body.insert(i as i32, *buf.get(i).unwrap());
            }
            
        }
        
        let _body = String::from_utf8_lossy(self.get_request_body().as_slice()).to_string();
        
        return Option::None;
        
    }
    
    pub fn get_stream(&mut self) -> &mut TcpStream {
        return &mut self.tcpstream;
    }

    pub fn get_peer_address(&self) -> Result<SocketAddr, Error> {
        return self.tcpstream.peer_addr();
    }
    
    pub fn get_local_address(&self) -> Result<SocketAddr, Error> {
        return self.tcpstream.local_addr();
    }
    
    pub fn respond_404(&self) {
        ENDPOINT_NOT_FOUND.to_wrapped_respond(&self.tcpstream);
    }
    
    // request
    pub fn get_request_method(&self) -> String {
        return self.req_method.to_string();
    }

    pub fn get_request_path(&self) -> String {
        return self.req_path.to_string();
    }

    pub fn get_request_protocol(&self) -> String {
        return self.req_protocol.to_string();
    }

    pub fn get_request_headers(&mut self) -> &mut BTreeMap<String, String> {
        return &mut self.req_headers;
    }
    
    pub fn get_request_body(&self) -> Vec<u8> {
        return self.req_body.values().cloned().collect();
    }
    
    // response
    pub fn get_response_headers(&mut self) -> &mut BTreeMap<String, String> {
        return &mut self.response_headers;
    }
    
    pub fn add_cross_origin_resource_sharing_headers(&mut self) {
        let response = self.get_response_headers();
        response.insert(String::from("Access-Control-Allow-Origin"), String::from("*"));
        response.insert(String::from("Access-Control-Allow-Headers"), String::from("Origin, X-Requested-With, Content-Type, Accept, Authorization"));
        response.insert(String::from("Access-Control-Allow-Methods"), String::from("GET, OPTIONS, HEAD, PUT, POST"));
        response.insert(String::from("Access-Control-Allow-Credentials"), String::from("true"));
    }
    
    pub fn write_response_headers(&mut self, response_code : i32, body_length : usize) -> Result<usize, Error> {
        
        if self.sent_headers {
            return Ok(0);
        }
        
        let start_line = "HTTP/1.1 ".to_owned() + response_code.to_string().as_str() + HttpConstants::get_code_text(response_code) + "\r\n";
        
        let response_headers = self.get_response_headers();
        response_headers.insert("Content-Length".to_string(), body_length.to_string());
        response_headers.insert("Date".to_string(), HttpConstants::get_current_formatted_date());
        
        let mut response_bytes : Vec<u8> = Vec::new();
        response_bytes.append(&mut start_line.clone().into_bytes());
        for entry in response_headers {
            response_bytes.append(&mut entry.0.clone().into_bytes());
            response_bytes.append(&mut ": ".to_string().into_bytes());
            response_bytes.append(&mut entry.1.clone().into_bytes());
            response_bytes.append(&mut "\r\n".to_string().into_bytes());
        }
        response_bytes.append(&mut "\r\n".to_string().into_bytes());
        
        let result = self.tcpstream.write(response_bytes.as_mut_slice());
        
        self.sent_headers = true;
        
        return result;
    }
    
    pub fn write_response_body(&mut self, body : &[u8]) -> Result<usize, Error> {
        return self.tcpstream.write(body);
    }
    
}
