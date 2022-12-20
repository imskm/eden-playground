use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{
	console,
	// HtmlElement,
	// MouseEvent,
	window
};
use yew::{
	html,
	Callback,
	// TargetCast,
	Properties,
};

// Prop for EdenStartButton click opens/closes UI controls
#[derive(Properties, PartialEq)]
pub struct EdenStartButtonProps {
	pub on_click_edenstartbutton: Callback<String>,
}

// use components::soundsvgicon::SoundSVGIcon;
#[function_component(SpeakerSVGIcon)]
fn speaker_svg_icon() -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"/><path d="M10 7.22L6.603 10H3v4h3.603L10 16.78V7.22zM5.889 16H2a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1h3.889l5.294-4.332a.5.5 0 0 1 .817.387v15.89a.5.5 0 0 1-.817.387L5.89 16zm13.517 4.134l-1.416-1.416A8.978 8.978 0 0 0 21 12a8.982 8.982 0 0 0-3.304-6.968l1.42-1.42A10.976 10.976 0 0 1 23 12c0 3.223-1.386 6.122-3.594 8.134zm-3.543-3.543l-1.422-1.422A3.993 3.993 0 0 0 16 12c0-1.43-.75-2.685-1.88-3.392l1.439-1.439A5.991 5.991 0 0 1 18 12c0 1.842-.83 3.49-2.137 4.591z"/></svg>
	}
}

#[function_component(MicSVGIcon)]
fn mic_svg_icon() -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"/><path d="M12 3a3 3 0 0 0-3 3v4a3 3 0 0 0 6 0V6a3 3 0 0 0-3-3zm0-2a5 5 0 0 1 5 5v4a5 5 0 0 1-10 0V6a5 5 0 0 1 5-5zM3.055 11H5.07a7.002 7.002 0 0 0 13.858 0h2.016A9.004 9.004 0 0 1 13 18.945V23h-2v-4.055A9.004 9.004 0 0 1 3.055 11z"/></svg>
	}
}

#[function_component(CameraSVGIcon)]
fn camera_svg_icon() -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"/><path d="M9.828 5l-2 2H4v12h16V7h-3.828l-2-2H9.828zM9 3h6l2 2h4a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1h4l2-2zm3 15a5.5 5.5 0 1 1 0-11 5.5 5.5 0 0 1 0 11zm0-2a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7z"/></svg>
	}
}

#[function_component(SpeakerMuteSVGIcon)]
fn spearker_mute_svg_icon() -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0z"/><path d="M5.889 16H2a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1h3.889l5.294-4.332a.5.5 0 0 1 .817.387v15.89a.5.5 0 0 1-.817.387L5.89 16zm14.525-4l3.536 3.536-1.414 1.414L19 13.414l-3.536 3.536-1.414-1.414L17.586 12 14.05 8.464l1.414-1.414L19 10.586l3.536-3.536 1.414 1.414L20.414 12z"/></svg>
	}
}

#[function_component(SpeakerButton)]
fn speaker_button() -> Html {
	html! {
		<button class="controls-menu-item-link">
			<SpeakerSVGIcon />
		</button>
	}
}

#[function_component(MicButton)]
fn mic_button() -> Html {
	html! {
		<button class="controls-menu-item-link">
			<MicSVGIcon />
		</button>
	}
}

#[function_component(CameraButton)]
fn camera_button() -> Html {
	html! {
		<button class="controls-menu-item-link">
			<CameraSVGIcon />
		</button>
	}
}

#[function_component(SpeakerMuteButton)]
fn speaker_mute_button() -> Html {
	html! {
		<button class="controls-menu-item-link">
			<SpeakerMuteSVGIcon />
		</button>
	}
}

#[function_component(CommandInput)]
fn command_input() -> Html {
	html! {
		<div class="command-box">
			<input class="command-input" name="user_command" type="text" placeholder="Type command.." />
		</div>
	}
}

#[function_component(EdenLogoSVG)]
fn eden_svg_log() -> Html {
	html! {
		<svg class="eden-logo" viewBox="0 0 234 234" fill="none" xmlns="http://www.w3.org/2000/svg">
		<g clip-path="url(#clip0_1_2)">
		<path d="M117.078 233.198C181.222 233.198 233.22 181.199 233.22 117.056C233.22 52.9123 181.222 0.913681 117.078 0.913681C52.9346 0.913681 0.936005 52.9123 0.936005 117.056C0.936005 181.199 52.9346 233.198 117.078 233.198Z" fill="#2C4BCB"/>
		<path d="M162.721 71.776L67.1762 95.158L74.8115 68.9785L143.86 52.0823L162.721 71.776Z" fill="#1F367F"/>
		<path d="M180.766 145.518L173.133 171.697L104.082 188.593L85.2219 168.9L180.766 145.518Z" fill="#1F367F"/>
		<path d="M193.033 103.437L189.215 116.528L64.3415 147.086L54.9113 137.239L58.7281 124.148L183.603 93.59L193.033 103.437Z" fill="#1F367F"/>
		<path d="M155.749 68.4391L60.2049 91.8193L67.8402 65.6417L136.889 48.7436L155.749 68.4391Z" fill="white"/>
		<path d="M173.795 142.181L166.162 168.358L97.111 185.256L78.2506 165.561L173.795 142.181Z" fill="white"/>
		<path d="M186.06 100.1L182.243 113.189L57.3684 143.747L47.9401 133.9L51.7568 120.811L176.632 90.2532L186.06 100.1Z" fill="white"/>
		</g>
		</svg>

	}
}

