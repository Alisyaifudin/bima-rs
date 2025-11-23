pub enum PayloadRef<'a, T> {
    Ref(&'a T),
    Mut(&'a mut T)
}

impl<'a, T> PayloadRef<'a, T> {
    pub fn as_ref(&self) -> &T {
        match self {
            PayloadRef::Ref(r) => r,
            PayloadRef::Mut(m) => m
        }
    }
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            PayloadRef::Ref(_) => None,
            PayloadRef::Mut(m) => Some(m)
        }
    
    }
}

pub trait Effect<E, Counter, Payload = ()> {
    fn update(&mut self, counter: Counter, payload: PayloadRef<Payload>) -> Result<(), E>;
}
