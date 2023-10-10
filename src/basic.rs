use std::{
    any::type_name,
    collections::HashMap,
    env::current_exe,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, Weak,
    },
    time::Duration,
};

use engine::{Dimension, Named, Server, World};

pub struct BasicServer {
    name: String,
    _id: AtomicUsize,
    _worlds: HashMap<String, Arc<Mutex<dyn World>>>,
    this: Option<Weak<Mutex<BasicServer>>>,
    path: PathBuf,
}

pub struct BasicWorld {
    name: String,
    _dimensions: HashMap<String, Arc<Mutex<dyn Dimension>>>,
    this: Option<Weak<Mutex<BasicWorld>>>,
}

pub struct BasicOverworld {}

static NEXT_SERVER_ID: AtomicUsize = AtomicUsize::new(0);

pub fn next_server_id() -> usize {
    NEXT_SERVER_ID.fetch_add(1, Ordering::SeqCst)
}

impl BasicServer {
    pub fn new(_name: Option<String>) -> Arc<Mutex<BasicServer>> {
        let new_id = next_server_id();
        let mut this_path = current_exe().unwrap();
        this_path.pop();
        let rv: Arc<Mutex<BasicServer>> = Arc::new(Mutex::new(BasicServer {
            name: match _name {
                Some(n) => n,
                None => {
                    let mut r = String::from("basic_server");
                    r.push_str(&new_id.to_string());
                    r
                }
            },
            _id: AtomicUsize::new(new_id),
            _worlds: HashMap::new(),
            this: None,
            path: this_path,
        }));
        rv.lock().unwrap().this = Some(Arc::downgrade(&rv.clone()));
        rv
    }
}

impl Server for BasicServer {
    fn get_self(&self) -> Weak<Mutex<dyn Server>> {
        self.this.clone().unwrap()
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn register(&mut self, w: Weak<Mutex<dyn World>>, s: String) -> bool {
        if self._worlds.contains_key(&s) {
            return false;
        } else {
            self._worlds.insert(s, w.upgrade().unwrap());
            return true;
        }
    }
}

impl Named for BasicServer {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl BasicWorld {
    pub fn new(name: String) -> Arc<Mutex<BasicWorld>> {
        let w = BasicWorld {
            name,
            _dimensions: HashMap::new(),
            this: None,
        };

        let rv = Arc::new(Mutex::new(w));

        rv.lock().unwrap().this = Some(Arc::downgrade(&rv));

        rv
    }
}

pub fn print_type<T>(_: &T) -> String {
    type_name::<T>().to_string()
}

fn basic_runner(_w: Arc<Mutex<dyn World>>) {
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

impl World for BasicWorld {
    fn runner(&self) -> fn(Arc<Mutex<dyn World>>) -> () {
        basic_runner
    }

    fn get_self(&self) -> Weak<Mutex<dyn World>> {
        self.this.clone().unwrap()
    }
}

impl Named for BasicWorld {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Dimension for BasicOverworld {}

impl Named for BasicOverworld {
    fn get_name(&self) -> String {
        String::from("Overworld")
    }
}
