/* Copyright 2023 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

pub const VERTEX_SHADER_SRC: &str = r#"
    #version 120

    attribute vec3 a_position;
    attribute vec3 a_normal;

    varying vec3 v_position;
    varying vec3 v_normal;

    uniform mat4 u_model_view_matrix;
    uniform mat4 u_perspective_matrix;

    void main() {
        v_position = vec3(u_model_view_matrix * vec4(a_position, 1.0));
        v_normal = mat3(u_model_view_matrix) * a_normal;
        gl_Position = u_perspective_matrix * u_model_view_matrix
                       * vec4(a_position, 1.0);
    }
    "#;

pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 120

    const float EPSILON = 0.00390625;
    const float TAU = 6.283185482025146484375;
    const float PI = 3.1415927410125732421875;
    const float SQRT_PI = 1.77245390415191650390625;

    struct AmbientLight {
        vec3 color;
    };
    struct DirectionalLight {
        vec3 color;
        vec3 direction;
    };
    struct PointLight {
        vec3 color;
        vec3 position;
    };
    struct Lights {
        AmbientLight ambient;
        DirectionalLight directional;
        PointLight point;
    };

    varying vec3 v_position;
    varying vec3 v_normal;

    uniform Lights u_lights;

    uniform vec3 u_diffuse;
    uniform vec3 u_specular;
    uniform float u_roughness;

    vec3 f_schlick(vec3 f0, float n_dot_l) {
        return f0 + (1.0 - f0) * pow(1.0 - n_dot_l, 5.0);
    }

    float d_ggx(float n_dot_h, float a2) {
        float denom = n_dot_h * n_dot_h * (a2 - 1.0) + 1.0;
        return a2 / (denom * denom * PI);
    }

    float g_smith_schlick_beckmann(float n_dot_l, float n_dot_v, float a2) {
        float b = a2 * SQRT_PI;
        float c = 1.0 - b;
        return n_dot_l * n_dot_v / ((n_dot_l * c + b) * (n_dot_v * c + b));
    }

    void main() {
        vec3 n = normalize(v_normal);
        vec3 v = normalize(-v_position);
        float n_dot_v = max(dot(n, v), 0.0);

        vec3 color = vec3(0.0);
        //vec3 color = u_lights.ambient.color
        //              * (u_diffuse * (1.0 - u_specular) + u_specular);

        vec3 l_pos = u_lights.point.position - v_position;
        float l_dist2 = l_pos.x*l_pos.x + l_pos.y*l_pos.y + l_pos.z*l_pos.z;

        vec3 l = normalize(l_pos);
        vec3 h = normalize(l + v);
        float n_dot_h = max(dot(n, h), 0.0);
        float n_dot_l = max(dot(n, l), 0.0);

        vec3 f = f_schlick(u_specular, n_dot_l);
        float d = d_ggx(n_dot_h, u_roughness);
        float g = g_smith_schlick_beckmann(n_dot_l, n_dot_v, u_roughness);

        vec3 diffuse = u_diffuse / PI * (1.0 - f);
        vec3 specular = f * d * g / (4 * n_dot_l * n_dot_v + EPSILON);

        vec3 ambient = 0.1 * (u_diffuse * (1.0 - u_specular) + u_specular);
        color += u_lights.point.color / (1 + l_dist2)
                  * (ambient + (diffuse + specular) * n_dot_l);
        //color += u_lights.point.color / (1 + l_dist2)
        //          * (diffuse + specular) * n_dot_l;

        gl_FragColor = vec4(color, 1.0);
    }
    "#;
