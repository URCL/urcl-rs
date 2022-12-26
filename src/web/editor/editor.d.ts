export declare class EditorWindow extends HTMLElement {
    #private;
    constructor();
    get value(): string;
    set value(value: string);
    set_pc_line(line: number): void;
    render_start(): void;
    render(content: string, class_name: string): void;
    render_end(): void;
    set highlighter(cb: (editor: EditorWindow) => void);
}
declare global {
    interface HTMLElementTagNameMap {
        "editor-window": EditorWindow;
    }
}
