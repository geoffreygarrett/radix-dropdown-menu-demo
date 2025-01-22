use floating_ui_leptos::{use_floating, Alignment, ApplyState, Arrow, ArrowData, ArrowOptions, AutoUpdateOptions, Boundary, DetectOverflowOptions, Flip, FlipOptions, Hide, HideData, HideOptions, HideStrategy, LimitShift, LimitShiftOptions, Middleware, MiddlewareReturn, MiddlewareState, MiddlewareVec, Offset, OffsetOptions, OffsetOptionsValues, Padding, PartialSideObject, Placement, Shift, ShiftOptions, Side, Size, SizeOptions, Strategy, UseFloatingOptions, UseFloatingReturn, WrappedMiddleware, ARROW_NAME, HIDE_NAME};

use leptos::{html, prelude::*};
use leptos::context::Provider;
use leptos::logging::{debug_warn, log};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_arrow::Arrow as ArrowPrimitive;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_primitive::{Primitive};
use radix_leptos_use_size::use_size;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Align {
    pub fn alignment(self) -> Option<Alignment> {
        match self {
            Align::Start => Some(Alignment::Start),
            Align::Center => None,
            Align::End => Some(Alignment::End),
        }
    }
}

impl From<Option<Alignment>> for Align {
    fn from(value: Option<Alignment>) -> Self {
        match value {
            Some(Alignment::Start) => Align::Start,
            Some(Alignment::End) => Align::End,
            None => Align::Center,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Sticky {
    Partial,
    Always,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UpdatePositionStrategy {
    Optimized,
    Always,
}

#[derive(Clone)]
struct PopperContextValue {
    pub anchor_ref: AnyNodeRef,
}

#[component]
#[allow(non_snake_case)]
pub fn Popper<C: IntoView + 'static>(children: TypedChildren<C>) -> impl IntoView {
    let anchor_ref: AnyNodeRef = AnyNodeRef::new();

    let context_value = PopperContextValue { anchor_ref };

    view! { <Provider value=context_value>{children.into_inner()()}</Provider> }
}

#[component]
#[allow(non_snake_case)]
pub fn PopperAnchor<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
) -> impl IntoView
{
    let context: PopperContextValue = expect_context();
    let composed_refs = use_composed_refs((node_ref, context.anchor_ref));

    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        debug_warn!("[PopperAnchor] Composed ref value: {:?}", composed_refs.get());
        debug_warn!("[PopperAnchor] Original anchor ref: {:?}", context.anchor_ref.get());
        debug_warn!("[PopperAnchor] Passed node ref: {:?}", node_ref.get());
    });

    view! { <Primitive children=children element=html::div as_child=as_child node_ref=composed_refs /> }
}

#[derive(Clone)]
struct PopperContentContextValue {
    pub placed_side: Signal<Side>,
    pub arrow_ref: AnyNodeRef,
    pub arrow_x: Signal<Option<f64>>,
    pub arrow_y: Signal<Option<f64>>,
    pub should_hide_arrow: Signal<bool>,
}

