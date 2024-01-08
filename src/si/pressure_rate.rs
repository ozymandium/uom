//! Pressure rate (base unit pascal per second, kg · m⁻¹ · s⁻³).

quantity! {
    /// Pressure rate (base unit pascal per second, kg · m⁻¹ · s⁻³).
    quantity: PressureRate; "pressure rate";
    /// Dimension of pressure, L⁻¹MT⁻³ (base unit pascal per second, kg · m⁻¹ · s⁻³).
    dimension: ISQ<
        N1,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::PressureRateKind);
    units {
        @pascal_per_second: prefix!(none); "Pa/s", "pascal per second", "pascals per second";
        @pascal_per_minute: 1.0_E0 / 6.0_E1; "Pa/min", "pascal per minute", "pascals per minute";
        @psi_per_minute: 6.894_757_E3 / 6.0_E1; "psi/min", "pounds per square inch per minute", "pounds per square inch per minute";
        @bar_per_minute: 1.0_E5 / 6.0_E1; "bar/min", "bar per minute", "bars per minute";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::lib::any::TypeId;
        use crate::num::One;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::pressure as p;
        use crate::si::pressure_rate as r;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: PressureRate<V> = (Pressure::new::<p::pascal>(V::one())
                / Time::new::<t::second>(V::one())).into();
        }

        #[test]
        fn check_units() {

            test::<p::pascal, t::second, r::pascal_per_second>();
            test::<p::pascal, t::minute, r::pascal_per_minute>();
            test::<p::psi, t::minute, r::psi_per_minute>();
            test::<p::bar, t::minute, r::bar_per_minute>();

            fn test<P: p::Conversion<V>, T: t::Conversion<V>, R: r::Conversion<V>>() {
                Test::assert_eq(
                    &PressureRate::new::<R>(V::one()),
                    &(
                        Pressure::new::<P>(V::one()) / Time::new::<T>(V::one())
                    ).into()
                );
            }
        }
    }
}