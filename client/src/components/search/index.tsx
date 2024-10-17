import {t_ui} from "../../i18n/ui.ts";
import {FiSearch} from "solid-icons/fi";

interface SearchProps {
  value?: string;
  onSearch?: () => void;
  onChange?: (query: string) => void;
}

export function Search(props: SearchProps) {
  return (
      <div class='flex items-center justify-center flex-nowrap gap-2 sm:gap-4 w-full'>
        <input
            class='inner-shadow block flex-1 bg-gray-100 outline-0 p-2 sm:p-3 rounded'
            value={props.value}
            on:input={(e) => {
              props.onChange?.(e.currentTarget.value)
            }}
            placeholder={t_ui('search')}
        />
        <button
            class='rounded-full outer-shadow p-2 sm:p-4 bg-gray-200 hover:bg-gray-100 duration-100 text-gray-500'
            on:click={() => props.onSearch?.()}
        >
          <FiSearch size={24}/>
        </button>
      </div>
  )
}