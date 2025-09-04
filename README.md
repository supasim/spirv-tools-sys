# Usage

I don't specify a minimum spirv tools version. Anything somewhat recent should work fine.

When using, you will have to specify the path of the libraries and the headers. When setting the `VULKAN_SDK` environment variable, these are automatically found.
When using this, it will search for headers in `$VULKAN_SDK/include/spirv-tools` subdirectory and libraries in `$VULKAN_SDK/lib` subdirectory.
You can also set the `SPIRV_TOOLS_DIR` environment variable.
When using this, it will search for headers in the `$SPIRV_TOOLS/include` subdirectory and libraries in the `$SPIRV_TOOLS/lib` subdirectory.
Directory names are capitalized on windows.
Finally, you can set the `SPIRV_TOOLS_HEADERS_DIR` and `SPIRV_TOOLS_LIBS_DIR` environment variables, which do as you might guess.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.