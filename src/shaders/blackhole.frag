#version 450

layout(location = 0) out vec4 outColor;

layout(push_constant) uniform PushConstants {
    vec2 resolution;
    float time;
    float padding;
} pc;

const float BLACK_HOLE_RADIUS = 1.85;
const float DISK_INNER_RADIUS = 2.8;
const float DISK_OUTER_RADIUS = 13.0;

const int MAX_STEPS = 128;
const float STEP_SIZE = 0.22;
const float DISK_HALF_THICKNESS = 0.12;

float hash21(vec2 p) {
    p = fract(p * vec2(123.34, 456.21));
    p += dot(p, p + 45.32);

    return fract(p.x * p.y);
}

float noise2d(vec2 p) {
    vec2 cell = floor(p);
    vec2 local = fract(p);

    local = local * local * (3.0 - 2.0 * local);

    float a = hash21(cell);
    float b = hash21(cell + vec2(1.0, 0.0));
    float c = hash21(cell + vec2(0.0, 1.0));
    float d = hash21(cell + vec2(1.0, 1.0));

    return mix(
            mix(a, b, local.x),
            mix(c, d, local.x),
            local.y
    );
}

float fbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.5;

    for (int i = 0; i < 4; i++) {
        value += noise2d(p) * amplitude;
        p = p * 2.03 + vec2(17.1, 9.2);
        amplitude *= 0.5;
    }

    return value;
}

vec3 starfield(vec3 direction) {
    vec2 sky = direction.xy / max(abs(direction.z), 0.2);
    vec2 grid = floor(sky * 95.0);
    vec2 local = fract(sky * 95.0) - 0.5;

    float seed = hash21(grid);

    if (seed < 0.990) {
        return vec3(0.0);
    }

    float starSize = exp(-dot(local, local) * 600.0);
    float starTemperature = hash21(grid + vec2(8.1, 3.7));

    vec3 starColor = mix(
            vec3(0.55, 0.72, 1.0),
            vec3(1.0, 0.78, 0.52),
            starTemperature
    );

    return starColor * starSize * 1.8;
}

vec3 diskEmission(vec3 position, vec3 rayDirection) {
    float radius = length(position.xz);

    if (radius < DISK_INNER_RADIUS || radius > DISK_OUTER_RADIUS) {
        return vec3(0.0);
    }

    float diskThickness = exp(
            -pow(position.y / DISK_HALF_THICKNESS, 2.0)
    );

    float radialFade = smoothstep(
            DISK_OUTER_RADIUS,
            DISK_OUTER_RADIUS - 1.4,
            radius
    );

    float innerHeat = exp(
            -(radius - DISK_INNER_RADIUS) * 0.24
    );

    float angle = atan(position.z, position.x);

    vec2 flowUv = vec2(
            radius * 2.5 + angle * 4.0 - pc.time * 0.22,
            angle * 9.0 + radius * 0.45 + pc.time * 0.08
    );

    float turbulence = fbm(flowUv);
    float fineTurbulence = noise2d(flowUv * 3.5);

    vec3 tangent = normalize(vec3(
            -position.z,
            0.0,
            position.x
    ));

    // Упрощённый эффект Доплера.
    float doppler = clamp(
            1.0 + dot(tangent, -rayDirection) * 0.75,
            0.30,
            1.90
    );

    float density = diskThickness
    * radialFade
    * (0.45 + turbulence * 1.15)
    * (0.75 + fineTurbulence * 0.35);

    vec3 darkOrange = vec3(0.20, 0.012, 0.002);
    vec3 orange = vec3(1.0, 0.20, 0.025);
    vec3 whiteHot = vec3(1.0, 0.92, 0.70);

    vec3 color = mix(
            darkOrange,
            orange,
            innerHeat
    );

    color = mix(
            color,
            whiteHot,
            innerHeat * innerHeat * 0.85
    );

    return color
    * density
    * doppler
    * (0.75 + innerHeat * 2.2);
}

vec3 renderScene(vec3 cameraPosition, vec3 rayDirection) {
    vec3 position = cameraPosition;
    vec3 direction = rayDirection;

    vec3 color = starfield(rayDirection);
    float transmission = 1.0;

    for (int step = 0; step < MAX_STEPS; step++) {
        float distanceToCenter = length(position);

        if (distanceToCenter < BLACK_HOLE_RADIUS) {
            return vec3(0.0);
        }

        float adaptiveStep = clamp(
                distanceToCenter * 0.055,
                0.035,
                STEP_SIZE
        );

        position += direction * adaptiveStep;
        distanceToCenter = length(position);

        if (distanceToCenter > 34.0) {
            break;
        }

        // Настоящий 3D-диск в плоскости XZ.
        if (abs(position.y) < DISK_HALF_THICKNESS * 2.0) {
            vec3 emission = diskEmission(position, direction);

            color += emission
            * transmission
            * adaptiveStep
            * 1.45;

            transmission *= exp(
                    -length(emission) * adaptiveStep * 0.08
            );
        }

        // Дешёвая аппроксимация гравитационного отклонения луча.
        vec3 radialDirection = -normalize(position);

        vec3 bendDirection = radialDirection
        - direction * dot(radialDirection, direction);

        float bendLength = length(bendDirection);

        if (bendLength > 0.0001) {
            bendDirection /= bendLength;
        }

        float gravitationalBend = 0.018 / max(distanceToCenter * distanceToCenter, 1.0);

        direction = normalize(
                direction + bendDirection * gravitationalBend
        );
    }

    return color;
}

void main() {
    vec2 uv = gl_FragCoord.xy / pc.resolution;
    uv = uv * 2.0 - 1.0;
    uv.x *= pc.resolution.x / pc.resolution.y;

    float orbitAngle = pc.time * 0.11;

    // Орбитальная камера вокруг чёрной дыры.
    vec3 cameraPosition = vec3(
            cos(orbitAngle) * 18.0,
            -4.5 + sin(orbitAngle * 0.65) * 0.35, //default 4.5, 0.65, 0.35
            sin(orbitAngle) * 18.0
    );

    vec3 cameraTarget = vec3(
            0.0,
            2.8,
            0.0
    );

    vec3 cameraForward = normalize(
            cameraTarget - cameraPosition
    );
    vec3 worldUp = vec3(0.0, 1.0, 0.0);

    vec3 cameraRight = normalize(
            cross(cameraForward, worldUp)
    );

    vec3 cameraUp = normalize(
            cross(cameraRight, cameraForward)
    );

    vec3 rayDirection = normalize(
            cameraForward
            + cameraRight * uv.x * 0.62
            + cameraUp * uv.y * 0.62
    );

    vec3 color = renderScene(
            cameraPosition,
            rayDirection
    );

    // Лёгкое кинематографичное свечение.
    color += color * color * 0.18;

    // Tone mapping.
    color = color / (vec3(1.0) + color);
    color = pow(color, vec3(0.88));

    outColor = vec4(color, 1.0);
}