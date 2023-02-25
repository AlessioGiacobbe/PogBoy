pub mod interrupt {

    #[derive(Debug)]
    pub struct Interrupt {
        pub(crate) master_enabled: u8, //ime
        pub(crate) enabled: u8, //ie
        pub(crate) flag: u8 //if
    }

    impl Interrupt {

    }
    
    impl Default for Interrupt {
        fn default() -> Interrupt {
            Interrupt {
                master_enabled: 0,
                enabled: 0,
                flag: 0
            }
        }
    }
}