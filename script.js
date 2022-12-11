import  init, {emulate, init_panic_hook}  from "./pkg/urcl_rs.js"

/** @type {HTMLPreElement} */
const stdout = document.getElementById("stdout");
let line = document.createElement("div");
stdout.appendChild(line);

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
    span.className = class_name
    line.appendChild(span)
}

export function remove_span() {
    line.innerHTML = "";
}

export function out_lf() {
    line = document.createElement("div");
    stdout.appendChild(line);
}

export function output_registers(regs) {

}



init().then(() => { // all code should go in here
    init_panic_hook();
    
    
    document.getElementById("emulate").onclick = function() {
        emulate(document.getElementById("stdin").value);
    };

});
