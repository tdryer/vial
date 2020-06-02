use std::{any::TypeId, rc::Rc};

pub fn set<T: 'static>(value: T) {
    crate::LOCAL_CACHE.with(|c| {
        c.borrow_mut().insert(
            TypeId::of::<T>(),
            Box::into_raw(Box::new(Rc::new(value))) as usize,
        );
    });
}

pub fn get<T: 'static>() -> Rc<T> {
    let mut ptr = 0;
    crate::LOCAL_CACHE.with(|c| {
        if let Some(p) = c.borrow().get(&TypeId::of::<T>()) {
            ptr = *p;
        } else {
            panic!("can't find {:?} in cache", TypeId::of::<T>());
        }
    });
    unsafe { *Box::from_raw(ptr as *mut Rc<T>).clone() }
}
