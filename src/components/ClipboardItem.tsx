import { component$, useSignal, useVisibleTask$ } from "@builder.io/qwik";
import { invoke } from "@tauri-apps/api/core";

import { IconHover } from "./Icon";
import { listen } from "@tauri-apps/api/event";

import hljs from 'highlight.js';
import "highlight.js/styles/github-dark.min.css";

export interface ClipboardItemProps {
  orientation: string;
  entry: {
    id: number;
    entry_type: string;
    content: string;
    added_at: string;
    pinned: boolean;
    forced_language: string | null;
  };
}

function stripAlphaChannel(color: string): string {
    // Match rgba or rgb
    const rgbaMatch = color.match(/rgba?\((\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})(?:,\s*[0-9.]+)?\)/i);
    if (rgbaMatch) {
      const r = parseInt(rgbaMatch[1]).toString(16).padStart(2, '0');
      const g = parseInt(rgbaMatch[2]).toString(16).padStart(2, '0');
      const b = parseInt(rgbaMatch[3]).toString(16).padStart(2, '0');
      return `#${r}${g}${b}`;
    }
  
    // Match hsla or hsl 
    const hslaMatch = color.match(/hsla?\((\d+(?:\.\d+)?),\s*(\d+)%?,\s*(\d+)%?(?:,\s*[0-9.]+)?\)/i);
    if (hslaMatch) {
      const h = parseFloat(hslaMatch[1]);
      const s = parseFloat(hslaMatch[2]) / 100;
      const l = parseFloat(hslaMatch[3]) / 100;
  
      const a = s * Math.min(l, 1 - l);
      const f = (n: number) => {
        const k = (n + h / 30) % 12;
        const c = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
        return Math.round(255 * c).toString(16).padStart(2, '0');
      };
  
      return `#${f(0)}${f(8)}${f(4)}`;
    }
  
    // If no match, return the original color
    return color;
}

