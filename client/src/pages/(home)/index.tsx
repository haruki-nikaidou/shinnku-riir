import {clsx} from "clsx";
import {Search} from "../../components/search";
import {FoldContainer} from "../../components/fold-container";
import {createEffect, createSignal, For, onMount} from "solid-js";
import {useLocation, useNavigate} from "@solidjs/router";
import {GameData, GameListItem, ResourceType} from "../../components/GameListItem";
import {t_ui} from "../../i18n/ui.ts";
import {BiRegularBrush} from "solid-icons/bi";
import {IoChatbubbles} from "solid-icons/io";
import {SiTelegram} from "solid-icons/si";

const ExampleGameData: GameData[] = [
  {
    resourceType: ResourceType.pc,
    name: 'Example Game 1',
    size: 1000000,
    id: 'example-game-1',
    tags: ['tag1', 'tag2', 'tag3']
  },
  {
    resourceType: ResourceType.apk,
    name: 'Example Game 2',
    size: 2000000,
    id: 'example-game-2',
    tags: ['tag1', 'tag2', 'tag3']
  },
  {
    resourceType: ResourceType.krkr,
    name: 'Example Game 3',
    size: 3000000,
    id: 'example-game-3',
    tags: ['tag1', 'tag2', 'tag3']
  },
  {
    resourceType: ResourceType.ons,
    name: 'Example Game 4',
    size: 4000000,
    id: 'example-game-4',
    tags: ['tag1', 'tag2', 'tag3']
  },
  {
    resourceType: ResourceType.tryanor,
    name: 'Example Game 5',
    size: 5000000,
    id: 'example-game-5',
    tags: ['tag1', 'tag2', 'tag3']
  },
  {
    resourceType: ResourceType.tools,
    name: 'Example Game 6',
    size: 6000000,
    id: 'example-game-6',
    tags: ['tag1', 'tag2', 'tag3']
  },
]

export function HomePage() {
  const [query, setQuery] = createSignal('');
  const navigate = useNavigate();
  const location = useLocation();
  // Update the input based on current query param on component mount
  onMount(() => {
    const queryParams = new URLSearchParams(location.search);
    const query = queryParams.get("q");
    if (query) {
      setQuery(query);
    }
  });
  createEffect(() => {
    if (!query()) {
      navigate(`?`, {replace: true});
    } else {
      navigate(`?q=${query()}`, {replace: true});
    }
  });
  const searchIsNotEmpty = () => query() !== '';
  return (
      <>
        <header class='flex items-center justify-end gap-4 w-full p-4'>
          <a
              class='bg-gray-200 outer-shadow-small rounded-full p-2 hover:bg-gray-100 duration-100'
              href='https://galgame.dev'
          >
            <IoChatbubbles size={24}/>
          </a>
          <a
              class='bg-gray-200 outer-shadow-small rounded-full p-2 hover:bg-gray-100 duration-100'
              href='https://galgame.dev'
          >
            <SiTelegram size={24}/>
          </a>
          <button class='bg-gray-200 outer-shadow-small rounded-full p-2 hover:bg-gray-100 duration-100'>
            <BiRegularBrush size={24}/>
          </button>
        </header>
        <main class={
          clsx(
              'container p-4 gap-4 flex flex-col items-center justify-center',
              searchIsNotEmpty() || 'pb-40 max-w-5xl',
              searchIsNotEmpty() && 'max-w-7xl'
          )
        }>
          <img src="/upsetgal-logo.webp" alt="失落的小站" class='max-w-md' loading='lazy'/>
          <Search
              value={query()}
              onChange={setQuery}
          />
          <FoldContainer
              maxHeight="75vh" isOpen={searchIsNotEmpty()}
              className='w-full'
          >
            <ul
                class='gap-4 flex flex-col w-full items-center px-2 py-4'
            >
              <For each={ExampleGameData}>
                {game => <GameListItem item={game}/>}
              </For>
            </ul>
          </FoldContainer>
          <div class='flex items-center justify-center gap-6'>
            <a class='rounded-full border outer-shadow-small text-sm bg-gray-200 p-3.5 hover:bg-gray-100 duration-100'>
              {t_ui('tutorial')}
            </a>
            <a class='rounded-full border outer-shadow-small text-sm bg-gray-200 p-3.5 hover:bg-gray-100 duration-100'>
              {t_ui('random_roll_one')}
            </a>
          </div>
          <div class='bg-gray-100 p-4 rounded-3xl mt-4 text-gray-500 text-center'>
            <p>
              It is possible to put some ADs here.
            </p>
            <p>
              Even, Ad can have multiple lines.
            </p>
          </div>
        </main>
        <footer class='w-full bg-neutral-400 p-3 text-xs text-gray-700'>
          <p>
            本资源仅供学习交流使用，请务必于下载后 24 小时内删除，如有能力请购买正版支持。
          </p>
        </footer>
      </>
  )
}