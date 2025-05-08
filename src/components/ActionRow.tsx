import { component$ } from "@builder.io/qwik";
import { IconHover } from "./Icon";
import { emit } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export interface ActionRowProps {
    layout: string;
}

export const ActionRow = component$<ActionRowProps>(({ layout }) => {
    return (
        <div class={`action-row action-row-${layout}`}>
            <button 
                class="action-row-button" 
                title="Unpin all items"
                onClick$={async (e) => {
                    e.stopPropagation();

                    await invoke("delete_all", {});
                }}
            >
                <IconHover regular="trash-can" solid="trash-can" class="trash-button" />
            </button>
            <button class="action-row-button" onClick$={
                async (e) => {
                    e.stopPropagation();

                    const res = await invoke("unpin_all", {});
                    if (res) {
                        emit("unpin-all");
                    }
                }}>
                <IconHover regular="star" solid="star" class="trash-button" />
            </button>
            <button class="action-row-button" onClick$={async () => {
                invoke("open_settings", {});
            }}>
                <IconHover regular="sun" solid="gear" class="preferences-button" />
            </button>
        </div>
    );
});

