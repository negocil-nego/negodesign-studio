// @ts-nocheck
import type { TextProps } from "../data/types";

export default function Text({ Text, for: htmlFor }: TextProps) {
    return (
        <div data-name="text">{text}</div>
    );
}