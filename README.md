# GLFW-Native
GLFW-Native is a glfw-wrapper for rust including the glfw3native.h header.
This allows you to easily use glfw with vulkan middleware since all the middleware support native vulkan surfaces.
This means that you can use glfw's abstractions to write crossplatform code.
And then when platform specific handles are needed they can easily be used.
