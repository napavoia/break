use std::{
    collections::{HashMap, HashSet},
    sync::atomic::AtomicU64,
};

use mlua::{IntoLuaMulti, Table};
use sdl2::{EventPump, event::Event};

mod object;

use crate::{
    context::{Shared, Window, shared},
    game::object::Object,
};

pub type Index = u64;
pub type Status = Option<Runtime>;

#[derive(Debug, PartialEq)]
pub enum Runtime {
    Exit,
}

pub struct Game {
    pub canvas: Shared<Window>,
    pub event_pump: EventPump,
    events: HashMap<String, HashSet<Index>>,
    objects: HashMap<Index, Object>,
}

impl Game {
    pub fn new(canvas: Window, event_pump: EventPump) -> Self {
        Self {
            canvas: shared(canvas),
            objects: HashMap::new(),
            events: HashMap::new(),
            event_pump,
        }
    }

    pub fn tick(&mut self) -> Status {
        // Its my comment, not AI
        // Game Event Handle
        if let Some(s) = self.handle_events() {
            return Some(s);
        }
        // Game Logic Update
        if let Some(s) = self.update() {
            return Some(s);
        }
        // Game Render
        if let Some(s) = self.render() {
            return Some(s);
        }

        None
    }

    #[allow(clippy::single_match)]
    pub fn handle_events(&mut self) -> Status {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Some(Runtime::Exit),
                _ => {}
            }
        }

        None
    }

    pub fn update(&mut self) -> Status {
        self.call("update", mlua::Nil);

        None
    }

    pub fn render(&mut self) -> Status {
        let mut canvas = self.canvas.borrow_mut();

        canvas.clear();
        // SELF RENDER

        //
        canvas.present();

        None
    }

    pub fn call<L>(&self, name: &str, args: L)
    where
        L: IntoLuaMulti + Clone,
    {
        if let Some(list) = self.events.get(name) {
            list.iter().for_each(|i| {
                if let Some(obj) = self.objects.get(i)
                    && let Err(err) = obj.call_function(name, args.clone())
                {
                    dbg!(&err);
                }
            });
        }
    }

    pub fn hook(&mut self, object: Index, name: String) {
        match self.events.contains_key(&name) {
            true => {
                let _ = self.events.get_mut(&name).unwrap().insert(object);
            }
            false => {
                let _ = self.events.insert(name, HashSet::from([object]));
            }
        }
    }

    pub fn instantiate(&mut self, table: Table) -> Index {
        static INDEX: AtomicU64 = AtomicU64::new(0);
        let index = INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.objects.insert(index, Object::new(table));

        index
    }
}

pub enum InnerFunctionEvent {
    Update,
}

impl InnerFunctionEvent {
    pub fn array() -> [Self; 1] {
        [Self::Update]
    }

    pub fn name(&self) -> &str {
        match self {
            InnerFunctionEvent::Update => "update",
        }
    }
}
