use probe::{class::Instance, JavaClass};

#[derive(JavaClass)]
#[package(net.minecraft.text)]
pub struct Text {
    #[instance]
    pub raw: Instance
}

#[probe::import]
impl Text {
    pub fn of(text: String) -> Self;
}