export const ClipboardItem = component$<ClipboardItemProps>(({ orientation, entry }) => {
  const overlayClass = useSignal("");
  const itemRef = useSignal<HTMLElement>();
  const language = useSignal("Raw text");


  // Listen for the unpin-all event and unpin all items
  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(() => {

    // Auto-highlight the text if the entry type is text
    if (entry.entry_type === "text") {

      // if the forced_language is not null, set the language to the forced_language
      if (entry.forced_language) {
        language.value = entry.forced_language;
        } else {

        const hl = hljs.highlightAuto(entry.content);

        // if the relevance is greater than 10, set the isCode to true to use
        // Syntax highlighting, otherwise set it to false as this is a normal text or not enough code to highlight
        if (hl.relevance > 5) {
          language.value = hl.language? hl.language : "raw text";
        }
      }
    }

    listen("unpin-all", async () => {
      if (itemRef.value?.classList.contains("clipboard-history-item-pinned")) {

        itemRef.value.classList.remove("expand-in");
        itemRef.value?.classList.add("shrink-out");
          setTimeout(() => {
            itemRef.value?.classList.remove("clipboard-history-item-pinned");
            itemRef.value?.classList.remove("shrink-out");
            itemRef.value?.classList.add("expand-in");
          }, 350);
      }
    });
  });

  return (
    <li
      key={entry.id}
      class={
        "clipboard-history-item clipboard-history-item-" +
        orientation +
        " " +
        (entry.pinned ? "clipboard-history-item-pinned" : "") /* +
        (isDeleting.value ? " shrink-out" : "") */
      }
      onClick$={(e, target) => {
        invoke("push_to_clipboard", { id: entry.id });

        const overlay = target?.querySelector(".click-overlay") as HTMLElement;
        const circle = overlay?.querySelector(".checkmark-circle");
        const check = overlay?.querySelector(".checkmark-check");

        overlay?.classList.remove("show");
        circle?.classList.remove("animate");
        check?.classList.remove("animate");

        void overlay?.offsetWidth;

        overlay?.classList.add("show");
        circle?.classList.add("animate");
        check?.classList.add("animate");

        overlayClass.value = "hide";
        setTimeout(() => {
          overlay?.classList.remove("show");
          circle?.classList.remove("animate");
          check?.classList.remove("animate");
        }, 700);
      }}
      onMouseEnter$={() => {
        overlayClass.value = "show";
      }}
      onMouseLeave$={() => {
        overlayClass.value = "hide";
        setTimeout(() => {
          overlayClass.value = "";
        }, 300);
      }}
    ref={itemRef}>
      {entry.entry_type === "image" ? (
        <img
          src={`data:image/png;base64,${entry.content}`}
          alt="Clipboard"
          class="clipboard-image"
        />
      ) : entry.entry_type === "color" ? (
        <div class="clipboard-item-color" style={{ backgroundColor: stripAlphaChannel(entry.content) }}>
          <div class="clipboard-item-color-text">{entry.content}</div>
        </div>
      ) : (
        <div class="clipboard-history-item-content">
          {language.value === "Raw text" ? (
            entry.content.split(/\r?\n/).map((line, i) => (
              <p key={i} class="clipboard-item-content-line">{line}</p>
            ))
          ) : entry.entry_type === "text" ? (
            <pre class="hljs" dangerouslySetInnerHTML={hljs.highlight(entry.content, { language: language.value }).value}></pre>
          ) : 
            <p class="email-or-link">{entry.content}</p>
          }          
        </div>
      )}

      {/* Hover overlay */}
      <div class={`hover-overlay ${overlayClass.value}`}>
      <div class="row-wrapper upper-wrapper">
        <div class="upper-row">
          <button
            onClick$={async (e) => {
              e.stopPropagation();
              console.log("del clicked", entry.id);
              console.log("Content", entry.content);
              itemRef.value?.classList.remove("expand-in");
              itemRef.value?.classList.add("shrink-out");
              setTimeout(() => {
                invoke("delete_item", { id: entry.id });
              }, 350);
            }}
            class="overlay-button"
          >
            <IconHover regular="trash-can" solid="trash-can" class="trash-button" />
          </button>
          
          { entry.entry_type === "text" && (
            <select
              class="overlay-button language-select"
              bind:value={language}
              style={{ marginLeft: '8px' }}
              onClick$={(e) => e.stopPropagation()}
              onChange$={async (e) => {
                const selectedLanguage = (e.target as HTMLSelectElement).value;
                invoke("force_language", { id: entry.id, language: selectedLanguage });
              }}
            >
              <option value="Raw text">Raw text</option>
              {hljs.listLanguages().map((lang) => (
                <option 
                key={lang} 
                value={lang} 
                selected={lang === language.value}
                >
                  {lang.charAt(0).toUpperCase() + lang.slice(1)}
                </option>
              ))}
            </select>
          )}
        </div>
      </div>
        <div class="row-wrapper lower-wrapper">
          <div class="lower-row">
            <div class="date">{entry.added_at}</div>
              <div class="lower-row-buttons">
                { entry.entry_type === "url" &&
                  <button
                    onClick$={async (e) => {
                      e.stopPropagation();
                      await invoke("open_url", { url: entry.content });
                    }}
                    class="overlay-button url-button"
                  >
                    <IconHover regular="share-from-square" solid="share-from-square" class="url-button" />
                  </button>
                }
                { entry.entry_type === "email" &&
                  <button
                    onClick$={async (e) => {
                      e.stopPropagation();
                      await invoke("open_url", { url: "mailto://"  + entry.content });
                    }}
                    class="overlay-button email-button"
                  >
                    <IconHover regular="envelope" solid="envelope" class="email-button" />
                  </button>
                }
                <button
                  onClick$={async (e) => {
                    e.stopPropagation();
                    const res = await invoke("toggle_pin", { id: entry.id, state: !entry.pinned });
                    itemRef.value?.classList.remove("expand-in");
                    itemRef.value?.classList.add("shrink-out");
                    if (res) {
                      setTimeout(() => {
                        itemRef.value?.classList.toggle("clipboard-history-item-pinned");
                        itemRef.value?.classList.remove("shrink-out");
                        itemRef.value?.classList.add("expand-in");
                      }, 350);
                    } else {
                      alert("Error pinning item");
                      itemRef.value?.classList.remove("shrink-out");
                      itemRef.value?.classList.add("expand-in");
                    }
                  }}
                  class="overlay-button star-button"
                >
                  <IconHover regular="star" solid="star" class={"star-button" + (entry.pinned ? " pinned" : "")} />
                </button>
              </div>
          </div>
        </div>
      </div>

      {/* Click overlay */}
      <div class="click-overlay">
        <svg class="checkmark" viewBox="0 0 52 52">
          <circle class="checkmark-circle" cx="26" cy="26" r="25" fill="none" />
          <path class="checkmark-check" fill="none" d="M14 27l7 7 16-16" />
        </svg>
      </div>
    </li>
  );
});
