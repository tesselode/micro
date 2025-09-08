use crate::{Entities, Events, World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Event(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Event2(i32);

fn print_event(entities: &mut Entities, events: &mut Events, event: &Event) {
	dbg!(event);
	events.emit(Event2(0));
}

fn print_event2(entities: &mut Entities, events: &mut Events, event: &Event2) {
	dbg!(event);
}

#[test]
fn runs_triggers() {
	let mut world = World::new()
		.with_trigger(print_event)
		.with_trigger(print_event2);
	world.emit_event(Event(10));
	world.emit_event(Event(20));
	world.emit_event(Event2(10));
	world.dispatch_events();
}
