

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Enumeration

/// Shader location index
pub enum ShaderLocationIndex {
	ShaderLocVertexPosition = 0, // Shader location: vertex attribute: position
	ShaderLocVertexTexcoord01,   // Shader location: vertex attribute: texcoord01
	ShaderLocVertexTexcoord02,   // Shader location: vertex attribute: texcoord02
	ShaderLocVertexNormal,       // Shader location: vertex attribute: normal
	ShaderLocVertexTangent,      // Shader location: vertex attribute: tangent
	ShaderLocVertexColor,        // Shader location: vertex attribute: color
	ShaderLocMatrixMvp,          // Shader location: matrix uniform: model-view-projection
	ShaderLocMatrixView,         // Shader location: matrix uniform: view (camera transform)
	ShaderLocMatrixProjection,   // Shader location: matrix uniform: projection
	ShaderLocMatrixModel,        // Shader location: matrix uniform: model (transform)
	ShaderLocMatrixNormal,       // Shader location: matrix uniform: normal
	ShaderLocVectorView,         // Shader location: vector uniform: view
	ShaderLocColorDiffuse,       // Shader location: vector uniform: diffuse color
	ShaderLocColorSpecular,      // Shader location: vector uniform: specular color
	ShaderLocColorAmbient,       // Shader location: vector uniform: ambient color
	ShaderLocMapAlbedo,          // Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
	ShaderLocMapMetalness,       // Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
	ShaderLocMapNormal,          // Shader location: sampler2d texture: normal
	ShaderLocMapRoughness,       // Shader location: sampler2d texture: roughness
	ShaderLocMapOcclusion,       // Shader location: sampler2d texture: occlusion
	ShaderLocMapEmission,        // Shader location: sampler2d texture: emission
	ShaderLocMapHeight,          // Shader location: sampler2d texture: height
	ShaderLocMapCubemap,         // Shader location: samplerCube texture: cubemap
	ShaderLocMapIrradiance,      // Shader location: samplerCube texture: irradiance
	ShaderLocMapPrefilter,       // Shader location: samplerCube texture: prefilter
	ShaderLocMapBrdf             // Shader location: sampler2d texture: brdf
}

/// Shader uniform data type
pub enum ShaderUniformDataType {
	ShaderUniformFloat = 0,       // Shader uniform type: float
	ShaderUniformVec2,            // Shader uniform type: vec2 (2 float)
	ShaderUniformVec3,            // Shader uniform type: vec3 (3 float)
	ShaderUniformVec4,            // Shader uniform type: vec4 (4 float)
	ShaderUniformInt,             // Shader uniform type: int
	ShaderUniformIvec2,           // Shader uniform type: ivec2 (2 int)
	ShaderUniformIvec3,           // Shader uniform type: ivec3 (3 int)
	ShaderUniformIvec4,           // Shader uniform type: ivec4 (4 int)
	ShaderUniformSampler2d        // Shader uniform type: sampler2d
}

/// Pixel formats
#[derive(Copy, Clone)]
pub enum PixelFormat {
	Unknown = 0,
	UncompressedGrayscale = 1,		// 8 bit per pixel (no alpha)
    UncompressedGrayAlpha = 2,		// 8*2 bpp (2 channels)
    UncompressedR5g6b5 = 3,			// 16 bpp
    UncompressedR8g8b8 = 4,			// 24 bpp
    UncompressedR5g5b5a1 = 5,		// 16 bpp (1 bit alpha)
    UncompressedR4g4b4a4 = 6,		// 16 bpp (4 bit alpha)
    UncompressedR8g8b8a8 = 7,		// 32 bpp
    UncompressedR32 = 8,			// 32 bpp (1 channel - float)
    UncompressedR32g32b32 = 9,		// 32*3 bpp (3 channels - float)
    UncompressedR32g32b32a32 = 10,	// 32*4 bpp (4 channels - float)
    UncompressedR16 = 11,			// 16 bpp (1 channel - half float)
    UncompressedR16g16b16 = 12,		// 16*3 bpp (3 channels - half float)
    UncompressedR16g16b16a16 = 13,	// 16*4 bpp (4 channels - half float)
    CompressedDxt1Rgb = 14,			// 4 bpp (no alpha)
    CompressedDxt1Rgba = 15,		// 4 bpp (1 bit alpha)
    CompressedDxt3Rgba = 16,		// 8 bpp
    CompressedDxt5Rgba = 17,		// 8 bpp
    CompressedEtc1Rgb = 18,			// 4 bpp
    CompressedEtc2Rgb = 19,			// 4 bpp
    CompressedEtc2EacRgba = 20,		// 8 bpp
    CompressedPvrtRgb = 21,			// 4 bpp
    CompressedPvrtRgba = 22,		// 4 bpp
    CompressedAstc4x4Rgba = 23,		// 8 bpp
    CompressedAstc8x8Rgba = 24,		// 2 bpp
}

