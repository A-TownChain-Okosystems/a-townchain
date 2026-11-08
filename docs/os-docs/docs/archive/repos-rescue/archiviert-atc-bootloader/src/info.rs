// Boot info structure and handoff
pub struct BootInfo {
    pub memory_map_addr: u64,
    pub cmdline: String,
    pub initrd_addr: Option<u64>,
}

impl BootInfo {
    pub fn new() -> Self {
        Self { memory_map_addr: 0x8000, cmdline: String::new(), initrd_addr: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bootinfo() {
        let bi = BootInfo::new();
        assert_eq!(bi.memory_map_addr, 0x8000);
        assert!(bi.initrd_addr.is_none());
    }
}
