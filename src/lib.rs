use ext_php_rs::prelude::*;
use ext_php_rs::{info_table_end, info_table_row, info_table_start, zend::ModuleEntry};
use php_tokio::{php_async_impl, EventLoop};

#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello {} from rust!", name)
}

#[php_class]
struct Ohp;

#[php_async_impl]
impl Ohp {
    pub fn init() -> PhpResult<u64> {
        EventLoop::init()
    }
}

pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("Oxidised Home Page", "enabled");
    info_table_row!("OHP Version", env!("CARGO_PKG_VERSION"));
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.info_function(php_module_info)
}

#[cfg(test)]
mod tests {
    use ext_php_rs::{embed::Embed, ffi::zend_register_module_ex};

    use crate::get_module;

    #[test]
    fn dummy_test() {
        Embed::run(|| {
            unsafe { zend_register_module_ex(get_module()) };
            let res = Embed::eval("$foo = hello_world('HIM');");
            assert!(res.is_ok());
            let zval = res.unwrap();
            assert!(zval.is_string());
            let zval = zval.string().unwrap();
            assert_eq!(zval, "Hello HIM from rust!")
        });
    }
}
