pub mod interrupt {

    pub struct Interrupt {
        pub(crate) enabled: bool
    }

    impl Interrupt {

    }
    
    impl Default for Interrupt {
        fn default() -> Interrupt {
            Interrupt {
                enabled: false
            }
        }
    }
}