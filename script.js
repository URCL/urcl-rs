import init, {output_highlight_span, init_panic_hook, emulate}  from "./pkg/urcl_rs.js"

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
const highlight = by_id(HTMLElement, "highlight");
const code_input = by_id(HTMLTextAreaElement, "code_input");
const auto_emulate = by_id(HTMLInputElement, "auto_emulate");

export function now() {
    return performance.now();
}

export function out_graphics(x, y, colour) {
    // 
}

export function out_err(text) {
    //
}

export function clear_text() {
    stdout.innerText = "";
}

export function in_text() { // needs to have a null terminate character if null terminate box is pressed
    // like stdin
}

export function out_text(text) {
    stdout.innerText += text;
}

export function out_debug(text) {
    out_text(text + "\n");
}
/**
 * @param {string} text 
 * @param {string} clazz 
 */
export function out_span(text, class_name) {
    const span = document.createElement("span");
    span.textContent = text;
    span.className = class_name;
    highlight.appendChild(span);
}

export function output_registers(regs) {

}

export async function clear_span() {
    highlight.innerHTML = "";
}

export function resync_highlight() {
    highlight.style.top      = code_input.getBoundingClientRect().top + "px";
    highlight.style.left     = code_input.getBoundingClientRect().left + "px";
    highlight.style.width    = (code_input.getBoundingClientRect().width  - parseFloat(getComputedStyle(highlight).fontSize)) + "px";
    highlight.style.height   = (code_input.getBoundingClientRect().height - parseFloat(getComputedStyle(highlight).fontSize)) + "px";
}

export function update_debug_buttons(new_state) {
    for (let i = 0; i < document.getElementsByClassName("debug_only").length; i++) {
        document.getElementsByClassName("debug_only")[i].style.display = new_state ? "initial" : "none";
    }
}

init().then(() => { // all code should go in here
    init_panic_hook();
    
    code_input.onkeydown = e => {
        if (e.key == 'Tab') {
            e.preventDefault();
            let a = code_input.selectionStart+1;
            code_input.value = code_input.value.substring(0, code_input.selectionStart) + "\t" + code_input.value.substring(code_input.selectionEnd);
            code_input.setSelectionRange(a, a);
        };
    };

    code_input.oninput                                  = () => output_highlight_span(code_input.value);
    code_input.onscroll                                 = () => highlight.scrollTo(0, code_input.scrollTop);
    document.getElementById("document_link").onclick    = () => window.open("https://github.com/ModPunchtree/URCL/releases/latest", "_blank");
    document.getElementById("emulate").onclick          = () => emulate(code_input.value);
    document.getElementById("clear").onclick            = () => clear_text();
    document.getElementById("debug_option").onchange    = () => update_debug_buttons(this.checked);
    document.getElementById("tab_size").onchange        = () => document.querySelector(":root").style.setProperty("--tab-size", this.value);
    document.getElementsByTagName("body")[0].onresize   = () => resync_highlight();


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
        document.getElementsByClassName("example_link")[i].onclick = () => location = this.dataset["link"];
    };

    auto_emulate.checked = localStorage.getItem("auto_emulate") != "f";

    document.getElementById("tab_size").value = localStorage.getItem("tab_size") == null ? 4 : localStorage.getItem("tab_size");
    document.querySelector(":root").style.setProperty("--tab-size", document.getElementById("tab_size").value);
    
    document.getElementById("debug_option").checked = localStorage.getItem("debug_option") == "t";
    update_debug_buttons(document.getElementById("debug_option").checked);

    resync_highlight();
    output_highlight_span(code_input.value);

    const params = new URLSearchParams(window.location.search);

    if (params.has("from-examples")) {
        alert("Examples are not done yet!");
        window.history.replaceState("", "urcl-rs", location.href.replace(window.location.search, ""));
    };
});
