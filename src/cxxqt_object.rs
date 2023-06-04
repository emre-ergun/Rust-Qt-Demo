#[cxx_qt::bridge]
mod my_object {
    #[cxx_qt::qobject(qml_uri = "demo", qml_version = "1.0")]
    #[derive(Default)]
    pub struct Hello {}

    impl qobject::Hello {
        #[qinvokable]
        pub fn say_hello(&self) {
            static mut LEVELS: u32 = 0;
            unsafe {
                println!("Hello World!: {}", LEVELS);
                LEVELS += 1;
            }
        }
    }
}