// use halo2_proofs::{
//     arithmetic::Field,
//     circuit::{Layouter, SimpleFloorPlanner},
//     plonk::{Circuit, ConstraintSystem, Error},
// };
// use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::{Field, FieldExt},
    circuit::{Chip, Layouter, Region, Value},
    plonk::{
        Advice, Column, ConstraintSystem, Error, Expression, Selector, TableColumn, VirtualCells,
    },
    poly::Rotation,
};

// mod account_node;
// mod account_parents;
// mod key;
mod division;
mod poseidon;
mod storage_leaf;
mod storage_parents;

// #[derive(Clone, Copy, Debug)]
// struct RangeCheckConfig(Column<Fixed>);

// impl ByteRangeCheckConfig {
//     fn configure<F: Field>(meta: &mut ConstraintSystem<F>) -> Self {
//         Self(meta.fixed_column())
//     }

//     fn assign<F: Field>(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
//         layouter.assign_region(
//             || "byte range check fixed column",
//             |mut region| {
//                 (0..256)
//                     .map(|i| region.assign_advice(|| "", self.0, i, || i.into()))
//                     .collect()
//             },
//         )
//     }

//     pub(crate) fn lookup_expressions<F: Field>(
//         &self,
//         meta: &mut VirtualCells<'_, F>,
//     ) -> Vec<Expression<F>> {
//         vec![meta.query_fixed(self.0, Rotation::cur())]
//     }
// }