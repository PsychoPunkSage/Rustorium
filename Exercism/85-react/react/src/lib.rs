use lazy_static::lazy_static;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::Mutex;

lazy_static! {
    static ref USED_IDS: Mutex<HashSet<u32>> = Mutex::new(HashSet::new());
}

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(u32);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct ComputeCell<T> {
    id: ComputeCellId,
    value: T,
    dependencies: Vec<CellId>,
    compute_func: Option<Box<dyn Fn(&[T]) -> T>>,
}
pub struct Reactor<T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    inputs: HashMap<InputCellId, T>,
    computes: Vec<ComputeCell<T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq + Default + Debug + Sized> Reactor<T> {
    pub fn new() -> Self {
        Reactor {
            inputs: HashMap::new(),
            computes: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = generate_id();
        self.inputs.insert(InputCellId(id), initial);
        InputCellId(id)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let compute_result = {
            let mut vals = Vec::new();
            for &cell_id in dependencies {
                match cell_id {
                    CellId::Input(input_id) => {
                        if let Some(value) = self.inputs.get(&input_id) {
                            vals.push(*value);
                        } else {
                            return Err(cell_id);
                        }
                    }
                    _ => return Err(cell_id),
                }
            }
            compute_func(&vals)
        };

        let id = generate_id();
        self.computes.push(ComputeCell {
            id: ComputeCellId(id),
            value: compute_result,
            dependencies: dependencies.to_vec(),
            compute_func: Some(Box::new(compute_func)),
        });
        Ok(ComputeCellId(id))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.inputs.get(&id).cloned(),
            CellId::Compute(id) => {
                if self.computes.iter().any(|ds| ds.id == id) {
                    self.computes
                        .iter()
                        .find(|ds| ds.id == id)
                        .map(|ds| ds.value.clone())
                } else {
                    None
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(input) = self.inputs.get_mut(&id) {
            *input = new_value;

            for i in self.computes.iter_mut() {
                let mut vals = Vec::new();

                for cell_id in &i.dependencies {
                    if let CellId::Input(input_id) = cell_id {
                        if let Some(value) = self.inputs.get(input_id) {
                            vals.push(*value);
                        } else {
                            continue;
                        }
                    }
                }

                if let Some(func) = &i.compute_func {
                    i.value = func(&vals);
                }
            }
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        todo!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        todo!(
            "Remove the callback identified by the CallbackId {callback:?} from the cell {cell:?}"
        )
    }
}

fn generate_id() -> u32 {
    loop {
        let id = rand::thread_rng().gen_range(100000..=999999) as u32;
        let mut ids = USED_IDS.lock().unwrap();

        if !ids.contains(&id) {
            ids.insert(id);
            return id;
        }
    }
}
