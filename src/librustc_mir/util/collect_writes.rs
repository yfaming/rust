use rustc::mir::visit::PlaceContext;
use rustc::mir::visit::Visitor;
use rustc::mir::ReadOnlyBodyAndCache;
use rustc::mir::{Local, Location};

crate trait FindAssignments {
    // Finds all statements that assign directly to local (i.e., X = ...)
    // and returns their locations.
    fn find_assignments(&self, local: Local) -> Vec<Location>;
}

impl<'a, 'tcx> FindAssignments for ReadOnlyBodyAndCache<'a, 'tcx> {
    fn find_assignments(&self, local: Local) -> Vec<Location> {
        let mut visitor = FindLocalAssignmentVisitor { needle: local, locations: vec![] };
        visitor.visit_body(*self);
        visitor.locations
    }
}

// The Visitor walks the MIR to return the assignment statements corresponding
// to a Local.
struct FindLocalAssignmentVisitor {
    needle: Local,
    locations: Vec<Location>,
}

impl<'tcx> Visitor<'tcx> for FindLocalAssignmentVisitor {
    fn visit_local(&mut self, local: &Local, place_context: PlaceContext, location: Location) {
        if self.needle != *local {
            return;
        }

        if place_context.is_place_assignment() {
            self.locations.push(location);
        }
    }
}
