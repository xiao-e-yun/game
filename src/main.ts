import init from "wasm";
import { createApp } from "vue";
import App from "./App.vue";
import "./style.scss";

await init();
createApp(App).mount("#app");
