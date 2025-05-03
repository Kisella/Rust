use core::str;
use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};

struct Case {
    master_ifs: HashMap<&'static str, Rc<RefCell<MasterInterface>>>,
    sub_ifs: HashMap<&'static str, Rc<RefCell<SubInterface>>>,
    warm_standby_groups: HashMap<&'static str, Rc<RefCell<WarmStanbyGroup>>>,
    hot_standby_groups: HashMap<&'static str, Rc<RefCell<SubInterface>>>,
}

struct MasterInterface {
    name: &'static str,
    sub_interface: HashMap<&'static str, Rc<RefCell<SubInterface>>>,
    warm_standby_group: Option<Weak<RefCell<WarmStanbyGroup>>>,
    hot_standby_group: Option<Weak<RefCell<HotStaInterface>>>,
}

struct SubInterface {
    name: &'static str,
    hot_standby_group: Option<Weak<RefCell<HotStaInterface>>>,
}

struct WarmStanbyGroup {
    name: &'static str,
    master_if: HashMap<&'static str, Rc<RefCell<MasterInterface>>>,
}

enum InterfacePair {
    MASTERPAIR(Rc<RefCell<MasterInterface>>, Rc<RefCell<MasterInterface>>),
    SUBPAIR(Rc<RefCell<SubInterface>>, Rc<RefCell<SubInterface>>),
    NULL,
}

struct HotStaInterface {
    name: &'static str,
    interface_pairs: InterfacePair,
}

impl WarmStanbyGroup {
    pub fn new(name: &'static str) -> WarmStanbyGroup {
        WarmStanbyGroup {
            name,
            master_if: HashMap::new(),
        }
    }

    pub fn add_master_if(warm_standby_group: &mut Rc<RefCell<WarmStanbyGroup>>, master: &mut Rc<RefCell<MasterInterface>>) {
        let if_name:&'static str= (*master.borrow()).name;
        (*master.borrow_mut()).warm_standby_group = Some(Rc::downgrade(warm_standby_group));
        (*warm_standby_group.borrow_mut()).master_if.insert(if_name, master.clone());
    }
}
