import { IconTypes } from "solid-icons";
import { Component, JSX } from "solid-js";

interface TextBlockProps {
  children: JSX.Element;
  Icon: IconTypes;
  title: string;
  className?: string;
}

export const TextBlock: Component<TextBlockProps> = ({
  children,
  Icon,
  title,
  className,
}) => {
  return (
    <div
      class={`mb-7 rounded-md border border-solid border-zinc-700 shadow-2xl ${className}`}
    >
      <div class="mb-2 flex items-center space-x-2 bg-zinc-800 px-5 pb-2.5 pt-5">
        <Icon />
        <h2 class="font-semibold">{title}</h2>
      </div>
      {children}
    </div>
  );
};