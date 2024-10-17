import {clsx} from "clsx";
import {Search} from "../../components/search";
import {FoldContainer} from "../../components/fold-container";
import {createEffect, createSignal, For, onMount} from "solid-js";
import {useLocation, useNavigate} from "@solidjs/router";
import {GameData, GameListItem, ResourceType} from "../../components/GameListItem";

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
  }
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
      <main class={
        clsx(
            'container p-4 gap-4 flex flex-col items-center justify-center',
            searchIsNotEmpty() || 'pb-56 max-w-5xl',
            searchIsNotEmpty() && 'max-w-7xl'
        )
      }>
        <img src="/upsetgal-logo.webp" alt="失落的小站" class='max-w-xs' loading='lazy'/>
        <Search
            value={query()}
            onChange={setQuery}
        />
        <FoldContainer
            maxHeight="75vh" isOpen={searchIsNotEmpty()}
            className='w-full'
        >
          <ul
              class='gap-4 flex flex-col w-full items-center'
          >
            <For each={ExampleGameData}>
              {game => <GameListItem item={game}/>}
            </For>
          </ul>
        </FoldContainer>
      </main>
  )
}