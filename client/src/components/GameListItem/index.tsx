import {GameListItemIcon} from "./ItemIcon.tsx";
import prettyBytes from "pretty-bytes";

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
  ons = 'ons',
  tryanor = 'tryanor',
  tools = 'tools',
}

function ResourceTypeTip(t: ResourceType): string {
  switch (t) {
    case ResourceType.pc:
      return 'Windows';
    case ResourceType.apk:
      return 'APK直装';
    case ResourceType.krkr:
      return 'Kirikiri2 模拟器';
    case ResourceType.ons:
      return 'ONS 模拟器';
    case ResourceType.tryanor:
      return 'Tryanor 模拟器';
    case ResourceType.tools:
      return '工具';
  }
}

interface GameListItemProps {
  item: GameData;
}

export function GameListItem(props: GameListItemProps) {
  return (
      <li class="p-4 bg-gray-200 rounded-lg w-full outer-shadow">
        <div class='flex flex-row gap-4'>
          <GameListItemIcon gameType={props.item.resourceType}/>
          <div class='space-y-0'>
            <p class='font-bold text-lg'>{props.item.name}</p>
            <p class='text-sm text-gray-500 space-x-2'>
              <span class='p-1 inner-shadow rounded'>
                {ResourceTypeTip(props.item.resourceType)}
              </span>
              <span>
                {prettyBytes(props.item.size)}
              </span>
            </p>
          </div>
        </div>
      </li>
  );
}