# GlobusOS Media Licensing Policy

## Objective

The mandatory GlobusOS media stack must remain usable without mandatory paid codec licences, proprietary SDK subscriptions, or proprietary runtime dependencies.

## Core policy

The default media core may ship only implementations whose software and patent terms have been reviewed for royalty-free/open deployment.

### Video core

- AV1
- VP9

### Image core

- JPEG
- PNG
- WebP
- AVIF

### Explicit exclusions from the mandatory core

- H.264 / AVC
- H.265 / HEVC
- proprietary codec SDKs
- paid decoder/encoder services
- DRM or proprietary media runtimes

An optional backend may exist only when it is clearly separated from the core and does not become a dependency of the default build.

## Important distinction

A codec being open or royalty-free does not make every media file free of copyright or other rights. GlobusOS therefore does not grant rights to user content. The policy concerns the software/codec technology used by the operating system.

Likewise, a free codec does not automatically make every third-party implementation free. Each concrete decoder/encoder dependency must have its own licence and patent review recorded before inclusion.

## AV1 / AVIF

AOMedia states that AV1 is developed under a royalty-free patent policy. AOMedia Patent License 1.0 grants a no-charge, royalty-free patent licence for covered implementations subject to its conditions. AOMedia also states that its software is released under BSD 3-Clause Clear together with the AOMedia patent licensing terms.

## Build rule

The default `globus-media` crate must not require H.264/HEVC libraries or proprietary SDKs. Hardware acceleration is an optimization/backend and must not change the licensing status of the mandatory media core.

## Evidence requirement

Before adding a media dependency, record:

1. package and exact version;
2. source repository;
3. software licence;
4. patent/codec terms where applicable;
5. transitive native dependencies;
6. whether the dependency is mandatory or optional;
7. whether redistribution is permitted without a paid licence;
8. the corresponding NOTICE/COPYING text required for redistribution.

No evidence, no dependency.
