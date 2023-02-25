pub mod interrupt {

    #[derive(Debug)]
    pub struct Interrupt {
        pub master_enabled: u8, //ime
        pub enabled: u8, //ie
        pub flag: u8 //if
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