/// Material map index
pub enum MaterialMapIndex {
	ALBEDO = 0,		// Albedo material (same as: MATERIAL_MAP_DIFFUSE)
	METALNESS,		// Metalness material (same as: MATERIAL_MAP_SPECULAR)
	NORMAL,			// Normal material
	ROUGHNESS,		// Roughness material
	OCCLUSION,		// Ambient occlusion material
	EMISSION,		// Emission material
	HEIGHT,			// Heightmap material
	CUBEMAP,		// Cubemap material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	IRRADIANCE,		// Irradiance material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	PREFILTER,		// Prefilter material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	BRDF,			// Brdf material
}


/// Framebuffer attachment type
pub enum RlFramebufferAttachType {
	RlAttachmentColorChannel0 = 0,	// Framebuffer attachment type: color 0
	RlAttachmentColorChannel1 = 1,	// Framebuffer attachment type: color 1
	RlAttachmentColorChannel2 = 2,	// Framebuffer attachment type: color 2
	RlAttachmentColorChannel3 = 3,	// Framebuffer attachment type: color 3
	RlAttachmentColorChannel4 = 4,	// Framebuffer attachment type: color 4
	RlAttachmentColorChannel5 = 5,	// Framebuffer attachment type: color 5
	RlAttachmentColorChannel6 = 6,	// Framebuffer attachment type: color 6
	RlAttachmentColorChannel7 = 7,	// Framebuffer attachment type: color 7
	RlAttachmentDepth = 100,		// Framebuffer attachment type: depth
	RlAttachmentStencil = 200,		// Framebuffer attachment type: stencil
}

/// Framebuffer texture attachment type
pub enum RlFramebufferAttachTextureType {
	RlAttachmentCubemapPositiveX = 0,   // Framebuffer texture attachment type: cubemap, +X side
    RlAttachmentCubemapNegativeX = 1,   // Framebuffer texture attachment type: cubemap, -X side
    RlAttachmentCubemapPositiveY = 2,   // Framebuffer texture attachment type: cubemap, +Y side
    RlAttachmentCubemapNegativeY = 3,   // Framebuffer texture attachment type: cubemap, -Y side
    RlAttachmentCubemapPositiveZ = 4,   // Framebuffer texture attachment type: cubemap, +Z side
    RlAttachmentCubemapNegativeZ = 5,   // Framebuffer texture attachment type: cubemap, -Z side
    RlAttachmentTexture2d = 100,          // Framebuffer texture attachment type: texture2d
    RlAttachmentRenderbuffer = 200,       // Framebuffer texture attachment type: renderbuffer
}

