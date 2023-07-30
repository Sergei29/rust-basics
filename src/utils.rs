use std::any::type_name;

pub fn divider() {
    println!(
        "\n********************************************************************************\n"
    );
}

pub fn type_of<T>(_value: T) -> &'static str {
    return type_name::<T>();
}
