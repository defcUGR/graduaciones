import { invoke } from "@tauri-apps/api";
import {
  emit,
  listen as tauriListen,
  once,
  UnlistenFn,
} from "@tauri-apps/api/event";
import { EffectScope, WatchStopHandle } from "vue";
import { onKeyStroke } from "@vueuse/core";
import { tryit } from "radash";

interface PortInfo {
  port_name: string;
  port_type:
    | string
    | {
        [type: string]: {
          manufacturer: string;
          pid: number;
          product: string;
          serial_number: string;
          vid: number;
        };
      };
}

type ComputedPortInfo = PortInfo | typeof KEYBOARD_PORT_KEY;

export const KEYBOARD_PORT_KEY = "keyboard";

class Port {
  private _info: ComputedPortInfo;
  private watcher_unlisten: UnlistenFn | WatchStopHandle | undefined;
  private _keyboard_scope: undefined | EffectScope;

  constructor(info: ComputedPortInfo) {
    this._info = info;
    // if (info === "keyboard") this._keyboard_scope = effectScope();
  }

  get info() {
    return this._info;
  }

  get port_type() {
    if (this._info === "keyboard") return;
    return {
      _type:
        typeof this._info.port_type === "string"
          ? this._info.port_type
          : Object.keys(this._info.port_type)[0],
      ...(typeof this._info.port_type === "string"
        ? {}
        : Object.values(this._info.port_type)[0]),
    };
  }
}

//@ts-ignore Weird stuff going on here
const ports: Ref<Port[]> = ref([new Port("keyboard")]);
const scanning = ref(false);

export class PortService {
  static get scanning() {
    return scanning.value;
  }

  static set scanning(val: boolean) {
    scanning.value = val;
  }

  static useScanning() {
    return scanning;
  }

  static get ports() {
    return ports.value;
  }

  static get usePorts() {
    return ports;
  }

  public static async fetchPorts(cmd: string | undefined = "get_serial_ports") {
    this.usePorts.value.push(
      ...((await invoke(cmd)) as PortInfo[]).map((info) => new Port(info))
    );
  }
}
