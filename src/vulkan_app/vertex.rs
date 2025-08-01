use ash::vk;
use std::mem::offset_of;

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn get_binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, pos) as u32)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, color) as u32)
                .build(),
        ]
    }
}

pub const VERTICES: [Vertex; 8] = [
    Vertex {
        pos: [-0.5, -0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, -0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, 0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [-0.5, 0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [-0.5, -0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, -0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, 0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [-0.5, 0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
];

pub const INDICES: [u16; 36] = [
    0, 1, 2, 2, 3, 0, // front
    4, 6, 5, 4, 7, 6, // back
    0, 7, 4, 0, 3, 7, // left
    1, 5, 6, 6, 2, 1, // right
    3, 2, 6, 6, 7, 3, // top
    0, 5, 1, 5, 0, 4, // bottom
];

pub fn generate_wireframe_vertices(divisions: u32) -> Vec<Vertex> {
    let color = [0.0, 0.0, 0.0];
    let mut vertices = Vec::new();
    let step = 1.0 / divisions as f32;

    // grid lines on the faces only
    for i in 1..divisions {
        let pos = -0.5 + i as f32 * step;
        // XY planes (z = ±0.5)
        vertices.push(Vertex { pos: [-0.5, pos, -0.5], color });
        vertices.push(Vertex { pos: [0.5, pos, -0.5], color });
        vertices.push(Vertex { pos: [-0.5, pos, 0.5], color });
        vertices.push(Vertex { pos: [0.5, pos, 0.5], color });

        vertices.push(Vertex { pos: [pos, -0.5, -0.5], color });
        vertices.push(Vertex { pos: [pos, 0.5, -0.5], color });
        vertices.push(Vertex { pos: [pos, -0.5, 0.5], color });
        vertices.push(Vertex { pos: [pos, 0.5, 0.5], color });

        // XZ planes (y = ±0.5)
        vertices.push(Vertex { pos: [-0.5, -0.5, pos], color });
        vertices.push(Vertex { pos: [0.5, -0.5, pos], color });
        vertices.push(Vertex { pos: [-0.5, 0.5, pos], color });
        vertices.push(Vertex { pos: [0.5, 0.5, pos], color });

        vertices.push(Vertex { pos: [pos, -0.5, -0.5], color });
        vertices.push(Vertex { pos: [pos, -0.5, 0.5], color });
        vertices.push(Vertex { pos: [pos, 0.5, -0.5], color });
        vertices.push(Vertex { pos: [pos, 0.5, 0.5], color });

        // YZ planes (x = ±0.5)
        vertices.push(Vertex { pos: [-0.5, -0.5, pos], color });
        vertices.push(Vertex { pos: [-0.5, 0.5, pos], color });
        vertices.push(Vertex { pos: [0.5, -0.5, pos], color });
        vertices.push(Vertex { pos: [0.5, 0.5, pos], color });

        vertices.push(Vertex { pos: [-0.5, pos, -0.5], color });
        vertices.push(Vertex { pos: [-0.5, pos, 0.5], color });
        vertices.push(Vertex { pos: [0.5, pos, -0.5], color });
        vertices.push(Vertex { pos: [0.5, pos, 0.5], color });
    }

    // cube edges
    let edges = [
        ([-0.5, -0.5, -0.5], [0.5, -0.5, -0.5]),
        ([-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]),
        ([-0.5, -0.5, 0.5], [0.5, -0.5, 0.5]),
        ([-0.5, 0.5, 0.5], [0.5, 0.5, 0.5]),
        ([-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5]),
        ([0.5, -0.5, -0.5], [0.5, 0.5, -0.5]),
        ([-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5]),
        ([0.5, -0.5, 0.5], [0.5, 0.5, 0.5]),
        ([-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5]),
        ([0.5, -0.5, -0.5], [0.5, -0.5, 0.5]),
        ([-0.5, 0.5, -0.5], [-0.5, 0.5, 0.5]),
        ([0.5, 0.5, -0.5], [0.5, 0.5, 0.5]),
    ];

    for &(start, end) in &edges {
        vertices.push(Vertex { pos: start, color });
        vertices.push(Vertex { pos: end, color });
    }

    vertices
}

pub fn generate_flat_world(width: u32, depth: u32) -> (Vec<Vertex>, Vec<u16>, Vec<Vertex>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let base_wire = generate_wireframe_vertices(24);
    let mut wire_vertices = Vec::new();

    for x in 0..width {
        for y in 0..depth {
            let offset = [
                x as f32 - width as f32 / 2.0 + 0.0,
                y as f32 - depth as f32 / 2.0 + 0.0,
                0.5,
            ];
            let base_index = vertices.len() as u16;
            for v in VERTICES.iter() {
                vertices.push(Vertex {
                    pos: [
                        v.pos[0] + offset[0],
                        v.pos[1] + offset[1],
                        v.pos[2] + offset[2],
                    ],
                    color: v.color,
                });
            }
            indices.extend(INDICES.iter().map(|&i| i + base_index));
            for wv in base_wire.iter() {
                wire_vertices.push(Vertex {
                    pos: [
                        wv.pos[0] + offset[0],
                        wv.pos[1] + offset[1],
                        wv.pos[2] + offset[2],
                    ],
                    color: wv.color,
                });
            }
        }
    }

    (vertices, indices, wire_vertices)
}
