use derivative::Derivative;
use http;
mod wasmedge_connect;
pub mod handle_func;

#[derive(Debug,Derivative)]
#[derivative( Default)]
pub struct WaafFunction {
    #[derivative(Default(value = "http::Method::GET"))]
    pub method: http::Method,
    #[derivative(Default(value = "1234.to_string()"))]
    pub port: String,
    pub handle_func: handle_func::HandleFunc,
}

impl WaafFunction {
    pub  fn start(self) {
        match  wasmedge_connect::start_socket(self){
            Ok(_) => {},
            Err(e) => {eprintln!("Error: {}", e)}
        };
    }
}


#[cfg(test)]
mod test;