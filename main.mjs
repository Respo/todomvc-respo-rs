import init, { loadApp } from "./pkg/todomvc_respo_rs/";

window.onload = () => {
  init().then(() => {
    loadApp(".app");
  });
};
