use leptos::prelude::*;
use leptos_use::core::IntoElementMaybeSignal;
use leptos_use::use_element_size;

#[derive(Clone, Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub fn use_size<El, M>(element_ref: El) -> ReadSignal<Option<Size>>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>
{
    let element_size = use_element_size(element_ref);
    let (size, set_size) = signal(None);

    Effect::new(move |_| {
        set_size.set(Some(Size {
            width: element_size.width.get(),
            height: element_size.height.get(),
        }));
    });

    size
}

/*
Original implementation to be ported:

pub fn use_size<E>(element_ref: NodeRef<E>) -> ReadSignal<Option<Size>>
where
    E: ElementType,
    E::Output: Clone + JsCast + 'static,
{
    let (size, set_size) = signal::<Option<Size>>(None);

    let resize_observer: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    let cleanup_resize_observer = resize_observer.clone();

    Effect::new(move |_| {
        if let Some(element) = element_ref.get() {
            // Provide size as early as possible.
            set_size.set(Some(Size {
                width: element.offset_width() as f64,
                height: element.offset_height() as f64,
            }));

            let resize_closure: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
                Closure::new(move |entries: Vec<ResizeObserverEntry>| {
                    if let Some(entry) = entries.first() {
                        let border_size_entry = entry.border_box_size().at(0);

                        if let Some(border_size_entry) =
                            border_size_entry.dyn_ref::<ResizeObserverSize>()
                        {
                            set_size.set(Some(Size {
                                width: border_size_entry.inline_size(),
                                height: border_size_entry.block_size(),
                            }));
                        }
                    }
                });

            resize_observer.replace(Some(
                ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
                    .expect("Resize observer should be created."),
            ));

            let options = ResizeObserverOptions::new();
            options.set_box(ResizeObserverBoxOptions::BorderBox);

            resize_observer
                .borrow()
                .as_ref()
                .expect("Resize observer should exist.")
                .observe_with_options(element.as_ref(), &options);
        } else {
            // We only want to reset to `None` when the element becomes `None`, not if it changes to another element.
            set_size.set(None);
        }
    });

    on_cleanup(move || {
        if let Some(resize_observer) = cleanup_resize_observer.take() {
            resize_observer.disconnect();
        }
    });

    size
}
*/