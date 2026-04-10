use log::{debug, info};

use crate::router::{
    Route,
    tree::{
        condition::{ConditionTree, ConditionType},
        pass::Pass,
    },
};

#[const_env::env_item]
const METHOD_PASS_N_ROUTE_THRESHOLD: usize = 8;

#[derive(Debug, Default)]
pub struct Length;

impl Length {
    fn partition(
        pass: &mut dyn Pass,
        routes: im::Vector<Route>,
    ) -> im::Vector<(ConditionType, im::Vector<Route>)> {
        let mut routes = routes.clone();
        let mut partitions: im::Vector<(ConditionType, im::Vector<Route>)> = im::Vector::new();

        while let Some(r) = routes.pop_front() {
            debug!("{:#?}", r);
            if let Some(existing) = partitions
                .iter_mut()
                .find(|x| x.0 == ConditionType::Length(r.get_path_str().len()))
            {
                existing.1.push_back(r);
            } else {
                info!("adding {}", r.get_path_str().len());
                let m = ConditionType::Length(r.get_path_str().len());
                let mut v = im::Vector::new();
                v.push_back(r);

                partitions.push_back((m, v));
            }
        }

        partitions
    }
}

impl Pass for Length {
    fn run(&mut self, tree: &mut ConditionTree) {
        tree.insert_root_condition(self, Self::partition);
    }
}
