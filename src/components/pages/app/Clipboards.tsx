import { invoke } from "@tauri-apps/api";
import { BaseDirectory, writeBinaryFile } from "@tauri-apps/api/fs";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import utc from "dayjs/plugin/utc";
import { BsImages } from "solid-icons/bs";
import { FiArrowUp, FiFileText } from "solid-icons/fi";
import { IoTrashOutline } from "solid-icons/io";
import { VsStarFull, VsSymbolColor } from "solid-icons/vs";
import { Component, For, Show, createSignal } from "solid-js";
import { Clips } from "../../../@types";
import clippy from "../../../assets/clippy.png";
import ClipboardStore from "../../../store/ClipboardStore";
import SettingsStore from "../../../store/SettingsStore";
import { formatBytes } from "../../../utils/helpers";

dayjs.extend(utc);
dayjs.extend(relativeTime);

interface ClipboardsProps {}

export const Clipboards: Component<ClipboardsProps> = ({}) => {
  let myRef: HTMLDivElement;
  let timer: NodeJS.Timeout;
  const [scrollToTop, setScrollToTop] = createSignal(false);

  const { clipboards, setClipboards, getClipboards, setWhere } = ClipboardStore;
  const { globalHotkeyEvent, hotkeys } = SettingsStore;

  const onScroll = async () => {
    const bottom =
      myRef && myRef.scrollHeight - myRef.scrollTop === myRef.clientHeight;

    if (myRef?.scrollTop !== 0) {
      setScrollToTop(true);
    } else {
      setScrollToTop(false);
    }

    if (bottom) {
      setWhere((prev) => ({ ...prev, cursor: clipboards().length }));
      const newClipboards = await getClipboards();
      setClipboards((prev) => [...prev, ...newClipboards]);
    }
  };

  const IconFunctions = ({ id, ...clipboard }: Clips) => (
    <>
      <VsStarFull
        onClick={async (e) => {
          e.stopPropagation();
          await invoke<boolean>("star_clipboard", {
            id,
            star: !clipboard.star,
          });
          setClipboards((prev) =>
            prev.map((o) => (o.id === id ? { ...o, star: !clipboard.star } : o))
          );
        }}
        class={`${
          clipboard.star
            ? "text-yellow-400 dark:text-yellow-300"
            : "hidden text-zinc-700"
        } z-10 text-xs hover:text-yellow-400 group-hover:block dark:text-white dark:hover:text-yellow-300`}
      />
      <IoTrashOutline
        onClick={async (e) => {
          e.stopPropagation();
          if (await invoke<boolean>("delete_clipboard", { id })) {
            setClipboards((prev) => prev.filter((o) => o.id !== id));
          }
        }}
        class="hidden text-xs text-zinc-700 hover:text-red-600 group-hover:block dark:text-white dark:hover:text-red-600"
      />
    </>
  );

  return (
    <Show
      when={clipboards().length}
      fallback={
        <div class="flex h-screen w-full flex-col items-center justify-center space-y-3 opacity-30">
          <img src={clippy} width="50%" alt="no Order" />

          <h2 class="text-2xl font-medium opacity-50">No Clipboards yet...</h2>
        </div>
      }
    >
      <div
        ref={(el) => (myRef = el)}
        onScroll={onScroll}
        class="h-full overflow-auto pb-5"
      >
        <Show when={scrollToTop()}>
          <button
            type="button"
            class="absolute bottom-5 right-4 rounded-full bg-neutral-700 px-2 py-1 hover:bg-gray-500"
            onClick={() => myRef?.scrollTo(0, 0)}
          >
            <div class="relative flex items-center justify-center py-1">
              <FiArrowUp class="text-xl !text-white dark:!text-white " />
              {globalHotkeyEvent() && (
                <div class="absolute left-0 top-0 -ml-3 -mt-3 rounded-sm bg-zinc-600 px-1 text-[12px] font-semibold">
                  {hotkeys().find((key) => key.event === "scroll_to_top")?.key}
                </div>
              )}
            </div>
          </button>
        </Show>

        <For each={clipboards()}>
          {(clipboard, index) => {
            const {
              content,
              type,
              id,
              created_date,
              blob,
              width,
              height,
              size,
            } = clipboard;

            const src = blob
              ? URL.createObjectURL(
                  new Blob([new Uint8Array(blob)], {
                    type: "image/png",
                  })
                )
              : null;

            return (
              <button
                type="button"
                class="group w-full cursor-pointer px-3 hover:bg-neutral-700"
                onClick={(e) => {
                  e.stopPropagation();

                  if (e.detail === 1) {
                    timer = setTimeout(async () => {
                      await invoke("copy_clipboard", { id });
                    }, 200);
                  }
                }}
                onDblClick={async (e) => {
                  clearTimeout(timer);
                  e.stopPropagation();
                  if (type !== "image" || !blob) return;
                  await writeBinaryFile(`clipboard-${id}.png`, blob, {
                    dir: BaseDirectory.Desktop,
                  });
                }}
              >
                <div class="flex justify-between py-3">
                  <div class="flex min-w-0">
                    <div class="flex items-center">
                      <div class="relative" title={id + ""}>
                        {type === "text" && (
                          <FiFileText class="text-2xl text-zinc-700 dark:text-white" />
                        )}
                        {type === "image" && (
                          <BsImages class="text-2xl text-zinc-700 dark:text-white" />
                        )}
                        {type === "color" && (
                          <VsSymbolColor class="text-2xl text-zinc-700 dark:text-white" />
                        )}
                        {globalHotkeyEvent() && (
                          <div class="absolute left-0 top-0 -ml-3 -mt-3 rounded-sm bg-zinc-600 px-1 text-[12px] font-semibold">
                            {index() + 1 < 10 && index() + 1}
                          </div>
                        )}
                      </div>
                    </div>
                    <div class="truncate px-5">
                      {src ? (
                        <img
                          src={src}
                          // style={{ height: '200px' }}
                          class="relative max-h-64 w-full"
                          alt={`${width}x${height} ${size}`}
                          title={`${width}x${height} ${formatBytes(
                            Number(size || "0")
                          )}`}
                        />
                      ) : (
                        <div class="flex" title={content || ""}>
                          {type === "color" && (
                            <div
                              class="mr-1 h-5 w-5 border border-solid border-black"
                              style={{
                                "background-color": `${
                                  content?.includes("#")
                                    ? `${content}`
                                    : `#${content}`
                                }`,
                              }}
                            />
                          )}
                          <p class="text-sm">{content || " "}</p>
                        </div>
                      )}
                      <div class="text-left text-xs text-zinc-400">
                        {dayjs.utc(created_date!).fromNow()}
                      </div>
                    </div>
                  </div>
                  <div class="flex w-12 flex-col items-end justify-between pl-1">
                    {IconFunctions(clipboard)}
                  </div>
                </div>
                <hr class="border-zinc-700" />
              </button>
            );
          }}
        </For>
      </div>
    </Show>
  );
};