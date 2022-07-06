import ElementPlus from "element-plus";
import { createPinia } from "pinia";
import App from "@front/App.vue";
import { createApp } from "vue";

const pinia = createPinia();

createApp(App).use(ElementPlus).use(pinia).mount("#app");
