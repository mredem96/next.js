use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    sync::{atomic::*, Arc},
    time::Duration,
};

use crate::RawVc;

pub struct TraceRawVcsContext {
    list: Vec<RawVc>,
}

impl TraceRawVcsContext {
    pub(crate) fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub(crate) fn into_vec(self) -> Vec<RawVc> {
        self.list
    }
}

/// Trait that allows to walk data to find all [RawVc]s contained.
///
/// This is important for Garbagge Collection to mark all Slots and
/// therefore Tasks that are still in use.
///
/// It can also be used to optimize transferring of Tasks, where knowning
/// the referenced Slots/Tasks allows pushing them earlier.
///
/// `#[derive(TraceRawVcs)]` is available.
/// `#[trace_ignore]` can be used on fields to skip tracing for them.
pub trait TraceRawVcs {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext);
    fn get_node_refs(&self) -> Vec<RawVc> {
        let mut context = TraceRawVcsContext::new();
        self.trace_node_refs(&mut context);
        context.into_vec()
    }
}

macro_rules! ignore {
  ($ty:ty) => {
    impl TraceRawVcs for $ty {
      fn trace_node_refs(&self, _context: &mut TraceRawVcsContext) {}
    }
  };

  ($ty:ty, $($tys:ty),+) => {
    ignore!($ty);
    ignore!($($tys),+);
  }
}

ignore!(i8, u8, i16, u16, i32, u32, i64, u64, char, bool, usize);
ignore!(
    AtomicI8,
    AtomicU8,
    AtomicI16,
    AtomicU16,
    AtomicI32,
    AtomicU32,
    AtomicI64,
    AtomicU64,
    AtomicBool,
    AtomicUsize
);
ignore!(String, Duration);

impl<A: TraceRawVcs> TraceRawVcs for (A,) {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        TraceRawVcs::trace_node_refs(&self.0, context);
    }
}

impl<A: TraceRawVcs, B: TraceRawVcs> TraceRawVcs for (A, B) {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        TraceRawVcs::trace_node_refs(&self.0, context);
        TraceRawVcs::trace_node_refs(&self.1, context);
    }
}

impl<A: TraceRawVcs, B: TraceRawVcs, C: TraceRawVcs> TraceRawVcs for (A, B, C) {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        TraceRawVcs::trace_node_refs(&self.0, context);
        TraceRawVcs::trace_node_refs(&self.1, context);
        TraceRawVcs::trace_node_refs(&self.2, context);
    }
}

impl<T: TraceRawVcs> TraceRawVcs for Option<T> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        if let Some(item) = self {
            TraceRawVcs::trace_node_refs(item, context);
        }
    }
}

impl<T: TraceRawVcs> TraceRawVcs for Vec<T> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        for item in self.iter() {
            TraceRawVcs::trace_node_refs(item, context);
        }
    }
}

impl<T: TraceRawVcs> TraceRawVcs for HashSet<T> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        for item in self.iter() {
            TraceRawVcs::trace_node_refs(item, context);
        }
    }
}

impl<T: TraceRawVcs> TraceRawVcs for BTreeSet<T> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        for item in self.iter() {
            TraceRawVcs::trace_node_refs(item, context);
        }
    }
}

impl<K: TraceRawVcs, V: TraceRawVcs> TraceRawVcs for HashMap<K, V> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        for (key, value) in self.iter() {
            TraceRawVcs::trace_node_refs(key, context);
            TraceRawVcs::trace_node_refs(value, context);
        }
    }
}

impl<K: TraceRawVcs, V: TraceRawVcs> TraceRawVcs for BTreeMap<K, V> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        for (key, value) in self.iter() {
            TraceRawVcs::trace_node_refs(key, context);
            TraceRawVcs::trace_node_refs(value, context);
        }
    }
}

impl<T: TraceRawVcs + ?Sized> TraceRawVcs for Box<T> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        TraceRawVcs::trace_node_refs(&**self, context);
    }
}

impl<T: TraceRawVcs + ?Sized> TraceRawVcs for Arc<T> {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        TraceRawVcs::trace_node_refs(&**self, context);
    }
}

impl TraceRawVcs for RawVc {
    fn trace_node_refs(&self, context: &mut TraceRawVcsContext) {
        context.list.push(self.clone());
    }
}

pub use turbo_tasks_macros::TraceRawVcs;
