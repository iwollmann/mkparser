use web_sys::{window, Navigator, Window};
use yew::*;
use yew_hooks::use_interval;

#[derive(Properties, Debug, PartialEq)]
pub struct CopyButtonProps {
    pub data: String,
}

#[function_component(CopyButton)]
pub fn copy_button(props: &CopyButtonProps) -> Html {
    let copied = use_state(|| false);
    let millis = use_state(|| 0);

    {
        let copied = copied.clone();
        use_interval(
            move || {
                copied.set(false);
            },
            *millis,
        );
    }

    let copy_on_click = {
        let data = props.data.clone();
        let copied = copied.clone();
        let millis = millis.clone();

        Callback::from(move |_| {
            let data = data.clone();
            let window: Window = window().expect("window not available");
            let navigator: Navigator = window.navigator();
            if let Some(clipboard) = navigator.clipboard() {
                let _promise = clipboard.write_text(&data.to_string());
                copied.set(true);
            }
            millis.set(2000);
        })
    };

    html! {
        <button class={classes!("absolute", "top-0", "right-0", "pr-8","pt-6")} onclick={copy_on_click}>
        if !*copied {
            <svg width="28" height="28" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M20.6667 1.33334H6.00001C4.53334 1.33334 3.33334 2.53334 3.33334 4.00001V21.3333C3.33334 22.0667 3.93334 22.6667 4.66668 22.6667C5.40001 22.6667 6.00001 22.0667 6.00001 21.3333V5.33334C6.00001 4.60001 6.60001 4.00001 7.33334 4.00001H20.6667C21.4 4.00001 22 3.40001 22 2.66668C22 1.93334 21.4 1.33334 20.6667 1.33334ZM26 6.66668H11.3333C9.86668 6.66668 8.66668 7.86668 8.66668 9.33334V28C8.66668 29.4667 9.86668 30.6667 11.3333 30.6667H26C27.4667 30.6667 28.6667 29.4667 28.6667 28V9.33334C28.6667 7.86668 27.4667 6.66668 26 6.66668ZM12.6667 28H24.6667C25.4 28 26 27.4 26 26.6667V10.6667C26 9.93334 25.4 9.33334 24.6667 9.33334H12.6667C11.9333 9.33334 11.3333 9.93334 11.3333 10.6667V26.6667C11.3333 27.4 11.9333 28 12.6667 28Z" fill="black" fill-opacity="0.54"/>
            </svg>
        } else {
            <svg width="28" height="28" viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M14.4667 18.6112L12.425 16.5695C12.3172 16.4604 12.1701 16.3989 12.0167 16.3989C11.8633 16.3989 11.7162 16.4604 11.6084 16.5695C11.3809 16.797 11.3809 17.1587 11.6084 17.3862L14.0525 19.8304C14.28 20.0579 14.6475 20.0579 14.875 19.8304L21.0584 13.6529C21.2859 13.4254 21.2859 13.0637 21.0584 12.8362C20.9505 12.727 20.8035 12.6656 20.65 12.6656C20.4966 12.6656 20.3495 12.727 20.2417 12.8362L14.4667 18.6112Z" fill="#2AD50E" fill-opacity="0.54"/>
                <path fill-rule="evenodd" clip-rule="evenodd" d="M18.0834 1.16669H5.25008C3.96675 1.16669 2.91675 2.21669 2.91675 3.50002V18.6667C2.91675 19.3084 3.44175 19.8334 4.08341 19.8334C4.72508 19.8334 5.25008 19.3084 5.25008 18.6667V4.66669C5.25008 4.02502 5.77508 3.50002 6.41675 3.50002H18.0834C18.7251 3.50002 19.2501 2.97502 19.2501 2.33335C19.2501 1.69169 18.7251 1.16669 18.0834 1.16669ZM22.7501 5.83335H9.91675C8.63341 5.83335 7.58342 6.88335 7.58342 8.16669V24.5C7.58342 25.7834 8.63341 26.8334 9.91675 26.8334H22.7501C24.0334 26.8334 25.0834 25.7834 25.0834 24.5V8.16669C25.0834 6.88335 24.0334 5.83335 22.7501 5.83335ZM11.0834 24.5H21.5834C22.2251 24.5 22.7501 23.975 22.7501 23.3334V9.33335C22.7501 8.69169 22.2251 8.16669 21.5834 8.16669H11.0834C10.4417 8.16669 9.91675 8.69169 9.91675 9.33335V23.3334C9.91675 23.975 10.4417 24.5 11.0834 24.5Z" fill="#2AD50E" fill-opacity="0.54"/>
            </svg>
        }
        </button>
    }
}
