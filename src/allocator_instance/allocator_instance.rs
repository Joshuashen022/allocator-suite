use lazy_static::*;

use crate::adaptors::prelude::*;
use crate::allocators::allocator::Allocator;
use crate::allocators::memory_map_allocator::MemoryMapAllocator;

#[allow(unused_imports)]
use crate::memory_sources::mmap::numa::numa_allocation_policy::NumaAllocationPolicy;
#[allow(unused_imports)]
use crate::memory_sources::mmap::numa::numa_node_bit_set::NumaNodeBitSet;
#[allow(unused_imports)]
use crate::memory_sources::mmap::numa::numa_settings::NumaSettings;
#[allow(unused_macros)]

macro_rules! allocator_instance_macro {
    ($hugepage:expr, $numa_policy:expr) => {
        {
            use crate::memory_sources::mmap::prelude::*;
            lazy_static! {
                static ref MMAP_ALLOC: MemoryMapAllocator = {
                    #[cfg(any(target_os = "android", target_os = "linux"))] {
                        use crate::memory_sources::mmap::numa::numa_settings::NumaSettings;
                        use crate::memory_sources::mmap::numa::numa_allocation_policy::NumaAllocationPolicy;

                        let numa_settings = NumaSettings::new(
                            $numa_policy,
                            true,
                        );
                        let mmap = MemoryMapSource::new(
                            true,
                            false,
                            false,
                            false,
                            $hugepage,
                            Some(numa_settings)
                        );
                        MemoryMapAllocator(mmap)
                    }

                    #[cfg(not(any(target_os = "android", target_os = "linux")))] {
                        let mmap = MemoryMapSource::default();
                        MemoryMapAllocator(mmap)
                    }
                };
            }
            lazy_static! {
                static ref MMAP_ADAPTER: AllocatorAdaptor<'static, MemoryMapAllocator> = {
                    MMAP_ALLOC.adapt()
                };
            }
            &*MMAP_ADAPTER
        }
    };
}

pub(crate) fn allocator_instance(
    huge_page: bool,
    node: Option<u8>,
) -> &'static AllocatorAdaptor<'static, MemoryMapAllocator> {
    match (huge_page, node) {
        (true, Some(0)) => {
            allocator_instance_macro!(
                HugePageSize::Default,
                NumaAllocationPolicy::Preferred(NumaNodeBitSet {
                    bits: 0,
                    static_nodes: true,
                    relative_nodes: false,
                })
            )
        }
        (true, Some(1)) => {
            allocator_instance_macro!(
                HugePageSize::Default,
                NumaAllocationPolicy::Preferred(NumaNodeBitSet {
                    bits: 1,
                    static_nodes: true,
                    relative_nodes: false,
                })
            )
        }
        (true, Some(n)) => {
            panic!("binding node {} not supported yet", n);
        }
        (true, None) => {
            allocator_instance_macro!(HugePageSize::Default, NumaAllocationPolicy::Local)
        }
        (false, Some(1)) => {
            allocator_instance_macro!(
                HugePageSize::None,
                NumaAllocationPolicy::Preferred(NumaNodeBitSet {
                    bits: 1,
                    static_nodes: true,
                    relative_nodes: false,
                })
            )
        }
        (false, Some(0)) => {
            allocator_instance_macro!(
                HugePageSize::None,
                NumaAllocationPolicy::Preferred(NumaNodeBitSet {
                    bits: 0,
                    static_nodes: true,
                    relative_nodes: false,
                })
            )
        }
        (false, Some(n)) => {
            panic!("binding node {} not supported yet", n);
        }
        (false, None) => {
            allocator_instance_macro!(HugePageSize::None, NumaAllocationPolicy::Local)
        }
    }
}

#[test]
fn hugepage_macros() {
    macro_rules! hello_macro {
        ($x:expr, $hugepage:expr) => {{
            if $hugepage {
                println!("using hugepage {:?}", $x);
            } else {
                println!("not using hugepage {:?}", $x);
            }
        }};
    }
    hello_macro!(1, false);
}
