/// NUMA nodes to allocate on.
///
/// If set to no nodes (the `Default::default()`) then memory is allocated on the local node if possible.
///
/// Ignored on operating systems other than Android and Linux.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumaNodeBitSet {
    pub(crate) bits: usize,

    /// Specifies physical node IDs.
    ///
    /// Linux does not remap the nodemask when the thread moves to a different cpuset context, nor when the set of nodes allowed by the thread's current cpuset context changes.
    ///
    /// (Not used if there are no nodes specified).
    pub static_nodes: bool,

    /// Specifies specifies node IDs that are relative to the set of node IDs allowed by the thread's current cpuset.
    ///
    /// (Not used if there are no nodes specified).
    pub relative_nodes: bool,
}

impl NumaNodeBitSet {
    #[allow(dead_code)]
    pub const NO_MODE_FLAGS_NODEMASK_MAXNODE: (i32, Option<usize>, usize) = (0, None, 0);

    /// Generate an empty struct
    #[inline(always)]
    pub fn new() -> Self {
        NumaNodeBitSet {
            bits: 0,
            static_nodes: false,
            relative_nodes: false,
        }
    }

    /// Generate a static struct
    #[inline(always)]
    pub fn new_static() -> Self {
        NumaNodeBitSet {
            bits: 0,
            static_nodes: true,
            relative_nodes: false,
        }
    }

    /// Generate a relative struct
    #[inline(always)]
    pub fn new_relative(&self) -> Self {
        NumaNodeBitSet {
            bits: 0,
            static_nodes: false,
            relative_nodes: true,
        }
    }

    /// Is this the empty set?
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Add a NUMA node into the set.
    #[inline(always)]
    pub fn insert_numa_node(&mut self, zero_based_node_index: u8) {
        self.bits |= 1 << (zero_based_node_index as usize)
    }

    /// Remove a NUMA node from the set.
    #[inline(always)]
    pub fn remove_numa_node(&mut self, zero_based_node_index: u8) {
        self.bits &= !(1 << (zero_based_node_index as usize))
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline(always)]
    pub fn mask_and_size(&self) -> (i32, Option<usize>, usize) {
        if likely!(self.is_empty()) {
            Self::NO_MODE_FLAGS_NODEMASK_MAXNODE
        } else {
            // let size = size_of::<usize>();

            let mut mode_flags = 0;
            if unlikely!(self.static_nodes) {
                const MPOL_F_STATIC_NODES: i32 = 1 << 15;
                mode_flags |= MPOL_F_STATIC_NODES
            }
            if unlikely!(self.relative_nodes) {
                const MPOL_F_RELATIVE_NODES: i32 = 1 << 14;
                mode_flags |= MPOL_F_RELATIVE_NODES
            }

            (mode_flags, Some(self.bits), 2 as usize)
        }
    }
}
