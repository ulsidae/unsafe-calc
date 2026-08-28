use crate::ast::Expr;

pub struct UnsafeStack {
    ptr: *mut Expr,
    len: usize,
    capacity: usize,
}

impl UnsafeStack {
    pub fn new() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, value: Expr) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            self.ptr.add(self.len).write(value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<Expr> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        unsafe {
            Some(self.ptr.add(self.len).read())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            8
        } else {
            self.capacity * 2
        };

        let layout =
            std::alloc::Layout::array::<Expr>(new_capacity)
                .expect("invalid layout");

        let new_ptr = unsafe {
            std::alloc::alloc(layout) as *mut Expr
        };

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        if !self.ptr.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.ptr,
                    new_ptr,
                    self.len,
                );

                let old_layout =
                    std::alloc::Layout::array::<Expr>(self.capacity)
                        .expect("invalid layout");

                std::alloc::dealloc(
                    self.ptr as *mut u8,
                    old_layout,
                );
            }
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl Drop for UnsafeStack {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                core::ptr::drop_in_place(
                    self.ptr.add(i)
                );
            }

            if !self.ptr.is_null() {
                let layout =
                    std::alloc::Layout::array::<Expr>(self.capacity)
                        .expect("invalid layout");

                std::alloc::dealloc(
                    self.ptr as *mut u8,
                    layout,
                );
            }
        }
    }
}