#[component]
#[allow(non_snake_case)]
pub fn PopperContent<C: IntoView + 'static>(
    #[prop(into, optional)] side: MaybeProp<Side>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_boundary: MaybeProp<Vec<AnyNodeRef>>,
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] update_position_strategy: MaybeProp<UpdatePositionStrategy>,
    #[prop(into, optional)] on_placed: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let side = move || side.get().unwrap_or(Side::Bottom);
    let side_offset = move || side_offset.get().unwrap_or(0.0);
    let align = move || align.get().unwrap_or(Align::Center);
    let align_offset = move || align_offset.get().unwrap_or(0.0);
    let arrow_padding = move || arrow_padding.get().unwrap_or(0.0);
    let avoid_collisions = move || avoid_collisions.get().unwrap_or(true);
    let collision_boundary = move || collision_boundary.get().unwrap_or_default();
    let collision_padding = move || collision_padding.get().unwrap_or(Padding::All(0.0));
    let sticky = move || sticky.get().unwrap_or(Sticky::Partial);
    let hide_when_detached = move || hide_when_detached.get().unwrap_or(false);
    let update_position_strategy = move || {
        update_position_strategy
            .get()
            .unwrap_or(UpdatePositionStrategy::Optimized)
    };

    let context: PopperContextValue = expect_context();

    let content_ref: AnyNodeRef = AnyNodeRef::new();
    let composed_refs = use_composed_refs([node_ref, content_ref]);

    let arrow_ref: AnyNodeRef = AnyNodeRef::new();
    let arrow_size = use_size(arrow_ref);
    let arrow_width = move || {
        arrow_size
            .get()
            .map(|arrow_size| arrow_size.width)
            .unwrap_or(0.0)
    };
    let arrow_height = move || {
        arrow_size
            .get()
            .map(|arrow_size| arrow_size.height)
            .unwrap_or(0.0)
    };


    let desired_placement = Signal::derive(move || Placement::from((side(), align().alignment())));

    let floating_ref: AnyNodeRef = AnyNodeRef::new();

    let UseFloatingReturn {
        floating_styles,
        placement,
        is_positioned,
        middleware_data,
        ..
    } = use_floating(
        context.anchor_ref,
        floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed.into())
            .placement(desired_placement.into())
            // .while_elements_mounted_auto_update()
            .while_elements_mounted_auto_update_with_options(Signal::derive(move || {
                AutoUpdateOptions::default()
                    .animation_frame(update_position_strategy() == UpdatePositionStrategy::Always)
            }))
            .middleware(MaybeProp::derive(move || {
                let detect_overflow_options = DetectOverflowOptions::default()
                    .padding(collision_padding())
                    .boundary(Boundary::Elements(collision_boundary().clone()))
                    .alt_boundary(!collision_boundary().is_empty());

                let mut middleware: MiddlewareVec =
                    vec![Box::new(Offset::new(OffsetOptions::Values(
                        OffsetOptionsValues::default()
                            .main_axis(side_offset() + arrow_height())
                            .alignment_axis(align_offset()),
                    )))
                    ];

                if avoid_collisions() {
                    let detect_overflow_options = DetectOverflowOptions::<web_sys::Element>::default()
                        .boundary(Boundary::ClippingAncestors)
                        .padding(Padding::All(0.0));

                    let mut shift_options = ShiftOptions::default()
                        .detect_overflow(detect_overflow_options.clone())
                        .main_axis(true)
                        .cross_axis(false);

                    if sticky() == Sticky::Partial {
                        shift_options = shift_options
                            .limiter(Box::new(LimitShift::new(LimitShiftOptions::default())));
                    }

                    middleware.push(Box::new(Shift::new(shift_options)));
                    middleware.push(Box::new(Flip::new(
                        FlipOptions::default().detect_overflow(detect_overflow_options),
                    )));
                }

                // For uniform padding on all sides
                let detect_overflow_options: DetectOverflowOptions<web_sys::Element> = DetectOverflowOptions::default()
                    .boundary(Boundary::ClippingAncestors)
                    .padding(Padding::All(0.0));

                middleware.push(Box::new(Size::new(
                    SizeOptions {
                        detect_overflow: Some(detect_overflow_options.clone()),
                        apply: Some(&move |ApplyState {
                                               state,
                                               available_width,
                                               available_height,
                                           }| {
                            let MiddlewareState {
                                elements, rects, ..
                            } = state;

                            // Clone and then cast to HtmlElement
                            let content_style = elements.floating.clone()
                                .unchecked_into::<web_sys::HtmlElement>()
                                .style();

                            let _ = content_style.set_property(
                                "--radix-popper-available-width",
                                &format!("{}px", available_width),
                            );
                            let _ = content_style.set_property(
                                "--radix-popper-available-height",
                                &format!("{}px", available_height),
                            );
                            let _ = content_style.set_property(
                                "--radix-popper-anchor-width",
                                &format!("{}px", rects.reference.width),
                            );
                            let _ = content_style.set_property(
                                "--radix-popper-anchor-height",
                                &format!("{}px", rects.reference.height),
                            );
                        }),
                    }
                )));

                middleware.push(Box::new(Arrow::new(
                    ArrowOptions::new(arrow_ref).padding(Padding::All(arrow_padding())),
                )));

                middleware.push(Box::new(TransformOrigin::new(TransformOriginOptions {
                    arrow_width: arrow_width(),
                    arrow_height: arrow_height(),
                })));

                if hide_when_detached() {
                    let detect_overflow_options: DetectOverflowOptions<web_sys::Element> = DetectOverflowOptions::default()
                        .boundary(Boundary::ClippingAncestors)
                        .padding(Padding::All(0.0));

                    middleware.push(Box::new(Hide::new(
                        HideOptions::<web_sys::Element>::default()  // Only needs Element type
                            .detect_overflow(detect_overflow_options)
                            .strategy(HideStrategy::ReferenceHidden),
                    )));
                }

                // Wrap the middleware vector in SendWrapper to match WrappedMiddleware type
                Some(WrappedMiddleware::new(middleware))
            })),
    );

    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        // Log anchor position
        if let Some(anchor) = context.anchor_ref.get() {
            let rect = anchor.get_bounding_client_rect();
            log!("Anchor position: top={}, bottom={}, height={}, y={}",
            rect.top(), rect.bottom(), rect.height(), rect.y());
        }
    });

    // Add these effects to debug arrow sizing
    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        if let Some(arrow_element) = arrow_ref.get() {
            let rect = arrow_element.get_bounding_client_rect();
            log!("ARROW DEBUG - DOM measurements: height={}, width={}", rect.height(), rect.width());
        }
    });

    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        log!("ARROW DEBUG - use_size values: {:?}", arrow_size.get());
        log!("ARROW DEBUG - computed height: {}", arrow_height());
    });

    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        // Log floating element position and styles
        if let Some(floating) = floating_ref.get() {
            let rect = floating.get_bounding_client_rect();
            log!("Floating element position: top={}, bottom={}, height={}, y={}",
            rect.top(), rect.bottom(), rect.height(), rect.y());

            // Get computed style
            if let Ok(Some(style)) = window().get_computed_style(&floating) {
                log!("Computed styles: position={}, top={}, left={}, transform={}",
                style.get_property_value("position").unwrap_or_default(),
                style.get_property_value("top").unwrap_or_default(),
                style.get_property_value("left").unwrap_or_default(),
                style.get_property_value("transform").unwrap_or_default()
            );
            }
        }
    });

    // Add logging for middleware data
    Effect::new(move |_| {
        if is_positioned.get() {
            let data = middleware_data.get();
            log!("Middleware data: {:?}", data);
        }
    });

    // Track ref changes
    Effect::new(move |_| {
        if let Some(anchor) = context.anchor_ref.get() {
            log!("Anchor ref changed: tag={}, id={}",
            anchor.tag_name().to_lowercase(),
            anchor.id()
        );
        }
        if let Some(floating) = floating_ref.get() {
            log!("Floating ref changed: tag={}, id={}",
            floating.tag_name().to_lowercase(),
            floating.id()
        );
        }
    });

    //
    // let UseFloatingReturn {
    //     floating_styles,
    //     placement,
    //     is_positioned,
    //     middleware_data,
    //     ..
    // } = use_floating(
    //     context.anchor_ref,
    //     floating_ref,
    //     UseFloatingOptions::default()
    //         .strategy(Strategy::Fixed.into())
    //         .placement(desired_placement.into())
    //         .while_elements_mounted_auto_update()  // Simplified auto-update
    //         .middleware(MaybeProp::derive(move || {
    //             let mut middleware: MiddlewareVec = vec![
    //                 // Basic offset for spacing from anchor
    //                 Box::new(Offset::new(OffsetOptions::Values(
    //                     OffsetOptionsValues::default()
    //                         .main_axis(8.0)  // Fixed offset instead of dynamic calculation
    //                         .alignment_axis(0.0),
    //                 ))),
    //                 // Basic flip and shift for collision detection
    //                 Box::new(Flip::new(FlipOptions::default())),
    //                 Box::new(Shift::new(ShiftOptions::default())),
    //                 // Size middleware for CSS variables
    //                 Box::new(Size::new(SizeOptions {
    //                     apply: Some(&move |ApplyState { state, available_width, available_height }| {
    //                         let content_style = state.elements.floating
    //                             .clone()
    //                             .unchecked_into::<web_sys::HtmlElement>()
    //                             .style();
    //
    //                         let _ = content_style.set_property(
    //                             "--radix-popper-available-width",
    //                             &format!("{}px", available_width),
    //                         );
    //                         let _ = content_style.set_property(
    //                             "--radix-popper-available-height",
    //                             &format!("{}px", available_height),
    //                         );
    //                     }),
    //                     ..Default::default()
    //                 })),
    //             ];
    //
    //             Some(WrappedMiddleware::new(middleware))
    //         })),
    // );

    Effect::new(move |_| {
        log!("Position state changed - is_positioned: {}", is_positioned.get());
        log!("Anchor ref exists: {:?}", context.anchor_ref.get().is_some());
        log!("Floating ref exists: {:?}", floating_ref.get().is_some());
        if is_positioned.get() {
            log!("Placement: {:?}", placement.get());
            log!("Floating styles: {:?}", floating_styles.get());
        }
    });

    let placed_side = Signal::derive(move || placement.get().side());
    let placed_align = move || Align::from(placement.get().alignment());

    Effect::new(move |_| {
        if is_positioned.get() {
            if let Some(on_placed) = on_placed {
                on_placed.run(());
            }
        }
    });

    let arrow_data = move || -> Option<ArrowData> { middleware_data.get().get_as(ARROW_NAME) };
    let arrow_x = Signal::derive(move || arrow_data().and_then(|arrow_data| arrow_data.x));
    let arrow_y = Signal::derive(move || arrow_data().and_then(|arrow_data| arrow_data.y));
    let cannot_center_arrow = Signal::derive(move || {
        arrow_data().is_none_or(|arrow_data| arrow_data.center_offset != 0.0)
    });

    let (content_z_index, set_content_z_index) = create_signal::<Option<String>>(None);
    Effect::new(move |_| {
        if let Some(content) = composed_refs.get() {
            set_content_z_index.set(Some(
                window()
                    .get_computed_style(&content)
                    .expect("Element is valid.")
                    .expect("Element should have computed style.")
                    .get_property_value("z-index")
                    .expect("Computed style should have z-index."),
            ));
        }
    });

    let transform_origin_data = move || -> Option<TransformOriginData> {
        middleware_data.get().get_as(TRANSFORM_ORIGIN_NAME)
    };
    let transform_origin = move || {
        transform_origin_data().map(|transform_origin_data| {
            format!("{} {}", transform_origin_data.x, transform_origin_data.y)
        })
    };
    let hide_data = move || -> Option<HideData> { middleware_data.get().get_as(HIDE_NAME) };
    let reference_hidden = move || {
        hide_data()
            .and_then(|hide_data| hide_data.reference_hidden)
            .unwrap_or(false)
    };

    // let dir = attrs
    //     .iter()
    //     .find_map(|(key, value)| (*key == "dir").then_some(value.clone()));

    let content_context_value = PopperContentContextValue {
        placed_side,
        arrow_ref,
        arrow_x,
        arrow_y,
        should_hide_arrow: cannot_center_arrow,
    };

    view! {
        <div
            node_ref=floating_ref
            data-radix-popper-content-wrapper=""
            data-side=move || format!("{:?}", placed_side.get()).to_lowercase()
            data-align=move || format!("{:?}", placed_align()).to_lowercase()
            style:animation=move || if !is_positioned.get() { "none" } else { "" }.to_string()
            style:position=move || floating_styles.get().style_position()
            style:top=move || floating_styles.get().style_top()
            style:left=move || floating_styles.get().style_left()
            style:transform=move || {
                if !is_positioned.get() {
                    "translate(0, -200%)".to_string()
                } else {
                    floating_styles.get().style_transform().unwrap_or_default()
                }
            }
            style:will-change=move || floating_styles.get().style_will_change().unwrap_or_default()
            style:min-width="max-content"
            style:z-index="50"
            // style:z-index=move || content_z_index.get().unwrap_or_default()
            style=("--radix-popper-transform-origin", transform_origin().unwrap_or_default())
            // style:radix-popper-transform-origin=transform_origin
            // Hide the content if using the hide middleware and should be hidden set visibility to hidden
            // and disable pointer events so the UI behaves as if the PopperContent isn't there at all.
            style:visibility=move || if reference_hidden() { "hidden" } else { "" }
            style:pointer-events=move || if reference_hidden() { "none" } else { "auto" }
        >
            // the reference/floating node, we must add this attribute here to ensure
            // this is calculated when portalled as well as inline.
            // dir={dir}
            <Provider value=content_context_value>
                <Primitive element=html::div as_child=as_child node_ref=composed_refs>
                    {children.with_value(|children| children())}
                </Primitive>
            </Provider>
        </div>
    }
}

