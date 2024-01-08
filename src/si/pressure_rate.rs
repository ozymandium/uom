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
        @psi_per_second: 6.894_757_889_515_779_E3; "psi/s", "pound-force per square inch per second",
            "pounds-force per square inch per second";
        @bar_per_second: 1.0_E5; "bar/s", "bar per second", "bars per second";

        @pascal_per_minute: 1.0_E0 / 6.0_E1; "Pa/min", "pascal per minute", "pascals per minute";
        @psi_per_minute: 6.894_757_889_515_779_E3 / 6.0_E1; "psi/min", "pound-force per square inch per minute", "pound-force per square inch per minute";
        @bar_per_minute: 1.0_E5 / 6.0_E1; "bar/min", "bar per minute", "bars per minute";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
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
            test::<p::psi, t::second, r::psi_per_second>();
            test::<p::bar, t::second, r::bar_per_second>();
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