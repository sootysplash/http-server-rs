use std::{collections::BTreeMap, io::Error, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream}, sync::{Arc, Mutex}};

use crate::{connectionthreadpool::ThreadPool, httpreader::HttpReader};


pub struct HttpServer
{
    port_num : u16,
    listener : Option<TcpListener>,
    handler : BTreeMap<String, HttpHandlerWrap>,
    started : bool,
    threadpool : ThreadPool,
}

impl HttpServer
{
    
    pub fn new(port : u16, thread_count : usize) -> Self {
        let server : HttpServer = HttpServer {
            port_num : port,
            listener : Option::None,
            handler : BTreeMap::new(),
            started : false,
            threadpool : ThreadPool::new(thread_count),
        };
        return server;
    }
    
    pub fn add_endpoint<F>(&mut self, path : &str, handler : F) 
    where F : Sized + FnMut(HttpReader) + Send + Sync + 'static {
        let value = Arc::new(Mutex::new(handler));
        self.handler.insert(String::from(path), value);
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
        
        if !self.started {        
            self.started = true;
            let listener_copy : &TcpListener = self.listener.as_ref().unwrap();
            let map_copy = self.handler.clone();
            for tcpstream in listener_copy.incoming() {
                let map_copy_copy = map_copy.clone();
                self.threadpool.execute(move || {
                    if tcpstream.is_ok() {
                        HttpServer::handle_connection(tcpstream.unwrap(), map_copy_copy);
                    }
                });
            }
        }
        
        return Result::Ok(true);
    }
    
    fn handle_connection(tcpstream : TcpStream, mut handler_map : BTreeMap<String, HttpHandlerWrap>) {
        let pre_check_http_reader = HttpReader::new(tcpstream);
        if pre_check_http_reader.is_ok() {
            let http_reader = pre_check_http_reader.unwrap();
            let handler = handler_map.get_mut(&http_reader.get_request_path());
            if handler.is_none() {
                let fallback_key = &String::from("*");
                let fallback = handler_map.get_mut(fallback_key);
                if fallback.is_some() {
                    let mut unwrapped = fallback.unwrap().lock().unwrap();
                    (unwrapped)(http_reader);
                } else {
                    http_reader.respond_404();
                }
            } else {
                let mut unwrapped = handler.unwrap().lock().unwrap();
                (unwrapped)(http_reader);
            }
        }
    }
    
}

type HttpHandler = dyn FnMut(HttpReader) + Send + Sync;
type HttpHandlerWrap = Arc<Mutex<HttpHandler>>;