use stylist::{css, StyleSource};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let class = css();
    let onclick = Callback::from(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let burger = document.query_selector(".burger").unwrap().unwrap();
        let nav = document.query_selector(".nav-links").unwrap().unwrap();
        let nav_links = document.query_selector_all(".nav-links li").unwrap();
        let nr_of_nav_links = nav_links.length();

        // Toggle Nav.
        nav.class_list().toggle("nav-active").unwrap();

        // Animate Links.
        (0..nr_of_nav_links).for_each(|index| {
            let link = nav_links.item(index).unwrap();
            let link = link.clone().dyn_into::<HtmlElement>().unwrap();

            let style = link.style();
            if style.get_property_value("animation").unwrap().is_empty() {
                let delay = index as f64 / nr_of_nav_links as f64 + 0.0;
                let animation = format!("navLinkFade 0.5s ease forwards {}s", delay);
                style.set_property("animation", &animation).unwrap();
            } else {
                style.set_property("animation", "").unwrap();
            }
        });

        // Animate burger.
        burger.class_list().toggle("toggle").unwrap();
    });

    html! {
        <div {class}>
            <nav>
                <div class="logo">
                  <h4>{"The nav"}</h4>
                </div>
                <ul class="nav-links">
                  <li><a href="#">{"Home"}</a></li>
                  <li><a href="#">{"About"}</a></li>
                  <li><a href="#">{"Work"}</a></li>
                  <li><a href="#">{"Projects"}</a></li>
                </ul>
                <div class="burger" {onclick}>
                  <div class="line1"></div>
                  <div class="line2"></div>
                  <div class="line3"></div>
                </div>
            </nav>
        </div>
    }
}

fn css() -> StyleSource {
    css!(
        "
            * {
                margin: 0px;
                padding: 0px;
                box-sizing: border-box;
            }
            
            nav {
                display: flex;
                justify-content: space-around;
                align-items: center;
                min-height: 10vh;
                background-color: #5d4954;
                font-family: 'Poppins', sans-serif;
            }
            
            .logo {
                color: #fff;
                text-transform: uppercase;
                letter-spacing: 5px;
                font-size: 22px;
            }
            
            .nav-links {
                display: flex;
                justify-content: space-around;
                width: 35%;
            }
            
            .nav-links li {
                list-style: none;
            
            }
            
            .nav-links a {
                color: rgb(226, 226, 226);
                text-decoration: none;
                letter-spacing: 3px;
                font-weight: bold;
                font-size: 14px;
            }
            
            .burger {
                display: none;
                cursor: pointer;
            }
            
            .burger div {
                width: 25px;
                height: 3px;
                background-color: #fff;
                margin: 5px;
                transition: all 0.3s ease
            }
            
            @media screen and (max-width:1024px) {
                .nav-links {
                    width: 60%;
                }
            }
            
            @media screen and (max-width:768px) {
                body {
                    overflow-x: hidden;
                }
            
                .nav-links {
                    position: absolute;
                    right: 0px;
                    height: 92vh;
                    top: 8vh;
                    background-color: #5d4954;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    width: 50%;
                    transform: translateX(100%);
                    transition: transform 0.5s ease-in ease-out;
                }
            
                .nav-links li {
                    opacity: 0;
                }
            
                .burger {
                    display: block;
                }
            }
            
            .nav-active {
                transform: translateX(0%);
            }
            
            @keyframes navLinkFade {
                from {
                    opacity: 0;
                    transform: translateX(50px);
                }
            
                to {
                    opacity: 1;
                    transform: translateX(0px);
                }
            }
            
            .toggle .line1 {
                transform: rotate(-40deg) translate(-5px, 6px);
            }
            
            .toggle .line2 {
                transform: rotate(90deg) translate(0px, 0px);
            }
            
            .toggle .line3 {
                transform: rotate(35deg) translate(-5px, -6px);
            }
                    "
    )
}
