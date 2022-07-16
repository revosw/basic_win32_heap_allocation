use std::error::Error;

use basic_win32_heap_allocation::Allocation;

#[derive(Default, Clone, Copy, Debug)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut allocation = Allocation::<RGBA>::new(4000).unwrap();
    allocation.deallocate();
    allocation.deallocate();
    allocation.allocate(2000);
    allocation.set(RGBA {..Default::default()});

    // rustc(E0382) borrow of moved value: `allocation`
    let from_heap = allocation.get();
    println!("{:?}", from_heap);
    Ok(())
}
