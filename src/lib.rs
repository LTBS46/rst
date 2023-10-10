use std::{
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
    thread::spawn,
};

pub struct Chunk {}

pub trait Dimension: Named {}

pub trait World: Named {
    fn start(&self) {
        let w_this = self.get_self();
        let r = self.runner();
        spawn(move || {
            r(w_this.upgrade().unwrap());
        });
    }

    fn get_self(&self) -> Weak<Mutex<dyn World>>;
    fn runner(&self) -> fn(_: Arc<Mutex<dyn World>>) -> ();
}

pub trait Server: Named {
    fn get_self(&self) -> Weak<Mutex<dyn Server>>;

    fn get_path(&self) -> PathBuf;

    fn register(&mut self, w: Weak<Mutex<dyn World>>, s: String) -> bool;

    fn auto_register(&mut self, w: Weak<Mutex<dyn World>>) -> bool {
        self.register(w.clone(), w.upgrade().unwrap().lock().unwrap().get_name())
    }
}

pub trait Named: Send + Sync {
    fn get_name(&self) -> String;
}

pub trait GameObject: Named {}

pub trait Entity: GameObject {}

pub trait Item: GameObject {}

pub trait Block: GameObject {}

pub trait BlockItem: Block + Item {}

impl<T: Block + Item> BlockItem for T {}

pub struct ServerHolder(Arc<Mutex<dyn Server>>);

impl Server for ServerHolder {
    fn get_self(&self) -> Weak<Mutex<dyn Server>> {
        Arc::downgrade(&self.0)
    }

    fn get_path(&self) -> PathBuf {
        self.0.lock().unwrap().get_path()
    }

    fn register(&mut self, w: Weak<Mutex<dyn World>>, s: String) -> bool {
        self.0.lock().unwrap().register(w, s)
    }
}

impl Named for ServerHolder {
    fn get_name(&self) -> String {
        self.0.lock().unwrap().get_name()
    }
}

impl ServerHolder {
    pub fn new(w: Arc<Mutex<dyn Server>>) -> Self {
        ServerHolder(w)
    }
}
