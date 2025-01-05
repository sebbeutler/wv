import { invoke } from "@tauri-apps/api/core";

export const quit = async () =>
    invoke<void>("quit");

export const get_board_props = async () =>
    invoke<void>("get_board_props");

export const gx_init = async () =>
    invoke<void>("gx_init");

// export const draw_shader = async (shaderId: string | number) =>
//     invoke<void>("draw_shader", { shaderId: +shaderId });

export const trace = async (prefix: string, msg: string) =>
    invoke<void>("trace", { prefix, msg });
  
export const debug = async (prefix: string, msg: string) =>
    invoke<void>("debug", { prefix, msg });

export const info = async (prefix: string, msg: string) =>
    invoke<void>("info", { prefix, msg });

export const warn = async (prefix: string, msg: string) =>
    invoke<void>("warn", { prefix, msg });

export const error = async (prefix: string, msg: string) =>
    invoke<void>("error", { prefix, msg });

export const devtools = async () =>
    invoke("devtools");

export const show_window = async () =>
    invoke("show_window");