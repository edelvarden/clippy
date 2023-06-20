import { invoke } from "@tauri-apps/api";
import {
  isRegistered,
  register,
  unregisterAll,
} from "@tauri-apps/api/globalShortcut";
import { Hotkey } from "../@types";

export const parseShortcut = (hotkey: Hotkey) => {
  const { ctrl, alt, shift, key } = hotkey;
  const modifiers = [];
  if (ctrl) modifiers.push("CommandOrControl");
  if (alt) modifiers.push("Alt");
  if (shift) modifiers.push("Shift");
  return `${modifiers.join("+")}+${key.toUpperCase()}`;
};

export async function registerHotkeys(hotkeys: Hotkey[]) {
  await unregisterAll();

  // unregister all hotkeys
  // for (const hotkey of hotkeys) {
  //   await unregister(hotkey.shortcut);
  // }
  // ############################################

  // Display and hide the app window
  const mainHotkey = hotkeys.find((h) => h.event === "window_display_toggle");

  if (
    mainHotkey &&
    mainHotkey?.shortcut &&
    !(await isRegistered(mainHotkey.shortcut)) &&
    mainHotkey.status
  ) {
    try {
      await register(mainHotkey.shortcut, () => invoke("window_on_mouse"));
    } catch (_) {
      console.log(_)
    }
  }
  // ############################################
}
