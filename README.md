<div align="center">

# cogl-rs

[![API Docs][docrs-badge]][docrs-url]
[![Crates.io][crates-badge]][crates-url]
[![Code coverage][codecov-badge]][codecov-url]
[![Tests][tests-badge]][tests-url]
[![MPL-2.0 licensed][license-badge]][license-url]
[![Gitter chat][gitter-badge]][gitter-url]
[![loc][loc-badge]][loc-url]
</div>

[docrs-badge]: https://img.shields.io/docsrs/cogl-rs?style=flat-square
[docrs-url]: https://docs.rs/cogl-rs/
[crates-badge]: https://img.shields.io/crates/v/cogl-rs.svg?style=flat-square
[crates-url]: https://crates.io/crates/cogl-rs
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square
[license-url]: https://github.com/angular-rust/cogl-rs/blob/master/LICENSE
[gitter-badge]: https://img.shields.io/gitter/room/angular_rust/community.svg?style=flat-square
[gitter-url]: https://gitter.im/angular_rust/community
[tests-badge]: https://img.shields.io/github/workflow/status/angular-rust/cogl-rs/Tests?label=tests&logo=github&style=flat-square
[tests-url]: https://github.com/angular-rust/cogl-rs/actions/workflows/tests.yml
[codecov-badge]: https://img.shields.io/codecov/c/github/angular-rust/cogl-rs?logo=codecov&style=flat-square&token=d0AlgG7AfE
[codecov-url]: https://codecov.io/gh/angular-rust/cogl-rs
[loc-badge]: https://img.shields.io/tokei/lines/github/angular-rust/cogl-rs?style=flat-square
[loc-url]: https://github.com/angular-rust/cogl-rs

**cogl-rs** is Rust bindings for the Cogl library. cogl-rs is being developed in the Angular Rust infrastructure since Angular Rust was depends on it.

**COGL** is a modern 3D graphics API with associated utility APIs designed to expose the features of 3D graphics hardware using a more object oriented design than OpenGL. It is used primarily by Clutter to provide a common rendering API that works transparently across OpenGL >=1.4, OpenGL ES 1.1 and OpenGL ES 2.0.
It is not tied to any one toolkit or even constrained to developing UI toolkits.

