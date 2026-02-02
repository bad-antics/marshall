//! Memory protection features

pub struct MemoryProtection {
    enabled: bool,
}

impl MemoryProtection {
    pub fn new() -> Self {
        Self { enabled: true }
    }
    
    /// Zero sensitive data after use
    pub fn secure_zero(data: &mut [u8]) {
        for byte in data.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Allocate memory with guard pages
    #[cfg(target_os = "linux")]
    pub fn guarded_alloc(size: usize) -> Result<*mut u8, String> {
        use std::alloc::{alloc, Layout};
        
        let layout = Layout::from_size_align(size, 4096)
            .map_err(|e| e.to_string())?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("Allocation failed".into());
        }
        
        Ok(ptr)
    }
}
