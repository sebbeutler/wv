export const quit = async () =>
    console.log("quit");

export const get_board_props = async () =>
    console.log("get_board_props");

export const gx_init = async () =>
    console.log("gx_init");

// export const draw_shader = async (shaderId: string | number) =>
//     console.log("draw_shader", { shaderId: +shaderId });

export const trace = async (prefix: string, msg: string) =>
    console.log("trace", { prefix, msg });
  
export const debug = async (prefix: string, msg: string) =>
    console.log("debug", { prefix, msg });

export const info = async (prefix: string, msg: string) =>
    console.log("info", { prefix, msg });

export const warn = async (prefix: string, msg: string) =>
    console.log("warn", { prefix, msg });

export const error = async (prefix: string, msg: string) =>
    console.log("error", { prefix, msg });

export const devtools = async () =>
    console.log("devtools");

export const show_window = async () =>
    console.log("show_window");