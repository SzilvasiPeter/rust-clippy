#![warn(clippy::misleading_mut_method_name)]
#![allow(dead_code, unused_variables)]

use std::sync::LockResult;

struct Mut;

struct Buf {
    data: Vec<u8>,
    name: String,
}

impl Buf {
    fn get_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, u8> {
        self.data.iter_mut()
    }

    fn opt_mut(&mut self) -> Option<&mut u8> {
        self.data.first_mut()
    }

    fn lock_result_mut(&mut self) -> LockResult<&mut u8> {
        Ok(&mut self.data[0])
    }

    fn split_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        self.data.split_at_mut(0)
    }

    fn get_disjoint_mut(&mut self, i: usize, j: usize) -> [&mut u8; 2] {
        self.data.get_disjoint_mut([i, j]).unwrap()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }

    fn clear_mut(&mut self) {
        self.data.clear();
    }

    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut u8) -> bool,
    {
        self.data.retain_mut(f);
    }

    fn from_mut(data: &mut u8) -> &u8 {
        data
    }

    fn window_mut(&self) -> &[u8] {
        &self.data
        //~^^ misleading_mut_method_name
    }

    fn text_mut(&self) -> &str {
        &self.name
        //~^^ misleading_mut_method_name
    }

    fn opt_shared_mut(&self) -> Option<&u8> {
        self.data.first()
        //~^^ misleading_mut_method_name
    }

    fn initialized_mut(&self) -> bool {
        self.data.is_empty()
        //~^^ misleading_mut_method_name
    }

    fn len_mut(&self) -> usize {
        self.data.len()
        //~^^ misleading_mut_method_name
    }

    fn owned_mut(&self) -> Mut {
        Mut
        //~^^ misleading_mut_method_name
    }

    fn into_mut(self) -> Mut {
        Mut
        //~^^ misleading_mut_method_name
    }
}

trait SplitMut {
    fn split_mut(&self) -> &Mut;
    //~^ misleading_mut_method_name
}

impl SplitMut for Mut {
    fn split_mut(&self) -> &Mut {
        self
        //~^^ misleading_mut_method_name
    }
}

fn main() {}
