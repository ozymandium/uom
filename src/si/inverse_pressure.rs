//! Inverse pressure (base unit 1/pascal, kg⁻¹ · m · s²).

quantity! {
    /// Inverse pressure (base unit 1/pascal, kg⁻¹ · m · s²).
    quantity: InversePressure; "inverse pressure";
    /// Dimension of inverse pressure, LM⁻¹T² (base unit 1/pascal, kg⁻¹ · m · s²).
    dimension: ISQ<
        P1,     // length
        N1,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @inverse_bar: 1.0_E-5; "bar⁻¹", "inverse bar", "inverse bar";
        @inverse_pascal: prefix!(none); "Pa⁻¹", "inverse pascal", "inverse pascals";
        @inverse_psi: 1.0 / 6.894_757_889_515_779_E3; "psi⁻¹", "inverse psi", "inverse psi";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::inverse_pressure as i;
        use crate::si::pressure as p;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: InversePressure<V> =
                V::one() / Pressure::new::<p::pascal>(V::one());
        }

        #[test]
        fn check_units() {
            test::<p::bar, i::inverse_bar>();
            test::<p::pascal, i::inverse_pascal>();
            test::<p::psi, i::inverse_psi>();

            fn test<P: p::Conversion<V>, I: i::Conversion<V>>() {
                Test::assert_approx_eq(
                    &InversePressure::new::<I>(V::one()),
                    &(V::one() / Pressure::new::<P>(V::one()))
                );
            }
        }
    }
}