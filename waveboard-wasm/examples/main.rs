struct ExampleDesc {
    name: &'static str,
    function: fn(),
    #[cfg_attr(not(target_arch = "wasm32"), expect(dead_code))]
    webgl: bool,
    #[cfg_attr(not(target_arch = "wasm32"), expect(dead_code))]
    webgpu: bool,
}

const EXAMPLES: &[ExampleDesc] = &[
    ExampleDesc {
        name: "boids",
        function: waveboard_wasm::boids::main,
        webgl: false, // No compute
        webgpu: true,
    },
    ExampleDesc {
        name: "bunnymark",
        function: waveboard_wasm::bunnymark::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "conservative_raster",
        function: waveboard_wasm::conservative_raster::main,
        webgl: false,  // No conservative raster
        webgpu: false, // No conservative raster
    },
    ExampleDesc {
        name: "cube",
        function: waveboard_wasm::cube::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "hello",
        function: waveboard_wasm::hello::main,
        webgl: false, // No canvas for WebGL
        webgpu: true,
    },
    ExampleDesc {
        name: "hello_compute",
        function: waveboard_wasm::hello_compute::main,
        webgl: false, // No compute
        webgpu: true,
    },
    ExampleDesc {
        name: "hello_synchronization",
        function: waveboard_wasm::hello_synchronization::main,
        webgl: false, // No canvas for WebGL
        webgpu: true,
    },
    ExampleDesc {
        name: "hello_triangle",
        function: waveboard_wasm::hello_triangle::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "hello_windows",
        function: waveboard_wasm::hello_windows::main,
        webgl: false,  // Native only example
        webgpu: false, // Native only example
    },
    ExampleDesc {
        name: "hello_workgroups",
        function: waveboard_wasm::hello_workgroups::main,
        webgl: false,
        webgpu: true,
    },
    ExampleDesc {
        name: "mipmap",
        function: waveboard_wasm::mipmap::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "msaa_line",
        function: waveboard_wasm::msaa_line::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "multiple_render_targets",
        function: waveboard_wasm::multiple_render_targets::main,
        webgl: false,
        webgpu: true,
    },
    ExampleDesc {
        name: "render_to_texture",
        function: waveboard_wasm::render_to_texture::main,
        webgl: false, // No canvas for WebGL
        webgpu: true,
    },
    ExampleDesc {
        name: "repeated_compute",
        function: waveboard_wasm::repeated_compute::main,
        webgl: false, // No compute
        webgpu: true,
    },
    ExampleDesc {
        name: "shadow",
        function: waveboard_wasm::shadow::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "skybox",
        function: waveboard_wasm::skybox::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "srgb_blend",
        function: waveboard_wasm::srgb_blend::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "stencil_triangles",
        function: waveboard_wasm::stencil_triangles::main,
        webgl: true,
        webgpu: true,
    },
    ExampleDesc {
        name: "storage_texture",
        function: waveboard_wasm::storage_texture::main,
        webgl: false, // No storage textures
        webgpu: true,
    },
    ExampleDesc {
        name: "texture_arrays",
        function: waveboard_wasm::texture_arrays::main,
        webgl: false,  // No texture arrays
        webgpu: false, // No texture arrays
    },
    ExampleDesc {
        name: "timestamp_queries",
        function: waveboard_wasm::timestamp_queries::main,
        webgl: false,  // No canvas for WebGL
        webgpu: false, // No timestamp queries
    },
    ExampleDesc {
        name: "uniform_values",
        function: waveboard_wasm::uniform_values::main,
        webgl: false, // No compute
        webgpu: true,
    },
    ExampleDesc {
        name: "water",
        function: waveboard_wasm::water::main,
        webgl: false, // No RODS
        webgpu: true,
    },
    ExampleDesc {
        name: "ray_cube_compute",
        function: waveboard_wasm::ray_cube_compute::main,
        webgl: false,  // No Ray-tracing extensions
        webgpu: false, // No Ray-tracing extensions (yet)
    },
    ExampleDesc {
        name: "ray_cube_fragment",
        function: waveboard_wasm::ray_cube_fragment::main,
        webgl: false,  // No Ray-tracing extensions
        webgpu: false, // No Ray-tracing extensions (yet)
    },
    ExampleDesc {
        name: "ray_scene",
        function: waveboard_wasm::ray_scene::main,
        webgl: false,  // No Ray-tracing extensions
        webgpu: false, // No Ray-tracing extensions (yet)
    },
    ExampleDesc {
        name: "ray_shadows",
        function: waveboard_wasm::ray_shadows::main,
        webgl: false,  // No Ray-tracing extensions
        webgpu: false, // No Ray-tracing extensions (yet)
    },
    ExampleDesc {
        name: "ray_traced_triangle",
        function: waveboard_wasm::ray_traced_triangle::main,
        webgl: false,
        webgpu: false,
    },
];

fn get_example_name() -> Option<String> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let query_string = web_sys::window()?.location().search().ok()?;

            waveboard_wasm::framework::parse_url_query_string(&query_string, "example").map(String::from)
        } else {
            std::env::args().nth(1)
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn print_examples() {
    // Get the document, header, and body elements.
    let document = web_sys::window().unwrap().document().unwrap();

    for backend in ["webgl2", "webgpu"] {
        let ul = document
            .get_element_by_id(&format!("{backend}-list"))
            .unwrap();

        for example in EXAMPLES {
            if backend == "webgl2" && !example.webgl {
                continue;
            }
            if backend == "webgpu" && !example.webgpu {
                continue;
            }

            let link = document.create_element("a").unwrap();
            link.set_text_content(Some(example.name));
            link.set_attribute(
                "href",
                &format!("?backend={backend}&example={}", example.name),
            )
            .unwrap();
            link.set_class_name("example-link");

            let item = document.create_element("div").unwrap();
            item.append_child(&link).unwrap();
            item.set_class_name("example-item");
            ul.append_child(&item).unwrap();
        }
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn waveboard() {
//     waveboard_wasm::waveboard::main();
// }

#[cfg(not(target_arch = "wasm32"))]
fn print_unknown_example(result: Option<String>) {
    if let Some(example) = result {
        println!("Unknown example: {example}");
    } else {
        println!("Please specify an example as the first argument!");
    }

    println!("\nAvailable Examples:");
    for examples in EXAMPLES {
        println!("\t{}", examples.name);
    }
}

fn main() {
    waveboard_wasm::bunnymark::main();
}
