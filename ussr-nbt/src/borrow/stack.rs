use std::hint::assert_unchecked;

pub struct Stack {
    stack: Vec<StackElement>,
}

impl Stack {
    #[inline]
    pub fn new(capacity: usize) -> Stack {
        Stack {
            stack: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    #[inline]
    pub fn push(&mut self, element: StackElement) {
        self.stack.push(element);
    }

    #[inline]
    pub unsafe fn push_unchecked(&mut self, element: StackElement) {
        let len: usize = self.stack.len();
        unsafe {
            let end: *mut StackElement = self.stack.as_mut_ptr().add(len);
            std::ptr::write(end, element);
            self.stack.set_len(len + 1);
        }

        // assert_unchecked(self.stack.len() < self.stack.capacity());
        // self.stack.push(element);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<StackElement> {
        self.stack.pop()
    }

    #[inline]
    pub unsafe fn pop_unchecked(&mut self) -> StackElement {
        unsafe {
            self.stack.set_len(self.len().unchecked_sub(1));
            assert_unchecked(self.stack.len() < self.stack.capacity());
            std::ptr::read(self.stack.as_ptr().add(self.len()))
        }

        // assert_unchecked(self.stack.len() != 0);
        // self.stack.pop().unwrap_unchecked()
    }

    #[inline]
    pub fn peek_mut(&mut self) -> Option<&mut StackElement> {
        self.stack.last_mut()
    }

    #[inline]
    pub unsafe fn peek_unchecked_mut(&mut self) -> &mut StackElement {
        let len: usize = self.stack.len();
        self.stack.get_unchecked_mut(len.unchecked_sub(1))
    }
}

pub enum StackElement {
    Compound { len: usize, index: usize },
    ListList { len: usize, index: usize },
    CompoundList { len: usize, index: usize },
}

impl StackElement {
    #[inline]
    pub const fn new_compound(index: usize) -> StackElement {
        StackElement::Compound { len: 0, index }
    }

    #[inline]
    pub const fn new_list_list(len: usize, index: usize) -> StackElement {
        StackElement::ListList { len, index }
    }

    #[inline]
    pub const fn new_compound_list(len: usize, index: usize) -> StackElement {
        StackElement::CompoundList { len, index }
    }
}
