import { $, component$, useSignal, useVisibleTask$ } from '@builder.io/qwik';
import { invoke } from '@tauri-apps/api/core';
import { Window as TauriWindow } from '@tauri-apps/api/window';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Tabs, Button, Tooltip, Spinner } from 'flowbite-qwik';
import { platform } from '@tauri-apps/plugin-os';
import { enable as enable_autostart, disable as disable_autostart, isEnabled as is_autostart_enabled } from '@tauri-apps/plugin-autostart';


export const SettingsPage = component$(() => {
  const config = useSignal<Record<string, any> | null>(null);
  const loading = useSignal(true);
  const auto_paste_on_copy = useSignal<HTMLElement>();
  const auto_paste_on_copy_text = useSignal<HTMLElement>();

  const mainWindow = useSignal<TauriWindow | null>(null);

  const currentPlatform = useSignal<string | null>(null);

  const autoStartRef = useSignal<HTMLInputElement>();
  const autoStartEnabled = useSignal<boolean>(false);

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {
    // Get the current platform
    const platformInfo = await platform();
    currentPlatform.value = platformInfo;
    console.log("Current platform:", currentPlatform.value);

    // Check if autostart is enabled
    autoStartEnabled.value = await is_autostart_enabled();

    // Get the full config object from the backend
    const raw = await invoke<string>('get_config_value', { property: null });
    config.value = JSON.parse(raw);

    // Get the main window object
    mainWindow.value = (await WebviewWindow.getByLabel('main'))?.window || null;

    // Display the settings in the UI
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
    return <div class="p-4 flex flex-col items-center justify-center h-screen space-y-4">

      <Spinner size="12" />
      <br />
      <p>Loading settings...</p>
      
      </div>;
  }

  return (
    
    <div class="p-6 space-y-2">
      <div class="flex flex-row items-center justify-between mb-4">
        <h1 class="text-4xl font-bold">TactiClip Settings</h1>
        <img src="../../../src-tauri/icons/icon.png" alt="TactiClip Icon" width={60} height={60}/>
      </div>

      <Tabs variant="underline">
      <Tabs.Tab title="Main window">
          <div class="space-y-4 mt-4 p-2 rounded-md overflow-y-auto h-[calc(100vh-250px)]">
            {/* Position (bind:value handles update) */}
            <div>
              <label class="block mb-1 font-medium">Position</label>
              <div class="flex items-center space-x-2">
                <select
                  class="form-select rounded-lg w-32"
                  value={config.value.window_position}
                  onChange$={(e) => {
                    if (!config.value) {return} else config.value.window_position = (e.target as HTMLSelectElement).value;
                  }}
                >
                  <option value="top">Top</option>
                  <option value="bottom">Bottom</option>
                  <option value="left">Left</option>
                  <option value="right">Right</option>
                </select>
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Position and layout of the main window.</div>
                </Tooltip>
              </div>
            </div>

            {/* Primary size factor */}
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
                      if (!config.value) {return} else config.value.window_primary_factor = val / 100;
                    }}
                  />
                  <span class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-500 pointer-events-none">%</span>
                </div>
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Window relative size, based on monitor size <br/> and layout direction.</div>
                </Tooltip>
              </div>
            </div>

            {/* Secondary size */}
            <div>
              <label class="block mb-1 font-medium">Secondary size (px)</label>
              <div class="flex items-center space-x-2">
                <input
                  type="number"
                  class="form-input rounded-lg w-32"
                  value={config.value.window_secondary_size}
                  onInput$={(e) => {
                    if (!config.value) {return} else config.value.window_secondary_size = Number((e.target as HTMLInputElement).value);
                  }}
                />
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Window size on the other axis (in pixels).</div>
                </Tooltip>
              </div>
            </div>

            {/* Horizontal offset */}
            <div>
              <label class="block mb-1 font-medium">Horizontal offset (px)</label>
              <div class="flex items-center space-x-2">
                <input
                  type="number"
                  class="form-input rounded-lg w-32"
                  value={config.value.window_padding_x}
                  onChange$={(e) => {
                    if (!config.value) {return} else config.value.window_padding_x = Number((e.target as HTMLInputElement).value);
                  }}
                />
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Margin from the border of the screen (in pixels).</div>
                </Tooltip>
              </div>
            </div>

            {/* Vertical margin */}
            <div>
              <label class="block mb-1 font-medium">Vertical margin (px)</label>
              <div class="flex items-center space-x-2">
                <input
                  type="number"
                  class="form-input rounded-lg w-32"
                  value={config.value.window_padding_y}
                  onChange$={(e) => {
                    if (!config.value) {return} else config.value.window_padding_y = Number((e.target as HTMLInputElement).value);
                  }}
                />
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Margin from the border of the screen (in pixels).</div>
                </Tooltip>
              </div>
            </div>
          </div>
        </Tabs.Tab>

        <Tabs.Tab title="Animation">
          <div class="space-y-4 mt-4 p-2 rounded-md overflow-y-auto">
            {/* Animation duration */}
            <div>
              <label class="block mb-1 font-medium">Animation duration (ms)</label>
              <div class="flex items-center space-x-2">
                <input
                  type="number"
                  class="form-input rounded-lg w-32"
                  value={config.value.window_animation_duration}
                  onInput$={(e) => {if (!config.value) {return} else config.value.window_animation_duration = Number((e.target as HTMLInputElement).value);}}
                />
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Sliding animation duration (in ms).<br/>The minimal duration depends on the <br />number of steps.</div>
                </Tooltip>
              </div>
            </div>

            {/* Step count */}
            <div>
              <label class="block mb-1 font-medium">Step count</label>
              <div class="flex items-center space-x-2">
                <input
                  type="number"
                  class="form-input rounded-lg w-32"
                  value={config.value.window_steps}
                  onInput$={(e) => {if (!config.value) {return} else config.value.window_steps = Number((e.target as HTMLInputElement).value);}}
                />
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Number of frames in the animation.<br/>Large numbers will make the animation <br/>smoother but might also make it longer.</div>
                </Tooltip>
              </div>
            </div>

            {/* Easing factor */}
            <div>
              <label class="block mb-1 font-medium">Easing factor</label>
              <div class="flex items-center space-x-2">
                <input
                  type="number"
                  class="form-input rounded-lg w-32"
                  value={config.value.ease_factor}
                  onInput$={(e) => {if (!config.value) {return} else config.value.ease_factor = Number((e.target as HTMLInputElement).value);}}
                />
                <Tooltip style="dark" placement="right">
                  <span q:slot="trigger" class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full">?</span>
                  <div q:slot="content">Acceleration and deceleration coefficients<br/>at the start and the end of the animation.</div>
                </Tooltip>
              </div>
            </div>
          </div>
        </Tabs.Tab>

        <Tabs.Tab title="Behavior">
          <div class="space-y-4 mt-4 p-2 rounded-md overflow-y-auto">
          <div class="flex items-center space-x-2">
              <input 
                type="checkbox" 
                checked={autoStartEnabled.value} 
                class="rounded-md"
                ref={autoStartRef}
              />
              <label>Start automatically</label>
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Start the application with user's session.</div>
                  </Tooltip>
            </div>   

            <div class="flex items-center space-x-2">
              <input 
                type="checkbox" 
                checked={config.value.window_rewrite_history_on_copy} 
                class="rounded-md" 
                onChange$={async (e) => {
                  if (!config.value) return;
                  const element = e.target as HTMLInputElement;
                  config.value.window_rewrite_history_on_copy = element.checked;
                }}
              />
              <label>Rewrite history on copy</label>
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">If enabled, moves an item that <br/>have been pushed to the clipboard <br/>at the first place in the history.</div>
                  </Tooltip>
            </div>   

            <div class="flex items-center space-x-2">
                <input 
                    type="checkbox"
                    checked={config.value.auto_hide_on_copy}
                    onChange$={async (e) => {
                        const element = e.target as HTMLInputElement;
                        await autoPasteAvailable(element.checked)

                        if (!config.value) return;
                        config.value.auto_hide_on_copy = element.checked;
                      }
                    }
                    class="rounded-md"
                    />
                  <label>Auto-hide after copy</label>
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">If enabled, slides the windows <br/>out of the screen after pushing <br />to the clipboard.</div>
                  </Tooltip>
            </div>

            <div class="flex items-center space-x-2">
                <input
                    type="checkbox"
                    checked={config.value.auto_paste_on_copy}
                    disabled={!config.value.auto_hide_on_copy}
                    ref={auto_paste_on_copy}
                    class="rounded-md"
                    onChange$={async (e) => {
                      if (!config.value) return;
                      const element = e.target as HTMLInputElement;
                      config.value.auto_paste_on_copy = element.checked;
                    }}
                />
                <label ref={auto_paste_on_copy_text}>Auto-paste after copy</label>
                <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Trigger pasting automatically.</div>
                </Tooltip>
            </div>

            <div>
              <label class="block mb-1 font-medium">Max characters displayed</label>
              <div class="flex items-center space-x-2">
                <input 
                  type="number" 
                  class="form-input rounded-lg w-32" 
                  value={config.value.max_displayed_characters}
                  onChange$={(e) => {
                    if (!config.value) {return} else config.value.max_displayed_characters = Number((e.target as HTMLInputElement).value);
                  }}
                />
                  <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Maximum amount of characters to be <br/>displayed in a single item.</div>
                  </Tooltip>
              </div>
            </div>
          </div>
        </Tabs.Tab>

        <Tabs.Tab title="Scrolling">
          <div class="space-y-4 mt-4 p-2 rounded-md overflow-y-auto">
            <div>
              <label class="block mb-1 font-medium">Scroll factor</label>
                  
              <div class="flex items-center space-x-2">
              <input 
                type="number" 
                step="0.1" 
                class="form-input rounded-lg w-32" 
                value={config.value.scroll_factor} 
                onChange$={(e) => {
                  if (!config.value) {return} else config.value.scroll_factor = Number((e.target as HTMLInputElement).value);
                }}
              />
              <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Changes the speed of scrolling. <br/>Smooth scroll might require significantly <br />higher values.</div>
              </Tooltip>
            </div>
            </div>

            <div class="flex items-center space-x-2">
              <input 
                type="checkbox" 
                class="rounded-md" 
                checked={config.value.smooth_scroll} 
                onChange$={async (e) => {
                  if (!config.value) return;
                  const element = e.target as HTMLInputElement;
                  config.value.smooth_scroll = element.checked;
                }}
              />
              <label>Smooth scrolling</label>
              <Tooltip style="dark" placement="right">
                    <span
                      q:slot="trigger"
                      class="flex items-center justify-center text-gray-400 cursor-help w-5 h-5 border border-gray-400 rounded-full"
                    >?</span>
                    <div q:slot="content">Smooth the scroll with acceleration and <br />deceleration. Migth create a difference of <br />scroll speed between axis.</div>
              </Tooltip>
            </div>

            <div class="flex items-center space-x-2">
              <input 
                type="checkbox" 
                class="rounded-md"  
                checked={config.value.reset_scroll_on_show} 
                onChange$={async (e) => {
                  if (!config.value) return; 
                  const element = e.target as HTMLInputElement;
                  config.value.reset_scroll_on_show = element.checked;
                }}
              />
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

        {/* Disable Windows native clipboard manager */}
        {currentPlatform.value === "windows" && (
          <Tabs.Tab title="Extras">
            <div class="space-y-4 mt-4 p-2 rounded-md overflow-y-auto">
              <div class="flex flex-col items-center space-y-2">
                <p class="text-sm text-gray-600 text-justify">
                  You can use those buttons to enable or disable the native Windows clipboard manager using Windows Registry keys.
                  <br />This will remove the binding of the Windows key + V shortcut to the native clipboard manager and make it exclusive to TactiClip.
                  <br />Note that this action needs to be performed with administrator privileges, a prompt will be shown if needed.
                </p>
                <Button 
                  color="default" 
                  class="w-48 h-10"
                  onClick$={
                    $(async () => {
                      await invoke("disable_windows_clipboard_history", {});                      // reload Settings
                    })
                  }
                >Disable Windows Clipboard Manager</Button>
                <Button 
                  color="dark" 
                  class="w-48 h-10"
                  onClick$={
                    $(async () => {
                      await invoke("enable_windows_clipboard_history", {});                      // reload Settings
                    })
                  }
                >Enable Windows Clipboard Manager</Button>
              </div>
            </div>
          </Tabs.Tab>
        )}
      </Tabs>

      <div class="w-full fixed bottom-[10px] left-0 px-4 bg-gradient-to-b from-transparent to-white ">
        <div class="flex justify-between items-center w-full gap-4 bg-transparent">
          <Button 
            color="red" 
            class="w-32 h-10"
            onClick$={
              $(async () => {
                invoke("reset_config")
              })
            }
          >Reset to defaults</Button>

          <div class="flex gap-2 bg-transparent">
            <Button 
              color="dark" 
              class="w-32 h-10" 
              onClick$={
                $(async () => {
                  invoke("cancel_config")
                })
              }
            >Cancel</Button>

            <Button 
              color="default" 
              class="w-32 h-10"
              onClick$={
                $(async () => {
                  invoke("preview_config", { payload: JSON.stringify(config.value) });
                })
              }
            >Preview</Button>

            <Button 
              color="green" 
              class="w-32 h-10"
              onClick$={
                $(async () => {
                  // Check if autostart is enabled
                  if (autoStartRef.value?.checked === autoStartEnabled.value) {
                    // No change, do nothing
                  } else if (autoStartRef.value?.checked) {
                    // Enable autostart
                    await enable_autostart();
                    console.log("Autostart enabled");
                  } else {
                    // Disable autostart
                    await disable_autostart();
                    console.log("Autostart disabled");
                  }
                  

                  invoke("save_config", { payload: JSON.stringify(config.value) });
                })
              }
            >Save</Button>
          </div>
        </div>
      </div>
    </div>
  );
});

export default SettingsPage;