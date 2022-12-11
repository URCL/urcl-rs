import  init, {emulate, init_panic_hook}  from "./pkg/urcl_rs.js"

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
let line = document.createElement("div");
stdout.appendChild(line);

const stdin = by_id(HTMLTextAreaElement, "stdin");
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
    stdout.innerText = htmlBuf;
}
/**
 * @param {string} text 
 * @param {string} clazz 
 */
export function out_span(text, class_name) {
    const span = document.createElement("span");
    span.textContent = text;
    span.className = class_name;
    line.appendChild(span);
}

export function out_lf() {
    line = document.createElement("div");
    stdout.appendChild(line);
}

export function output_registers(regs) {

}

export async function clear_span() {
    htmlBuf = "";
    line.innerHTML = "";
    stdout.innerHTML = "";
    stdout.appendChild(line);
}

init().then(() => { // all code should go in here
    init_panic_hook();
    
    stdin.oninput = e => {
        if (auto_emulate.checked) {
            emulate(stdin.value);
        }
    }
    
    document.getElementById("emulate").onclick = function() {
        emulate(document.getElementById("stdin").value);
    };
    
    document.getElementById("clear").onclick = function() {
        clear_span();
    };
    
    document.getElementsByTagName("body")[0].onbeforeunload = function() {
        localStorage.setItem("auto_emulate", auto_emulate.checked ? "t" : "f");
    }

    auto_emulate.checked = localStorage.getItem("auto_emulate") == "t" ? true : false;
});
