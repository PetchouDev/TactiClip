import { component$, useSignal, useVisibleTask$ } from '@builder.io/qwik';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { ClipboardItem } from './ClipboardItem';
import { ActionRow } from './ActionRow';

type ClipboardEntry = {
  id: number;
  enntry_type: string;
  content: string;
  added_at: string;
  pinned: boolean;
  forced_language: string;
};

export default component$(() => {
  const loading = useSignal(true);
  const clipboardData = useSignal<any[]>([]);
  const progress = useSignal(0);

  const listRef = useSignal<HTMLElement>();

  const layout = useSignal("horizontal")
  const positon = useSignal("top")

  // Setup the window logic and callbacks once the ClipboardHistory component is mounted
  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(async () => {

    // Get the ids of the clipboard entries from the backend
    console.log('Loading clipboard history...');
    const ids: number[] = await invoke('get_clipboard_entries_ids');
    const total = ids.length;
    console.log('Clipboard IDs:', ids);

    const SCROLL_FACTOR_STR: string = await invoke("get_config_value", { property: "scroll_factor" });
    const SMOOTH_SCROLL_STR: string = await invoke("get_config_value", { property: "smooth_scroll" });
    const POSITION: string          = await invoke("get_config_value", { property: "window_position" });
 
    // Parse the scroll factor to a number
    const scroll_factor_float = parseFloat(SCROLL_FACTOR_STR);
    const SCROLL_FACTOR = (!isNaN(scroll_factor_float) && scroll_factor_float > 0) ? scroll_factor_float : 1.0; // I know this is ugly, but otherwise I had a scope issue with a if/else statement

    // Parse the smooth scroll to a boolean
    const SMOOTH_SCROLL = SMOOTH_SCROLL_STR === "true"; // Parse the smooth scroll to a boolean    

    // Parse the layout to a string
    const LAYOUT = ["top", "bottom"].includes(POSITION) ? "horizontal" : "vertical";
    // Set the layout to the signal
    layout.value = LAYOUT;
    // Set the position to the signal
    positon.value = POSITION;

    // Load the clipboard entries one by one and update the progress
    for (let i = 0; i < total; i++) {
      const entry = await invoke('get_clipboard_entry', { id: ids[i] });
      clipboardData.value.push(entry);
      progress.value = Math.floor(((i + 1) / total) * 100);
    }

    // Resize the window and place it when loading is complete and trigger the sliding animation
    setTimeout(async () => {
      loading.value = false;
      invoke("resize_window", {});
    }
    , 1000);

    // Set a listener to receive new clipboard items from the backend
    listen<ClipboardEntry>("new-clipboard-item", (event) => {
      const payload = event.payload;
      if (typeof payload === "object") {
        payload.content = payload.content.replace(/\\n/g, "\n");
        clipboardData.value = [payload, ...clipboardData.value];
        console.log("New clipboard item:", payload);
      }
    });

    // Set a listener to reset the scroll of the clipboard history list when the window is resized
    listen("reset-scroll", () => {
      console.log("Resetting scroll position");
      listRef.value?.scrollTo({
        top: 0,
        left: 0,
        behavior: SMOOTH_SCROLL ? 'smooth' : 'auto'
      });
    });

    // Set a listener for reloadding the window
    listen("reload-window", () => {
      window.location.reload();
    });

    // Set a listener to remove an item from the clipboard history when it is deleted
    listen("delete-item", (event) => {
      const payload = Number(event.payload);
      console.log("Deleting item:", payload);

      console.log(payload, typeof payload);
      
      const item_to_delete = clipboardData.value.find((item) => item.id === payload);
      console.log("Item to delete:", item_to_delete);
      // Print the list of ids
      console.log("Clipboard IDs:", clipboardData.value.map((item) => item.id));
      if (item_to_delete) {

        let data = [...clipboardData.value];
        data = data.filter((item) => item.id !== item_to_delete.id);
        console.log("Filtered data:", data);
        clipboardData.value = data;
        //clipboardData.value = clipboardData.value.filter((item) => item.id !== item_to_delete.id);
        console.log("Deleted item:", item_to_delete);
      }      
      // Print the list of ids
      console.log("Clipboard IDs:", clipboardData.value.map((item) => item.id));
    });

    listen("delete-all-items", () => {
      console.log("Deleting all items");
      
      // Add the shrink-out class to all items in the list
      const items = listRef.value?.querySelectorAll(".clipboard-history-item");
      items?.forEach((item) => {
        item.classList.add("shrink-out");
      });
      setTimeout(() => {
        clipboardData.value = [];
      }, 400);
    });
    
    addEventListener(
      'wheel',
      (e: WheelEvent) => {
        if (!listRef.value) return;
    
        const dx = e.deltaX;
        const dy = e.deltaY;

        let direction = 0;
        // Get the main direction
        if (layout.value === "horizontal" && dx !== 0) {
          direction = dx / Math.abs(dx);
        }

        if (dx === 0 && dy !== 0) { {
          direction = dy / Math.abs(dy);
        }
    
        // Compute the vector norm (magnitude of the scroll input)
        const norm = Math.sqrt(dx * dx + dy * dy);
        if (norm === 0) return;
    
        e.preventDefault();
    
        const from = LAYOUT === "horizontal" ? 'left' : 'top';
    
        listRef.value.scrollBy({
          [from]: direction * norm * SCROLL_FACTOR,
          behavior: SMOOTH_SCROLL ? 'smooth' : 'auto',
        });
      }}
    , { passive: false });
  });

  return (
    <>
      {loading.value ? (
        <div class={loading ? "loader" : "hidden-div"}>
        <div class="circle-progress-container">
          <svg class="progress-ring" width="120" height="120">
            <circle class="progress-ring-bg" stroke="#e6e6e6" stroke-width="10" fill="transparent" r="50" cx="60" cy="60" />
            <circle
              class="progress-ring-fill"
              stroke="#f5ca5b"
              stroke-width="10"
              fill="transparent"
              r="50"
              cx="60"
              cy="60"
              stroke-dasharray="314"
              stroke-dashoffset={314 - (progress.value / 100) * 314}
            />
          </svg>
          <span class="progress-text">{progress.value}%</span>
        </div>
          <div class="loader-text">
            <div class="loader-text-header">
              <h1>TactiClip</h1>
              <img src="../../../src-tauri/icons/128x128.png" alt="" />
            </div>
            <p>Loading clipboard history...</p>
          </div>
        </div>
      ) : (
        <div class={"clipboard-history-container clipboard-history-container-" + layout.value + " scroll-" + positon.value}>
          <ActionRow layout={layout.value}/>
          <ul class={"clipboard-history-list clipboard-history-list-"+ layout.value} ref={listRef}>
            {clipboardData.value.map((entry) => (
              <ClipboardItem orientation={layout.value} entry={entry} key={entry.id}/>
            ))}
          <li class="dummy-item">
            <div class="dummy-item"></div>
          </li>
          </ul>
          <div class={"gradient gradient-"+ layout.value}></div>
        </div>
      )}
    </>
  );
 });