/// Keyboard keys (US keyboard layout)
pub enum KeyboardKey {
	KeyNull		= 0,		// Key: NULL, used for no key pressed
	// Alphanumeric keys
	APOSTROPHE	= 39,		// Key: '
	Comma		= 44,		// Key: ,
	Minus		= 45,		// Key: -
	Period		= 46,		// Key: .
	Slash		= 47,		// Key: /
	Zero		= 48,		// Key: 0
	One			= 49,		// Key: 1
	Two			= 50,		// Key: 2
	Three		= 51,		// Key: 3
	Four		= 52,		// Key: 4
	Five		= 53,		// Key: 5
	Six			= 54,		// Key: 6
	Seven		= 55,		// Key: 7
	Eight		= 56,		// Key: 8
	Nine		= 57,		// Key: 9
	Semicolon	= 59,		// Key: ;
	Equal		= 61,		// Key: =
	A			= 65,		// Key: A | a
	B			= 66,		// Key: B | b
	C			= 67,		// Key: C | c
	D			= 68,		// Key: D | d
	E			= 69,		// Key: E | e
	F			= 70,		// Key: F | f
	G			= 71,		// Key: G | g
	H			= 72,		// Key: H | h
	I			= 73,		// Key: I | i
	J			= 74,		// Key: J | j
	K			= 75,		// Key: K | k
	L			= 76,		// Key: L | l
	M			= 77,		// Key: M | m
	N			= 78,		// Key: N | n
	O			= 79,		// Key: O | o
	P			= 80,		// Key: P | p
	Q			= 81,		// Key: Q | q
	R			= 82,		// Key: R | r
	S			= 83,		// Key: S | s
	T			= 84,		// Key: T | t
	U			= 85,		// Key: U | u
	V			= 86,		// Key: V | v
	W			= 87,		// Key: W | w
	X			= 88,		// Key: X | x
	Y			= 89,		// Key: Y | y
	Z			= 90,		// Key: Z | z
	LeftBracket	= 91,		// Key: [
	BACKSLASH	= 92,		// Key: '\'
	RightBracket	= 93,	// Key: ]
	GRAVE		= 96,		// Key: `
	// Function keys
	SPACE		= 32,		// Key: Space
	ESCAPE		= 256,		// Key: Esc
	ENTER		= 257,		// Key: Enter
	TAB			= 258,		// Key: Tab
	BACKSPACE	= 259,		// Key: Backspace
	INSERT		= 260,		// Key: Ins
	DELETE		= 261,		// Key: Del
	RIGHT		= 262,		// Key: Cursor right
	LEFT		= 263,		// Key: Cursor left
	DOWN		= 264,		// Key: Cursor down
	UP			= 265,		// Key: Cursor up
	PageUp		= 266,		// Key: Page up
	PageDown	= 267,		// Key: Page down
	HOME		= 268,		// Key: Home
	END			= 269,		// Key: End
	CapsLock	= 280,		// Key: Caps lock
	ScrollLock	= 281,		// Key: Scroll down
	NumLock		= 282,		// Key: Num lock
	PrintScreen	= 283,		// Key: Print screen
	PAUSE		= 284,		// Key: Pause
	F1			= 290,		// Key: F1
	F2			= 291,		// Key: F2
	F3			= 292,		// Key: F3
	F4			= 293,		// Key: F4
	F5			= 294,		// Key: F5
	F6			= 295,		// Key: F6
	F7			= 296,		// Key: F7
	F8			= 297,		// Key: F8
	F9			= 298,		// Key: F9
	F10			= 299,		// Key: F10
	F11			= 300,		// Key: F11
	F12			= 301,		// Key: F12
	LeftShift	= 340,		// Key: Shift left
	LeftControl	= 341,		// Key: Control left
	LeftAlt		= 342,		// Key: Alt left
	LeftSuper	= 343,		// Key: Super left
	RightShift	= 344,		// Key: Shift right
	RightControl	= 345,	// Key: Control right
	RightAlt	= 346,		// Key: Alt right
	RightSuper	= 347,		// Key: Super right
	KbMenu		= 348,		// Key: KB menu
	// Keypad keys
	Kp0			= 320,		// Key: Keypad 0
	Kp1			= 321,		// Key: Keypad 1
	Kp2			= 322,		// Key: Keypad 2
	Kp3			= 323,		// Key: Keypad 3
	Kp4			= 324,		// Key: Keypad 4
	Kp5			= 325,		// Key: Keypad 5
	Kp6			= 326,		// Key: Keypad 6
	Kp7			= 327,		// Key: Keypad 7
	Kp8			= 328,		// Key: Keypad 8
	Kp9			= 329,		// Key: Keypad 9
	KpDecimal	= 330,		// Key: Keypad .
	KpDivide	= 331,		// Key: Keypad /
	KpMultiply	= 332,		// Key: Keypad *
	KpSubtract	= 333,		// Key: Keypad -
	KpAdd		= 334,		// Key: Keypad +
	KpEnter		= 335,		// Key: Keypad Enter
	KpEqual		= 336,		// Key: Keypad =
	// Android key buttons
	//BACK		= 4,		// Key: Android back button
	//MENU		= 82,		// Key: Android menu button
	//VolumeUp	= 24,		// Key: Android volume up button
	//VolumeDown	= 25,	// Key: Android volume down button
}

