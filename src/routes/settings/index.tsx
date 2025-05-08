import { $, component$, useSignal, useVisibleTask$ } from '@builder.io/qwik';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, Window as TauriWindow } from '@tauri-apps/api/window';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Tabs, Button, Tooltip } from 'flowbite-qwik';

export const SettingsPage = component$(() => {
    const config = useSignal<Record<string, any> | null>(null);
    const loading = useSignal(true);
    const auto_paste_on_copy = useSignal<HTMLElement>();
    const auto_paste_on_copy_text = useSignal<HTMLElement>();

    const mainWindow = useSignal<TauriWindow | null>(null);

    // récupère le payload prêt pour invoke
  const getPayload = $((): string => JSON.stringify(config.value));

  // callbacks pour les boutons
  const resetDefaults = $(async () => {
    await invoke('reset_config');
    mainWindow.value?.emit("reload", {});     // reload fenêtre principale
    (await getCurrentWindow()).destroy();                             // reload Settings
  });

  const preview = $(async () => {
    await invoke('preview_config', { payload: getPayload() });
    mainWindow.value?.emit("reload-window", {});     // reload fenêtre principale
  });

  const cancel = $(async () => {
    await invoke('cancel_config');
    (await getCurrentWindow()).destroy(); 
  });

  const save = $(async () => {
    await invoke('save_config', { payload: getPayload() });
    mainWindow.value?.emit("reload", {});     // reload fenêtre principale
  });
    

  useVisibleTask$(async () => {
    const raw = await invoke<string>('get_config_value', { property: null });
    config.value = JSON.parse(raw);
    mainWindow.value = (await WebviewWindow.getByLabel('main'))?.window || null;
    loading.value = false;
  });

  const autoPasteAvailable = $(async (available: boolean) => {
    if (available) {
        auto_paste_on_copy.value?.removeAttribute("disabled");
        auto_paste_on_copy.value?.classList.remove("cursor-not-allowed", "opacity-50");
        auto_paste_on_copy_text.value?.classList.remove("cursor-not-allowed", "opacity-50");
    } else {
        auto_paste_on_copy.value?.setAttribute("disabled", "true");
        auto_paste_on_copy.value?.classList.add("cursor-not-allowed", "opacity-50");
        auto_paste_on_copy_text.value?.classList.add("cursor-not-allowed", "opacity-50");
    }
  })

  if (loading.value || !config.value) {
    return <div class="p-4">Loading settings...</div>;
  }

  return (
    
    <div class="p-6 space-y-4">
      <h1 class="text-4xl font-bold">TactiClip Settings</h1>

      <Tabs variant="underline">
        <Tabs.Tab title="Main window">
          <div class="space-y-4 mt-4 p-2 rounded-md overflow-x-auto">
            <div>
              <label class="block mb-1 font-medium">Position</label>
              <div class="flex items-center space-x-2">
                <select class="form-select rounded-lg w-32" bind:value={config.value.window_position}>
                  <option value="top">Top</option>
                  <option value="bottom">Bottom</option>
                  <option value="left">Left</option>
                  <option value="right">Right</option>
                </select>
                <Tooltip style="dark" placement="right">
                  <span
                    q:slot="trigger"
                    class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                  >?</span>
                  <div q:slot="content">Position and layout of the main window.</div>
                </Tooltip>
              </div>
            </div>

            <div>
              <label class="block mb-1 font-medium">Primary size factor (% of screen size)</label>
              <div class="flex items-center space-x-2">
                <div class="relative">
                  <input
                    type="number"
                    min={0}
                    max={100}
                    step={1}
                    class="form-input rounded-lg pr-8 w-32"
                    value={config.value.window_primary_factor * 100}
                    onInput$={(e) => {
                      const val = Math.max(0, Math.min(100, Number((e.target as HTMLInputElement).value)));
                      if (config.value) {
                        config.value.window_primary_factor = val / 100;
                      }
                    }}
                  />
                  <span class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-500 pointer-events-none">%</span>
                </div>
                <Tooltip style="dark" placement="right">
                  <span
                    q:slot="trigger"
                    class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                  >?</span>
                  <div q:slot="content">Window relative size, based on monitor size <br/> and layout direction.</div>
                </Tooltip>
              </div>
            </div>

            <div>
              <label class="block mb-1 font-medium">Secondary size (px)</label>
              <div class="flex items-center space-x-2">
                <input type="number" class="form-input rounded-lg w-32" value={config.value.window_secondary_size} />
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Window size on the other axis (in pixels).</div>
                </Tooltip>
              </div>
            </div>

            <div>
              <label class="block mb-1 font-medium">Horizontal offset (px)</label>
              <div class="flex items-center space-x-2">
                <input type="number" class="form-input rounded-lg w-32" value={config.value.window_padding_x} />
                <Tooltip style="dark" placement="right">
                  <span
                    q:slot="trigger"
                    class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                  >?</span>
                  <div q:slot="content">Margin from the border of the screen (in pixels).</div>
                </Tooltip>
              </div>
            </div>

            <div>
              <label class="block mb-1 font-medium">Vertical margin (px)</label>
              <div class="flex items-center space-x-2">
                <input type="number" class="form-input rounded-lg w-32" value={config.value.window_padding_y} />
                <Tooltip style="dark" placement="right">
                  <span
                    q:slot="trigger"
                    class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                  >?</span>
                  <div q:slot="content">Margin from the border of the screen (in pixels).</div>
                </Tooltip>
              </div>
            </div>
          </div>
        </Tabs.Tab>

        <Tabs.Tab title="Animation">
          <div class="space-y-4 mt-4 p-2 rounded-md overflow-x-auto">
            <div>
              <label class="block mb-1 font-medium">Animation duration (ms)</label>
              <div class="flex items-center space-x-2">
                <input type="number" class="form-input rounded-lg w-32" value={config.value.window_animation_duration} />
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Sliding animation duration (in ms).<br/>The minimal duration depends on the number of steps.</div>
                </Tooltip>
              </div>
            </div>

            <div>
              <label class="block mb-1 font-medium">Step count</label>
              <div class="flex items-center space-x-2">
                <input type="number" class="form-input rounded-lg w-32" value={config.value.window_steps} />
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Number of frames in the animation.<br/>Large numbers will make the animation smoother<br/>but might also make it longer.</div>
                </Tooltip>
              </div>
              
            </div>

            <div>
              <label class="block mb-1 font-medium">Easing factor</label>
              <div class="flex items-center space-x-2">
              <input type="number" class="form-input rounded-lg w-32" value={config.value.ease_factor} />
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Acceleration and deceleration coefficients<br/>at the start and the end of the animation.</div>
                </Tooltip>
              </div>
            </div>
          </div>
        </Tabs.Tab>

        <Tabs.Tab title="Behavior">
          <div class="space-y-4 mt-4 p-2 rounded-md overflow-x-auto">
            <div class="flex items-center space-x-2">
            <div class="flex items-center space-x-2">
              <input type="checkbox" checked={config.value.window_rewrite_history_on_copy} class="rounded-md" />
              <label>Rewrite history on copy</label>
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">If enabled, moves an item that have been<br/>pushed to the clipboard at the first place<br/>in the history.</div>
                  </Tooltip>
            </div>
              
            </div>              
            <div class="flex items-center space-x-2">
                <input 
                    type="checkbox"
                    checked={config.value.auto_hide_on_copy}
                    onChange$={async (e) => {
                            await autoPasteAvailable((e.target as HTMLInputElement).checked)
                        }
                    }
                    class="rounded-md"
                    />
                  <label>Auto-hide window after copy</label>
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">If enabled, slides the windowsout of the <br/>screen after pushing to the clipboard.</div>
                  </Tooltip>
            </div>

            <div class="flex items-center space-x-2">
                <input
                    type="checkbox"
                    checked={config.value.auto_paste_on_copy}
                    disabled={!config.value.auto_hide_on_copy}
                    ref={auto_paste_on_copy}
                    class="rounded-md"
                />
                <label ref={auto_paste_on_copy_text}>Auto-paste after copy</label>
                <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Trigger pasting once the window is hidden.</div>
                </Tooltip>
            </div>

            <div>
              <label class="block mb-1 font-medium">Max characters displayed</label>
              <div class="flex items-center space-x-2">
                <input type="number" class="form-input rounded-lg w-32" value={config.value.max_displayed_characters} />
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Maximum amount of characters to be displayed<br/>in a single item.</div>
                  </Tooltip>
              </div>
            </div>
          </div>
        </Tabs.Tab>

        <Tabs.Tab title="Scrolling">
          <div class="space-y-4 mt-4 p-2 rounded-md">
            <div>
              <label class="block mb-1 font-medium">Scroll factor</label>
                  
              <div class="flex items-center space-x-2">
              <input type="number" step="0.1" class="form-input rounded-lg w-32" value={config.value.scroll_factor} />
              <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Changes the speed of scrolling.</div>
              </Tooltip>
            </div>
            </div>

            <div class="flex items-center space-x-2">
              <input type="checkbox" checked={config.value.smooth_scroll} class="rounded-md" />
              <label>Smooth scrolling</label>
              <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Smooth the scroll with acceleration and deceleration.</div>
              </Tooltip>
            </div>

            <div class="flex items-center space-x-2">
              <input type="checkbox" checked={config.value.reset_scroll_on_show} class="rounded-md"/>
              <label>Reset scroll when shown</label>
              <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Set scrolling position to 0 on.</div>
              </Tooltip>
            </div>

          </div>
        </Tabs.Tab>
      </Tabs>

      <div class="w-full fixed bottom-[15px] left-0 px-4">
        <div class="flex justify-between items-center w-full gap-4">
          <Button color="red" class="w-32 h-10">Reset to defaults</Button>

          <div class="flex gap-2">
            <Button color="dark" class="w-32 h-10">Cancel</Button>
            <Button color="default" class="w-32 h-10">Preview</Button>
            <Button color="green" class="w-32 h-10">Save</Button>
          </div>
        </div>
      </div>
    </div>
  );
});

export default SettingsPage;