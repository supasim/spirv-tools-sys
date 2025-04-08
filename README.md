# Usage

I don't specify a minimum spirv tools version. Anything somewhat recent should work fine.

When using, you will have to specify the path of the libraries and the headers. When setting the `VULKAN_SDK` environment variable, these are automatically found.
When using this, it will search for headers in `$VULKAN_SDK/include/spirv-tools` subdirectory and libraries in `$VULKAN_SDK/lib` subdirectory.
You can also set the `SPIRV_TOOLS` environment variable.
When using this, it will search for headers in the `$SPIRV_TOOLS/include` subdirectory and libraries in the `$SPIRV_TOOLS/lib` subdirectory.
Finally, you can set the `SPIRV_TOOLS_HEADERS_DIR` and `SPIRV_TOOLS_LIBS_DIR` environment variables, which do as you might guess.