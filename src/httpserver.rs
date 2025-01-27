use std::{collections::BTreeMap, io::Error, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}};

use crate::httpreader::HttpReader;

pub struct HttpServer<F>
where
    F: FnMut(HttpReader),
{
    port_num : u16,
    listener : Option<TcpListener>,
    handler : BTreeMap<String, F>,
}

impl<F> HttpServer<F>
where
    F: FnMut(HttpReader),
{
    
    pub fn new(port : u16, initial_path : &str, initial_handler : F) -> HttpServer<F> {
        let mut server : HttpServer<F> =  HttpServer {
            port_num : port,
            listener : Option::None,
            handler : BTreeMap::new(),
        };
        server.add_endpoint(initial_path, initial_handler);
        return server;
    }
    
    pub fn add_endpoint(&mut self, path : &str, handler : F) {
        self.handler.insert(String::from(path), handler);
    }
    
    pub fn init_listener(&mut self) -> Result<TcpListener, Error> {
        let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port_num));
        if listener.is_err() {
            return Result::Err(listener.unwrap_err());
        } else {
            let l = listener.unwrap();
            let result : Result<TcpListener, Error> = Result::Ok(l.try_clone().unwrap());
            self.listener = Option::Some(l);
            return result;
        }
    }
    
    pub fn start(&mut self) -> Result<bool, Error> {
        if self.listener.is_none() {
            let init = self.init_listener();
            if init.is_err() {
                return Result::Err(init.unwrap_err());
            }
        }
        
        for tcpstream in self.listener.as_ref().unwrap().incoming() {
            if tcpstream.is_ok() {
                let pre_check_http_reader = HttpReader::new(tcpstream.unwrap());
                if !pre_check_http_reader.is_ok() {
                    continue;
                }
                let http_reader = pre_check_http_reader.unwrap();
                let handler = self.handler.get_mut(&http_reader.get_request_path());
                if handler.is_none() {
                    let fallback_key = &String::from("*");
                    let fallback = self.handler.get_mut(fallback_key);
                    if fallback.is_some() {
                        let unwrapped = fallback.unwrap();
                        (unwrapped)(http_reader);
                    } else {
                        http_reader.respond_404();
                    }
                } else {
                    let unwrapped = handler.unwrap();
                    (unwrapped)(http_reader);
                }
                
            }
        }
        return Result::Ok(true); // should never be reached?
    }
    
}