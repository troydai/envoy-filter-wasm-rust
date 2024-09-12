use log::info;
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::{LogLevel, Action};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(SimpleFilter)
    });
}}

struct SimpleFilter;

impl Context for SimpleFilter {}

impl RootContext for SimpleFilter {
    fn get_type(&self) -> Option<proxy_wasm::types::ContextType> {
        Some(proxy_wasm::types::ContextType::HttpContext)
    }

    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        info!("Simple filter on start");
        true
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        info!("Simple filter create http context");
        Some(Box::new(SimpleHttpContext))
    }
}

struct SimpleHttpContext;

impl Context for SimpleHttpContext {}

impl HttpContext for SimpleHttpContext {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        for header in self.get_http_request_headers() {
            if header.0 == "x-request-id" {
                self.set_http_request_header("x-test", Some(header.1.as_str()));
            }
        }
        
        Action::Continue
    }
}