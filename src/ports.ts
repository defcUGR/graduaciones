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
import { toast } from "vue-sonner";
import type { TicketData } from "./schema";

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

  public async install(input: Ref<string>) {
    if (this._info === "keyboard") {
      this._keyboard_scope = effectScope();
      this._keyboard_scope?.run(() => {
        onKeyStroke(
          ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
          (e) => {
            e.preventDefault();
            if (PortService.scanning) input.value += e.key;
          },
          { dedupe: true }
        );
        onKeyStroke(["Delete", "Backspace"], (e) => {
          e.preventDefault();
          if (input.value.length > 0 && PortService.scanning)
            input.value = input.value.slice(0, input.value.length - 1);
        });
        onKeyStroke(["V", "v"], (e) => {
          e.preventDefault();
          if (PortService.scanning && e.ctrlKey)
            invoke("paste").then((res) => {
              if (parseInt(res as string) && /\d{7}/.test(res as string)) {
                input.value = res as string;
              } else {
                toast("El contenido pegado no es un número de TUI válido.");
              }
            });
        });
      });
    } else {
      const [err, _] = await tryit(invoke)("start_scan", {
        portName: this._info.port_name,
      });
      if (err) {
        toast(err);
        return false;
      }
    }
    PortService.scanning = true;
    return true;
  }

  public async uninstall() {
    if (this._info === "keyboard" && this._keyboard_scope)
      this._keyboard_scope?.stop();
  }

  public stop() {
    new Promise<undefined | boolean>((resolve) => {
      if (!PortService.scanning) resolve(false);
      PortService.scanning = false;
      if (this._info === "keyboard") resolve(undefined);
      emit("close_scan");
      once("scan_closed", ({ payload }) => resolve(payload as true));
    });
  }

  public async restart(input: Ref<string>) {
    if (this._info === "keyboard") await this.install(input);
    else {
      const [err, _] = await tryit(invoke)("start_scan", {
        portName: this._info.port_name,
      });
      if (err) toast(err);
      else PortService.scanning = true;
    }
  }

  public async listen(scanned: Ref<TicketData[]>, input: Ref<string>) {
    if (this._info === "keyboard")
      this.watcher_unlisten = watch(input, () => {
        if (input.value.length !== 7) {
          return;
        }

        const readInput = input.value;
        input.value = "";

        console.log("Typed: ", readInput);
      });
    else
      this.watcher_unlisten = await tauriListen<TicketData>(
        "id_scanned",
        ({ payload }) => {
          if (!payload) return;

          scanned.value.push(payload);
        }
      );
  }

  public unlisten() {
    if (this.watcher_unlisten) {
      this.watcher_unlisten();
    }
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
