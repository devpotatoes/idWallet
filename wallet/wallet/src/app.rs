use leptos::task::spawn_local;
use leptos::{prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use leptos::*;
use wasm_bindgen::closure::Closure;
use web_sys::window;
use leptos::prelude::Effect;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn set_timeout<F>(f: F, ms: i32)
    where
        F: FnOnce() + 'static,
    {
        let cb = Closure::once_into_js(f);

        window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(cb.unchecked_ref(), ms).unwrap();
    }

#[derive(Clone)]
struct Ctx {
    page_name: WriteSignal<String>,
    load_wallet_surname: WriteSignal<String>,
    load_wallet_name: WriteSignal<String>,
}

#[component]
pub fn App() -> impl IntoView {
    let base_page_name = String::from("login_account");
    let (page_name, set_page_name) = signal(base_page_name);

    let (load_wallet_surname, set_load_wallet_surname) = signal(String::new());
    let (load_wallet_name, set_load_wallet_name) = signal(String::new());

    provide_context(Ctx {
        page_name: set_page_name,
        load_wallet_surname: set_load_wallet_surname,
        load_wallet_name: set_load_wallet_name
    });

    view! {
        {
            move || {
                match page_name.get().as_str() {
                    "login_account" => view! {
                        <LoginAccountPage/>
                    }.into_any(),
                    "create_account" => view! {
                        <CreateAccountPage/>
                    }.into_any(),
                    "create_identity_card" => view! {
                        <CreateIdentityCardPage
                            account_surname = { load_wallet_surname.get() }
                            account_name = { load_wallet_name.get() }
                        />
                    }.into_any(),
                    "wallet" => view! {
                        <Wallet
                            account_surname = { load_wallet_surname.get() }
                            account_name = { load_wallet_name.get() }
                        />
                    }.into_any(),
                    _ => view! {
                        <p>Error 404</p>
                    }.into_any()
                }
            }
        }
    }
}

#[component]
pub fn LoginAccountPage() -> impl IntoView {
    let ctx = use_context::<Ctx>().unwrap();

    let (surname, set_surname) = signal(String::new());
    let (name, set_name) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let (login_successful, set_login_successful) = signal(false);
    let (login_error, set_login_error) = signal(false);

    let check_account = move |surname: String, name: String, password: String| {
        spawn_local(async move {
            #[derive(Serialize, Deserialize)]
            struct Args {
                surname: String,
                name: String,
                password: String
            }
            
            let args = serde_wasm_bindgen::to_value(&Args {
                surname: surname.clone(),
                name: name.clone(),
                password: password.clone()
            }).unwrap();
        
            let response = invoke("login_account", args).await.as_bool().unwrap();
            
            if response == true {
                set_login_successful.set(true);

                ctx.load_wallet_surname.set(surname.clone());
                ctx.load_wallet_name.set(name.clone());
                
                set_timeout(move || {
                    ctx.page_name.set(String::from("wallet"));
                }, 2500);
            } else {
                set_login_error.set(true);

                set_timeout(move || {
                    set_login_error.set(false);
                }, 5000);
            };
        });
    };

    view! {
        <main class="flex items-center justify-center min-h-screen py-4 flex-col">
            <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4">
                <h1 class="text-center text-xl">Login</h1>
                <label class="label mt-2">Surname</label>
                <input type="text" class="input mb-2" placeholder="Surname"
                    on:input = move |ev| {
                        set_surname.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Name</label>
                <input type="text" class="input mb-2" placeholder="Name"
                    on:input = move |ev| {
                        set_name.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Password</label>
                <input type="password" class="input mb-2" placeholder="Password"
                    on:input=move |ev| {
                        set_password.set(event_target_value(&ev));
                    }
                />
                <Show when=move || login_successful.get() == false>
                    <button class="btn btn-neutral mt-4"
                        on:click = move |_| {
                            check_account(surname.get(), name.get(), password.get());
                        }
                    >Login</button>
                </Show>
                <Show when=move || login_successful.get() == true>
                    <span class="loading loading-spinner loading-xl mx-auto block"></span>
                </Show>
                <p class="text-center cursor-pointer underline mt-2"
                    on:click = move |_| {
                        ctx.page_name.set(String::from("create_account"));
                    }
                >Create an account</p>
            </fieldset>
            <Show when=move || login_error.get() == true>
                <div class="toast">
                    <div class="alert alert-error">
                        <span>Incorrect username or password. Please try again.</span>
                    </div>
                </div>
            </Show>
        </main>
    }
}

#[component]
pub fn CreateAccountPage() -> impl IntoView {
    let ctx = use_context::<Ctx>().unwrap();

    let (surname, set_surname) = signal(String::new());
    let (name, set_name) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    
    let (creation_successful, set_creation_successful) = signal(false);
    let (fields_completion_error, set_fields_completion_error) = signal(false);
    let (fields_completion_error_message, set_fields_completion_error_message) = signal(String::new());

    let create_account = move |surname: String, name: String, password: String, confirm_password: String| {
        if surname == String::from("") || name == String::from("") || password == String::from("") {
            set_fields_completion_error.set(true);
            set_fields_completion_error_message.set(String::from("Every field must be filled."));

            set_timeout(move || {
                set_fields_completion_error.set(false);
            }, 5000);

            return;
        };

        if password == confirm_password {
            spawn_local(async move {
                #[derive(Serialize, Deserialize)]
                struct Args {
                    surname: String,
                    name: String,
                    password: String
                }
                
                let args = serde_wasm_bindgen::to_value(&Args {
                    surname: surname.clone(),
                    name: name.clone(),
                    password: password.clone()
                }).unwrap();
            
                let response = invoke("create_account", args).await.as_bool().unwrap();

                if response == true {
                    set_creation_successful.set(true);

                    ctx.load_wallet_surname.set(surname.clone());
                    ctx.load_wallet_name.set(name.clone());

                    set_timeout(move || {
                        ctx.page_name.set(String::from("create_identity_card"));
                    }, 2500);
                };
            });
        } else {
            set_fields_completion_error.set(true);
            set_fields_completion_error_message.set(String::from("The two passwords must be identical."));

            set_timeout(move || {
                set_fields_completion_error.set(false);
            }, 5000);
        };
    };

    view! {
        <main class="flex items-center justify-center min-h-screen py-4 flex-col">
            <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4">
                <h1 class="text-center text-xl">Create account</h1>
                <label class="label mt-2">Surname</label>
                <input type="text" class="input mb-2" placeholder="Surname"
                    on:input = move |ev| {
                        set_surname.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Name</label>
                <input type="text" class="input mb-2" placeholder="Name"
                    on:input = move |ev| {
                        set_name.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Password</label>
                <input type="password" class="input mb-2" placeholder="Password"
                    on:input = move |ev| {
                        set_password.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Confirm your password</label>
                <input type="password" class="input mb-2" placeholder="Password"
                    on:input = move |ev| {
                        set_confirm_password.set(event_target_value(&ev));
                    }
                />
                <Show when=move || creation_successful.get() == false>
                    <button class="btn btn-neutral mt-4"
                        on:click = move |_| {
                            create_account(surname.get(), name.get(), password.get(), confirm_password.get());
                        }
                    >Create account</button>
                </Show>
                <Show when=move || creation_successful.get() == true>
                    <span class="loading loading-spinner loading-xl mx-auto block"></span>
                </Show>
                <p class="text-center cursor-pointer underline mt-2"
                    on:click = move |_| {
                        ctx.page_name.set(String::from("login_account"));
                    }
                >Login to an account</p>
            </fieldset>
            <Show when=move || fields_completion_error.get() == true>
                <div class="toast">
                    <div class="alert alert-error">
                        <span>
                            {
                                let msg = fields_completion_error_message.get();

                                if msg.is_empty() {
                                    String::from("An error occurred.")
                                } else {
                                    msg
                                }
                            }
                        </span>
                    </div>
                </div>
            </Show>
        </main>
    }
}

#[component]
pub fn CreateIdentityCardPage(account_surname: String, account_name: String) -> impl IntoView {
    let ctx = use_context::<Ctx>().unwrap();

    let (surname, set_surname) = signal(String::new());
    let (name, set_name) = signal(String::new());
    let (country, set_country) = signal(String::new());
    let (sex, set_sex) = signal(String::new());
    let (date_of_birth, set_date_of_birth) = signal(String::new());
    let (place_of_birth, set_place_of_birth) = signal(String::new());
    let (document_no, set_document_no) = signal(String::new());
    let (expiry_date, set_expiry_date) = signal(String::new());

    let (creation_successful, set_creation_successful) = signal(false);
    let (fields_completion_error, set_fields_completion_error) = signal(false);

    Effect::new(move |_| {
        set_surname.set(account_surname.clone());
        set_name.set(account_name.clone());
    });

    let create_card = move |surname: String, name: String, country: String, sex: String, date_of_birth: String, place_of_birth: String, document_no: String, expiry_date: String| {
        if surname == String::from("") || name == String::from("") || country == String::from("") || sex == String::from("") || date_of_birth == String::from("") || place_of_birth == String::from("") || document_no == String::from("") || expiry_date == String::from("") {
            set_fields_completion_error.set(true);

            set_timeout(move || {
                set_fields_completion_error.set(false);
            }, 5000);
            
            return;
        };
        
        spawn_local(async move {
            #[derive(Serialize, Deserialize)]
            struct Args {
                surname: String,
                name: String,
                country: String,
                sex: String,
                dateOfBirth: String,
                placeOfBirth: String,
                documentNo: String,
                expiryDate: String
            }
            
            let args = serde_wasm_bindgen::to_value(&Args {
                surname: surname.clone(),
                name: name.clone(),
                country: country.clone(),
                sex: sex.clone(),
                dateOfBirth: date_of_birth.clone(),
                placeOfBirth: place_of_birth.clone(),
                documentNo: document_no.clone(),
                expiryDate: expiry_date.clone()
            }).unwrap();
            
            let response = invoke("create_card", args).await.as_bool().unwrap();

            if response == true {
                set_creation_successful.set(true);

                set_timeout(move || {
                    ctx.page_name.set(String::from("wallet"));
                }, 2500);
            };
        });
    };

    view! {
        <main class="flex items-center justify-center min-h-screen py-4 flex-col">
            <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4">
                <h1 class="text-center text-xl">Create identity card</h1>
                <div class="hover-3d mt-2 mb-2">
                    <div class="card w-128 bg-black text-white bg-[radial-gradient(circle_at_bottom_left,#ffffff04_35%,transparent_36%),radial-gradient(circle_at_top_right,#ffffff04_35%,transparent_36%)] bg-size-[4.95em_4.95em]">
                        <div class="card-body">
                            <div class="flex justify-between">
                                <div class="font-bold">IDENTITY CARD</div>
                                <svg class="translate-y-[-16px] translate-x-[16px]" fill="#e8eaed" xmlns="http://www.w3.org/2000/svg" height="48px" width="48px" viewBox="0 -960 960 960">
                                    <path d="M560-440h200v-80H560v80Zm0-120h200v-80H560v80ZM200-320h320v-22q0-45-44-71.5T360-440q-72 0-116 26.5T200-342v22Zm160-160q33 0 56.5-23.5T440-560q0-33-23.5-56.5T360-640q-33 0-56.5 23.5T280-560q0 33 23.5 56.5T360-480ZM160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h640q33 0 56.5 23.5T880-720v480q0 33-23.5 56.5T800-160H160Zm0-80h640v-480H160v480Zm0 0v-480 480Z"/>
                                </svg>
                            </div>
                            <div class="flex justify-between align-center">
                                <div class="relative">
                                    <div class="absolute top-0 left-0 w-8 h-8 border-t-2 border-l-2 border-white rounded-tl-lg"></div>
                                    <div class="absolute top-0 right-0 w-8 h-8 border-t-2 border-r-2 border-white rounded-tr-lg"></div>
                                    <div class="absolute bottom-0 left-0 w-8 h-8 border-b-2 border-l-2 border-white rounded-bl-lg"></div>
                                    <div class="absolute bottom-0 right-0 w-8 h-8 border-b-2 border-r-2 border-white rounded-br-lg"></div>
                                    <div class="w-full h-full flex items-center justify-center p-8">
                                        <svg xmlns="http://www.w3.org/2000/svg" height="96px" width="96px" viewBox="0 -960 960 960" fill="#e8eaed">
                                            <path d="M480-480q-66 0-113-47t-47-113q0-66 47-113t113-47q66 0 113 47t47 113q0 66-47 113t-113 47ZM160-160v-112q0-34 17.5-62.5T224-378q62-31 126-46.5T480-440q66 0 130 15.5T736-378q29 15 46.5 43.5T800-272v112H160Z"/>
                                        </svg>
                                    </div>
                                </div>
                                <div class="w-full h-full pl-8">
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Surname</div>
                                            <div>
                                                {surname}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Name</div>
                                            <div>
                                                {name}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Sex</div>
                                            <Show when=move || sex.get() == String::from("")>
                                                <div class="skeleton h-4 w-full"></div>
                                            </Show>
                                            <Show when=move || sex.get() != String::from("")>
                                                <div>
                                                    {sex.get()}
                                                </div>
                                            </Show>
                                        </div>
                                        <div>
                                            <div class="text-xs opacity-25">Nationality</div>
                                            <Show when=move || country.get() == String::from("")>
                                                <div class="skeleton h-4 w-full"></div>
                                            </Show>
                                            <Show when=move || country.get() != String::from("")>
                                                <div>
                                                    {country.get()}
                                                </div>
                                            </Show>
                                        </div>
                                        <div>
                                            <div class="text-xs opacity-25">Date of birth</div>
                                            <Show when=move || date_of_birth.get() == String::from("")>
                                                <div class="skeleton h-4 w-full"></div>
                                            </Show>
                                            <Show when=move || date_of_birth.get() != String::from("")>
                                                <div>
                                                    {date_of_birth.get()}
                                                </div>
                                            </Show>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Place of birth</div>
                                            <Show when=move || place_of_birth.get() == String::from("")>
                                                <div class="skeleton h-4 w-full"></div>
                                            </Show>
                                            <Show when=move || place_of_birth.get() != String::from("")>
                                                <div>
                                                    {place_of_birth.get()}
                                                </div>
                                            </Show>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Document No.</div>
                                            <Show when=move || document_no.get() == String::from("")>
                                                <div class="skeleton h-4 w-full"></div>
                                            </Show>
                                            <Show when=move || document_no.get() != String::from("")>
                                                <div>
                                                    {document_no.get()}
                                                </div>
                                            </Show>
                                        </div>
                                        <div>
                                            <div class="text-xs opacity-25">Expiry date</div>
                                            <Show when=move || expiry_date.get() == String::from("")>
                                                <div class="skeleton h-4 w-full"></div>
                                            </Show>
                                            <Show when=move || expiry_date.get() != String::from("")>
                                                <div>
                                                    {expiry_date.get()}
                                                </div>
                                            </Show>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <label class="label mt-2">Surname</label>
                <input type="text" class="input mb-2 w-full hover:cursor-not-allowed" placeholder="Surname" value={ surname } readonly=true/>
                <label class="label mt-2">Name</label>
                <input type="text" class="input mb-2 w-full hover:cursor-not-allowed" placeholder="Name" value={ name } readonly=true/>
                <label class="label mt-2">Country</label>
                <select class="select select-bordered w-full">
                    <option disabled selected>Select a country</option>
                    <option
                        on:click = move |_| {
                            set_country.set(String::from("CA"));
                        }
                    >Canada</option>
                    <option
                        on:click = move |_| {
                            set_country.set(String::from("FR"));
                        }
                    >France</option>
                    <option
                        on:click = move |_| {
                            set_country.set(String::from("JP"));
                        }
                    >Japan</option>
                    <option
                        on:click = move |_| {
                            set_country.set(String::from("NZ"));
                        }
                    >New Zealand</option>
                    <option
                        on:click = move |_| {
                            set_country.set(String::from("SA"));
                        }
                    >Saudi Arabia</option>
                    <option
                        on:click = move |_| {
                            set_country.set(String::from("US"));
                        }
                    >United States</option>
                </select>
                <label class="label mt-2">Sex</label>
                <select class="select select-bordered mb-2 w-full">
                    <option disabled selected>Select your sex</option>
                    <option
                        on:click = move |_| {
                            set_sex.set(String::from("M"));
                        }
                    >M</option>
                    <option
                        on:click = move |_| {
                            set_sex.set(String::from("F"));
                        }
                    >F</option>
                </select>
                <label class="label mt-2">Date of birth</label>
                <input type="date" class="input mb-2 w-full"
                    on:input = move |ev| {
                        set_date_of_birth.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Place of birth</label>
                <input type="text" class="input mb-2 w-full" placeholder="Place of birth"
                    on:input = move |ev| {
                        set_place_of_birth.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Document No.</label>
                <input type="text" class="input mb-2 w-full" placeholder="Document No."
                    on:input = move |ev| {
                        set_document_no.set(event_target_value(&ev));
                    }
                />
                <label class="label mt-2">Expiry date</label>
                <input type="date" class="input mb-2 w-full"
                    on:input = move |ev| {
                        set_expiry_date.set(event_target_value(&ev));
                    }
                />
                <Show when=move || creation_successful.get() == false>
                    <button class="btn btn-neutral mt-4"
                        on:click = move |_| {
                            create_card(surname.get(), name.get(), country.get(), sex.get(), date_of_birth.get(), place_of_birth.get(), document_no.get(), expiry_date.get());
                        }
                    >Create card</button>
                </Show>
                <Show when=move || creation_successful.get() == true>
                    <span class="loading loading-spinner loading-xl mx-auto block"></span>
                </Show>
            </fieldset>
            <Show when=move || fields_completion_error.get() == true>
                <div class="toast">
                    <div class="alert alert-error">
                        <span>Every field must be filled.</span>
                    </div>
                </div>
            </Show>
        </main>
    }
}

#[component]
pub fn Wallet(account_surname: String, account_name: String) -> impl IntoView {
    #[derive(Serialize, Deserialize)]
    struct IdentityCard {
        surname: String,
        name: String,
        country: String,
        sex: String,
        dateOfBirth: String,
        placeOfBirth: String,
        documentNo: String,
        expiryDate: String
    }

    enum WalletItem {
        IdentityCard(IdentityCard)
    }

    let (surname, set_surname) = signal(String::new());
    let (name, set_name) = signal(String::new());
    let (country, set_country) = signal(String::new());
    let (sex, set_sex) = signal(String::new());
    let (date_of_birth, set_date_of_birth) = signal(String::new());
    let (place_of_birth, set_place_of_birth) = signal(String::new());
    let (document_no, set_document_no) = signal(String::new());
    let (expiry_date, set_expiry_date) = signal(String::new());

    let (surname_checkbox, set_surname_checkbox) = signal(true);
    let (name_checkbox, set_name_checkbox) = signal(true);
    let (country_checkbox, set_country_checkbox) = signal(true);
    let (sex_checkbox, set_sex_checkbox) = signal(true);
    let (date_of_birth_checkbox, set_date_of_birth_checkbox) = signal(true);
    let (place_of_birth_checkbox, set_place_of_birth_checkbox) = signal(true);
    let (document_no_checkbox, set_document_no_checkbox) = signal(true);
    let (expiry_date_checkbox, set_expiry_date_checkbox) = signal(true);

    let (signature_generated, set_signature_generated) = signal(false);

    let (signature, set_signature) = signal(String::new());
    let (public_key, set_public_key) = signal(String::new());

    let (disclosed_messages, set_disclosed_messages) = signal(Vec::<(usize, String)>::new());
    let (signature_successfully_verified, set_signature_successfully_verified) = signal(None);
    let (partial_signature_successfully_verified, set_partial_signature_successfully_verified) = signal(None);

    let fetch_wallet_data = move |surname: String, name: String| {
        spawn_local(async move {
            #[derive(Serialize, Deserialize, Debug)]
            struct Args {
                surname: String,
                name: String
            }
            
            let args = serde_wasm_bindgen::to_value(&Args {
                surname: surname.clone(),
                name: name.clone()
            }).unwrap();
            
            let response: Vec<IdentityCard> = serde_wasm_bindgen::from_value(invoke("fetch_wallet_data", args).await).unwrap();

            if response.len() > 0 {
                set_surname.set(response[0].surname.clone());
                set_name.set(response[0].name.clone());
                set_country.set(response[0].country.clone());
                set_sex.set(response[0].sex.clone());
                set_date_of_birth.set(response[0].dateOfBirth.clone());
                set_place_of_birth.set(response[0].placeOfBirth.clone());
                set_document_no.set(response[0].documentNo.clone());
                set_expiry_date.set(response[0].expiryDate.clone());
            };
        });
    };
    
    Effect::new(move |_| {
        fetch_wallet_data(account_surname.clone(), account_name.clone());
    });

    #[derive(Serialize, Deserialize)]
    struct Signature {
        signature: String,
        public_key: String
    }

    let create_signature = move || {
        spawn_local(async move {
            #[derive(Serialize, Deserialize, Debug)]
            struct Args {
                messagesArray: Vec<String>
            }
            
            let args = serde_wasm_bindgen::to_value(&Args {
                messagesArray: vec![
                    surname.get_untracked(),
                    name.get_untracked(),
                    country.get_untracked(),
                    sex.get_untracked(),
                    date_of_birth.get_untracked(),
                    place_of_birth.get_untracked(),
                    document_no.get_untracked(),
                    expiry_date.get_untracked()
                ]
            }).unwrap();

            let response: Signature = serde_wasm_bindgen::from_value(invoke("create_signature", args).await).unwrap();
            
            set_signature.set(response.signature);
            set_public_key.set(response.public_key);
            set_signature_generated.set(true);
        });
    };

    let verify_signature = move || {
        spawn_local(async move {
            #[derive(Serialize, Deserialize, Debug)]
            struct Args {
                signatureHex: String,
                publicKeyHex: String,
                messagesArray: Vec<String>
            }

            let args = serde_wasm_bindgen::to_value(&Args {
                signatureHex: signature.get_untracked(),
                publicKeyHex: public_key.get_untracked(),
                messagesArray: vec![
                    surname.get_untracked(),
                    name.get_untracked(),
                    country.get_untracked(),
                    sex.get_untracked(),
                    date_of_birth.get_untracked(),
                    place_of_birth.get_untracked(),
                    document_no.get_untracked(),
                    expiry_date.get_untracked()
                ]
            }).unwrap();

            let response = invoke("verify_signature", args).await.as_bool().unwrap();

            set_signature_successfully_verified.set(Some(response));

            set_timeout(move || {
                set_signature_successfully_verified.set(None);
            }, 5000);
        });

        let indices_array = [surname_checkbox.get(), name_checkbox.get(), country_checkbox.get(), sex_checkbox.get(), date_of_birth_checkbox.get(), place_of_birth_checkbox.get(), document_no_checkbox.get(), expiry_date_checkbox.get()];

        spawn_local(async move {
            #[derive(Serialize, Deserialize, Debug)]
            struct Args {
                signatureHex: String,
                publicKeyHex: String,
                messagesArray: Vec<String>,
                indicesArray: Vec<usize>
            }
        
            let args = serde_wasm_bindgen::to_value(&Args {
                signatureHex: signature.get_untracked(),
                publicKeyHex: public_key.get_untracked(),
                messagesArray: vec![
                    surname.get_untracked(),
                    name.get_untracked(),
                    country.get_untracked(),
                    sex.get_untracked(),
                    date_of_birth.get_untracked(),
                    place_of_birth.get_untracked(),
                    document_no.get_untracked(),
                    expiry_date.get_untracked()
                ],
                indicesArray: indices_array.iter().enumerate().filter_map(|(i, &value)| if value { Some(i) } else { None }).collect()
            }).unwrap();

            #[derive(Serialize, Deserialize, Debug)]
            pub struct DisclosedMessage {
                pub index: usize,
                pub value: String,
            }

            #[derive(Serialize, Deserialize, Debug)]
            pub struct PartialSignatureResult {
                pub verified: bool,
                pub disclosed_messages: Vec<DisclosedMessage>,
            }

            let response: PartialSignatureResult = match serde_wasm_bindgen::from_value(invoke("verify_signature_indices", args).await) {
                Ok(r) => r,
                Err(_err) => {
                    PartialSignatureResult {
                        verified: false,
                        disclosed_messages: vec![]
                    }
                }
            };

            set_disclosed_messages.set(response.disclosed_messages.iter().map(|dm| (dm.index, dm.value.clone())).collect());

            set_partial_signature_successfully_verified.set(Some(response.verified));

            set_timeout(move || {
                set_partial_signature_successfully_verified.set(None);
            }, 5000);
        });
    };

    view! {
        <main class="flex items-center justify-center min-h-screen py-4 flex-col">
            <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4">
                <h1 class="text-center text-xl">Wallet</h1>
                <div class="hover-3d mt-2 mb-2">
                    <div class="card w-128 bg-black text-white bg-[radial-gradient(circle_at_bottom_left,#ffffff04_35%,transparent_36%),radial-gradient(circle_at_top_right,#ffffff04_35%,transparent_36%)] bg-size-[4.95em_4.95em]">
                        <div class="card-body">
                            <div class="flex justify-between">
                                <div class="font-bold">IDENTITY CARD</div>
                                <svg class="translate-y-[-16px] translate-x-[16px]" fill="#e8eaed" xmlns="http://www.w3.org/2000/svg" height="48px" width="48px" viewBox="0 -960 960 960">
                                    <path d="M560-440h200v-80H560v80Zm0-120h200v-80H560v80ZM200-320h320v-22q0-45-44-71.5T360-440q-72 0-116 26.5T200-342v22Zm160-160q33 0 56.5-23.5T440-560q0-33-23.5-56.5T360-640q-33 0-56.5 23.5T280-560q0 33 23.5 56.5T360-480ZM160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h640q33 0 56.5 23.5T880-720v480q0 33-23.5 56.5T800-160H160Zm0-80h640v-480H160v480Zm0 0v-480 480Z"/>
                                </svg>
                            </div>
                            <div class="flex justify-between align-center">
                                <div class="relative">
                                    <div class="absolute top-0 left-0 w-8 h-8 border-t-2 border-l-2 border-white rounded-tl-lg"></div>
                                    <div class="absolute top-0 right-0 w-8 h-8 border-t-2 border-r-2 border-white rounded-tr-lg"></div>
                                    <div class="absolute bottom-0 left-0 w-8 h-8 border-b-2 border-l-2 border-white rounded-bl-lg"></div>
                                    <div class="absolute bottom-0 right-0 w-8 h-8 border-b-2 border-r-2 border-white rounded-br-lg"></div>
                                    <div class="w-full h-full flex items-center justify-center p-8">
                                        <svg xmlns="http://www.w3.org/2000/svg" height="96px" width="96px" viewBox="0 -960 960 960" fill="#e8eaed">
                                            <path d="M480-480q-66 0-113-47t-47-113q0-66 47-113t113-47q66 0 113 47t47 113q0 66-47 113t-113 47ZM160-160v-112q0-34 17.5-62.5T224-378q62-31 126-46.5T480-440q66 0 130 15.5T736-378q29 15 46.5 43.5T800-272v112H160Z"/>
                                        </svg>
                                    </div>
                                </div>
                                <div class="w-full h-full pl-8">
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Surname</div>
                                            <div>
                                                {surname}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Name</div>
                                            <div>
                                                {name}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Sex</div>
                                            <div>
                                                {sex}
                                            </div>
                                        </div>
                                        <div>
                                            <div class="text-xs opacity-25">Nationality</div>
                                            <div>
                                                {country}
                                            </div>
                                        </div>
                                        <div>
                                            <div class="text-xs opacity-25">Date of birth</div>
                                            <div>
                                                {date_of_birth}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Place of birth</div>
                                            <div>
                                                {place_of_birth}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="w-full flex align-center justify-between">
                                        <div>
                                            <div class="text-xs opacity-25">Document No.</div>
                                            <div>
                                                {document_no}
                                            </div>
                                        </div>
                                        <div>
                                            <div class="text-xs opacity-25">Expiry date</div>
                                            <div>
                                                {expiry_date}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <h2 class="text-center text-lg">Share informations</h2>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_surname_checkbox.set(!surname_checkbox.get());
                        }
                    />
                    Surname
                </label>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_name_checkbox.set(!name_checkbox.get());
                        }
                    />
                    Name
                </label>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_sex_checkbox.set(!sex_checkbox.get());
                        }
                    />
                    Sex
                </label>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_country_checkbox.set(!country_checkbox.get());
                        }
                    />
                    Country
                </label>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_date_of_birth_checkbox.set(!date_of_birth_checkbox.get());
                        }
                    />
                    Date of birth
                </label>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_place_of_birth_checkbox.set(!place_of_birth_checkbox.get());
                        }
                    />
                    Place of birth
                </label>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_document_no_checkbox.set(!document_no_checkbox.get());
                        }
                    />
                    Document No
                </label>
                <label class="label">
                    <input type="checkbox" checked="checked" class="checkbox"
                        on:click = move |_| {
                            set_expiry_date_checkbox.set(!expiry_date_checkbox.get());
                        }
                    />
                    Expiry date
                </label>
                <button class="btn btn-neutral mt-4"
                    on:click = move |_| {
                        create_signature();
                    }
                >Create signature</button>
                <Show when=move || signature_generated.get() == true>
                    <fieldset class="fieldset">
                        <legend class="fieldset-legend">Signature</legend>
                        <textarea class="textarea h-24 w-full"
                            on:input = move |ev| {
                                set_signature.set(event_target_value(&ev));
                            }
                        >{signature.get()}</textarea>
                    </fieldset>
                    <fieldset class="fieldset">
                        <legend class="fieldset-legend">Public key</legend>
                        <textarea class="textarea h-24 w-full"
                            on:input = move |ev| {
                                set_public_key.set(event_target_value(&ev));
                            }
                        >{public_key.get()}</textarea>
                    </fieldset>
                    <button class="btn btn-neutral mt-4"
                        on:click = move |_| {
                            verify_signature();
                        }
                    >Verify signature</button>
                </Show>
                <Show when=move || disclosed_messages.get().len() != 0>
                    <fieldset class="fieldset">
                        <legend class="fieldset-legend">Disclosed messages</legend>
                        <textarea class="textarea h-24 w-full">
                            {
                                disclosed_messages.get().iter().map(|(index, msg)| format!("[{}] {}", index, msg)).collect::<Vec<String>>().join("\n")
                            }
                        </textarea>
                    </fieldset>
                </Show>
            </fieldset>
            <div class="toast toast-end">
                <Show when=move || signature_successfully_verified.get() == Some(true)>
                    <div class="alert alert-success">
                        <span>Signature BBS+ successfully verified.</span>
                    </div>
                </Show>
                <Show when=move || signature_successfully_verified.get() == Some(false)>
                    <div class="alert alert-error">
                        <span>Signature BBS+ failed the verification.</span>
                    </div>
                </Show>
                <Show when=move || partial_signature_successfully_verified.get() == Some(true)> 
                    <div class="alert alert-success">
                        <span>Signature ZKP BBS+ successfully verified.</span>
                    </div>
                </Show>
                <Show when=move || partial_signature_successfully_verified.get() == Some(false)>
                    <div class="alert alert-error">
                        <span>Signature ZKP BBS+ failed the verification.</span>
                    </div>
                </Show>
            </div>
        </main>
    }
}