import { component$ } from '@builder.io/qwik';

interface Props {
  regular: string;
  solid: string;
  class?: string;
}

export const IconHover = component$(({ regular, solid, class: extraClass = '' }: Props) => {
    return (
      <span class={`fa-stack icon-hover group ${extraClass} dual-icon`}>
        <i class={`far fa-${regular} fa-stack-1x regular transition-colors`}></i>
        <i class={`fas fa-${solid} fa-stack-1x solid transition-colors`}></i>
      </span>
    );
  });