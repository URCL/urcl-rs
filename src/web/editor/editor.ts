type L<T extends string | HTMLElement> =  T extends keyof HTMLElementTagNameMap ? HTMLElementTagNameMap[T]: (T extends HTMLElement ? T : HTMLElement)
type RPartial<T> = {[K in keyof T]?: T[K] extends object ? RPartial<T[K]> : T[K]};

/*
a new HTML-element is made with the tag
if there is an object apply its attributes to the element
append each given child to the element
*/
function l<T extends keyof HTMLElementTagNameMap | HTMLElement>
    (tagOrElement: T = "DIV" as any, attributes: RPartial<L<T>> & {tagName?: never} = {}, ...children: (Node|string)[]): L<T>
{
    const element: HTMLElement = typeof tagOrElement === "string" ? document.createElement(tagOrElement) : tagOrElement;
    attribute(element, attributes as any)
    element.append(...children)
    return element as any;
}

// applies all attributes in an object to a HTML-element
function attribute<T extends Record<string, any>>(element: T, attributes: RPartial<T>){
    for (const [key, value] of Object.entries(attributes)){
        if (typeof value === "object"){
            attribute(element[key], value);
        } else {
            element[key as keyof T] = value;
        }
    }
}


export class EditorWindow extends HTMLElement {
    #line_nrs: HTMLElement;
    #code: HTMLElement;
    #input: HTMLTextAreaElement;
    #colors: HTMLElement;
    constructor(){
        super();
        l(this, {}, 
            this.#line_nrs = l("div", {className: "line-nrs"}),
            this.#code = l("div", {className: "code"},
                this.#input = l("textarea", {spellcheck: false}),
                this.#colors = l("code", {className: "colors"})
            ),
        );

        this.#input.addEventListener("keydown", this.#keydown_cb.bind(this));
        this.#input.addEventListener("input", this.#input_cb.bind(this));
        this.onscroll = () => this.render_end();

        const resize_observer = new ResizeObserver(() => this.#layout());
        resize_observer.observe(this);
        this.render_end();
    }

    get value(){
        return this.#input.value;
    }
    set value(value){
        this.#input.value = value;
        this.#input_cb()
    }
    #pc_line = 0;
    set_pc_line(line: number){
        const old = this.#line_nrs.children[this.#pc_line];
        if (old){
            old.classList.remove("pc-line");
        }

        const child = this.#line_nrs.children[line];
        if (child){
            child.classList.add("pc-line");
        }
        this.#pc_line = line;

    }
    #keydown_cb(event: KeyboardEvent){
        if (event.key === "Tab") {
            event.preventDefault();
            const {value, selectionStart, selectionEnd} = this.#input;
            if (selectionStart === selectionEnd) {
                this.#input.value = value.substring(0, selectionStart) + "\t" + value.substring(selectionEnd);
                this.#input.selectionStart = this.#input.selectionEnd = selectionStart+1;
                this.#input_cb();
            }
        }
    }
    #input_cb(){
        this.#highlighter(this);
    }
    render_start() {
        this.#colors.innerHTML = "";
    }
    render(content: string, class_name: string) {
        const span = document.createElement("span");
        span.textContent = content;
        span.className = class_name;
        this.#colors.appendChild(span);
    }

    render_end(){
        const lines = this.#input.value.split("\n");
        const width = (lines.length+"").length
        this.#line_nrs.innerHTML = "";
        const line_count = Math.max(1, lines.length);
        for (let i = 0; i < line_count; i++) {
            const div = this.#line_nrs.appendChild(document.createElement("div"));
            div.textContent = ("" + (i + 1)).padStart(width, " ");
        }
        this.#layout();
    }
    #layout() {
        this.#input.style.height = "0px";
        this.#input.style.width = "0px";
        
        const height = Math.max(this.#input.scrollHeight, this.scrollHeight);
        
        this.#input.style.height = height + "px";
        this.#input.style.width = this.#input.scrollWidth + "px";
    }


    #highlighter = (editor: EditorWindow) => {
        editor.render_start();
        editor.render(editor.value, "");
        editor.render_end()
    };
    public set highlighter(cb: (editor: EditorWindow) => void) {
        this.#highlighter = cb;
    }
}
customElements.define("editor-window", EditorWindow);
declare global {
    interface HTMLElementTagNameMap {
        "editor-window": EditorWindow
    }
}