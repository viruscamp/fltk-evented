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


#[derive(Debug, Clone)]
pub struct TriggeredListener(Rc<Cell<bool>>);

impl<T: WidgetBase + WidgetExt> ValueListener<T> for TriggeredListener {
    type Value = bool;

    fn new(wid: &mut T) -> (Self, &mut T) {
        let triggered = Rc::new(Cell::new(false));
        wid.set_callback({
            let triggered = triggered.clone();
            move |_| {
                triggered.set(true);
            }
        });
        (TriggeredListener(triggered), wid)
    }

    fn value(&self) -> bool {
        self.0.replace(false)
    }
}

impl<T: WidgetBase + WidgetExt> TriggeredWidget<T> {
    /// Check whether a widget was triggered
    pub fn triggered(&self) -> bool {
        ValueListener::<T>::value(&self.trig)
    }
}


#[derive(Debug, Clone)]
pub struct EventListener(Rc<Cell<Event>>);

impl<T: WidgetBase + WidgetExt> ValueListener<T> for EventListener {
    type Value = Event;

    fn new(wid: &mut T) -> (Self, &mut T) {
        let event = Rc::new(Cell::new(Event::NoEvent));
        wid.handle({
            let event = event.clone();
            move |_, evt| {
                event.set(evt);
                true
            }
        });
        (EventListener(event), wid)
    }

    fn value(&self) -> Event {
        self.0.replace(Event::NoEvent)
    }
}

impl<T: WidgetBase + WidgetExt> EventWidget<T> {
    /// Get an event the widget received,
    /// returns [`Event::NoEvent`] if no events received
    pub fn event(&self) -> Event {
        ValueListener::<T>::value(&self.trig)
    }
}


#[derive(Debug, Clone)]
pub struct DualListener(TriggeredListener, EventListener);

impl<T: WidgetBase + WidgetExt> ValueListener<T> for DualListener {
    type Value = ();

    fn new(wid: &mut T) -> (Self, &mut T) {
        // the `&mut T` returned is used here
        let (triggered_listener, wid) = TriggeredListener::new(wid);
        let (event_listener, wid) = EventListener::new(wid);
        (Self(triggered_listener, event_listener), wid)
    }

    /// should not be called
    fn value(&self) {}
}

impl<T: WidgetBase + WidgetExt> ListenerWidget<T> {
    /// Check whether a widget was triggered
    pub fn triggered(&self) -> bool {
        ValueListener::<T>::value(&self.trig.0)
    }

    /// Get an event the widget received,
    /// returns [`Event::NoEvent`] if no events received
    pub fn event(&self) -> Event {
        ValueListener::<T>::value(&self.trig.1)
    }
}
