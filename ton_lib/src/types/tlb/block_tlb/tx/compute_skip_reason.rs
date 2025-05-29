use ton_lib_macros::TLBDerive;

#[derive(Clone, Debug, PartialEq, TLBDerive)]
pub enum ComputeSkipReason {
    NoState(ComputeSkipReasonNoState),
    BadState(ComputeSkipReasonBadState),
    NoGas(ComputeSkipReasonNoGas),
    Suspended(ComputeSkipReasonSuspended),
}

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b00, bits_len = 2)]
pub struct ComputeSkipReasonNoState;

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b01, bits_len = 2)]
pub struct ComputeSkipReasonBadState;

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b10, bits_len = 2)]
pub struct ComputeSkipReasonNoGas;

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b110, bits_len = 3)]
pub struct ComputeSkipReasonSuspended;
