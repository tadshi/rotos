use core::marker::PhantomData;
use core::{borrow::BorrowMut, mem::MaybeUninit};
use core::ptr::NonNull;

pub struct KListElement<T> where T: ?Sized + 'static {
    content: &'static mut T,
    next: Option<NonNull<KListElement<T>>>
}

impl<T> KListElement<T> {
    pub fn get(&mut self) -> &mut T {
        return (*(self.content)).borrow_mut()
    }
}

pub struct KLinkedList<T: ?Sized + 'static> {
    head: Option<NonNull<KListElement<T>>>,
    tail: Option<NonNull<KListElement<T>>>,
}

impl<T> KLinkedList<T> {
    pub const fn new() -> Self {
        KLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn from_prepared(data_arr:&'static mut [T], wrap_arr: &'static mut [MaybeUninit<KListElement<T>>], start:usize, end: usize) -> KLinkedList<T> {
        unsafe {
            let end = end - 1;
            let data_ptr = data_arr.as_mut_ptr();
            let wrap_ptr = wrap_arr.as_mut_ptr();
            for i in start..end {
                let target = wrap_ptr.add(i);
                (*target).write(KListElement { content: &mut (*data_ptr.add(i)), next: Some(NonNull::from((*target.add(1)).assume_init_mut())) });
            }
            KLinkedList {
                head: Some(NonNull::from((*wrap_ptr.add(start)).assume_init_mut())),
                tail: Some(NonNull::from(wrap_arr[end].assume_init_mut())), 
            }   
        }
    }

    pub fn push_front(&mut self, elem: &mut KListElement<T>) {
        elem.next = self.head;
        let some_elem = Some(NonNull::from(elem));
        if self.head.is_none() {
            self.tail = some_elem
        }
        self.head = some_elem
    }

    pub fn push_tail(&mut self, elem: &mut KListElement<T>) {
        elem.next = None;
        let node = Some(NonNull::from(elem.borrow_mut()));
        unsafe {
            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
        }
        self.tail = node
    }

    pub fn pop_front(&mut self) -> Option<&mut KListElement<T>> {
        self.head.and_then(|mut head|
            unsafe {
                let elem = head.as_mut();
                self.head = elem.next;
                if elem.next.is_none() {
                    self.tail = None;
                }
                Some(elem)
            }
        )
    }

    pub fn iter(&self) -> KListIterator<'_, T>{
        return KListIterator { 
            cur: self.head,
            marker: PhantomData 
        }
    }

    pub fn iter_mut(&mut self) -> KListMIterator<'_, T>{
        return KListMIterator { 
            cur: self.head,
            marker: PhantomData
        }
    }
}

pub struct KListIterator<'a, T: 'static> {
    cur: Option<NonNull<KListElement<T>>>,
    marker: PhantomData<&'a KListElement<T>>
}
pub struct KListMIterator<'a, T: 'static> {
    cur: Option<NonNull<KListElement<T>>>,
    marker: PhantomData<&'a mut KListElement<T>>
}

impl<'a, T> Iterator for KListIterator<'a, T> {
    type Item = &'a KListElement<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.and_then(|elem| {
            self.cur = Some(elem);
            unsafe {
                let elem = elem.as_ref();
                Some(elem)
            }
        })
    }
}

impl<'a, T> Iterator for KListMIterator<'a, T> {
    type Item = &'a mut KListElement<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.and_then(|mut elem| {
            self.cur = Some(elem);
            unsafe {
                let elem = elem.as_mut();
                Some(elem)
            }
        })
    }
}

impl <'a, T> IntoIterator for &'a KLinkedList<T> {
    type Item = &'a KListElement<T>;

    type IntoIter = KListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl <'a, T> IntoIterator for &'a mut KLinkedList<T> {
    type Item = &'a mut KListElement<T>;

    type IntoIter = KListMIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub trait Bss {
    const ZERO: Self;
}

#[macro_export]
// Remember that you're defining TWO static mut array!!
macro_rules! prepare_k_list {
    ($id:ident [$t:ty ; $len:tt]) => {
        paste::paste! {
        #[link_section = ".bss"]
        static mut [<$id _ELEM>]: [$t; $len] = [<$t>::ZERO;$len];
        #[link_section = ".bss"]
        static mut [<$id _NODES>]: [core::mem::MaybeUninit<$crate::utils::KListElement<$t>>;$len] = 
            core::mem::MaybeUninit::uninit_array();
        const [<$id _DEBUG>]: usize = $len;
    }
    }
}

#[macro_export]
macro_rules! from_prepared {
    ($id:ident, $start:expr, $end:expr) => {
        paste::paste! {
            crate::utils::KLinkedList::from_prepared(&mut [<$id _ELEM>], &mut [<$id _NODES>], $start, $end)
        }
    };
}

#[macro_export]
macro_rules! k_list_eforeach {
    ($id:ident, $func:expr) => {
        paste::paste! {
            [<$id _ELEM>].iter_mut().enumerate().for_each($func)
        }
    };
}