#[function_component(EdentStartButton)]
fn edent_start_button(props: &EdenStartButtonProps) -> Html {
	// let window = window().expect_throw("Window is undefined").window();
	let screen = window().expect_throw("Window is undefined").screen().expect_throw("screen is undefined");
	// let viewport = xr_viewport().expect_throw("xr viewport not found");
	let w = screen.width().unwrap();
	console::log_1(&screen.into());
	// console::log_1(&window.into());
	let x = format!("{}", w);
	console::log_1(&x.into());

	let t = props.on_click_edenstartbutton.clone();
	let onclick = {
		move |_|  {
			let ok = format!("OK");
			console::log_1(&ok.into());
			t.emit(String::from("Bob"));
		}
	};

	html! {
		<button {onclick} class={classes!("eden-control-main")}>
			<EdenLogoSVG />
		</button>
	}
}


#[function_component(AppControls)]
fn app_controls() -> Html {
	let open_controls = use_state(|| false);
	let t = open_controls.clone();
	let on_click_edenstartbutton: Callback<String> = Callback::from(move |_name: String| {
		let oc = open_controls.clone();
		oc.set(!*oc);
		// t = *oc;
		let v = format!("Controls open? {}", *oc);
		console::log_1(&v.into());
	});
	if *t {
		html! {
			<div class="eden-controls">
				<div>
					<CommandInput />
				</div>
				<div class="controls-menu-buttons">
					<ul class={classes!("controls-menu", "open")}>
						<li class="controls-menu-item menu-item-2">
							<SpeakerButton />
						</li>
					</ul>
					<EdentStartButton {on_click_edenstartbutton} />
				</div>
			</div>
		}
	} else {
		html! {
			<div class="eden-controls">
				<div>
					<CommandInput />
				</div>
				<div class="controls-menu-buttons">
					<ul class={classes!("controls-menu")}>
						<li class="controls-menu-item menu-item-1">
							<SpeakerButton />
						</li>
					</ul>
					<EdentStartButton {on_click_edenstartbutton} />
				</div>
			</div>
		}
	}
}

#[function_component]
fn App() -> Html {
	html! {
		<div>
			// <canvas class="canvas"></canvas>
			<div style="height: 100vh; background: url('https://images.unsplash.com/photo-1546552768-2e5b568b0680?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1171&q=80'); background-size: cover; background-position: center;"></div>
			<AppControls />
		</div>
	}
}



#[derive(Properties, PartialEq)]
pub struct Props {
	pub button_text: &'static str,
	#[prop_or_default]
	pub is_loading: bool,
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log(a: u32);
}

#[function_component]
fn LikeButton(prop: &Props) -> Html {
	html! {
		<>
			<button style="background: red">{ prop.button_text.clone() }{" "}{prop.is_loading}</button>
			<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="none" d="M0 0h24v24H0z"/><path d="M10 7.22L6.603 10H3v4h3.603L10 16.78V7.22zM5.889 16H2a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1h3.889l5.294-4.332a.5.5 0 0 1 .817.387v15.89a.5.5 0 0 1-.817.387L5.89 16zm13.517 4.134l-1.416-1.416A8.978 8.978 0 0 0 21 12a8.982 8.982 0 0 0-3.304-6.968l1.42-1.42A10.976 10.976 0 0 1 23 12c0 3.223-1.386 6.122-3.594 8.134zm-3.543-3.543l-1.422-1.422A3.993 3.993 0 0 0 16 12c0-1.43-.75-2.685-1.88-3.392l1.439-1.439A5.991 5.991 0 0 1 18 12c0 1.842-.83 3.49-2.137 4.591z"/></svg>
			<script>{ "
				console.log('HELLO');
			" }</script>
		</>
	}
}

// #[function_component]
// fn App() -> Html {

// 	// 
// 	// let onmousemove = Callback::from(|e: MouseEvent| {
// 	// 	if let Some(target) = e.target_dyn_into::<HtmlElement>() {
// 	// 		console::log_1("test".to_string());
// 	// 	}
// 	// });

// 	let counter = use_state(|| 0);
// 	let text = "Test".to_string();
// 	let onclick = {
// 		let counter = counter.clone();
// 		move |_| {
// 			let value = *counter + 1;
// 			counter.set(value);
// 			log(45);
// 		}
// 	};

// 	html! {
// 		<div>
// 			<button {onclick}>{ "+1" }</button>
// 			<p>{text} { *counter }</p>
// 			<LikeButton button_text={"Black Button"} />
// 			<LikeButton button_text={"Black Button"} is_loading={true} />
// 		</div>
// 	}
// }


fn main() {
	yew::Renderer::<App>::new().render();
}
