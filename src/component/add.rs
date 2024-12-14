use leptos::*;
use leptos::prelude::{RenderHtml, ElementChild, ClassAttribute};
use validator::Validate;
mod appserve;
use appserve::erreur::*;
use appserve::fonction::*;
use appserve::class::*;

#[component]
pub fn AddPersonModal(
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_added: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {

    let (person_name, set_person_name) = create_signal(String::new());
    let (person_title, set_person_title) = create_signal(String::new());
    let (person_categories, set_person_categories) = create_signal(String::new());
    

    let on_close = move |_| {
        set_if_show_modal(false);
    };

    let on_click = move |_| {
        let add_person = Unit::new(
            person_name(),
            person_title(),
            person_categories(),

        );

        let is_valid = add_person.validate();
        match is_valid {
            Ok(valid) => println!("valid"),
            Err(e) => println!("{}", e),
        }
    };

    view! {
        <div class="flex flex-col w-full h-full z-50 mx-auto items-center align-center">
        <div class={move || {
            if if_error() { ERROR_STYLE }
            else { NO_ERROR_STYLE }
        }}>
            <Show when=move || { if_error() }>
                <p class="text-white bg-red-500 rounded w-full h-12 px-5 py-3
                    transition-all duration-750 ease-in-out">
                    { error_message() }
                </p>
            </Show>
            <p class="text-white pt-5">"Add New Employee"</p>
            <input type="text" placeholder="Name"
                class=INPUT_STYLE
                value=person_name
                on:input=move |event| {
                    set_person_name(event_target_value(&event));
                }
            />
            <input type="text" placeholder="Title"
                class=INPUT_STYLE
                value=person_title
                on:input=move |event| {
                    set_person_title(event_target_value(&event));
                }
            />
            <input type="text" placeholder="Level"
                class=INPUT_STYLE
                value=person_level
                on:input=move |event| {
                    set_person_categories(event_target_value(&event));
                }
            />
           
            <div class="flex flex-row w-full items-right justify-right">
                <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                    "Cancel"
                </button>
                <button on:click=on_click class=ADD_BUTTON_STYLE>
                    "Add"
                </button>
            </div>
        </div>
    </div>

    }
}