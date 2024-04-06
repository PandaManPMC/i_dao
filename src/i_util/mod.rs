use log::debug;

pub fn print_type_of<T>(_: &T) {
    debug!("print_type_of 类型是 = {}", std::any::type_name::<T>())
}
