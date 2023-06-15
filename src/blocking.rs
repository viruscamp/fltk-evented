use fltk::enums::Event;
use fltk::prelude::{WidgetBase, WidgetExt};
use std::cell::Cell;
use std::rc::Rc;
use crate::base::{BaseListenerWidget, ValueListener};

/// The blocking widget listener for compatibility, just same as [`ListenerWidget`].
pub type Listener<T> = BaseListenerWidget<T, DualListener>;

/// A widget listener recieves both `triggered: bool` from [`ListenerWidget<T>::triggered`],
/// and [`Event`] from [`ListenerWidget<T>::event`].
/// This is a combine of [`TriggeredWidget`] and [`EventWidget`].
pub type ListenerWidget<T> = BaseListenerWidget<T, DualListener>;

/// A widget listener recieves `triggered: bool` from [`TriggeredWidget<T>::triggered`],
/// calls [`WidgetExt::set_callback`] to register.
pub type TriggeredWidget<T> = BaseListenerWidget<T, TriggeredListener>;

/// A widget listener recieves [`Event`] from [`EventWidget<T>::event`],
/// calls [`WidgetBase::handle`] to register.
pub type EventWidget<T> = BaseListenerWidget<T, EventListener>;


pub struct TriggeredListener(Rc<Cell<bool>>);

impl<T: WidgetBase + WidgetExt> ValueListener<T, bool> for TriggeredListener {
    fn new(wid: &mut T) -> Self {
        let triggered = Rc::new(Cell::new(false));
        wid.set_callback({
            let triggered = triggered.clone();
            move |_| {
                triggered.set(true);
            }
        });
        TriggeredListener(triggered)
    }

    fn value(&self) -> bool {
        self.0.replace(false)
    }
}

impl<T: WidgetBase + WidgetExt> From<T> for TriggeredWidget<T> {
    fn from(mut wid: T) -> Self {
        let trig = TriggeredListener::new(&mut wid);
        Self { wid, trig }
    }
}

impl<T: WidgetBase + WidgetExt> TriggeredWidget<T> {
    /// Check whether a widget was triggered
    pub fn triggered(&self) -> bool {
        ValueListener::<T, _>::value(&self.trig)
    }
}


pub struct EventListener(Rc<Cell<Event>>);

impl<T: WidgetBase + WidgetExt> ValueListener<T, Event> for EventListener {
    fn new(wid: &mut T) -> Self {
        let event = Rc::new(Cell::new(Event::NoEvent));
        wid.handle({
            let event = event.clone();
            move |_, evt| {
                event.set(evt);
                true
            }
        });
        EventListener(event)
    }

    fn value(&self) -> Event {
        self.0.replace(Event::NoEvent)
    }
}

impl<T: WidgetBase + WidgetExt> From<T> for EventWidget<T> {
    fn from(mut wid: T) -> Self {
        let trig = EventListener::new(&mut wid);
        Self { wid, trig }
    }
}

impl<T: WidgetBase + WidgetExt> EventWidget<T> {
    /// Get an event the widget received,
    /// returns [`Event::NoEvent`] if no events received
    pub fn event(&self) -> Event {
        ValueListener::<T, _>::value(&self.trig)
    }
}


pub struct DualListener(TriggeredListener, EventListener);

impl<T: WidgetBase + WidgetExt> From<T> for ListenerWidget<T> {
    fn from(mut wid: T) -> Self {
        let triggered = TriggeredListener::new(&mut wid);
        let event = EventListener::new(&mut wid);
        let trig = DualListener(triggered, event);
        Self { wid, trig }
    }
}

impl<T: WidgetBase + WidgetExt> ListenerWidget<T> {
    /// Check whether a widget was triggered
    pub fn triggered(&self) -> bool {
        ValueListener::<T, _>::value(&self.trig.0)
    }

    /// Get an event the widget received,
    /// returns [`Event::NoEvent`] if no events received
    pub fn event(&self) -> Event {
        ValueListener::<T, _>::value(&self.trig.1)
    }
}
