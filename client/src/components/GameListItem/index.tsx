export interface GameData {
    resourceType: ResourceType;
    name: string;
    size: number;
    id: string;
    tags: string[];
}

export enum ResourceType {
  pc = 'pc',
  apk = 'apk',
  krkr = 'krkr',
}

interface GameListItemProps {
    item: GameData;
}

export function GameListItem(props: GameListItemProps) {
    return (
        <div class="flex flex-col items-center justify-center p-4 bg-gray-100 rounded-lg shadow-lg">
            <h2 class="text-xl font-bold">{props.item.name}</h2>
            <p class="text-sm text-gray-700">Size: {props.item.size}MB</p>
            <div class="flex flex-wrap gap-2">
                {props.item.tags.map(tag => (
                    <span class="px-2 py-1 bg-gray-200 text-gray-700 rounded-full text-xs">{tag}</span>
                ))}
            </div>
        </div>
    );
}