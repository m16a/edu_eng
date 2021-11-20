use std::collections::{HashMap};
use std::any::{Any, TypeId};


type EID = u64;
//const EID_INVALID: EID = 0;

// pub struct Entity{
//     eid : EID
// }

pub trait Component{
    fn new() -> Self;
}

pub trait Storage : Any{

}

struct ComponentStorage<T: Component>{
    components : Vec<T>,

    //hash map: ent_id to index
    //hash map: index to ent_id

}

impl<T: 'static + Component> Storage for ComponentStorage<T>{
}

impl<T: Component> ComponentStorage<T> {
    pub fn new() -> ComponentStorage<T>  {
        let components = Vec::new();
        ComponentStorage { components }
    }

    #[allow(dead_code)]
    pub fn add(&mut self) {
        let c = T::new();
        self.components.push(c);
    }
}
trait System{
    fn update();
}

pub struct ECS{
    entities : Vec<EID>,
    next_free_eid : EID,
    //system vec
    components_store: HashMap<std::any::TypeId, Box<dyn Storage>>
}


impl ECS {
    pub fn new() -> ECS {
        let storages = HashMap::new();
        let entities = Vec::new();
        let next_free_eid = 1;
        ECS {
            entities: entities,
            components_store: storages,
            next_free_eid: next_free_eid
        }
    }

    pub fn update(&mut self) {
        println!("ECS udpate");
    }

    pub fn register_component<T: 'static + Component>(&mut self) {
        let t = TypeId::of::<T>();
        let o = self.components_store.get(&t);

        match o {
            Some(_x) => println!("Type {} was added already", std::any::type_name::<T>()),
            None => {
                self.components_store.insert(t, Box::new(ComponentStorage::<T>::new()));
                println!("Type {} was registered", std::any::type_name::<T>());
            }
        }
    }

    pub fn create_entity(&mut self) -> EID {
        let new_eid = self.next_free_eid;
        self.entities.push(new_eid);
        self.next_free_eid = self.next_free_eid + 1;
        new_eid
    }
            //     pub fn add_component_to_entity<T: 'static + Component + Any>(&mut self, eid: EID) {
            //         let t = TypeId::of::<T>();
            //         let o = self.components_store.get(&t);

            //         match o {
            //             Some(x) => {
            //                 let c = T::new();
            //                 x.downcast_ref::<T>().unwrawp().push(c);
            //             },
            //             None => {
            //     println!("Type {} is not registered", std::any::type_name::<T>());
            // }
        //}
    //}

    #[warn(unused_variables)]
    pub fn get_component<T: Component>(&self, _eid: EID) {
        //get storage

        //obtain component ref from storage
    }

    #[warn(unused_variables)]
    pub fn remove_component<T: Component>(&mut self, _eid: EID) {

    }
}