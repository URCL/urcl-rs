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
const code_input = by_id(HTMLTextAreaElement, "code_input");
const auto_emulate = by_id(HTMLInputElement, "auto-emulate");

export function out_text(text) {
    //
}

export function out_graphics(x, y, colour) {
    // 
}

export function out_err(text) {
    //
}

export function in_text() { // needs to have a null terminate character if null terminate box is pressed
    // like stdin
}

let htmlBuf = "";


export function out_html(text) {
    htmlBuf += text + '\n';
    highlight.innerText = htmlBuf;
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

export function out_lf() {
    out_span("\n", "white");
}

export function output_registers(regs) {

}

export async function clear_span() {
    htmlBuf = "";
    highlight.innerHTML = "";
}

init().then(() => { // all code should go in here
    init_panic_hook();
    
    code_input.oninput = e => { output_highlight_span(code_input.value); }
    
    code_input.onkeydown = e => {
        if (e.key == 'Tab') {
            e.preventDefault();
            code_input.value = code_input.value.substring(0, code_input.selectionStart) + "\t" + code_input.value.substring(code_input.selectionEnd);
            code_input.selectionStart = code_input.selectionEnd = code_input.selectionStart + 1;
            output_highlight_span(code_input.value);
        }
    };
    
    document.getElementById("emulate").onclick = function() {
        // output_highlight_span(document.getElementById("code_input").value);
        // emulate(code_input.value);
    };

    document.getElementById("clear").onclick = function() {
        clear_span();
    };

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

    document.getElementById("highlight").style.top      = code_input.getBoundingClientRect().top + "px";
    document.getElementById("highlight").style.bottom   = code_input.getBoundingClientRect().bottom + "px";
    document.getElementById("highlight").style.left     = code_input.getBoundingClientRect().left + "px";
    document.getElementById("highlight").style.right    = code_input.getBoundingClientRect().right + "px";

    document.getElementsByTagName("body")[0].onbeforeunload = function() {
        localStorage.setItem("auto_emulate", auto_emulate.checked ? "t" : "f");
    }

    auto_emulate.checked = localStorage.getItem("auto_emulate") == "t" ? true : false;
    output_highlight_span(code_input.value);
});