> If you want to use a more actively developed library with the same functionality, but with better performance and new functionality, then I advise you to use [UX-dx](https://github.com/angular-rust/ux-dx) as the successor to cogl-rs.

**Angular Rust** is a high productivity, `platform-agnostic` frontend framework for the [Rust language](https://www.rust-lang.org/). It now supports desktop and web development. Angular Rust currently uses GTK for desktop development and WebAssembly for web development. We are planning to add support for mobile development.

![Angular Rust structure](https://dudochkin-victor.github.io/assets/angular-rust/structure.svg)

## Quick Start

Install Cogl-rs:

	cargo add cogl-rs

For real example you need also glib, so 

	cargo add glib

**Here is a usage example**

```rust
use cogl::*;

fn main() {
    let triangle_vertices = vec![
        &VertexP2C4 {
            x: 0.0, y: 0.7, r: 0xFF, g: 0x00, b: 0x00, a: 0xFF,
        },
        &VertexP2C4 {
            x: -0.7, y: -0.7, r: 0x00, g: 0xFF, b: 0x00, a: 0xFF,
        },
        &VertexP2C4 {
            x: 0.7, y: -0.7, r: 0x00, g: 0x00, b: 0xFF, a: 0xFF,
        },
    ];

    match Context::new(None) {
        Ok(ctx) => {
            let onscreen = Onscreen::new(&ctx, 640, 480);

            onscreen.show();
            onscreen.set_resizable(true);

            let triangle = Primitive::new_p2c4(
                &ctx,
                cogl::VerticesMode::Triangles,
                triangle_vertices.as_slice(),
            );

            let pipeline = Pipeline::new(&ctx);
            let cogl_source = cogl::source_new(&ctx, glib::PRIORITY_DEFAULT);
            cogl_source.attach(None);

            onscreen.add_frame_callback(|_onscreen, _event, _info| {
            });

            onscreen.add_dirty_callback(move |onscreen, _info| {
                // buffers should be enum COGL_BUFFER_BIT_COLOR
                onscreen.clear4f(1, 0.0, 0.0, 0.0, 1.0);
                triangle.draw(onscreen, &pipeline);
                onscreen.swap_buffers();
            });

            let main_loop = glib::MainLoop::new(None, true);
            main_loop.run();
        }
        Err(err) => {
            println!("Failed to create context: {}", err);
        }
    }
}
```
## Learn More

* [Manual, Docs, etc](https://angular-rust.github.io/)
* [Samples](https://github.com/angular-rust/ux-samples)
* [Apps using Angular Rust](https://github.com/angular-rust/cogl-rs/wiki/Apps-in-the-Wild)
* [Articles Featuring Angular Rust](https://github.com/angular-rust/cogl-rs/wiki/Articles)

## Community

 [![](https://img.shields.io/badge/Facebook-1877F2?style=for-the-badge&logo=facebook&logoColor=white)](https://www.facebook.com/groups/angular.rust) 
 [![](https://img.shields.io/badge/Stack_Overflow-FE7A16?style=for-the-badge&logo=stack-overflow&logoColor=white)](https://stackoverflow.com/questions/tagged/angular-rust) 
 [![](https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white)](https://www.youtube.com/channel/UCBJTkSl_JWShuolUy4JksTQ) 
 [![](https://img.shields.io/badge/Medium-12100E?style=for-the-badge&logo=medium&logoColor=white)](https://medium.com/@angular.rust) 
 [![](https://img.shields.io/gitter/room/angular_rust/angular_rust?style=for-the-badge)](https://gitter.im/angular_rust/community)


## Contributing

We believe the wider community can create better code. The first tool for improving the community is to tell the developers about the project by giving it a star. More stars - more members.

 [![](https://dudochkin-victor.github.io/assets/star-me-wide.svg)](https://github.com/angular-rust/cogl-rs#top)
 
Angular Rust is a community effort and we welcome all kinds of contributions, big or small, from developers of all backgrounds. We want the Angular Rust community to be a fun and friendly place, so please review our [Code of Conduct](CODE_OF_CONDUCT.md) to learn what behavior will not be tolerated.

### New to Angular Rust?

Start learning about the framework by helping us improve our [documentation](https://angular-rust.github.io/). Pull requests which improve test coverage are also very welcome.

### Looking for inspiration?

Check out the community curated list of awesome things related to Angular Rust / WebAssembly at [awesome-angular-rust](https://github.com/angular-rust/awesome-angular-rust).

### Confused about something?

Feel free to drop into our [Gitter chatroom](https://gitter.im/angular_rust/community) or open a [new "Question" issue](https://github.com/angular-rust/cogl-rs/issues/new/choose) to get help from contributors. Often questions lead to improvements to the ergonomics of the framework, better documentation, and even new features!

### Ready to dive into the code?

After reviewing the [Contributing Code Guidelines](CONTRIBUTING.md), check out the ["Good First Issues"](https://github.com/angular-rust/cogl-rs/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) (they are eager for attention!). Once you find one that interests you, feel free to assign yourself to an issue and don't hesitate to reach out for guidance, the issues vary in complexity.

### Let's help each other!

Come help us on the [issues that matter that the most](https://github.com/angular-rust/cogl-rs/labels/%3Adollar%3A%20Funded%20on%20Issuehunt) and receive a small cash reward for your troubles. We use [Issuehunt](https://issuehunt.io/r/angular-rust/cogl-rs/) to fund issues from our Open Collective funds. If you really care about an issue, you can choose to add funds yourself! 

### Found a bug?

Please [report all bugs!](https://github.com/angular-rust/cogl-rs/issues/new/choose) We are happy to help support developers fix the bugs they find if they are interested and have the time.

## Todo
- [ ] Documentation
