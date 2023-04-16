use leptos::leptos_dom::ev::{Event, SubmitEvent};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{to_value, from_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
  async fn open(args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
  name: &'a str,
}

#[derive(Serialize, Deserialize, Default)]
struct SearchFileArgs<'a> {
  location: Option<&'a str>,
  guess: &'a str,
}

#[derive(Serialize, Deserialize)]
struct DialogFilter {
  /** Filter name. */
  name: String,
  /**
   * Extensions to filter, without a `.` prefix.
   * @example
   * ```typescript
   * extensions: ['svg', 'png']
   * ```
   */
  extensions: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct OpenDialog {
  /** The title of the dialog window. */
  title: Option<String>,
  /** The filters of the dialog. */
  filters: Option<DialogFilter>,
  /** Initial directory or file path. */
  defaultPath: Option<String>,
  /** Whether the dialog allows multiple selection or not. */
  multiple: Option<String>,
  /** Whether the dialog is a directory selection or not. */
  directory: bool,
  /**
   * If `directory` is true, indicates that it will be read recursively later.
   * Defines whether subdirectories will be allowed on the scope or not.
   */
  recursive: Option<bool>,
}
#[component]
pub fn TextBox<F>(cx: Scope, on_change: F, value: ReadSignal<String>) -> impl IntoView
where
  F: FnMut(Event) + 'static,
{
  let (folder, set_folder) = create_signal(cx, String::new());
  let (matching_files, set_matching_files) = create_signal(cx, vec![String::new()]);
  let (guess, set_guess) = create_signal(cx, String::new());

  let open_dialog = move |_| {
    spawn_local(async move {
      let folderOptions = to_value(&OpenDialog {
        directory: true,
        ..OpenDialog::default()
      })
      .unwrap();
      let value = open(folderOptions)
        .await
        .as_string()
        .unwrap_or("".to_string());
      set_folder.set(value);
    });
  };

  let search_files = move |ev: SubmitEvent| {
    ev.prevent_default();
    spawn_local(async move {

      let args = to_value(&SearchFileArgs {
        location: None,
          guess: &guess.get(),
      })
      .unwrap();
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      let new_msg: Vec<String> = from_value(invoke("search_file", args).await).unwrap();

        set_matching_files.set(new_msg);
    });
  };
  view! { cx,
          <>
          <textarea class="treesitter-input"
              placeholder="Type here:"
              on:input=on_change />
          <div class="treesitter-input">
            <button on:click=open_dialog>"Open directory"</button>
            <div>{folder}</div>
          </div>
          </>
  }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  let (name, set_name) = create_signal(cx, String::new());
  let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

  let update_name = move |ev: Event| {
    let v = event_target_value(&ev);
    log!("{}", v);
    set_name.set(v);
  };
  let greet = move |ev: SubmitEvent| {
    ev.prevent_default();
    spawn_local(async move {
      if name.get().is_empty() {
        return;
      }

      let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      let new_msg = invoke("greet", args).await.as_string().unwrap();
      set_greet_msg.set(new_msg);
    });
  };

  view! { cx,
      <main class="container">
          <TextBox on_change=update_name value=name/>
      </main>
  }
}
