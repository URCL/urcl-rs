export declare class Scroll_Out extends HTMLElement {
    scroll_div: HTMLDivElement;
    content: HTMLDivElement;
    char: HTMLDivElement;
    cw: number;
    ch: number;
    lines: string[];
    size: number;
    constructor();
    get_text(): string;
    update(): void;
    resize(): boolean;
    private buf;
    private text_width;
    clear(): void;
    write(text_to_add: string): void;
    flush(): void;
    render(x: number, y: number, w: number, h: number): void;
}
