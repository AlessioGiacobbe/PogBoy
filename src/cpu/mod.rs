mod registers;

pub mod CPU{
    use crate::cpu::registers::Registers::Registers;
    use crate::decoder::decoder::Decoder;

    pub struct CPU {
        pub(crate) Registers: Registers,
        pub(crate) Decoder: Decoder,
    }

    impl CPU {
        pub(crate) fn new() -> CPU {
            let mut Registers: Registers = Registers::new();
            let Decoder: Decoder = Decoder::new();

            CPU {
                Registers,
                Decoder
            }
        }
    }
}