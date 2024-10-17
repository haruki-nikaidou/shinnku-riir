import {ResourceType} from "./index.tsx";
import {Match, Switch} from "solid-js";
import {FaBrandsWindows} from "solid-icons/fa";
import {SiAndroid} from "solid-icons/si";
import {KrkrIcon} from "../icons/KrKrIcon.tsx";
import {OnesIcon} from "../icons/OnesIcon.tsx";
import {OcTerminal2} from "solid-icons/oc";

interface GameListItemProps {
  gameType: ResourceType
}

export function GameListItemIcon(props: GameListItemProps) {
  return (
      <>
        <Switch>
          <Match when={props.gameType === ResourceType.pc}>
            <span class='rounded-full inner-shadow p-4 flex items-center justify-center bg-gray-200'>
            <FaBrandsWindows size={24}/>
            </span>
          </Match>
          <Match when={props.gameType === ResourceType.apk}>
            <span class='rounded-full inner-shadow p-4 flex items-center justify-center bg-gray-200'>
            <SiAndroid  size={24}/>
            </span>
          </Match>
          <Match when={props.gameType === ResourceType.krkr}>
            <span class='rounded-full inner-shadow p-4 flex items-center justify-center bg-gray-200'>
            <KrkrIcon size={24}/>
            </span>
          </Match>
          <Match when={props.gameType === ResourceType.ons}>
            <span class='rounded-full inner-shadow p-2 flex items-center justify-center bg-gray-200'>
            <OnesIcon size={40}/>
            </span>
          </Match>
          <Match when={props.gameType === ResourceType.tryanor}>
            <img src="/Tyranor.webp" width={56} height={56} alt="tyranor" class='inner-shadow rounded-full'/>
          </Match>
          <Match when={props.gameType === ResourceType.tools}>
            <span class='rounded-full inner-shadow p-4 flex items-center justify-center bg-gray-200'>
            <OcTerminal2 size={24}/>
            </span>
          </Match>
        </Switch>
      </>
  );
}