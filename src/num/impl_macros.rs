macro_rules! int_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ty
    ) => {
        impl ::corex::num::zero::Zeroable for $SelfT {
            const ZERO: Self = !(<$UnsignedT>::MAX) as Self;
        }
        impl ::corex::num::sign::Signable for $SelfT {
            const SIGNED: bool = true;
        }
    };
}
macro_rules! uint_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        SignedT = $SignedT:ty
    ) => {
        impl ::corex::num::zero::Zeroable for $SelfT {
            const ZERO: Self = $ActualT::MIN as Self;
        }
        impl ::corex::num::sign::Signable for $SelfT {
            const SIGNED: bool = false;
        }
    };
}
macro_rules! float_impl {
    (
        Self = $SelfT:ty,
        ActualT = $ActualT:ident,
        UnsignedT = $UnsignedT:ident
    ) => {
        impl ::corex::num::zero::Zeroable for $SelfT {
            const ZERO: Self = <$UnsignedT as ::corex::num::zero::Zeroable>::ZERO as Self;
        }
        impl ::corex::num::sign::Signable for $SelfT {
            const SIGNED: bool = true;
        }
    };
}
