use yew::prelude::*;
#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <form>
        <label>{"Username"}</label>
        <input type="text"/>
        <label>{"Password"}</label>
        <input type="password"/>
        <button type="submit">{"Login"}</button>
        </form>
     }
}