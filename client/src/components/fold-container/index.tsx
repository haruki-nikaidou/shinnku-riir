import {JSX} from "solid-js";
import {clsx} from "clsx";

interface FoldContainerProps {
  children?: JSX.Element[] | JSX.Element;
  maxHeight?: string;
  minHeight?: string;
  className?: string;
  isOpen: boolean;
}

export function FoldContainer(props: FoldContainerProps) {
  const heightStyle = () => {
    return {
      "max-height": props.isOpen ? props.maxHeight : "0px",
      "min-height": props.isOpen ? props.minHeight : "0px",
    }
  }
  return (
      <div
          class={clsx(
              'transition-all duration-300 overflow-x-hidden',
              props.isOpen ? 'overflow-y-auto' : 'overflow-y-hidden',
              props.className,
          )}
          style={heightStyle()}
      >
        {props.children}
      </div>
  )
}