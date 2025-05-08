import { $, component$, useSignal, useVisibleTask$ } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import ClipboardHistory from "~/components/ClipboardHistory";

export default component$(() => {
  const progress = useSignal(0);

  // Set a listener for the progress update event
  useVisibleTask$(() => {
    listen("progress-update", (event) => {
      const { payload } = event;
      if (typeof payload === "number") {
        console.log("Progress update:", payload);
        if (payload === 200) {
          progress.value = 100;
          console.log("Loading complete");
          

          setTimeout(() => {
            invoke("resize_window", {});
          }
          , 500);
        }
        else {
          progress.value = payload;
        }
      }
    }
    );
  });
  

  return (
      <>
        <ClipboardHistory />
      </>
  );
});

export const head: DocumentHead = {
  title: "TactiClip",
  meta: [
    {
      name: "description",
      content: "Lightweight but powerful clipboard manager",
    },
  ],
};