#[component]
pub fn PopperArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let content_context: PopperContentContextValue = expect_context();
    let arrow_ref = content_context.arrow_ref;
    let base_side = move || content_context.placed_side.get().opposite();

    let left = move || match base_side() {
        Side::Left => "0px".to_string(),
        _ => content_context.arrow_x.get()
            .map(|arrow_x| format!("{}px", arrow_x))
            .unwrap_or_default()
    };

    let top = move || match base_side() {
        Side::Top => "0px".to_string(),
        _ => content_context.arrow_y.get()
            .map(|arrow_y| format!("{}px", arrow_y))
            .unwrap_or_default()
    };

    let right = move || match base_side() {
        Side::Right => "0px".to_string(),
        _ => String::new()
    };

    let bottom = move || match base_side() {
        Side::Bottom => "0px".to_string(),
        _ => String::new()
    };

    let transform_origin = move || match content_context.placed_side.get() {
        Side::Top => "",
        Side::Right => "0 0",
        Side::Bottom => "center 0",
        Side::Left => "100% 0"
    };

    let transform = move || match content_context.placed_side.get() {
        Side::Top => "translateY(100%)",
        Side::Right => "translateY(50%) rotate(90deg) translateX(-50%)",
        Side::Bottom => "rotate(180deg)",
        Side::Left => "translateY(50%) rotate(-90deg) translateX(50%)"
    };

    let visibility = move || {
        if content_context.should_hide_arrow.get() {
            "hidden"
        } else {
            "visible"
        }
    };

    view! {
        <span
            style:position="absolute"
            style:left=left
            style:top=top
            style:right=right
            style:bottom=bottom
            style:transform-origin=transform_origin
            style:transform=transform
            style:visibility=visibility
            node_ref=arrow_ref
        >
            <ArrowPrimitive
                width=width
                height=height
                as_child=as_child
                node_ref=node_ref
                style:display="block"
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </ArrowPrimitive>
        </span>
    }
}

