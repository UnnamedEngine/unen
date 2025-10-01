use std::any::Any;

pub trait Event: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub struct EventBox(Box<dyn Event>);

impl EventBox {
    pub fn new<E: Event>(event: E) -> Self {
        Self(Box::new(event))
    }

    pub fn downcast_ref<E: Event>(&self) -> Option<&E> {
        self.0.as_any().downcast_ref::<E>()
    }
}
