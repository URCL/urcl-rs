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


export function output_registers(regs) {

}
init().then(() => { // all code should go in here
    init_panic_hook();
    emulate("\"string test\"\n .test\n'e'\n  imm r69");
});