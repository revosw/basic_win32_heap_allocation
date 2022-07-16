use std::ffi::c_void;
use windows::Win32::System::Memory::*;

pub struct Allocation<T> {
    heap_handle: HeapHandle,
    bytes_allocated: usize,
    allocation_address: *mut T,
}

impl<T> Allocation<T> where T : Copy {
    pub fn new(bytes_to_allocate: usize) -> Result<Allocation<T>, windows::core::Error>{
        unsafe {
            let heap_handle = HeapCreate(HEAP_FLAGS(0), bytes_to_allocate, bytes_to_allocate + 1000)?;
            Ok(Allocation { heap_handle, bytes_allocated: bytes_to_allocate, allocation_address: 0 as *mut T })
        }
    }
    pub fn delete(self) {
        unsafe {
            HeapDestroy(self.heap_handle);
        }
    }
    pub fn allocate(&mut self, bytes_to_allocate: usize) -> Result<(), &'static str> {
        if !self.allocation_address.is_null() {
            return Err("Bytes have already been allocated. Deallocate before allocating again.");
        }
        unsafe {
            self.allocation_address = HeapAlloc(self.heap_handle, HEAP_FLAGS(0), 100).cast::<T>();
        }
        Ok(())
    }
    pub fn deallocate(&mut self) {
        if self.allocation_address.is_null() {
            println!("It's already deallocated. Allocate something first.");
            return;
        }
        unsafe {
            if HeapFree(self.heap_handle, HEAP_FLAGS(0), self.allocation_address.cast::<c_void>()) == false {
                println!("Coudln't free data on heap");
            }
        }
        self.allocation_address = 0 as *mut T
    }
    pub fn get(&mut self) -> Result<T, ()> {
        if self.allocation_address.is_null() {
            return Err(());
        }
        unsafe {
            Ok(*self.allocation_address)
        }
    }
    pub fn set(self, data: T) -> Result<(), ()> {
        unsafe {
            *self.allocation_address = data;
        }
        Ok(())
    }
}