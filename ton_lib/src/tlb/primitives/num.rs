use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;

macro_rules! tlb_num_impl {
    ($t:ty, $bits:tt) => {
        impl TLBType for $t {
            fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> { parser.read_num($bits) }

            fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
                builder.write_num(self, $bits)
            }
        }
    };
}

// BigNum doesn't have predefined len, so can't be implemented here
tlb_num_impl!(i8, 8);
tlb_num_impl!(i16, 16);
tlb_num_impl!(i32, 32);
tlb_num_impl!(i64, 64);
tlb_num_impl!(i128, 128);
tlb_num_impl!(u8, 8);
tlb_num_impl!(u16, 16);
tlb_num_impl!(u32, 32);
tlb_num_impl!(u64, 64);
tlb_num_impl!(u128, 128);
tlb_num_impl!(usize, 64);
