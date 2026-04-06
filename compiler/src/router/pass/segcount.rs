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
pub struct Segcount;

impl Segcount {
    fn partition(
        pass: &mut dyn Pass,
        routes: im::Vector<Route>,
    ) -> im::Vector<(ConditionType, im::Vector<Route>)> {
        let mut routes = routes.clone();
        let mut partitions: im::Vector<(ConditionType, im::Vector<Route>)> = im::Vector::new();

        while let Some(r) = routes.pop_front() {
            if let Some(existing) = partitions
                .iter_mut()
                .find(|x| x.0 == ConditionType::SegmentCount(r.path.len()))
            {
                existing.1.push_back(r);
            } else {
                let m = ConditionType::SegmentCount(r.path.len());
                let mut v = im::Vector::new();
                v.push_back(r);

                partitions.push_back((m, v));
            }
        }

        partitions
    }
}

impl Pass for Segcount {
    fn run(&mut self, tree: &mut ConditionTree) {
        tree.insert_root_condition(self, Self::partition);
    }
}
