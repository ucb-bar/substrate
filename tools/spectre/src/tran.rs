//! Spectre transient analysis options and data structures.

use crate::{node_voltage_path, ErrPreset, SimSignal, Spectre};
use arcstr::ArcStr;
use rust_decimal::Decimal;
use scir::netlist::NetlistLibConversion;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use substrate::io::{NodePath, TerminalPath};
use substrate::schematic::conv::RawLib;
use substrate::schematic::{Cell, ExportsSchematicData};
use substrate::simulation::data::{FromSaved, HasSimData, Save};
use substrate::simulation::{Analysis, SimulationContext, Simulator, Supports};
use substrate::type_dispatch::impl_dispatch;

/// A transient analysis.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tran {
    /// Stop time (sec).
    pub stop: Decimal,
    /// Start time (sec).
    ///
    /// Defaults to 0.
    pub start: Option<Decimal>,

    /// The error preset.
    pub errpreset: Option<ErrPreset>,
}

/// The result of a transient analysis.
#[derive(Debug, Clone)]
pub struct TranOutput {
    pub(crate) lib: Arc<RawLib>,
    pub(crate) conv: Arc<NetlistLibConversion>,
    /// The time points of the transient simulation.
    pub time: Arc<Vec<f64>>,
    /// A map from signal name to values.
    pub raw_values: HashMap<ArcStr, Arc<Vec<f64>>>,
    /// A map from a save ID to a raw value identifier.
    pub(crate) saved_values: HashMap<u64, ArcStr>,
}

impl FromSaved<Spectre, Tran> for TranOutput {
    type Key = ();
    fn from_saved(output: &<Tran as Analysis>::Output, _key: Self::Key) -> Self {
        (*output).clone()
    }
}

impl<T: ExportsSchematicData> Save<Spectre, Tran, &Cell<T>> for TranOutput {
    fn save(
        _ctx: &SimulationContext,
        _to_save: &Cell<T>,
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

impl Save<Spectre, Tran, ()> for TranOutput {
    fn save(
        _ctx: &SimulationContext,
        _to_save: (),
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

/// The time points of a transient simulation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranTime(pub(crate) Arc<Vec<f64>>);

impl Deref for TranTime {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromSaved<Spectre, Tran> for TranTime {
    type Key = ();
    fn from_saved(output: &<Tran as Analysis>::Output, _key: Self::Key) -> Self {
        TranTime(output.time.clone())
    }
}

impl<T: ExportsSchematicData> Save<Spectre, Tran, &Cell<T>> for TranTime {
    fn save(
        _ctx: &SimulationContext,
        _to_save: &Cell<T>,
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

impl Save<Spectre, Tran, ()> for TranTime {
    fn save(
        _ctx: &SimulationContext,
        _to_save: (),
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

/// An identifier for a saved transient voltage.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranVoltageKey(pub(crate) u64);

/// A saved transient voltage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranVoltage(pub(crate) Arc<Vec<f64>>);

impl Deref for TranVoltage {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromSaved<Spectre, Tran> for TranVoltage {
    type Key = TranVoltageKey;
    fn from_saved(output: &<Tran as Analysis>::Output, key: Self::Key) -> Self {
        TranVoltage(
            output
                .raw_values
                .get(output.saved_values.get(&key.0).unwrap())
                .unwrap()
                .clone(),
        )
    }
}

#[impl_dispatch({&str; &String; ArcStr; String; SimSignal})]
impl<T> Save<Spectre, Tran, T> for TranVoltage {
    fn save(
        _ctx: &SimulationContext,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_voltage(to_save)
    }
}

impl Save<Spectre, Tran, &scir::SignalPath> for TranVoltage {
    fn save(
        _ctx: &SimulationContext,
        to_save: &scir::SignalPath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_voltage(SimSignal::ScirVoltage(to_save.clone()))
    }
}

impl Save<Spectre, Tran, &NodePath> for TranVoltage {
    fn save(
        ctx: &SimulationContext,
        to_save: &NodePath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(ctx, ctx.lib.convert_node_path(to_save).unwrap(), opts)
    }
}

#[impl_dispatch({scir::SignalPath; NodePath})]
impl<T> Save<Spectre, Tran, T> for TranVoltage {
    fn save(
        ctx: &SimulationContext,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(ctx, &to_save, opts)
    }
}

/// An identifier for a saved transient current.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranCurrentKey(pub(crate) Vec<u64>);

/// A saved transient current.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranCurrent(pub(crate) Arc<Vec<f64>>);

impl Deref for TranCurrent {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromSaved<Spectre, Tran> for TranCurrent {
    type Key = TranCurrentKey;
    fn from_saved(output: &<Tran as Analysis>::Output, key: Self::Key) -> Self {
        let currents: Vec<Arc<Vec<f64>>> = key
            .0
            .iter()
            .map(|key| {
                output
                    .raw_values
                    .get(output.saved_values.get(key).unwrap())
                    .unwrap()
                    .clone()
            })
            .collect();

        let mut total_current = vec![0.; output.time.len()];
        for tran_current in currents {
            for (i, current) in tran_current.iter().enumerate() {
                total_current[i] += *current;
            }
        }
        TranCurrent(Arc::new(total_current))
    }
}

#[impl_dispatch({&str; &String; ArcStr; String; SimSignal})]
impl<T> Save<Spectre, Tran, T> for TranCurrent {
    fn save(
        _ctx: &SimulationContext,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_current(to_save)
    }
}

impl Save<Spectre, Tran, &scir::SignalPath> for TranCurrent {
    fn save(
        _ctx: &SimulationContext,
        to_save: &scir::SignalPath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_current(SimSignal::ScirCurrent(to_save.clone()))
    }
}

impl Save<Spectre, Tran, &TerminalPath> for TranCurrent {
    fn save(
        ctx: &SimulationContext,
        to_save: &TerminalPath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        TranCurrentKey(
            ctx.lib
                .convert_terminal_path(to_save)
                .unwrap()
                .into_iter()
                .flat_map(|path| Self::save(ctx, path, opts).0)
                .collect(),
        )
    }
}

#[impl_dispatch({scir::SignalPath; TerminalPath})]
impl<T> Save<Spectre, Tran, T> for TranCurrent {
    fn save(
        ctx: &SimulationContext,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(ctx, &to_save, opts)
    }
}

impl HasSimData<str, Vec<f64>> for TranOutput {
    fn get_data(&self, k: &str) -> Option<&Vec<f64>> {
        self.raw_values.get(k).map(|x| x.as_ref())
    }
}

impl HasSimData<scir::SignalPath, Vec<f64>> for TranOutput {
    fn get_data(&self, k: &scir::SignalPath) -> Option<&Vec<f64>> {
        self.get_data(&*node_voltage_path(
            &self.lib.scir,
            &self.conv,
            &self.lib.scir.simplify_path(k.clone()),
        ))
    }
}

impl HasSimData<NodePath, Vec<f64>> for TranOutput {
    fn get_data(&self, k: &NodePath) -> Option<&Vec<f64>> {
        self.get_data(&self.lib.convert_node_path(k)?)
    }
}

impl Analysis for Tran {
    type Output = TranOutput;
}

impl Supports<Tran> for Spectre {
    fn into_input(a: Tran, inputs: &mut Vec<Self::Input>) {
        inputs.push(a.into());
    }
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> <Tran as Analysis>::Output {
        let item = outputs.next().unwrap();
        item.try_into().unwrap()
    }
}
