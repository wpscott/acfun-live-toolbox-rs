<script setup lang="ts">
  import { onMounted, onBeforeUnmount, ref, reactive } from "vue";

  import { invoke } from "@tauri-apps/api";
  import { listen, UnlistenFn } from "@tauri-apps/api/event";
  import { appWindow } from "@tauri-apps/api/window";

  import { User } from "./models";

  const src = ref("");
  const caption = ref("");

  let unlisten: UnlistenFn | null = null;

  const is_login = ref(false);
  const user = reactive<User>({
    userid: 0,
    username: "",
    avatar: "",
    passtoken: "",
  });

  onBeforeUnmount(() => {
    unlisten?.();
  });

  onMounted(async () => {
    document
      .getElementById("titlebar-minimize")
      ?.addEventListener("click", () => appWindow.minimize());
    document
      .getElementById("titlebar-maximize")
      ?.addEventListener("click", () => appWindow.toggleMaximize());
    document
      .getElementById("titlebar-close")
      ?.addEventListener("click", () => appWindow.close());

    is_login.value = await invoke<boolean>("is_login");
    if (!is_login.value) {
      unlisten = await listen<{ caption: string; message: string }>(
        "qr-login",
        (event) => {
          caption.value = event.payload.caption;
          if (event.payload.message) {
            src.value = event.payload.message;
          }
        }
      );
      await invoke<never>("qr_login");
    }
    const u = await invoke<User>("get_user");
    user.userid = u.userid;
    user.username = u.username;
    user.avatar = u.avatar;
    user.passtoken = u.passtoken;
  });
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div id="titlebar-minimize" class="titlebar-button">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div id="titlebar-maximize" class="titlebar-button">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div id="titlebar-close" class="titlebar-button">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>
  </div>
  <div v-if="is_login">
    <p>{{ user.username }}</p>
    <img :src="user.avatar" width="64" />
  </div>
  <div v-else>
    <img v-if="src" :src="src" />
    <p>{{ caption }}</p>
  </div>
</template>

<style>
  #app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    margin-top: 30px;
  }

  .titlebar {
    height: 30px;
    background: #66ccff;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
  }
  .titlebar-button:hover {
    background: #77ddff;
  }
</style>
