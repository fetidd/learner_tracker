use web_sys::HtmlElement;
use yew::prelude::*;

pub type ModalCallbacks = (Callback<Html>, Callback<MouseEvent>);

#[function_component(ModalProvider)]
pub fn modal_provider(props: &ModalProviderProps) -> Html {
    let html_handle = use_state(|| Option::<Html>::None);
    let modal_dialog = use_node_ref();
    let modal_backdrop = use_node_ref();
    let is_displayed = use_state(|| false);

    let invoke_callback: Callback<Html> = {
        clone!(html_handle, modal_dialog, modal_backdrop, is_displayed);
        Callback::from(move |html: Html| {
            clone!(html_handle, modal_dialog, modal_backdrop, is_displayed);
            html_handle.set(Some(html));
            open_modal(modal_dialog, modal_backdrop, is_displayed);
        })
    };

    let dismiss_callback: Callback<MouseEvent> = {
        clone!(modal_dialog, modal_backdrop, is_displayed);
        Callback::from(move |_| {
            clone!(modal_dialog, modal_backdrop, is_displayed);
            close_modal(modal_dialog, modal_backdrop, is_displayed);

        })
    };

    let modal_callbacks = (invoke_callback, dismiss_callback);

    if html_handle.is_some() {
        html! {
            <>
            <ContextProvider<ModalCallbacks> context={modal_callbacks}>
                {for props.children.iter()}
                <p>{"hello"}</p>
            </ContextProvider<ModalCallbacks>>

            <div ref={modal_backdrop} class="hidden modal-backdrop">
                <div ref={modal_dialog} class="hidden modal-dialog">
                    {(*html_handle).clone()} // insert the html for the dialog box
                </div>
            </div>
            </>
        }
    } else {
        html!()
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
) {
    let modal_dialog: HtmlElement = modal_dialog.cast().expect("cast modal to htmlelement");
    let mut box_classes = Classes::from(modal_dialog.class_name());
    let backdrop = modal_backdrop
        .cast::<HtmlElement>()
        .expect("cast modal_backdrop to htmlelement");
    let mut backdrop_classes = Classes::from(backdrop.class_name());
    if !*is_displayed {
        box_classes = box_classes
            .into_iter()
            .filter(|cl| cl != "hidden")
            .collect();
        backdrop_classes = backdrop_classes
            .into_iter()
            .filter(|cl| cl != "hidden")
            .collect();
        is_displayed.set(true);
    }
    modal_dialog.set_class_name(&box_classes.to_string());
    backdrop.set_class_name(&backdrop_classes.to_string());
}

fn close_modal(
    modal_dialog: NodeRef,
    modal_backdrop: NodeRef,
    is_displayed: UseStateHandle<bool>,
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