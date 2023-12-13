use glow::{
	NativeBuffer, NativeFramebuffer, NativeProgram, NativeRenderbuffer, NativeTexture,
	NativeVertexArray,
};

pub(crate) enum UnusedGraphicsResource {
	VertexArray(NativeVertexArray),
	Buffer(NativeBuffer),
	Framebuffer(NativeFramebuffer),
	Renderbuffer(NativeRenderbuffer),
	Texture(NativeTexture),
	Program(NativeProgram),
}
