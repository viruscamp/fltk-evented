use fltk::{
    app,
    enums::Event,
    prelude::{WidgetBase, WidgetExt},
};
use std::sync::atomic::{AtomicBool, Ordering, AtomicI32};
use std::sync::Arc;
use crate::base::{BaseListenerWidget, ValueListener};

#[cfg(feature = "tokio")]
use tokio::spawn;

#[cfg(feature = "async-std")]
use async_std::task::spawn;


/// The async widget listener for compatibility, just same as [`ListenerWidget`].
pub type AsyncListener<T> = BaseListenerWidget<T, DualListener>;

/// A async widget listener recieves both `triggered: bool` from [`ListenerWidget<T>::triggered`],
/// and [`Event`] from [`ListenerWidget<T>::event`].
/// This is a combine of [`TriggeredWidget`] and [`EventWidget`].
pub type ListenerWidget<T> = BaseListenerWidget<T, DualListener>;

/// A async widget listener recieves `triggered: bool` from [`TriggeredWidget<T>::triggered`],
/// calls [`WidgetExt::set_callback`] to register.
pub type TriggeredWidget<T> = BaseListenerWidget<T, TriggeredListener>;

/// A async widget listener recieves [`Event`] from [`EventWidget<T>::event`],
/// calls [`WidgetBase::handle`] to register.
pub type EventWidget<T> = BaseListenerWidget<T, EventListener>;


pub struct TriggeredListener(Arc<AtomicBool>);

impl<T: WidgetBase + WidgetExt> ValueListener<T, bool> for TriggeredListener {
    fn new(wid: &mut T) -> Self {
        let triggered = Arc::new(AtomicBool::new(false));
        wid.set_callback({
            let triggered = triggered.clone();
            move |_| {
                let triggered = triggered.clone();
                spawn(async move {
                    triggered.store(true, Ordering::Relaxed);
                    app::awake();
                });
            }
        });
        TriggeredListener(triggered)
    }

    fn value(&self) -> bool {
        self.0.swap(false, Ordering::Relaxed)
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
    pub async fn triggered(&self) -> bool {
        ValueListener::<T, _>::value(&self.trig)
    }
}


pub struct EventListener(Arc<AtomicI32>);

impl<T: WidgetBase + WidgetExt> ValueListener<T, Event> for EventListener {
    fn new(wid: &mut T) -> Self {
        let event = Arc::new(AtomicI32::new(Event::NoEvent.bits()));
        wid.handle({
            let event = event.clone();
            move |_, evt| {
                let event = event.clone();
                spawn(async move {
                    event.store(evt.bits(), Ordering::Relaxed);
                    app::awake();
                });
                false
            }
        });
        EventListener(event)
    }

    fn value(&self) -> Event {
        self.0.swap(Event::NoEvent.bits(), Ordering::Relaxed).into()
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
    pub async fn event(&self) -> Event {
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
    pub async fn triggered(&self) -> bool {
        ValueListener::<T, _>::value(&self.trig.0)
    }

    /// Get an event the widget received,
    /// returns [`Event::NoEvent`] if no events received
    pub async fn event(&self) -> Event {
        ValueListener::<T, _>::value(&self.trig.1)
    }
}
