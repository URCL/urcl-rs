import { clear } from "console";
import  init, {emulate, init_panic_hook}  from "./pkg/urcl_rs.js"

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
    document.getElementById("stdout").innerText = htmlBuf;
}

export function output_registers(regs) {

}



init().then(() => { // all code should go in here
    init_panic_hook();
    
    
    document.getElementById("green").onclick = function() {
        emulate(document.getElementById("stdin").value);
    };

});