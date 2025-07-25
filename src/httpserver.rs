use std::{collections::BTreeMap, io::Error, net::{TcpListener, TcpStream, ToSocketAddrs}, sync::{Arc, Mutex}};

use crate::{executor::Executor, httpreader::HttpReader};


pub struct HttpServer<T : Executor>
{
    listener : Option<TcpListener>,
    handler : BTreeMap<String, HttpHandlerWrap>,
    started : bool,
    executor : T,
    max_client_timeout_ms : u64,
}

impl<T : Executor> HttpServer<T>

{
    
    pub fn new(max_client_timeout_ms : u64, executor : T) -> Self {
        let server : HttpServer<T> = HttpServer {
            listener : Option::None,
            handler : BTreeMap::new(),
            started : false,
            executor,
            max_client_timeout_ms,
        };
        return server;
    }
    
    pub fn add_endpoint<F>(&mut self, path : &str, handler : F) 
    where F : Sized + FnMut(HttpReader) + Send + Sync + 'static {
        let value = Arc::new(Mutex::new(handler));
        self.handler.insert(String::from(path), value);
    }
    
    pub fn init_listener<SA : ToSocketAddrs>(&mut self, socket_address : SA) -> Result<TcpListener, Error> {
        let listener = TcpListener::bind(socket_address);
        if listener.is_err() {
            return Result::Err(listener.unwrap_err());
        } else {
            let l = listener.unwrap();
            let result : Result<TcpListener, Error> = Result::Ok(l.try_clone().unwrap());
            self.listener = Option::Some(l);
            return result;
        }
    }
    
    pub fn start<SA : ToSocketAddrs>(&mut self, socket_address : SA) -> Result<bool, Error> {
        if self.listener.is_none() {
            let init = self.init_listener(socket_address);
            if init.is_err() {
                return Result::Err(init.unwrap_err());
            }
        }
        
        if !self.started {        
            self.started = true;
            let listener_copy : &TcpListener = self.listener.as_ref().unwrap();
            let max_client_timeout_ms = self.max_client_timeout_ms;
            let map_copy = self.handler.clone();
            for tcpstream in listener_copy.incoming() {
                let map_copy_copy = map_copy.clone();
                self.executor.execute_mut(move || {
                    if tcpstream.is_ok() {
                        HttpServer::<T>::handle_connection(tcpstream.unwrap(), map_copy_copy, max_client_timeout_ms);
                    }
                });
            }
        }
        
        return Result::Ok(true);
    }
    
    fn handle_connection(tcpstream : TcpStream, mut handler_map : BTreeMap<String, HttpHandlerWrap>, max_client_timeout_ms : u64) {
        let pre_check_http_reader = HttpReader::new(tcpstream, max_client_timeout_ms);
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