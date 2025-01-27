use std::{io::Write, net::TcpStream};
use crate::httpconstants::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct HttpErrorWrapper {
    possible_http_error : Option<HttpError>,
}

#[allow(dead_code)]
impl HttpErrorWrapper {    
    pub fn get_http_error(self) -> Option<HttpError> {
        return self.possible_http_error;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HttpError {
    error_type : HttpErrorType,
    code : i32,
    message : &'static str,
}

impl HttpError {
    const fn new(error_type : HttpErrorType, code : i32, message : &'static str) -> HttpError {
        return HttpError {error_type, code, message};
    }
    
    pub fn to_wrapped(self) -> HttpErrorWrapper {
        return HttpErrorWrapper {possible_http_error : Option::Some(self)};
    }
    
    pub fn to_wrapped_respond(self, mut tcpstream : &TcpStream) -> HttpErrorWrapper {
        let line = self.code.to_string() + HttpConstants::get_code_text(self.code);
        let error_msg = "<h1>".to_string() + &line + "</h1>" + self.message;
        
        let mut response_bytes : Vec<u8> = Vec::new();
        
        response_bytes.append(
            &mut ("HTTP/1.1 ".to_string() + 
            line.as_str() + 
            "\r\n")
            .into_bytes());
        
        response_bytes.append(
            &mut (
            "Content-Length: ".to_string() +
            error_msg.len().to_string().as_str() +
            "\r\n" +
            "Content-Type: text/html\r\n" + 
            "Connection: close\r\n" +
            "\r\n" +
            &error_msg
            ).into_bytes());
        
        tcpstream.write(response_bytes.as_slice()).unwrap_or_default();
                
        return self.to_wrapped();
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(deprecated)]
#[non_exhaustive]
pub enum HttpErrorType {
    BadRequestLine,
    IncompleteHttpRequest,
    IncompleteHttpBody,
    IncorrectContentLength,
    IncorrectHeaderFormat,
    EndpointNotFound,
}

pub const BAD_REQUEST_LINE : HttpError = HttpError::new(HttpErrorType::BadRequestLine, HTTP_BAD_REQUEST, "Bad Request Line");
pub const INCOMPLETE_REQUEST : HttpError = HttpError::new(HttpErrorType::IncompleteHttpRequest, HTTP_BAD_REQUEST, "Incomplete Request");
pub const INCOMPLETE_BODY : HttpError = HttpError::new(HttpErrorType::IncompleteHttpBody, HTTP_BAD_REQUEST, "Incomplete Body");
pub const INCORRECT_CONTENT_LENGTH : HttpError = HttpError::new(HttpErrorType::IncorrectContentLength, HTTP_BAD_REQUEST, "Incorrect Content Length");
pub const INCORRECT_HEADER_FORMAT : HttpError = HttpError::new(HttpErrorType::IncorrectHeaderFormat, HTTP_BAD_REQUEST, "Incorrect Header Format");
pub const ENDPOINT_NOT_FOUND : HttpError = HttpError::new(HttpErrorType::EndpointNotFound, HTTP_NOT_FOUND, "Endpoint Not Found");