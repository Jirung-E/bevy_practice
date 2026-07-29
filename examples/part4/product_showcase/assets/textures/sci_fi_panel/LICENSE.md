# Sci-Fi Panel PBR Texture Set

Files:

- `base_color.png`, `base_color_clamp.png`
- `normal.png`, `normal_clamp.png`
- `emissive.png`, `emissive_clamp.png`

The three `*_clamp.png` files are byte-for-byte duplicates used to load the same image content with a different Bevy sampler configuration.

All images are square PNG files at 1254×1254 pixels. The three map types share the same dimensions and feature alignment.

## Source

Created specifically for this Bevy practice book with OpenAI image generation on 2026-07-29. No third-party image, logo, character, or trademark was used as a source asset.

Generation prompts:

1. Base Color: square seamless stylized sci-fi navy metal panel, asymmetric cyan L-shaped stripe, two orange diagonal inset strips, panel seams and bolts; flat albedo without baked lighting, text, logo, or watermark.
2. Normal: edit the Base Color while preserving the exact layout; convert panel seams, bolts, borders, and strips into a tangent-space OpenGL normal map on a neutral blue background.
3. Emissive: edit the Base Color while preserving the exact layout; make all ordinary metal black and retain only the cyan L-shaped stripe and two orange strips as emissive colors.

## License

These texture files are included under the same dual license as the example source:

- MIT License
- Apache License, Version 2.0

You may copy, modify, and redistribute them under either license.
