use web_sys::HtmlElement;
use yew::prelude::*;

pub type ModalCallbacks = (Callback<(MouseEvent, Html, Classes)>, Callback<MouseEvent>);

#[function_component(ModalProvider)]
pub fn modal_provider(props: &ModalProviderProps) -> Html {
    let html_handle = use_state(|| html!());
    let modal_dialog = use_node_ref();
    let modal_backdrop = use_node_ref();
    let is_displayed = use_state(|| false);

    let invoke_callback: Callback<(MouseEvent, Html, Classes)> = {
        clone!(html_handle, modal_dialog, modal_backdrop, is_displayed);
        Callback::from(
            move |(ev, html, extra_classes): (MouseEvent, Html, Classes)| {
                clone!(html_handle, modal_dialog, modal_backdrop, is_displayed);
                html_handle.set(html);
                open_modal(
                    modal_dialog,
                    modal_backdrop,
                    is_displayed,
                    extra_classes,
                    ev,
                );
            },
        )
    };

    let dismiss_callback: Callback<MouseEvent> = {
        clone!(modal_dialog, modal_backdrop, is_displayed, html_handle);
        Callback::from(move |ev| {
            clone!(modal_dialog, modal_backdrop, is_displayed, html_handle);
            close_modal(modal_dialog, modal_backdrop, is_displayed, ev);
            html_handle.set(html!());
        })
    };

    let modal_callbacks = (invoke_callback, dismiss_callback);

    html! {
        <>
        <ContextProvider<ModalCallbacks> context={modal_callbacks}>
            {for props.children.iter()}
        </ContextProvider<ModalCallbacks>>

        <div ref={modal_backdrop} class="hidden modal-backdrop">
            <div ref={modal_dialog} class="hidden modal-dialog">
                {(*html_handle).clone()} // insert the html for the dialog box
            </div>
        </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct ModalProviderProps {
    pub children: Children,
}

fn open_modal(
    modal_dialog: NodeRef,
    modal_backdrop: NodeRef,
    is_displayed: UseStateHandle<bool>,
    extra_classes: Classes,
    _event: MouseEvent,
) {
    let modal_dialog: HtmlElement = modal_dialog.cast().expect("cast modal to htmlelement");
    let mut dialog_classes = Classes::from(modal_dialog.class_name());
    let backdrop = modal_backdrop
        .cast::<HtmlElement>()
        .expect("cast modal_backdrop to htmlelement");
    let mut backdrop_classes = Classes::from(backdrop.class_name());
    if !*is_displayed {
        dialog_classes = dialog_classes
            .into_iter()
            .filter(|cl| cl != "hidden")
            .collect();
        backdrop_classes = backdrop_classes
            .into_iter()
            .filter(|cl| cl != "hidden")
            .collect();
        is_displayed.set(true);
    }
    dialog_classes.extend(extra_classes);
    modal_dialog.set_class_name(&dialog_classes.to_string());
    backdrop.set_class_name(&backdrop_classes.to_string());
}

fn close_modal(
    modal_dialog: NodeRef,
    modal_backdrop: NodeRef,
    is_displayed: UseStateHandle<bool>,
    _event: MouseEvent,
) {
    let modal_dialog: HtmlElement = modal_dialog.cast().expect("cast modal to htmlelement");
    let mut box_classes = Classes::from(modal_dialog.class_name());
    let backdrop = modal_backdrop
        .cast::<HtmlElement>()
        .expect("cast modal_backdrop to htmlelement");
    let mut backdrop_classes = Classes::from(backdrop.class_name());
    box_classes.push("hidden");
    backdrop_classes.push("hidden");
    is_displayed.set(false);
    modal_dialog.set_class_name(&box_classes.to_string());
    backdrop.set_class_name(&backdrop_classes.to_string());
}
