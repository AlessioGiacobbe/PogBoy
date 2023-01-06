pub mod interrupt {

    pub struct Interrupt {
        pub(crate) enabled: bool
    }

    impl MMU {

    }
    
    impl Default for Interrupt {
        fn default() -> Interrupt {
            Interrupt {
                enabled: false
            }
        }
    }
}