/// Mouse buttons
pub enum MouseButton {
	Left	= 0,			// Mouse button left
	Right	= 1,			// Mouse button right
	Middle	= 2,			// Mouse button middle (pressed wheel)
	Side	= 3,			// Mouse button side (advanced mouse device)
	Extra	= 4,			// Mouse button extra (advanced mouse device)
	Forward	= 5,			// Mouse button fordward (advanced mouse device)
	Back	= 6,			// Mouse button back (advanced mouse device)
}

/// Gamepad buttons
pub enum GamepadButton {
	Unknown = 0,			// Unknown button, just for error checking
	LeftFaceUp,				// Gamepad left DPAD up button
	LeftFaceRight,			// Gamepad left DPAD right button
	LeftFaceDown,			// Gamepad left DPAD down button
	LeftFaceLeft,			// Gamepad left DPAD left button
	RightFaceUp,			// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
	RightFaceRight,			// Gamepad right button right (i.e. PS3: Square, Xbox: X)
	RightFaceDown,			// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
	RightFaceLeft,			// Gamepad right button left (i.e. PS3: Circle, Xbox: B)
	LeftTrigger1,			// Gamepad top/back trigger left (first), it could be a trailing button
	LeftTrigger2,			// Gamepad top/back trigger left (second), it could be a trailing button
	RightTrigger1,			// Gamepad top/back trigger right (one), it could be a trailing button
	RightTrigger2,			// Gamepad top/back trigger right (second), it could be a trailing button
	MiddleLeft,				// Gamepad center buttons, left one (i.e. PS3: Select)
	Middle,					// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
	MiddleRight,			// Gamepad center buttons, right one (i.e. PS3: Start)
	LeftThumb,				// Gamepad joystick pressed button left
	RightThumb,				// Gamepad joystick pressed button right
}

/// Gamepad axis
pub enum GamepadAxis {
	LeftX			= 0,	// Gamepad left stick X axis
	LeftY			= 1,	// Gamepad left stick Y axis
	RightX			= 2,	// Gamepad right stick X axis
	RightY			= 3,	// Gamepad right stick Y axis
	LeftTrigger		= 4,	// Gamepad back trigger left, pressure level: [1..-1]
	RightTrigger	= 5,	// Gamepad back trigger right, pressure level: [1..-1]
}


//= Procedures

impl PixelFormat {
	pub fn from_i32(value: i32) -> Self {
		match value {
			 1 => PixelFormat::UncompressedGrayscale,
			 2 => PixelFormat::UncompressedGrayAlpha,
			 3 => PixelFormat::UncompressedR5g6b5,
			 4 => PixelFormat::UncompressedR8g8b8,
			 5 => PixelFormat::UncompressedR5g5b5a1,
			 6 => PixelFormat::UncompressedR4g4b4a4,
			 7 => PixelFormat::UncompressedR8g8b8a8,
			 8 => PixelFormat::UncompressedR32,
			 9 => PixelFormat::UncompressedR32g32b32,
			10 => PixelFormat::UncompressedR32g32b32a32,
			11 => PixelFormat::UncompressedR16,
			12 => PixelFormat::UncompressedR16g16b16,
			13 => PixelFormat::UncompressedR16g16b16a16,
			14 => PixelFormat::CompressedDxt1Rgb,
			15 => PixelFormat::CompressedDxt1Rgba,
			16 => PixelFormat::CompressedDxt3Rgba,
			17 => PixelFormat::CompressedDxt5Rgba,
			18 => PixelFormat::CompressedEtc1Rgb,
			19 => PixelFormat::CompressedEtc2Rgb,
			20 => PixelFormat::CompressedEtc2EacRgba,
			21 => PixelFormat::CompressedPvrtRgb,
			22 => PixelFormat::CompressedPvrtRgba,
			23 => PixelFormat::CompressedAstc4x4Rgba,
			24 => PixelFormat::CompressedAstc8x8Rgba,
			 _ => PixelFormat::Unknown,
		}
	}
}