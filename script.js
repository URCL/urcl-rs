import init, {output_highlight_span, init_panic_hook, emulate, EmulatorState}  from "./pkg/urcl_rs.js"
import { StepResult } from "./pkg/urcl_rs.js";
import { EditorWindow } from "./src/web/editor/editor.js";

/**
 * @template T
 * @param {{new(...args: any): T}} clazz 
 * @param {*} obj 
 * @returns {T}
 */
function with_class(clazz, obj) {
    if (obj instanceof clazz) {
        return obj;
    } else {
        throw new Error(`expected ${clazz.name} but got ${obj?.constructor?.name}`);
    }
}

/**
 * @template {HTMLElement} T
 * @param {{new(...args: any): T}} clazz 
 * @param {string} name 
 * @returns {T}
*/
function by_id(clazz, name) {
    return with_class(clazz, document.getElementById(name));
}

const stdout = by_id(HTMLElement, "stdout");
const pause_button = by_id(HTMLButtonElement, "pause");
const code_input = by_id(EditorWindow, "code_input");
const auto_emulate = by_id(HTMLInputElement, "auto_emulate");

export function now() {
    return performance.now();
}

export function out_graphics(x, y, colour) {
    // 
}

export function clear_text() {
    stdout.innerHTML = "";
}

export function in_text() { // needs to have a null terminate character if null terminate box is pressed
    // like stdin
}

export function out_text(text) {
    const do_scroll = stdout.scrollTop === stdout.scrollHeight - stdout.clientHeight
    
    stdout.innerHTML += text;
    
    if (do_scroll) {
        stdout.scrollTop = stdout.scrollHeight*2;
    }
}

export function out_err(text) {
    let a = document.createElement("span");
    a.classList.add("error");
    a.innerText = text;
    stdout.appendChild(a);
}

export function out_debug(text) {
    out_text(text + "\n");
}
/**
 * @param {string} text 
 * @param {string} class_name 
 */

export function out_span(text, class_name) {
    code_input.render(text, class_name);
}

export function out_linenumber(text) {
    // TODO: remove this function
}

const screen_canvas = by_id(HTMLCanvasElement, "screen");
const screen_ctx = screen_canvas.getContext("2d");

export function clear_screen() {
    screen_ctx.clearRect(0, 0, screen_canvas.width, screen_canvas.height);
}

/**
 * 
 * @param {number} width 
 * @param {number} height 
 * @param {Uint32Array} pixels 
 */
export function out_screen(width, height, pixels) {
    if (screen_canvas.width !== width || screen_canvas.height !== height) {
        screen_canvas.width  = width;
        screen_canvas.height = height;
    }
    const image_data = new ImageData(new Uint8ClampedArray(pixels.buffer, pixels.byteOffset, pixels.byteLength), width, height);
    screen_ctx.putImageData(image_data, 0, 0);
}

export function output_registers(regs) {

}

export async function clear_span() {
    // TODO: remove this function

}

export function resync_element_size() {
    // TODO: remove this function

}

export function update_debug_buttons(new_state) {
    for (let i = 0; i < document.getElementsByClassName("debug_only").length; i++) {
        document.getElementsByClassName("debug_only")[i].style.display = new_state ? "initial" : "none";
    }
}

/** @type {undefined | EmulatorState} */
let emulator;
/** @type {undefined | number} */
let frame_id;

/**
 * 
 * @param {string} source 
 */
function start_emulation(source) {
    emulator = emulate(source);
    continue_emulation();
}

function continue_emulation() {
    cancel_emulation();
    if (!emulator) {
        return;
    }
    const result = emulator.run_for_ms(16);
    if (result === StepResult.Continue) {
        frame_id = requestAnimationFrame(continue_emulation);
        pause_button.disabled = false;
        pause_button.textContent = "PAUSE";
    } else {
        pause_button.disabled = true;
        pause_button.textContent = "DONE";
        if (emulator) {
            emulator.free();
        } 
        emulator = undefined;
    }
}
function cancel_emulation() {
    if (frame_id !== undefined) {
        cancelAnimationFrame(frame_id)
        frame_id = undefined;
    }
}

init().then(() => { // all code should go in here
    init_panic_hook();

    pause_button.onclick = () => {
        console.log(frame_id, emulator);
        if (frame_id) {
            pause_button.textContent = "CONTINUE";
            pause_button.disabled = false;
            cancel_emulation();
        } else if (emulator) {
            continue_emulation();
        }
    }

    code_input.highlighter = editor => {
        editor.render_start();
        output_highlight_span(editor.value);
        editor.render_end();
        if (auto_emulate.checked) start_emulation(code_input.value);
    };

    document.getElementById("document_link").onclick    = function() { window.open("https://github.com/ModPunchtree/URCL/releases/latest", "_blank"); };
    document.getElementById("emulate").onclick          = function() { start_emulation(code_input.value); };
    document.getElementById("clear").onclick            = function() { clear_text(); };
    document.getElementById("debug_option").onchange    = function() { update_debug_buttons(this.checked); };
    document.getElementById("tab_size").onchange        = function() { document.querySelector(":root").style.setProperty("--tab-size", this.value); };
    document.getElementsByTagName("body")[0].onresize   = function() { resync_element_size(); };


    document.getElementById("settings").onclick = function() {
        document.getElementById("settings_sec").style.opacity       = 1;
        document.getElementById("settings_sec").style.zIndex        = 999;
        document.getElementById("settings_sec").style.pointerEvents = "auto";
    };

    document.getElementById("exit_settings").onclick = function() {
        document.getElementById("settings_sec").style.opacity           = 0;
        setTimeout(() => {
            document.getElementById("settings_sec").style.zIndex        = -999;
            document.getElementById("settings_sec").style.pointerEvents = "none";
        }, 250);
    };
    
    document.getElementById("examples").onclick = function() {
        document.getElementById("example_sec").style.opacity       = 1;
        document.getElementById("example_sec").style.zIndex        = 999;
        document.getElementById("example_sec").style.pointerEvents = "auto";
    };

    document.getElementById("exit_examples").onclick = function() {
        document.getElementById("example_sec").style.opacity           = 0;
        setTimeout(() => {
            document.getElementById("example_sec").style.zIndex        = -999;
            document.getElementById("example_sec").style.pointerEvents = "none";
        }, 250);
    };

    document.getElementsByTagName("body")[0].onbeforeunload = function() {
        localStorage.setItem("auto_emulate", auto_emulate.checked ? "t" : "f");
        localStorage.setItem("tab_size", document.getElementById("tab_size").value);
        localStorage.setItem("debug_option", document.getElementById("debug_option").checked ? "t" : "f");
    };

    for (let i = 0; i < document.getElementsByClassName("example_link").length; i++) {
        document.getElementsByClassName("example_link")[i].onclick = function() {location = this.dataset["link"]};
    };

    auto_emulate.checked = localStorage.getItem("auto_emulate") != "f";

    document.getElementById("tab_size").value = localStorage.getItem("tab_size") == null ? 4 : localStorage.getItem("tab_size");
    document.querySelector(":root").style.setProperty("--tab-size", document.getElementById("tab_size").value);
    
    document.getElementById("debug_option").checked = localStorage.getItem("debug_option") == "t";
    update_debug_buttons(document.getElementById("debug_option").checked);

    resync_element_size();
    output_highlight_span(code_input.value);

    const params = new URLSearchParams(window.location.search);

    if (params.has("from-examples")) {
        alert("Examples are not done yet!");
        window.history.replaceState("", "urcl-rs", location.href.replace(window.location.search, ""));
    };
});