const TRANSFORM_ORIGIN_NAME: &str = "transformOrigin";

/// Options for [`TransformOrigin`] middleware.
#[derive(Clone, PartialEq)]
struct TransformOriginOptions {
    arrow_width: f64,
    arrow_height: f64,
}

/// Data stored by [`TransformOrigin`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TransformOriginData {
    pub x: String,
    pub y: String,
}

#[derive(Clone, PartialEq)]
struct TransformOrigin {
    options: TransformOriginOptions,
}

impl TransformOrigin {
    fn new(options: TransformOriginOptions) -> Self {
        Self { options }
    }
}

impl Middleware<web_sys::Element, web_sys::Window> for TransformOrigin {
    fn name(&self) -> &'static str {
        TRANSFORM_ORIGIN_NAME
    }

    fn compute(
        &self,
        state: MiddlewareState<web_sys::Element, web_sys::Window>,
    ) -> MiddlewareReturn {
        let MiddlewareState {
            placement,
            rects,
            middleware_data,
            ..
        } = state;

        let arrow_data: Option<ArrowData> = middleware_data.get_as(ARROW_NAME);
        let cannot_center_arrow = arrow_data
            .as_ref()
            .is_none_or(|arrow_data| arrow_data.center_offset != 0.0);
        let is_arrow_hidden = cannot_center_arrow;
        let arrow_width = match is_arrow_hidden {
            true => 0.0,
            false => self.options.arrow_width,
        };
        let arrow_height = match is_arrow_hidden {
            true => 0.0,
            false => self.options.arrow_height,
        };

        let placed_side = placement.side();
        let placed_align = Align::from(placement.alignment());
        let no_arrow_align = match placed_align {
            Align::Start => "0%",
            Align::Center => "50%",
            Align::End => "100%",
        };

        let arrow_x_center = arrow_data
            .as_ref()
            .and_then(|arrow_data| arrow_data.x)
            .unwrap_or(0.0)
            + arrow_width / 2.0;
        let arrow_y_center = arrow_data
            .as_ref()
            .and_then(|arrow_data| arrow_data.y)
            .unwrap_or(0.0)
            + arrow_height / 2.0;

        let (x, y) = match placed_side {
            Side::Top => (
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{}px", arrow_x_center),
                },
                format!("{}px", rects.floating.height + arrow_height),
            ),
            Side::Right => (
                format!("{}px", -arrow_height),
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{}px", arrow_y_center),
                },
            ),
            Side::Bottom => (
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{}px", arrow_x_center),
                },
                format!("{}px", -arrow_height),
            ),
            Side::Left => (
                format!("{}px", rects.floating.width + arrow_height),
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{}px", arrow_y_center),
                },
            ),
        };

        MiddlewareReturn {
            x: None,
            y: None,
            data: Some(
                serde_json::to_value(TransformOriginData { x, y })
                    .expect("Data should be valid JSON."),
            ),
            reset: None,
        }
    }
}
