// @ts-nocheck
import type { TextProps } from "../data/types";

export default function Text({ text }: TextProps) {
    return (
        <div data-name="text">{text}</div>
    );
}
